import { EventMessage } from '@eventific/core';


export interface Context<T> {
  aggregateName: string,
  aggregateId: string,
  trigger: EventMessage,
  version: number,
  state: T
}
