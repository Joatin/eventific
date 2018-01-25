import { Schema } from 'joi';
import { EventMessage } from './EventMessage';
import { Injector } from './Injector';
import { Logger } from './Logger';
/**
 * OBS: Needed until typescript supports decorator type extensions.
 *
 * @since 1.0.0
 */
export declare abstract class IEventHandler<T, R> {
    static _InstantiateEventHandler: (injector: Injector) => IEventHandler<any, any>;
    static Type: string;
    static Event: string;
    readonly event: string;
    abstract handle(event: EventMessage<T>, state: R): Promise<R>;
    _validateAndHandle: (event: EventMessage<T>, state: R) => Promise<R>;
}
export interface EventHandlerOptions {
    event: string;
    schema?: Schema;
}
/**
 * Creates a new event.
 *
 * @since 1.0.0
 * @returns {IEventHandler<any>} A decorated class that implements IEvent
 */
export declare function EventHandler(options: EventHandlerOptions): <T extends new (...args: any[]) => {}>(Class: T) => {
    new (...args: any[]): {
        event: string;
        _logger: Logger;
        handle: (event: EventMessage<any>, state: any) => Promise<any>;
        _validateAndHandle(event: EventMessage<any>, state: any): Promise<any>;
    };
    _InstantiateEventHandler(parentInjector: Injector): IEventHandler<any, any>;
    Type: string;
    Event: string;
} & T;
