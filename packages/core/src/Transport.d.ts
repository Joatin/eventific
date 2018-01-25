import { CommandMessage } from './CommandMessage';
import { Injector } from './Injector';
export declare abstract class ITransport {
    static _CreateTransport: (injector: Injector) => ITransport;
    static Settings: (settings: object) => {
        _CreateTransport: (injector: Injector) => ITransport;
    };
    abstract start(): Promise<void>;
    onCommand?(handler: (data: CommandMessage) => Promise<void>): void;
    sendCommand?(data: CommandMessage): Promise<void>;
}
export interface TransportOptions {
    name: string;
}
export declare function Transport(options: TransportOptions): <T extends new (...args: any[]) => {}>(Class: T) => T;
