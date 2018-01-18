import { CommandMessage } from './CommandMessage';
import { BaseEvent } from './Event';

export abstract class BaseCommand<T = undefined> {
  public static Name: string;
  public readonly name: string;
  public readonly aggregateId: string;
  public readonly headers: {
    createdDate: Date
  };
  public readonly content: T;

  public abstract handle(state: any, version: number): Promise<BaseEvent[]>;
}

export interface CommandOptions {
  name: string;
}

export function Command(options: CommandOptions) {
  return <T extends {new(...args: any[]): {}}>(constructor: T) => {
    return class extends constructor {
      public static Name = options.name;
      public readonly name = options.name;
      public readonly aggregateId: string;
      public readonly headers: {
        createdDate: Date
      };
      public readonly content: any;

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
