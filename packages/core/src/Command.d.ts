import { CommandMessage } from './CommandMessage';
import { EventMessage } from './EventMessage';
import { Injector } from './Injector';
export declare abstract class ICommandHandler<T, R> {
    static _InstantiateCommandHandler: (injector: Injector) => ICommandHandler<any, any>;
    static Command: string;
    readonly command: string;
    abstract handle(message: CommandMessage<T>, state: R, version: number): Promise<EventMessage[]>;
}
export interface CommandHandlerOptions {
    command: string;
}
/**
 *
 * @param {CommandHandlerOptions} options
 * @returns {<T extends {new(...args: any[]) => {}}>(Class: T) => {Command: any; _InstantiateCommandHandler: ((parentInjector: Injector) => ICommandHandler<any, any>); new(...args: any[]) => {command: string; handle: ((message: CommandMessage<any>, state: any, version: number) => Promise<EventMessage[]>)}}}
 * @constructor
 */
export declare function CommandHandler(options: CommandHandlerOptions): <T extends new (...args: any[]) => {}>(Class: T) => {
    new (...args: any[]): {
        readonly command: string;
        handle: (message: CommandMessage<any>, state: any, version: number) => Promise<EventMessage<undefined>[]>;
    };
    Command: string;
    _InstantiateCommandHandler(parentInjector: Injector): ICommandHandler<any, any>;
} & T;
