import { CommandMessage, IAggregate, IStore, Injector, Logger, Bootstrapable, ITransport } from '@eventific/core';
/**
 * Defines params for the command manager
 *
 * @since 1.0
 */
export interface CommandManagerOptions {
    extensions?: any[];
    /**
     * The aggregate to issue commands against
     *
     * @since 1.0
     */
    aggregate: {
        _InstantiateAggregate(injector: Injector): IAggregate;
    };
    /**
     * The store that should be used to persist events
     *
     * @since 1.0
     */
    store: {
        _CreateStore(injector: Injector): IStore;
    };
    /**
     * An array of transports that is used to receive commands
     *
     * @since 1.0
     */
    transports: Array<{
        _CreateTransport(injector: Injector): ITransport;
    }>;
    /**
     * An array of providers to be used in Eventifics IOC
     *
     * @since 1.0
     */
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
