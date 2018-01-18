import { options, Schema } from 'joi';
import { EventMessage } from './EventMessage';

export abstract class BaseEvent<T = undefined> {
  public static Name: string;
  public name: string;
  public id: number;
  public aggregateId: string;
  public content: T;
  public abstract apply(state: any): Promise<any>;
  public toMessage: () => EventMessage;
}

export interface EventOptions {
  name: string;
  schema: Schema;
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
      public static Name = options.name;
      public name = options.name;
      public aggregateId: string;
      public id: number;
      public content: any;
      public createdDate: Date;

      public apply: (state: any) => Promise<any>;

      constructor(...args: any[]) {
        super();
        const message: EventMessage = args[0];
        this.id = message.eventId;
        this.aggregateId = message.aggregateId;
        this.content = message.content;
        this.createdDate = message.header.createdDate;
      }

      public toMessage(): EventMessage {
        return {
          event: this.name,
          eventId: this.id,
          aggregateId: this.aggregateId,
          header: {
            createdDate: this.createdDate
          },
          content: this.content
        };
      }
    };
  };
}
