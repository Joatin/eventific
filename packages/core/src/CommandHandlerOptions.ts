import * as Joi from 'joi';

/**
 * @public
 */
export interface CommandHandlerOptions {
  command: string;
}

/**
 * @internal
 */
export const commandHandlerOptionsSchema = Joi.object().keys({
  command: Joi.string().min(3).required()
});
