import * as Joi from 'joi';
import { EventHandlerOptions } from './EventHandlerOptions';
import { EventMessage, eventMessageSchema } from './EventMessage';
import { IEventHandler } from './IEventHandler';
import { Injector } from './Injector';
import { Logger } from './Logger';


/**
 * Creates a new event.
 *
 * @public
 */
export function EventHandler(options: EventHandlerOptions) {
  return <T extends {new(...args: any[]): {}}>(Class: T) => {
    return class extends Class {
      public static Event = options.event;
      public static Type = 'Event';

      public static _InstantiateEventHandler(parentInjector: Injector): IEventHandler<any, any> {
        const injector = parentInjector.newChildInjector();
        return new this(injector);
      }

      public event = options.event;
      public handle: (event: EventMessage<any>, state: any) => Promise<any>;
      public _logger: Logger;

      constructor(...args: any[]) {
        super(...args[0].args(Class));
        this._logger = (args[0] as Injector).get<Logger>(Logger);
      }

      public async _validateAndHandle(event: EventMessage<any>, state: any): Promise<any> {
        let schema = eventMessageSchema;
        if (options.schema) {
          schema = eventMessageSchema.keys({
            content: options.schema.required()
          });
        } else {
          schema = eventMessageSchema.keys({
            content: Joi.any()
          });
        }
        Joi.assert(event , schema);
        if (this.handle) {
          return await this.handle(event, state);
        } else {
          this._logger.error(`The event handler "${this.event}" has no handle method`);
          throw new Error('No handle method');
        }
      }
    };
  };
}
