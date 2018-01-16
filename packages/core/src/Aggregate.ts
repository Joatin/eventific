import { BaseEvent } from './Event';
import { BaseCommand } from './Command';

export interface AggregateOptions {
  name: string,
  events: { new(...args: any[]): BaseEvent; Name: string; }[],
  commands: { new(...args: any[]): BaseCommand; Name: string; }[],
  services: any[]
}

export function Aggregate(options: AggregateOptions) {
  return <T extends {new(...args: any[]): {}}>(constructor: T) => {
    return class extends constructor {
      static Name = options.name;
      name = options.name;
      _commands = new Map(options.commands.map<[string, { new(...args: any[]): BaseCommand; Name: string; }]>(cmd => [cmd.Name, cmd]));
      _events = new Map(options.events.map<[string, { new(...args: any[]): BaseEvent; Name: string; }]>(ev => [ev.Name, ev]));

      async _handleCommand(cmd: any) {
        // TODO: Validate it here

        const type = this._commands.get(cmd.name);
        if(type) {
          const instance = new type(cmd);
          const result = await this._buildState(cmd.aggregateId);
          instance.handle(result.state, result.version);
        } else {
          throw Error('No handler for the command: ' + cmd.name);
        }
      }

      async _buildState(aggregateId: string): Promise<{version: number, state: any}> {
        return {version: -1, state: {}}
      }
    };
  };
}