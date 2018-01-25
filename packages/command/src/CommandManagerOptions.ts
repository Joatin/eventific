import { IAggregate, Injector, IStore, ITransport } from '@eventific/core';
import * as Joi from 'joi';

/**
 * Defines params for the command manager
 *
 * @since 1.0
 */
export interface CommandManagerOptions {
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
   * An array of transports that is used to receive commands
   *
   * @since 1.0
   */
  transports: Array<{
    _CreateTransport(injector: Injector): ITransport
  }>;

  /**
   * An array of providers to be used in Eventifics IOC
   *
   * @since 1.0
   */
  providers?: any[];
}

export const commandManagerOptionsSchema = Joi.object().keys({
  aggregate: (Joi as any).func().unknown().keys({
    Name: Joi.string().required(),
    Type: Joi.string().required(),
    _InstantiateAggregate: Joi.func().required()
  }).required(),
  extensions: Joi.array().items(Joi.any()).optional(),
  providers: Joi.array().items(Joi.any()).optional(),
  store: (Joi as any).func().unknown().keys({
    _CreateStore: Joi.func().required()
  }).required(),
  transports: Joi.array().min(1).items((Joi as any).func().unknown().keys({
    _CreateTransport: Joi.func().required()
  })).required()
});
