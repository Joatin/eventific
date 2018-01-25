import { Injector } from './Injector';
import { ITransport } from './ITransport';
import { TransportOptions } from './TransportOptions';


export function Transport(options: TransportOptions) {
  return <T extends {new(...args: any[]): {}}>(Class: T): T => {
    return class extends Class {
      public static Name = options.name;

      public static _CreateTransport(injector: Injector) {
        return new this(...injector.args(Class));
      }

      public static Settings(settings: object): { _CreateTransport: (injector: Injector) => ITransport } {
        return {
          _CreateTransport(injector: Injector) {
            return new this(...injector.args(Class, settings));
          }
        };
      }

      public name = options.name;
    };
  };
}
