import * as assert from 'assert';
import chalk from 'chalk';
import * as Joi from 'joi';
import { CommandMessage, commandMessageSchema } from '../command/CommandMessage';
import { ICommandHandler } from '../command/ICommandHandler';
import { EventMessage, eventMessageSchema } from '../event/EventMessage';
import { IEventHandler } from '../event/IEventHandler';
import { Injector } from '../injector/Injector';
import { InternalLogger } from '../logger/InternalLogger';
import { Logger } from '../logger/Logger';
import { IStore } from '../store/IStore';
import { Store } from '../store/Store';
import { AggregateOptions } from './AggregateOptions';
import { IAggregate } from './IAggregate';

// tslint:disable-next-line
const pascalCase = require('pascal-case');

/**
 *
 * @public
 */
export function Aggregate(options: AggregateOptions) {
  return <T extends {new(...args: any[]): {}}>(Class: T): T => {
    return class extends Class {
      public static Type = 'Aggregate';
      public static Name = options.name;

      public static _InstantiateAggregate(parentInjector: Injector): IAggregate {
        assert.ok(parentInjector);
        const injector = parentInjector.newChildInjector();
        injector.set({
          provide: Logger,
          useConstant: new InternalLogger(chalk.yellow(`${pascalCase(options.name)}Aggregate`))
        });
        return new this(injector);
      }

      public name = options.name;
      public _injector: Injector;
      public _commandHandlers: Map<string, ICommandHandler<any, any>>;
      public _eventHandlers: Map<string, IEventHandler<any, any>>;
      public _store: IStore;
      public _logger: Logger;

      constructor(...args: any[]) {
        super(...(args[0] as Injector).args(Class));

        this._injector = args[0];
        this._injector.set({provide: Aggregate, useConstant: this});

        this._commandHandlers = new Map(
          options.commandHandlers.map<[string, ICommandHandler<any, any>]>(
            (cmd) => [cmd.Command, cmd._InstantiateCommandHandler(this._injector)]
          )
        );
        this._eventHandlers = new Map(
          options.eventHandlers.map<[string, IEventHandler<any, any>]>(
            (cmd) => [cmd.Event, cmd._InstantiateEventHandler(this._injector)]
          )
        );

        this._logger = this._injector.get<Logger>(Logger);
        this._store = this._injector.get<IStore>(Store);

        this._logger.verbose(`Registered events:\n  - ${Array.from(this._eventHandlers.keys()).join(',\n  - ')}`);
        this._logger.verbose(`Registered commands:\n  - ${Array.from(this._commandHandlers.keys()).join(',\n  - ')}`);
      }

      public async handleCommand(commandMessage: CommandMessage): Promise<void> {
        const validatedCommandMessage = await this._validateCommand(commandMessage);
        const handler = this._commandHandlers.get(validatedCommandMessage.command);
        const stateResult = await this.getState(validatedCommandMessage.aggregateId);
        if (handler) {
          const events = await handler.handle(validatedCommandMessage, stateResult.state, stateResult.version);
          if (!events || events.length <= 0) {
            this._logger.error(
              `Command handler for command ${
                validatedCommandMessage.command
              } did not return any events. A command has to return at least one event!`
            );
            throw Error('Internal Server Error');
          }
          // TODO: retry insert to store
          await this.applyToState({state: stateResult.state, version: stateResult.version}, events);
          await this._store.applyEvents(this.name, events);
        } else {
          this._logger.error(`Received a unknown command "${validatedCommandMessage.command}"`);
          throw Error(`UnknownCommand: ${validatedCommandMessage.command}`);
        }
      }

      public async applyToState(stateDef: {version: number, state: any}, events: EventMessage[]) {
        const sortedEvents = events.sort((e1, e2) => e1.eventId - e2.eventId);
        let state: any = stateDef.state;
        let version = stateDef.version;
        for (const event of sortedEvents) {
          Joi.assert(event, eventMessageSchema);
          if ( state === null && event.eventId !== 0 ) {
            throw new Error('State can not be null if this is not the initial event');
          }
          if ( event.eventId !== version + 1 ) {
            throw new Error('Events are not applied in sequential order');
          }
          const handler = this._eventHandlers.get(event.event);
          if (handler) {
            state = {
              ...await handler._validateAndHandle(event, state)
            };
            version = event.eventId;
          } else {
            throw new Error(`Handler missing for event ${event.event}`);
          }
        }

        return {version, state};
      }

      public async getState(aggregateId: string): Promise<{version: number, state: any}> {
        const eventResult = await this._store.getEvents(this.name, aggregateId);
        let state: any = null;
        let version: number = -1;
        if (eventResult.snapshot) {
          state = eventResult.snapshot.state || state;
          version = eventResult.snapshot.version || version;
        }
        return await this.applyToState({state, version}, eventResult.events);
      }

      public getEventNames(): string[] {
        return Array.from<string>(this._eventHandlers.keys());
      }

      public getCommandNames(): string[] {
        return Array.from<string>(this._commandHandlers.keys());
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
