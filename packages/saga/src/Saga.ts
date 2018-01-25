import {
  CommandMessage,
  commandMessageSchema,
  EventMessage,
  eventMessageSchema,
  IAggregate,
  Injector,
  InternalLogger,
  IStore,
  ITransport,
  Logger,
  Store
} from '@eventific/core';
import chalk from 'chalk';
import * as Joi from 'joi';
import pascalCase = require('pascal-case');
import { Context } from './Context';
import { SagaOptions, sagaOptionsSchema } from './SagaOptions';


export function Saga(options: SagaOptions) {
  Joi.assert(options, sagaOptionsSchema);
  return <T extends {new(...args: any[]): {}}>(Class: T) => {
    return class extends Class {
      public static Type = 'Saga';
      public static _Instantiate(parentInjector: Injector): T {
        const injector = parentInjector.newChildInjector();
        const store = options.store._CreateStore(injector);
        injector.set({provide: Store, useConstant: store});
        injector.set({provide: Logger, useConstant: new InternalLogger(chalk.green(pascalCase('Saga')))});

        return new this({
          aggregates: options.aggregates.map( (a) => a._InstantiateAggregate(injector)),
          injector,
          transport: options.transport._CreateTransport(injector)
        }) as any;
      }

      public _injector: Injector;
      public _store: IStore;
      public _transport: ITransport;
      public _aggregates: IAggregate[];
      public _logger: Logger;

      constructor(...args: any[]) {
        super(...args[0].injector.args(Class));
        this._injector = args[0].injector;
        this._store = this._injector.get<IStore>(Store);
        this._logger = this._injector.get<Logger>(Logger);
        this._transport = args[0].transport;
        this._aggregates = args[0].aggregates;
      }

      public async sendCommand(message: CommandMessage): Promise<void> {
        Joi.assert(message, commandMessageSchema); // TODO: This should be verified with the handler instead
        if (this._transport.sendCommand) {
          await this._transport.sendCommand(message);
        } else {
          throw new Error('Transport does not support sending commands');
        }
      }


      public async _start(): Promise<void> {
        await this._store.start();
        await this._transport.start();
        await this._startTriggers();
      }

      public async _startTriggers(): Promise<void> {
        const triggerDefs = (this as any)._triggerDefinitions || [];
        for (const def of triggerDefs) {
          for (const trigger of def.triggers) {
            if (trigger.Type === 'Aggregate') {
              const aggregate = this._aggregates.find((a) => a.name === trigger.Name);
              if (!aggregate) {
                throw new Error(`You have to add the triggering aggregate "${trigger.Name}" to the saga`);
              }
              this._logger.verbose(
                `Registering trigger for all events on aggregate ${
                  chalk.bgYellowBright(trigger.Name)
                } for method ${
                  chalk.bgYellowBright(def.propertyKey)
                }`
              );
              this._store.onEvent(trigger.Name, '', async (event: EventMessage) => {
                try {
                  Joi.assert(event, eventMessageSchema);
                  this._logger.info(`Method "${def.propertyKey}" triggered by event "${event.event}"`);
                  const stateResult = await aggregate.getState(event.aggregateId);
                  await (this as any)[def.propertyKey]({
                    aggregateId: event.aggregateId,
                    aggregateName: aggregate.name,
                    state: stateResult.state,
                    trigger: event,
                    version: stateResult.version
                  } as Context<any>);
                } catch (ex) {
                  this._logger.error('Error occurred', ex);
                }
              });
            } else if (trigger.Type === 'Event') {
              const aggregate = this._aggregates.find((a) => a.getEventNames().includes(trigger.Event));
              if (aggregate) {
                this._logger.verbose(
                  `Registering trigger for event ${
                    chalk.bgYellowBright(trigger.Event)
                  } on aggregate ${
                    chalk.bgYellowBright(aggregate.name)
                  } for method ${
                    chalk.bgYellowBright(def.propertyKey)
                  }`);

                this._store.onEvent(trigger.Name, trigger.Event, async (event: EventMessage) => {
                  try {
                    Joi.assert(event, eventMessageSchema);
                    this._logger.info(`Method "${def.propertyKey}" triggered by event "${event.event}"`);
                    const stateResult = await aggregate.getState(event.aggregateId);
                    await (this as any)[def.propertyKey]({
                      aggregateId: event.aggregateId,
                      aggregateName: aggregate.name,
                      state: stateResult.state,
                      trigger: event,
                      version: stateResult.version
                    } as Context<any>);
                  } catch (ex) {
                    this._logger.error('Error occurred', ex);
                  }
                });
              } else {
                throw new Error(`This event "${trigger.Event}" does not belong to any aggregate known to this saga`);
              }
            } else {
              throw new Error('Unknown trigger type');
            }
          }
        }
      }

    };
  };
}
