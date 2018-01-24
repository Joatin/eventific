import { ISaga } from './Saga';
import * as Joi from 'joi';


export function EventTrigger(...triggers: any[]) {
  Joi.assert(triggers, Joi.array().min(1), 'You must have at least one trigger');
  return (target: ISaga, propertyKey: string, descriptor: PropertyDescriptor) => {
    target._triggerDefinitions = target._triggerDefinitions || [];
    target._triggerDefinitions.push({
      triggers,
      propertyKey
    });
  }
}
