import { CommandMessage, IAggregate, IStore, Injector, Logger, Bootstrapable, ITransport } from '@eventific/core';
export interface CommandManagerOptions {
    extensions?: any[];
    aggregate: {
        _InstantiateAggregate(injector: Injector): IAggregate;
    };
    store: {
        _CreateStore(injector: Injector): IStore;
    };
    transports: Array<{
        _CreateTransport(injector: Injector): ITransport;
    }>;
    providers?: any[];
}
export declare abstract class ICommandManager extends Bootstrapable {
}
/**
 *
 * @param {CommandManagerOptions} options
 * @returns T The decorated class
 * @Annotation
 */
export declare function CommandManager(options: CommandManagerOptions): <T extends new (...args: any[]) => {}>(Class: T) => {
    new (...args: any[]): {
        readonly _injector: Injector;
        readonly _store: IStore;
        readonly _transports: ITransport[];
        readonly _aggregate: IAggregate;
        readonly _logger: Logger;
        _start(): Promise<void>;
        _handleCommand(commandMessage: CommandMessage<{}>): Promise<void>;
        onInit?: (() => void) | undefined;
    };
    Type: string;
    _Instantiate(parentInjector: Injector): T;
} & T;
