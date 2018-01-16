import { Schema } from 'joi';


export abstract class BaseEvent<T = undefined> {
  static Name: string;
  name: string;
  id: number;
  aggregateId: string;
  data: T;
  abstract apply(state: any): Promise<any>
}

export interface EventOptions {
  name: string,
  schema: Schema
}

/**
 * Creates a new event
 * @returns {<T extends {new(...args: any[]) => {}}>(constructor: T) => {new() => {handleEvent: (())}}}
 * @constructor
 */
export function Event(options: EventOptions) {
  return <T extends {new(...args: any[]): {}}>(constructor: T) => {
    return class extends constructor {
      static Name = options.name;
      name = options.name;
    };
  };
}
