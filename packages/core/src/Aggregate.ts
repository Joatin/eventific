import * as Joi from 'joi';

import { BaseCommand } from './Command';
import { CommandMessage, commandMessageSchema } from './CommandMessage';
import { BaseEvent } from './Event';

export interface AggregateOptions {
  name: string;
  events: Array<{ new(...args: any[]): BaseEvent; Name: string; }>;
  commands: Array<{ new(...args: any[]): BaseCommand; Name: string; }>;
  services: any[];
}

/**
 * Represents a aggregate instance
 *
 * @since 1.0.0
 */
export interface IAggregate {

  /**
   * The name of this aggregate
   *
   * @since 1.0.0
   */
  readonly name: string;

  /**
   * Returns a command based on the provided command message
   *
   * @since 1.0.0
   *
   * @param {CommandMessage} commandMessage The command message to convert to a command instance
   * @returns {Promise<BaseCommand>} A new command instance
   */
  getCommand(commandMessage: CommandMessage): Promise<BaseCommand>;

  /**
   * Gets the current state
   * @param {string} aggregateId
   * @returns {Promise<{version: number; state: any}>}
   */
  getState(aggregateId: string): Promise<{ version: number, state: any }>;
}

export function Aggregate(options: AggregateOptions) {
  return <T extends {new(...args: any[]): {}}>(constructor: T): T & {new(...args: any[]): IAggregate} => {
    return class extends constructor {
      public static getEvents: (aggregateId: string) => {snapshot: any, events: any[]};
      public static Name = options.name;

      static _InstantiateAggregate(): IAggregate {
        return new this();
      }

      public name = options.name;
      public _commands = new Map(options.commands.map<[string, { new(...args: any[]): BaseCommand; Name: string; }]>((cmd) => [cmd.Name, cmd]));
      public _events = new Map(options.events.map<[string, { new(...args: any[]): BaseEvent; Name: string; }]>((ev) => [ev.Name, ev]));

      public async getCommand(commandMessage: CommandMessage): Promise<BaseCommand> {
        const validatedCommandMessage = await this._validateCommand(commandMessage);
        const commandType = this._commands.get(validatedCommandMessage.command);
        if (commandType) {
          return new commandType(validatedCommandMessage);
        } else {
          throw Error('No type for the command: ' + validatedCommandMessage.command);
        }
      }

      public async getState(aggregateId: string): Promise<{version: number, state: any}> {
        return {version: -1, state: {}};
      }

      public async _validateCommand(cmd: CommandMessage): Promise<CommandMessage> {
        return new Promise<CommandMessage>((resolve, reject) => {
          Joi.validate(cmd, commandMessageSchema, {}, (error, command: CommandMessage) => {
            if (error) {
              reject(error);
            } else {
              resolve(command);
            }
          });
        });
      }
    };
  };
}
