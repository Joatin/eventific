import { CommandMessage, IAggregate, Store, Transport } from '@eventific/core';

export interface CommandManagerOptions {
  extensions?: any[];
  aggregate: IAggregate;
  store: {
    CreateStore(): Store
  };
  transports: Array<{
    CreateTransport(): Transport
  }>;
  services?: any[];
}

/**
 *
 * @param {CommandManagerOptions} options
 * @returns T The decorated class
 * @Annotation
 */
export function CommandManager(options: CommandManagerOptions) {
  const {extensions, aggregate, store, transports, services} = options;
  const storeInstance = store.CreateStore();
  const transportInstances = transports.map((t) => t.CreateTransport());
  return <T extends {new(...args: any[]): {}}>(Class: T) => {
    return class extends Class {
      public static Type = 'CommandManager';
      public static _Instantiate(): T {
        return new this() as any;
      }

      public async _start() {
        if (this.onInit) {
          await this.onInit();
        }

        await storeInstance.start();

        for (const transport of transportInstances) {
          transport.onCommand(async (cmd: any) => {
            await this._handleCommand(cmd);
          });
          await transport.start();
        }
      }

      public async _handleCommand(commandMessage: CommandMessage): Promise<void> {
        const command = await aggregate.getCommand(commandMessage);
        const stateDef = await aggregate.getState(command.aggregateId);
        const events = await command.handle(stateDef.state, stateDef.state);
        await storeInstance.applyEvents(aggregate.name, events.map((e) => e.toMessage()));
      }

      public onInit?: () => void;

    };
  };
}
