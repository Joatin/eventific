import * as assert from 'assert';
import 'reflect-metadata';

export const SettingSymbol = Symbol('SETTINGS');

export interface ClassDependencyDefinition {
  provide?: string | symbol | Function;
  useClass: Function;
  dynamic?: true;
}

export interface ValueDependencyDefinition {
  provide: string | symbol | Function;
  useConstant: any;
}

export class Injector {
  private _parent?: Injector;
  private _dependencies = new Map<string | symbol, ClassDependencyDefinition | ValueDependencyDefinition>();

  constructor(parent?: Injector) {
    this._parent = parent;
  }

  public set(dependency: ClassDependencyDefinition | ValueDependencyDefinition | { new(...args: any[]): {} }): void {
    if ((dependency as ClassDependencyDefinition).useClass) {
      assert(
        isClass((dependency as ClassDependencyDefinition).useClass),
        'The provided class has to actually be a class'
      );
      if ((dependency as ClassDependencyDefinition).provide) {
        const key = this._getProvideKey((dependency as ClassDependencyDefinition).provide);
        this._dependencies.set(key, (dependency as ClassDependencyDefinition));
      } else {
        this._dependencies.set(
          (dependency as ClassDependencyDefinition).useClass.name,
          (dependency as ClassDependencyDefinition)
        );
      }
    } else if ((dependency as ValueDependencyDefinition).useConstant) {
      const key = this._getProvideKey((dependency as ValueDependencyDefinition).provide);
      this._dependencies.set(key, (dependency as ValueDependencyDefinition));
    } else {
      assert(isClass((dependency as { new(...args: any[]): {} })), 'The provided class has to actually be a class');
      this._dependencies.set(
        (dependency as { new(...args: any[]): {} }).name,
        { useClass: (dependency as { new(...args: any[]): {} }) }
      );
    }
  }

  public get<T = any>(type: string | symbol | Function): T {
    const result = this.getOptional<T>(type);
    if (result) {
      return result;
    } else {
      throw new Error('InjectionError: No provider for type: ' + type.toString());
    }
  }

  public getOptional<T = any>(type: string | symbol | Function): T | undefined {
    const key = this._getProvideKey(type);
    const result = this._dependencies.get(key);
    if (!result && this._parent) {
      return this._parent.getOptional<T>(type);
    } else if (result) {
      if ((result as ClassDependencyDefinition).useClass) {
        return new ((result as ClassDependencyDefinition).useClass as any)
        (...this.args((result as ClassDependencyDefinition).useClass)) as T;
      } else {
        return (result as ValueDependencyDefinition).useConstant as any;
      }
    } else {
      return undefined;
    }
  }

  public args(type: Function, setting?: object): any[] {
    const types = this._getTypes(type);
    const args: any[] = [];
    types.forEach((param) => {
      if (param.type === SettingSymbol) {
        args.push(setting);
      } else {
        if (param.required) {
          args.push(this.get(param.type));
        } else {
          args.push(this.getOptional(param.type));
        }
      }

    });
    return args;
  }

  public newChildInjector(): Injector {
    return new Injector(this);
  }

  private _getTypes(type: Function) {
    assert.notEqual(type, undefined);
    const definedTypes = Reflect.getMetadata('injector:params', type) || [];
    const params: any[] = Reflect.getMetadata('design:paramtypes', type) || [];
    const types = new Array(params.length);
    params.forEach((param: Function, index: number) => {
      types[index] = {
        required: true,
        type: param.name
      };
    });
    definedTypes.forEach((def: any) => {
      if (def.index >= params.length) {
        throw new Error(
          'InjectionError: injector:params has defined a param that has a greater index than the total amount of params'
        );
      }
      types[def.index] = {
        required: def.required,
        type: def.type
      };
    });

    types.forEach((param, index) => {
      if (
        param.type === 'Number' ||
        param.type === 'String' ||
        param.type === 'Boolean' ||
        param.type === 'Object' ||
        param.type === 'Array' ||
        param.type === 'Function' ||
        param.type === 'Object'
      ) {
        throw new Error(
          `InjectionError: param at index: ${
            index
            } on type ${
            type.name
            } is of a basic type and does not have a @Inject annotation`
        );
      }
    });

    return types;
  }

  private _getProvideKey(provide: any) {
    const type = typeof provide;
    switch (type) {
      case 'string': {
        return provide;
      }
      case 'number': {
        throw new Error('InjectionError: Numbers are not a supported provide type');
      }
      case 'boolean': {
        throw new Error('InjectionError: Booleans are not a supported provide type');
      }
      case 'function': {
        return provide.name;
      }
      case 'symbol': {
        return provide;
      }
      default: {
        throw new Error(`InjectionError: ${type} are not a supported provide type`);
      }
    }
  }
}

function isClass(v: any) {
  // return typeof v === 'function' && /^\s*class\s+/.test(v.toString());
  return typeof v === 'function';
}
