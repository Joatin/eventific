import { EventMessage } from '@eventific/core';
import { CommandMessage } from '../../core/src/CommandMessage';


export interface Context<T> {
  aggregateName: string;
  aggregateId: string;
  trigger: EventMessage;
  version: number;
  state: T;
  dispatch: (command: CommandMessage & { header?: any}) => Promise<void>;
}
