import * as Joi from 'joi';

/**
 * @public
 */
export interface CommandMessage<T = {}> {
  aggregateId: string;
  command: string;
  content: T;
  header: {
    createdBy?: string;
    createdDate: Date;
  };
}

/**
 * @internal
 */
export const commandMessageSchema = Joi.object().keys({
  aggregateId: Joi.string().uuid().required(),
  command: Joi.string().required(),
  content: Joi.any().optional(),
  header: Joi.object().keys({
    createdBy: Joi.string().optional(),
    createdDate: Joi.date().required()
  }).required()
});
