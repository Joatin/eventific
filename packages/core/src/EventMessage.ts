import * as Joi from 'joi';

/**
 * @public
 */
export interface EventMessage<T = undefined> {
  aggregateId: string;
  content: T;
  event: string;
  eventId: number;
  header: {
    createdDate: Date
  };
}

/**
 * @internal
 */
export const eventMessageSchema = Joi.object().keys({
  aggregateId: Joi.string().guid().required(),
  content: Joi.any().optional(),
  event: Joi.string().min(3).required(),
  eventId: Joi.number().min(0).required(),
  header: Joi.object().keys({
    createdDate: Joi.date().required()
  }).required()
});
