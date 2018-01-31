import chalk from 'chalk';

// tslint:disable-next-line
const pascalCase = require('pascal-case');
import { Injector } from './Injector';
import { InternalLogger } from './InternalLogger';
import { IStore } from './IStore';
import { Logger } from './Logger';
import { StoreOptions } from './StoreOptions';


/**
 * Store decorator
 */
export function Store(options: StoreOptions) {
  return <T extends {new(...args: any[]): {}}>(Class: T): T => {
    return class extends Class {
      public static Name = options.name;

      public static Settings(settings: object): { _CreateStore: (injector: Injector) => IStore } {
        return {
          _CreateStore(parentInjector: Injector) {
            const injector = parentInjector.newChildInjector();
            injector.set({
              provide: Logger,
              useConstant: new InternalLogger(chalk.magenta(`${pascalCase(options.name)}Store`))
            });
            return new this(...injector.args(Class, settings));
          }
        };
      }

      public static _CreateStore(parentInjector: Injector) {
        const injector = parentInjector.newChildInjector();
        injector.set({
          provide: Logger,
          useConstant: new InternalLogger(chalk.magenta(`${pascalCase(options.name)}Store`))
        });
        return new this(...injector.args(Class));
      }

      public name = options.name;
    };
  };
}
