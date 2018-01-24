import * as Joi from 'joi';

export interface EventMessage<T = undefined> {
  event: string;
  eventId: number;
  aggregateId: string;
  header: {
    createdDate: Date
  };
  content: T;
}

export const eventMessageSchema = Joi.object().keys({
  event: Joi.string().min(3).required(),
  eventId: Joi.number().min(0).required(),
  aggregateId: Joi.string().guid().required(),
  header: Joi.object().keys({
    createdDate: Joi.date().required()
  }).required(),
  content: Joi.any().optional()
});
