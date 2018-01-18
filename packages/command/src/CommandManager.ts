import { Store, Transport, IAggregate, CommandMessage } from "@eventific/core";

export interface CommandManagerOptions {
  extensions: any[];
  aggregate: IAggregate;
  store: {
    CreateStore(): Store
  };
  transports: {
    CreateTransport(): Transport
  }[];
  services: any[];
}

export function CommandManager({extensions, aggregate, store, transports, services}: CommandManagerOptions) {
  const storeInstance = store.CreateStore();
  const transportInstances = transports.map(t => t.CreateTransport());
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

        await storeInstance.start();

        for(let transport of transportInstances) {
          transport.onCommand(async (cmd: any) => {
            await this._handleCommand(cmd);
          });
          await transport.start();
        }
      }

      async _handleCommand(commandMessage: CommandMessage): Promise<void> {
        const command = await aggregate.getCommand(commandMessage);
        const stateDef = await aggregate.getState(command.aggregateId);
        const events = await command.handle(stateDef.state, stateDef.state);
        await storeInstance.applyEvents(events.map(e => e.toMessage()));
      }

      onInit?: () => void

    };
  };
}
