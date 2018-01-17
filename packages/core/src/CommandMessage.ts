import * as Joi from 'joi';

export interface CommandMessage<T = undefined> {
  aggregateId: string;
  command: string,
  headers: {
    createdDate: Date;
    createdBy: string;
  };
  content: T
}

export const commandMessageSchema = Joi.object().keys({
  aggregateId: Joi.string().uuid().required(),
  command: Joi.string().required(),
  headers: Joi.object().keys({
    createdDate: Joi.date().required(),
    createdBy: Joi.string().optional()
  }).required(),
  content: Joi.any().optional()
});