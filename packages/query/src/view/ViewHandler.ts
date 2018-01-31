import { Injector, InternalLogger, Logger } from '@eventific/core';
import chalk from 'chalk';
import * as Joi from 'joi';
import pascalCase = require('pascal-case');
import { IViewHandler } from './IViewHandler';
import { ViewHandlerOptions } from './ViewHandlerOptions';


export function ViewHandler(options: ViewHandlerOptions) {
  Joi.assert(options, Joi.object());
  return <T extends {new(...args: any[]): {}}>(Class: T) => {
    return class extends Class {
      public static Name = options.name;
      public static _InstantiateViewHandler(parentInjector: Injector): IViewHandler {
        const injector = parentInjector.newChildInjector();
        injector.set({
          provide: Logger,
          useConstant: new InternalLogger(chalk.blue(`${pascalCase(options.name)}CommandHandler`))
        });
        return new this(injector);
      }

      public readonly name = options.name;
      public buildAndPersistView: (aggregateId: string, state: any, version: number) => Promise<void>;
      public start: () => Promise<string>;
      constructor(...args: any[]) {
        super(...args[0].args(Class));
      }
    };
  };
}
