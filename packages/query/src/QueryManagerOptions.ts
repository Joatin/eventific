import { IAggregate, Injector, IStore } from '@eventific/core';
import * as Joi from 'joi';


export interface QueryManagerOptions {
  extensions?: any[];

  /**
   * The aggregate to issue commands against
   *
   * @since 1.0
   */
  aggregate: {
    _InstantiateAggregate(injector: Injector): IAggregate;
  };

  /**
   * The store that should be used to persist events
   *
   * @since 1.0
   */
  store: {
    _CreateStore(injector: Injector): IStore
  };

  /**
   * An array of providers to be used in Eventifics IOC
   *
   * @since 1.0
   */
  providers?: any[];
}

export const queryManagerOptionsSchema = Joi.object();
