import { IAggregate, Injector, InternalLogger, IStore, Logger, Store } from '@eventific/core';
import chalk from 'chalk';
import * as Joi from 'joi';
import * as emoji from 'node-emoji';
import pascalCase = require('pascal-case');
import { QueryManagerOptions, queryManagerOptionsSchema } from './QueryManagerOptions';


export function QueryManager(options: QueryManagerOptions) {
  Joi.assert(options, queryManagerOptionsSchema);
  return <T extends {new(...args: any[]): {}}>(Class: T) => {
    return class extends Class {
      public static Type = 'QueryManager';

      public static _Instantiate(parentInjector: Injector): T {
        const injector = parentInjector.newChildInjector();
        const store = options.store._CreateStore(injector);
        injector.set({provide: Store, useConstant: store});
        injector.set({provide: Logger, useConstant: new InternalLogger(chalk.green(pascalCase('QueryManager')))});

        return new this({
          aggregate: options.aggregate._InstantiateAggregate(injector),
          injector,
          store
        }) as any;
      }


      public onInit?: () => void;
      public readonly _injector: Injector;
      public readonly _store: IStore;
      public readonly _aggregate: IAggregate;
      public readonly _logger: Logger;

      /* istanbul ignore next */
      constructor(...args: any[]) {
        super(...args[0].injector.args(Class));
        const params = args[0];
        this._injector = params.injector;
        this._store = params.store;
        this._aggregate = params.aggregate;
        this._logger = this._injector.get<Logger>(Logger);
      }

      public async _start() {
        if (this.onInit) {
          await this.onInit();
        }

        await this._store.start();
        this._logger.info(`All setup and ready ${emoji.get('sparkles')}`);
      }

    };
  };
}
