import * as Joi from 'joi';


export interface CommandHandlerOptions {
  command: string;
}

export const commandHandlerOptionsSchema = Joi.object().keys({
  command: Joi.string().min(3).required()
});
