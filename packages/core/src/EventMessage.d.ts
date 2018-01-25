import * as Joi from 'joi';
export interface EventMessage<T = undefined> {
    aggregateId: string;
    content: T;
    event: string;
    eventId: number;
    header: {
        createdDate: Date;
    };
}
export declare const eventMessageSchema: Joi.ObjectSchema;
