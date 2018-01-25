"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
require("reflect-metadata");
const assert = require("assert");
class Injector {
    constructor(parent) {
        this._dependencies = new Map();
        this._parent = parent;
    }
    set(dependency) {
        if (dependency.useClass) {
            assert(isClass(dependency.useClass), 'The provided class has to actually be a class');
            if (dependency.provide) {
                const key = this._getProvideKey(dependency.provide);
                this._dependencies.set(key, dependency);
            }
            else {
                this._dependencies.set(dependency.useClass.name, dependency);
            }
        }
        else if (dependency.useConstant) {
            const key = this._getProvideKey(dependency.provide);
            this._dependencies.set(key, dependency);
        }
        else {
            assert(isClass(dependency), 'The provided class has to actually be a class');
            this._dependencies.set(dependency.name, { useClass: dependency });
        }
    }
    get(type) {
        const result = this.getOptional(type);
        if (result) {
            return result;
        }
        else {
            throw new Error('InjectionError: No provider for type: ' + type.toString());
        }
    }
    getOptional(type) {
        const key = this._getProvideKey(type);
        let result = this._dependencies.get(key);
        if (!result && this._parent) {
            return this._parent.getOptional(type);
        }
        else if (result) {
            if (result.useClass) {
                return new result.useClass(...this.args(result.useClass));
            }
            else {
                return result.useConstant;
            }
        }
        else {
            return undefined;
        }
    }
    args(type) {
        const types = this._getTypes(type);
        const args = [];
        types.forEach((param) => {
            if (param.required) {
                args.push(this.get(param.type));
            }
            else {
                args.push(this.getOptional(param.type));
            }
        });
        return args;
    }
    _getTypes(type) {
        assert.notEqual(type, undefined);
        const definedTypes = Reflect.getMetadata('injector:params', type) || [];
        const params = Reflect.getMetadata("design:paramtypes", type) || [];
        const types = new Array(params.length);
        params.forEach((param, index) => {
            types[index] = {
                required: true,
                type: param.name
            };
        });
        definedTypes.forEach((def) => {
            if (def.index >= params.length) {
                throw new Error('InjectionError: injector:params has defined a param that has a greater index than the total amount of params');
            }
            types[def.index] = {
                required: def.required,
                type: def.type
            };
        });
        types.forEach((param, index) => {
            if (param.type === 'Number' ||
                param.type === 'String' ||
                param.type === 'Boolean' ||
                param.type === 'Object' ||
                param.type === 'Array' ||
                param.type === 'Function' ||
                param.type === 'Object') {
                throw new Error(`InjectionError: param at index: ${index} on type ${type.name} is of a basic type and does not have a @Inject annotation`);
            }
        });
        return types;
    }
    _getProvideKey(provide) {
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
            default: {
                throw new Error(`InjectionError: ${type} are not a supported provide type`);
            }
        }
    }
    newChildInjector() {
        return new Injector(this);
    }
}
exports.Injector = Injector;
function isClass(v) {
    // return typeof v === 'function' && /^\s*class\s+/.test(v.toString());
    return typeof v === 'function';
}
//# sourceMappingURL=Injector.js.map