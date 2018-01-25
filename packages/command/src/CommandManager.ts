import {
  CommandMessage,
  IAggregate,
  IEventHandler,
  Injector,
  InternalLogger,
  IStore,
  ITransport,
  Logger,
  Store
} from '@eventific/core';
import chalk from 'chalk';
import * as Joi from 'joi';
import * as emoji from 'node-emoji';
import pascalCase = require('pascal-case');
import { CommandManagerOptions, commandManagerOptionsSchema } from './CommandManagerOptions';


/**
 *
 * @param {CommandManagerOptions} options
 * @returns T The decorated class
 * @Annotation
 */
export function CommandManager(options: CommandManagerOptions) {
  Joi.assert(options, commandManagerOptionsSchema);
  return <T extends {new(...args: any[]): {}}>(Class: T) => {
    return class extends Class {
      public static Type = 'CommandManager';

      public static _Instantiate(parentInjector: Injector): T {
        const injector = parentInjector.newChildInjector();
        const store = options.store._CreateStore(injector);
        injector.set({provide: Store, useConstant: store});
        injector.set({provide: Logger, useConstant: new InternalLogger(chalk.green(pascalCase('CommandManager')))});

        return new this({
          aggregate: options.aggregate._InstantiateAggregate(injector),
          injector,
          store,
          transports: options.transports.map((t) => t._CreateTransport(injector))
        }) as any;
      }


      public onInit?: () => void;
      public readonly _injector: Injector;
      public readonly _store: IStore;
      public readonly _transports: ITransport[];
      public readonly _aggregate: IAggregate;
      public readonly _logger: Logger;

      /* istanbul ignore next */
      constructor(...args: any[]) {
        super(...args[0].injector.args(Class));
        const params = args[0];
        this._injector = params.injector;
        this._store = params.store;
        this._transports = params.transports;
        this._aggregate = params.aggregate;
        this._logger = this._injector.get<Logger>(Logger);
      }

      public async _start() {
        if (this.onInit) {
          await this.onInit();
        }

        await this._store.start();

        for (const transport of this._transports) {
          await transport.start();
          if (transport.onCommand) {
            transport.onCommand(async (cmd: any) => {
              await this._handleCommand(cmd);
            });
          }
        }
        this._logger.info(`All setup and ready ${emoji.get('sparkles')}`);
      }

      public async _handleCommand(commandMessage: CommandMessage): Promise<void> {
        await this._aggregate.handleCommand(commandMessage);
      }

    };
  };
}
