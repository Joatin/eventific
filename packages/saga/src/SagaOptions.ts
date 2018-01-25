import { IAggregate, Injector, IStore, ITransport } from '@eventific/core';
import * as Joi from 'joi';

export interface SagaOptions {
  aggregates: Array<{
    _InstantiateAggregate(injector: Injector): IAggregate;
  }>;
  providers?: any[];
  store: {
    _CreateStore(injector: Injector): IStore
  };
  transport: {
    _CreateTransport(injector: Injector): ITransport
  };
}

export const sagaOptionsSchema = Joi.object().keys({
  aggregates: Joi.array().min(1).required(),
  store: Joi.any().required(),
  transport: Joi.any().optional()
});
