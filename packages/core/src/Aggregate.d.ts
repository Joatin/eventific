import { ICommandHandler } from './Command';
import { CommandMessage } from './CommandMessage';
import { IEventHandler } from './Event';
import { Injector } from './Injector';
export interface AggregateOptions {
    /**
     * The name of the aggregate, should be written in PascalCase
     *
     * @since 1.0
     */
    name: string;
    /**
     * The event handlers to add tom this aggregate
     *
     * @since 1.0
     */
    eventHandlers: Array<{
        _InstantiateEventHandler: (injector: Injector) => IEventHandler<any, any>;
        new (...args: any[]): IEventHandler<any, any>;
        Event: string;
    }>;
    /**
     * The command handlers to add to this aggregate
     *
     * @since 1.0
     */
    commandHandlers: Array<{
        _InstantiateCommandHandler(injector: Injector): ICommandHandler<any, any>;
        new (...args: any[]): ICommandHandler<any, any>;
        Command: string;
    }>;
    providers?: any[];
}
/**
 * Represents a aggregate instance
 *
 * @since 1.0.0
 */
export declare abstract class IAggregate {
    static Type: string;
    static Name: string;
    static _InstantiateAggregate: (parentInjector: Injector) => IAggregate;
    /**
     * The name of this aggregate
     *
     * @since 1.0.0
     */
    readonly name: string;
    /**
     * Returns a command based on the provided command message
     *
     * @since 1.0.0
     *
     * @param {CommandMessage} commandMessage The command message to convert to a command instance
     * @returns {Promise<EventMessage<any>[]>} A new command instance
     */
    handleCommand: (commandMessage: CommandMessage) => Promise<void>;
    getState: (aggregateId: string) => Promise<{
        version: number;
        state: any;
    }>;
    getEventNames: () => string[];
}
export declare function Aggregate(options: AggregateOptions): <T extends new (...args: any[]) => {}>(Class: T) => T;
