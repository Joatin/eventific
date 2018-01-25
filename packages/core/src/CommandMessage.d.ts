import * as Joi from 'joi';
export interface CommandMessage<T = {}> {
    aggregateId: string;
    command: string;
    header: {
        createdDate: Date;
        createdBy: string;
    };
    content: T;
}
export declare const commandMessageSchema: Joi.ObjectSchema;
