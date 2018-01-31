import chalk from 'chalk';

// tslint:disable-next-line
const pascalCase = require('pascal-case');
import { Injector } from '../injector/Injector';
import { InternalLogger } from '../logger/InternalLogger';
import { Logger } from '../logger/Logger';
import { ITransport } from './ITransport';
import { TransportOptions } from './TransportOptions';


export function Transport(options: TransportOptions) {
  return <T extends {new(...args: any[]): {}}>(Class: T): T => {
    return class ExtendedTransport extends Class {
      public static Name = options.name;

      public static _CreateTransport(parentInjector: Injector) {
        const injector = parentInjector.newChildInjector();
        injector.set({
          provide: Logger,
          useConstant: new InternalLogger(chalk.blue(`${pascalCase(options.name)}Transport`))
        });
        return new ExtendedTransport(...injector.args(Class));
      }

      public static Settings(settings: object): { _CreateTransport: (injector: Injector) => ITransport } {
        return {
          _CreateTransport(parentInjector: Injector) {
            const injector = parentInjector.newChildInjector();
            injector.set({
              provide: Logger,
              useConstant: new InternalLogger(chalk.blue(`${pascalCase(options.name)}Transport`))
            });
            return new ExtendedTransport(...injector.args(Class, settings));
          }
        };
      }

      public name = options.name;
      public start: () => Promise<void>;
    };
  };
}
