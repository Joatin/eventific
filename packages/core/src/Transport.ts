import { CommandMessage } from './CommandMessage';
import { options } from 'joi';
import { Injector } from './Injector';

export abstract class ITransport {
  static _CreateTransport: (injector: Injector) => ITransport;
  static Settings: (settings: object) => { _CreateTransport: (injector: Injector) => ITransport };
  abstract start(): Promise<void>;
  onCommand?(handler: (data: CommandMessage) => Promise<void>): void;
  sendCommand?(data: CommandMessage): Promise<void>;
}

export interface TransportOptions {
  name: string;
}

export function Transport(options: TransportOptions) {
  return <T extends {new(...args: any[]): {}}>(Class: T): T => {
    return class extends Class {
      static Name = options.name;
      name = options.name;

      static _CreateTransport(injector: Injector) {
        return new this(...injector.args(Class))
      }

      static Settings(settings: object): { _CreateTransport: (injector: Injector) => ITransport } {
        return {
          _CreateTransport(injector: Injector) {
            return new this(...injector.args(Class, settings))
          }
        }
      }
    }
  };
}
