import { ICommandHandler } from '../command/ICommandHandler';
import { IEventHandler } from '../event/IEventHandler';
import { Injector } from '../injector/Injector';

/**
 * @public
 */
export interface AggregateOptions {
  /**
   * The name of the aggregate, should be written in PascalCase
   *
   */
  name: string;
  providers?: any[];

  /**
   * The event handlers to add tom this aggregate
   *
   */
  eventHandlers: Array<{
    Event: string;
    new(...args: any[]): IEventHandler<any, any>;
    _InstantiateEventHandler(injector: Injector): IEventHandler<any, any>;
  }>;

  /**
   * The command handlers to add to this aggregate
   *
   */
  commandHandlers: Array<{
    Command: string;
    new(...args: any[]): ICommandHandler<any, any>;
    _InstantiateCommandHandler(injector: Injector): ICommandHandler<any, any>;
  }>;
}
