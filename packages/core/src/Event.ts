import { options, Schema } from 'joi';
import { EventMessage } from './EventMessage';


export abstract class BaseEvent<T = undefined> {
  static Name: string;
  name: string;
  id: number;
  aggregateId: string;
  content: T;
  abstract apply(state: any): Promise<any>
  toMessage: () => EventMessage
}

export interface EventOptions {
  name: string,
  schema: Schema
}

export interface IEvent {
  name: string;
}

/**
 * Creates a new event
 * @returns {<T extends {new(...args: any[]) => {}}>(constructor: T) => {new() => {handleEvent: (())}}}
 * @constructor
 */
export function Event(options: EventOptions) {
  return <T extends {new(...args: any[]): {}}>(constructor: T): {new(...args: any[]): BaseEvent} => {
    return class extends constructor {
      static Name = options.name;
      name = options.name;
      aggregateId: string;
      id: number;
      content: any;
      createdDate: Date;

      apply: (state: any) => Promise<any>;

      constructor(...args: any[]) {
        super();
        const message: EventMessage = args[0];
        this.id = message.eventId;
        this.aggregateId = message.aggregateId;
        this.content = message.content;
        this.createdDate = message.header.createdDate;
      }

      toMessage(): EventMessage {
        return {
          event: this.name,
          eventId: this.id,
          aggregateId: this.aggregateId,
          header: {
            createdDate: this.createdDate
          },
          content: this.content
        }
      }
    };
  };
}
