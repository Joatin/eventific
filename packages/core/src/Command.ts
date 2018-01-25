import { CommandMessage } from './CommandMessage';
import { EventMessage } from './EventMessage';
import { Injector } from './Injector';
import { Logger } from './Logger';
import { InternalLogger } from './InternalLogger';
import chalk from 'chalk';
import * as Joi from 'joi';

const pascalCase = require('pascal-case');

export abstract class ICommandHandler<T, R> {
  static _InstantiateCommandHandler: (injector: Injector) => ICommandHandler<any, any>;
  static Command: string;
  public readonly command: string;
  public abstract handle(message: CommandMessage<T>, state: R, version: number): Promise<EventMessage[]>;
}

export interface CommandHandlerOptions {
  command: string;
}

const commandHandlerOptionsSchema = Joi.object().keys({
  command: Joi.string().min(3).required()
});

/**
 *
 * @param {CommandHandlerOptions} options
 * @returns {<T extends {new(...args: any[]) => {}}>(Class: T) => {Command: any; _InstantiateCommandHandler: ((parentInjector: Injector) => ICommandHandler<any, any>); new(...args: any[]) => {command: string; handle: ((message: CommandMessage<any>, state: any, version: number) => Promise<EventMessage[]>)}}}
 * @constructor
 */
export function CommandHandler(options: CommandHandlerOptions) {
  Joi.assert(options, commandHandlerOptionsSchema);
  return <T extends {new(...args: any[]): {}}>(Class: T) => {
    return class extends Class {
      static Command = options.command;
      public readonly command = options.command;
      static _InstantiateCommandHandler(parentInjector: Injector): ICommandHandler<any, any> {
        const injector = parentInjector.newChildInjector();
        injector.set({provide: Logger, useConstant: new InternalLogger(chalk.bgGreen(`${pascalCase(options.command)}Handler`))});
        return new this(injector);
      }

      handle: (message: CommandMessage<any>, state: any, version: number) => Promise<EventMessage[]>;

      constructor(...args: any[]) {
        super(...args[0].args(Class));
      }
    };
  };
}
