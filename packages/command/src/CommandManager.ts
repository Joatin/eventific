import { Store, Transport, IAggregate, CommandMessage } from "@eventific/core";

export interface CommandManagerOptions {
  extensions: any[];
  aggregate: IAggregate;
  store: Store;
  transports: Transport[];
  services: any[];
}

export function CommandManager({extensions, aggregate, store, transports, services}: CommandManagerOptions) {

  return <T extends {new(...args: any[]): {}}>(Class: T) => {
    return class extends Class {
      static Type = 'CommandManager';
      static _Instantiate(): T {
        return new this() as any;
      }

      async _start() {
        if(this.onInit) {
          await this.onInit();
        }

        await store.start();

        for(let transport of transports) {
          transport.onCommand(async (cmd: any) => {
            await this._handleCommand(cmd);
          });
          await transport.start();
        }
      }

      async _handleCommand(commandMessage: CommandMessage): Promise<void> {
        const command = await aggregate.getCommand(commandMessage);
        const stateDef = await aggregate.getState(command.aggregateId);
        const events = command.handle(stateDef.state, stateDef.state);
      }

      onInit?: () => void

    };
  };
}
