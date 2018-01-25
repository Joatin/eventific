import { CommandMessage, IAggregate, Injector, IStore, ITransport, Logger } from '@eventific/core';
import { CommandManagerOptions } from './CommandManagerOptions';
/**
 *
 * @param {CommandManagerOptions} options
 * @returns T The decorated class
 * @Annotation
 */
export declare function CommandManager(options: CommandManagerOptions): <T extends new (...args: any[]) => {}>(Class: T) => {
    new (...args: any[]): {
        onInit?: (() => void) | undefined;
        readonly _injector: Injector;
        readonly _store: IStore;
        readonly _transports: ITransport[];
        readonly _aggregate: IAggregate;
        readonly _logger: Logger;
        _start(): Promise<void>;
        _handleCommand(commandMessage: CommandMessage<{}>): Promise<void>;
    };
    Type: string;
    _Instantiate(parentInjector: Injector): T;
} & T;
