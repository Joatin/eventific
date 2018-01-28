import { CommandMessage } from './CommandMessage';
import { Injector } from './Injector';


/**
 * Represents a aggregate instance
 *
 * @since 1.0.0
 */
export abstract class IAggregate {

  public static Type: string;
  public static Name: string;
  public static _InstantiateAggregate: (parentInjector: Injector) => IAggregate;

  /**
   * The name of this aggregate
   *
   * @since 1.0.0
   */
  public readonly name: string;

  /**
   * Returns a command based on the provided command message
   *
   * @since 1.0.0
   *
   * @param {CommandMessage} commandMessage The command message to convert to a command instance
   * @returns {Promise<EventMessage<any>[]>} A new command instance
   */
  public handleCommand: (commandMessage: CommandMessage) => Promise<void>;

  public getState: (aggregateId: string) => Promise<{version: number, state: any}>;

  public getEventNames: () => string[];
  public getCommandNames: () => string[];

}
