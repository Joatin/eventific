import * as Joi from 'joi';
export interface CommandMessage<T = {}> {
    aggregateId: string;
    command: string;
    content: T;
    header: {
        createdBy: string;
        createdDate: Date;
    };
}
export declare const commandMessageSchema: Joi.ObjectSchema;
