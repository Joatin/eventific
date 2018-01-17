import { BaseEvent } from './Event';
import { CommandMessage } from './CommandMessage';


export abstract class BaseCommand<T = undefined> {
  static Name: string;
  readonly name: string;
  readonly aggregateId: string;
  readonly headers: {
    createdDate: Date
  };
  readonly content: T;

  abstract handle(state: any, version: number): Promise<BaseEvent[]>
}

export interface CommandOptions {
  name: string;
}

export function Command(options: CommandOptions) {
  return <T extends {new(...args: any[]): {}}>(constructor: T) => {
    return class extends constructor {
      static Name = options.name;
      readonly name = options.name;
      readonly aggregateId: string;
      readonly headers: {
        createdDate: Date
      };
      readonly content: any;

      constructor(...rest: any[]) {
        super();
        const commandMessage = rest[0];
        // TODO: assert that commandMessage.command equals options.name
        this.aggregateId = commandMessage.aggregateId;
        this.headers = commandMessage.headers;
        this.content = commandMessage.content;
      }
    };
  };
}