import { Logger } from './Logger';
export declare class InternalLogger extends Logger {
    readonly loggerName: string | undefined;
    readonly name: string;
    constructor(loggerName?: string | undefined);
    raw(message: string): void;
    error(message: string, ...meta: any[]): void;
    warn(message: string, ...meta: any[]): void;
    info(message: string, ...meta: any[]): void;
    verbose(message: string, ...meta: any[]): void;
    debug(message: string, ...meta: any[]): void;
    silly(message: string, ...meta: any[]): void;
    getNamed(name: string): Logger;
}
