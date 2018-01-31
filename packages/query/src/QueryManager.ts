import { IAggregate, Injector, InternalLogger, IStore, Logger, Store } from '@eventific/core';
import chalk from 'chalk';
import * as Joi from 'joi';
import * as emoji from 'node-emoji';
import pascalCase = require('pascal-case');
import { QueryManagerOptions, queryManagerOptionsSchema } from './QueryManagerOptions';
import { IViewHandler } from './view/IViewHandler';


export function QueryManager(options: QueryManagerOptions) {
  Joi.assert(options, queryManagerOptionsSchema);
  return <T extends {new(...args: any[]): {}}>(Class: T) => {
    return class extends Class {
      public static Type = 'QueryManager';

      public static _Instantiate(parentInjector: Injector): T {
        const injector = parentInjector.newChildInjector();
        const store = options.store._CreateStore(injector);
        for (const prov of (options.providers || [])) {
          injector.set(prov);
        }
        injector.set({provide: Store, useConstant: store});
        injector.set({provide: Logger, useConstant: new InternalLogger(chalk.green(pascalCase('QueryManager')))});

        return new this({
          aggregates: options.aggregates.map( (a) => a._InstantiateAggregate(injector)),
          injector,
          store,
          viewHandlers: options.viewHandlers.map( (v) => v._InstantiateViewHandler(injector))
        }) as any;
      }


      public onInit?: (ctx: {injector: Injector}) => void;
      public readonly _injector: Injector;
      public readonly _store: IStore;
      public readonly _aggregates: IAggregate[];
      public readonly _logger: Logger;
      public readonly _viewHandlers: IViewHandler[];

      /* istanbul ignore next */
      constructor(...args: any[]) {
        super(...args[0].injector.args(Class));
        const params = args[0];
        this._injector = params.injector;
        this._store = params.store;
        this._aggregates = params.aggregates;
        this._viewHandlers = params.viewHandlers;
        this._logger = this._injector.get<Logger>(Logger);
      }

      public async _start() {
        if (this.onInit) {
          await this.onInit({injector: this._injector});
        }

        await this._store.start();
        for (const handler of this._viewHandlers) {
          await handler.start();
        }

        for (const agg of this._aggregates) {
          await this._store.onEvent(agg.name, null, async (event) => {
            const stateResult = await agg.getState(event.aggregateId);
            for (const handler of this._viewHandlers) {
              await handler.buildAndPersistView(event.aggregateId, stateResult.state, stateResult.version);
            }
          });
        }


        this._logger.info(`All setup and ready ${emoji.get('sparkles')}`);
      }

    };
  };
}
