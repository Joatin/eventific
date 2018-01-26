import chalk from 'chalk';
import pascalCase = require('pascal-case');
import { Injector } from './Injector';
import { InternalLogger } from './InternalLogger';
import { ITransport } from './ITransport';
import { Logger } from './Logger';
import { TransportOptions } from './TransportOptions';


export function Transport(options: TransportOptions) {
  return <T extends {new(...args: any[]): {}}>(Class: T): T => {
    return class extends Class {
      public static Name = options.name;

      public static _CreateTransport(parentInjector: Injector) {
        const injector = parentInjector.newChildInjector();
        injector.set({
          provide: Logger,
          useConstant: new InternalLogger(chalk.blue(`${pascalCase(options.name)}Transport`))
        });
        return new this(...injector.args(Class));
      }

      public static Settings(settings: object): { _CreateTransport: (injector: Injector) => ITransport } {
        return {
          _CreateTransport(parentInjector: Injector) {
            const injector = parentInjector.newChildInjector();
            injector.set({
              provide: Logger,
              useConstant: new InternalLogger(chalk.blue(`${pascalCase(options.name)}Transport`))
            });
            return new this(...injector.args(Class, settings));
          }
        };
      }

      public name = options.name;
    };
  };
}
