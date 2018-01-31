import * as Joi from 'joi';
import { ISaga } from './ISaga';

/**
 *
 * @module @eventific/saga
 * @param triggers
 * @returns {(target: ISaga, propertyKey: string, descriptor: PropertyDescriptor) => any}
 * @constructor
 */
export function EventTrigger(...triggers: any[]) {
  Joi.assert(triggers, Joi.array().min(1), 'You must have at least one trigger');
  return (target: ISaga, propertyKey: string, descriptor: PropertyDescriptor) => {
    target._triggerDefinitions = target._triggerDefinitions || [];
    target._triggerDefinitions.push({
      propertyKey,
      triggers
    });
  };
}
