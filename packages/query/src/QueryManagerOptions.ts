import { IAggregate, Injector, IStore } from '@eventific/core';
import * as Joi from 'joi';
import { IViewHandler } from './view/IViewHandler';


export interface QueryManagerOptions {
  extensions?: any[];

  /**
   * The aggregate to issue commands against
   *
   * @since 1.0
   */
  aggregates: Array<{
    _InstantiateAggregate(injector: Injector): IAggregate;
  }>;

  /**
   * An array of providers to be used in Eventifics IOC
   *
   * @since 1.0
   */
  providers?: any[];

  /**
   * The store that should be used to persist events
   *
   * @since 1.0
   */
  store: {
    _CreateStore(injector: Injector): IStore
  };

  viewHandlers: Array<{
    _InstantiateViewHandler(injector: Injector): IViewHandler;
  }>;
}

export const queryManagerOptionsSchema = Joi.object().keys({
  aggregates: Joi.array().min(1).required(),
  providers: Joi.array().optional(),
  store: Joi.any().required(),
  viewHandlers: Joi.array().min(1).required()
});
