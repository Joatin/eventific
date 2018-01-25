import { TransportOptions } from './TransportOptions';
export declare function Transport(options: TransportOptions): <T extends new (...args: any[]) => {}>(Class: T) => T;
