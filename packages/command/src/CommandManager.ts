import { CommandMessage, IAggregate, Store, Transport } from '@eventific/core';

export interface CommandManagerOptions {
  extensions?: any[];
  aggregate: {
    _InstantiateAggregate(): IAggregate;
  };
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

  return <T extends {new(...args: any[]): {}}>(Class: T) => {
    return class extends Class {
      public static Type = 'CommandManager';
      public static _Instantiate(): T {
        return new this({
          store: options.store.CreateStore(),
          transport: options.transports.map((t) => t.CreateTransport()),
          aggregate: options.aggregate._InstantiateAggregate()
        }) as any;
      }

      readonly _store: Store;
      readonly _transports: Transport[];
      readonly _aggregate: IAggregate;

      constructor(...args: any[]) {
        super();
        const params = args[0];
        this._store = params.store;
        this._transports = params.transports;
        this._aggregate = params.aggregate;
      }

      public async _start() {
        if (this.onInit) {
          await this.onInit();
        }

        await this._store.start();

        for (const transport of this._transports) {
          transport.onCommand(async (cmd: any) => {
            await this._handleCommand(cmd);
          });
          await transport.start();
        }
      }

      public async _handleCommand(commandMessage: CommandMessage): Promise<void> {
        const command = await this._aggregate.getCommand(commandMessage);
        const stateDef = await this._aggregate.getState(command.aggregateId);
        const events = await command.handle(stateDef.state, stateDef.state);
        await this._store.applyEvents(this._aggregate.name, events.map((e) => e.toMessage()));
      }

      public onInit?: () => void;

    };
  };
}
