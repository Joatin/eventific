import { options, Schema } from 'joi';
import { EventMessage, eventMessageSchema } from './EventMessage';
import { Injector } from './Injector';
import * as Joi from 'joi';
import { IAggregate } from './Aggregate';
import { Logger } from './Logger';

/**
 * OBS: Needed until typescript supports decorator type extensions.
 *
 * @since 1.0.0
 */
export abstract class IEventHandler<T, R> {
  public static _InstantiateEventHandler: (injector: Injector) => IEventHandler<any, any>;
  public static Type: string;
  public static Event: string;
  public readonly event: string;
  public abstract handle(event: EventMessage<T>, state: R): Promise<R>;
  _validateAndHandle: (event: EventMessage<T>, state: R) => Promise<R>;
}

export interface EventHandlerOptions {
  event: string;
  schema?: Schema;
}

/**
 * Creates a new event.
 *
 * @since 1.0.0
 * @returns {IEventHandler<any>} A decorated class that implements IEvent
 */
export function EventHandler(options: EventHandlerOptions) {
  return <T extends {new(...args: any[]): {}}>(Class: T) => {
    return class extends Class {
      static _InstantiateEventHandler(parentInjector: Injector): IEventHandler<any, any> {
        const injector = parentInjector.newChildInjector();
        return new this(injector);
      }
      public static Type = 'Event';
      public static Event = options.event;
      public event = options.event;

      _logger: Logger;

      constructor(...args: any[]) {
        super(...args[0].args(Class));
        this._logger = (args[0] as Injector).get<Logger>(Logger);
      }

      handle: (event: EventMessage<any>, state: any) => Promise<any>;

      async _validateAndHandle(event: EventMessage<any>, state: any): Promise<any> {
        let schema = eventMessageSchema;
        if(options.schema) {
          schema = eventMessageSchema.keys({
            content: options.schema.required()
          });
        } else {
          schema = eventMessageSchema.keys({
            content: Joi.any()
          });
        }
        Joi.assert(event ,schema);
        if(this.handle) {
          return await this.handle(event, state);
        } else {
          this._logger.error(`The event handler "${this.event}" has no handle method`);
          throw new Error('No handle method');
        }
      }
    };
  };
}
