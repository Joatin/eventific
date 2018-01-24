import { Bootstrapable, Injector, IStore, IAggregate, InternalLogger, Logger, Store, ITransport, EventMessage, eventMessageSchema, CommandMessage, commandMessageSchema} from '@eventific/core';
import * as Joi from 'joi';
import { Context } from './Context';
import chalk from 'chalk';

const pascalCase = require('pascal-case');

export abstract class ISaga extends Bootstrapable {
  _triggerDefinitions: {
    triggers: any[],
    propertyKey: string;
  }[];
  sendCommand: (message: CommandMessage) => Promise<void>
}

export interface SagaOptions {
  aggregates: Array<{
    _InstantiateAggregate(injector: Injector): IAggregate;
  }>;
  store: {
    _CreateStore(injector: Injector): IStore
  };
  transport: {
    _CreateTransport(injector: Injector): ITransport
  };
  providers?: any[];
}

const sagaOptionsSchema = Joi.object().keys({
  aggregates: Joi.array().min(1).required(),
  store: Joi.any().required(),
  transport: Joi.any().optional()
});

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
          injector,
          transport: options.transport._CreateTransport(injector),
          aggregates: options.aggregates.map( a => a._InstantiateAggregate(injector))
        }) as any;
      }

      _injector: Injector;
      _store: IStore;
      _transport: ITransport;
      _aggregates: IAggregate[];
      _logger: Logger;

      constructor(...args: any[]) {
        super(args[0].injector.args(Class));
        this._injector = args[0].injector;
        this._store = this._injector.get<IStore>(Store);
        this._logger = this._injector.get<Logger>(Logger);
        this._transport = args[0].transport;
        this._aggregates = args[0].aggregates;
      }

      async sendCommand(message: CommandMessage): Promise<void> {
        Joi.assert(message, commandMessageSchema); // TODO: This should be verified with the handler instead
        if(this._transport.sendCommand) {
          await this._transport.sendCommand(message);
        } else {
          throw new Error('Transport does not support sending commands');
        }
      }


      async _start(): Promise<void> {
        await this._store.start();
        await this._transport.start();
        await this._startTriggers();
      }

      async _startTriggers(): Promise<void> {
        const triggerDefs = (this as any)._triggerDefinitions || [];
        for(const def of triggerDefs) {
          for(const trigger of def.triggers) {
            if(trigger.Type === 'Aggregate') {
              const aggregate = this._aggregates.find(a => a.name === trigger.Name);
              if(!aggregate) {
                throw new Error(`You have to add the triggering aggregate "${trigger.Name}" to the saga`);
              }
              this._logger.verbose(`Registering trigger for all events on aggregate ${chalk.bgYellowBright(trigger.Name)} for method ${chalk.bgYellowBright(def.propertyKey)}`);
              this._store.onEvent(trigger.Name, '', async (event: EventMessage) => {
                try {
                  Joi.assert(event, eventMessageSchema);
                  this._logger.info(`Method "${def.propertyKey}" triggered by event "${event.event}"`);
                  const stateResult = await aggregate.getState(event.aggregateId);
                  await (this as any)[def.propertyKey](<Context<any>>{
                    aggregateName: aggregate.name,
                    aggregateId: event.aggregateId,
                    trigger: event,
                    version: stateResult.version,
                    state: stateResult.state
                  });
                } catch (ex) {
                  console.log(ex);
                }
              });
            } else if(trigger.Type === 'Event') {
              const aggregate = this._aggregates.find(a => a.getEventNames().includes(trigger.Event));
              if(aggregate) {
                this._logger.verbose(`Registering trigger for event ${chalk.bgYellowBright(trigger.Event)} on aggregate ${chalk.bgYellowBright(aggregate.name)} for method ${chalk.bgYellowBright(def.propertyKey)}`);
                this._store.onEvent(trigger.Name, trigger.Event, async (event: EventMessage) => {
                  try {
                    Joi.assert(event, eventMessageSchema);
                    this._logger.info(`Method "${def.propertyKey}" triggered by event "${event.event}"`);
                    const stateResult = await aggregate.getState(event.aggregateId);
                    await (this as any)[def.propertyKey](<Context<any>>{
                      aggregateName: aggregate.name,
                      aggregateId: event.aggregateId,
                      trigger: event,
                      version: stateResult.version,
                      state: stateResult.state
                    });
                  } catch (ex) {
                    console.log(ex);
                  }
                });
              }else {
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
