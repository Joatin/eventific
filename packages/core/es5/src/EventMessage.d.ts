import * as Joi from 'joi';
export interface EventMessage<T = undefined> {
    event: string;
    eventId: number;
    aggregateId: string;
    header: {
        createdDate: Date;
    };
    content: T;
}
export declare const eventMessageSchema: Joi.ObjectSchema;
