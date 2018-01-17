import { Store, Transport, IAggregate } from "@eventific/core";

export interface CommandManagerOptions {
  extensions: any[];
  aggregate: IAggregate;
  store: Store;
  transports: Transport[];
  services: any[];
}

export function CommandManager(options: CommandManagerOptions) {

  return <T extends {new(...args: any[]): {}}>(constructor: T) => {
    return class extends constructor {
      static Type = 'CommandManager';
      static _Instantiate(): T {
        return new this() as any;
      }

      _store = options.store;

      async _start() {
        if(this.onInit) {
          await this.onInit();
        }
        for(let transport of options.transports) {
          transport.onCommand(async (cmd: any) => {
            await options.aggregate._handleCommand(cmd);
          });
        }
      }

      onInit?: () => void

    };
  };
}
