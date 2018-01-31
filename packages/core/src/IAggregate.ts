import { CommandMessage } from './CommandMessage';
import { Injector } from './Injector';


/**
 * Represents a aggregate instance
 *
 * @public
 */
export abstract class IAggregate {

  public static Type: string;
  public static Name: string;
  public static _InstantiateAggregate: (parentInjector: Injector) => IAggregate;

  /**
   * The name of this aggregate
   *
   */
  public readonly name: string;

  /**
   * Returns a command based on the provided command message
   *
   */
  public handleCommand: (commandMessage: CommandMessage) => Promise<void>;

  public getState: (aggregateId: string) => Promise<{version: number, state: any}>;

  public getEventNames: () => string[];
  public getCommandNames: () => string[];

}
