import { BaseEvent } from './Event';


export abstract class BaseCommand<T = undefined> {
    static Name: string;
    readonly name: string;
    readonly data: T;

    abstract handle(state: any, version: number): Promise<BaseEvent[]>
}

export interface CommandOptions {
  name: string;
}

export function Command(options: CommandOptions) {
  return <T extends {new(...args: any[]): {}}>(constructor: T) => {
    return class extends constructor {
      static Name = options.name;
      name = options.name;
    };
  };
}