import { ICommandHandler } from './ICommandHandler';
import { IEventHandler } from './IEventHandler';
import { Injector } from './Injector';


export interface AggregateOptions {
  /**
   * The name of the aggregate, should be written in PascalCase
   *
   * @since 1.0
   */
  name: string;
  providers?: any[];

  /**
   * The event handlers to add tom this aggregate
   *
   * @since 1.0
   */
  eventHandlers: Array<{
    Event: string;
    new(...args: any[]): IEventHandler<any, any>;
    _InstantiateEventHandler(injector: Injector): IEventHandler<any, any>;
  }>;

  /**
   * The command handlers to add to this aggregate
   *
   * @since 1.0
   */
  commandHandlers: Array<{
    Command: string;
    new(...args: any[]): ICommandHandler<any, any>;
    _InstantiateCommandHandler(injector: Injector): ICommandHandler<any, any>;
  }>;
}
