import chalk from 'chalk';
import * as Joi from 'joi';

// tslint:disable-next-line
const pascalCase = require('pascal-case');
import { EventMessage } from '../event/EventMessage';
import { Injector } from '../injector/Injector';
import { InternalLogger } from '../logger/InternalLogger';
import { Logger } from '../logger/Logger';
import { CommandHandlerOptions, commandHandlerOptionsSchema } from './CommandHandlerOptions';
import { CommandMessage } from './CommandMessage';
import { ICommandHandler } from './ICommandHandler';



/**
 *
 * @public
 */
export function CommandHandler(options: CommandHandlerOptions) {
  Joi.assert(options, commandHandlerOptionsSchema);
  return <T extends {new(...args: any[]): {}}>(Class: T) => {
    return class extends Class {
      public static Command = options.command;
      public static _InstantiateCommandHandler(parentInjector: Injector): ICommandHandler<any, any> {
        const injector = parentInjector.newChildInjector();
        injector.set({
          provide: Logger,
          useConstant: new InternalLogger(chalk.blue(`${pascalCase(options.command)}CommandHandler`))
        });
        return new this(injector);
      }

      public readonly command = options.command;
      public handle: (message: CommandMessage<any>, state: any, version: number) => Promise<EventMessage[]>;

      constructor(...args: any[]) {
        super(...args[0].args(Class));
      }
    };
  };
}
