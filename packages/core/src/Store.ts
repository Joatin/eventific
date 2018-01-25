import { Injector } from './Injector';
import { IStore } from './IStore';
import { StoreOptions } from './StoreOptions';


/**
 * Store decorator
 * @param {StoreOptions} options
 * @returns {<T extends {new(...args: any[]) => {}}>(Class: T) => T}
 * @constructor
 */
export function Store(options: StoreOptions) {
  return <T extends {new(...args: any[]): {}}>(Class: T): T => {
    return class extends Class {
      public static Name = options.name;

      public static Settings(settings: object): { _CreateStore: (injector: Injector) => IStore } {
        return {
          _CreateStore(injector: Injector) {
            return new this(...injector.args(Class, settings));
          }
        };
      }

      public static _CreateStore(injector: Injector) {
        return new this(...injector.args(Class));
      }

      public name = options.name;
    };
  };
}
