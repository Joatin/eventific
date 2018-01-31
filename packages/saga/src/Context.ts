import { CommandMessage, EventMessage } from '@eventific/core';


/**
 *
 * @module @eventific/saga
 */
export interface Context<T> {
  aggregateName: string;
  aggregateId: string;
  trigger: EventMessage;
  version: number;
  state: T;
  dispatch: (command: CommandMessage & { header?: any}) => Promise<void>;
}
