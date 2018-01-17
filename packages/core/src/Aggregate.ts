import * as Joi from 'joi';

import { BaseEvent } from './Event';
import { BaseCommand } from './Command';
import { CommandMessage, commandMessageSchema } from './CommandMessage';

export interface AggregateOptions {
  name: string,
  events: { new(...args: any[]): BaseEvent; Name: string; }[],
  commands: { new(...args: any[]): BaseCommand; Name: string; }[],
  services: any[]
}

export interface IAggregate {
  /**
   * Internal use
   * @param {CommandMessage} commandMessage
   * @returns {Promise<void>}
   * @private
   */
  getCommand(commandMessage: CommandMessage): Promise<BaseCommand>
  getState(aggregateId: string): Promise<{ version: number, state: any }>
}

export function Aggregate(options: AggregateOptions) {
  return <T extends {new(...args: any[]): {}}>(constructor: T): T & {new(...args: any[]): IAggregate} => {
    return class extends constructor {
      static getEvents: (aggregateId: string) => {snapshot: any, events: any[]};
      static Name = options.name;
      name = options.name;
      _commands = new Map(options.commands.map<[string, { new(...args: any[]): BaseCommand; Name: string; }]>(cmd => [cmd.Name, cmd]));
      _events = new Map(options.events.map<[string, { new(...args: any[]): BaseEvent; Name: string; }]>(ev => [ev.Name, ev]));

      async getCommand(commandMessage: CommandMessage): Promise<BaseCommand> {
        const validatedCommandMessage = await this._validateCommand(commandMessage);
        const commandType = this._commands.get(validatedCommandMessage.command);
        if(commandType) {
          return new commandType(validatedCommandMessage);
        } else {
          throw Error('No type for the command: ' + validatedCommandMessage.command);
        }
      }

      async getState(aggregateId: string): Promise<{version: number, state: any}> {
        return {version: -1, state: {}}
      }

      async _validateCommand(cmd: CommandMessage): Promise<CommandMessage> {
        return new Promise<CommandMessage>((resolve, reject) => {
          Joi.validate(cmd, commandMessageSchema, {}, (error, command: CommandMessage) => {
            if(error) {
              reject(error);
            } else {
              resolve(command);
            }
          })
        });
      }
    };
  };
}