import { StoreOptions } from './StoreOptions';
/**
 * Store decorator
 * @param {StoreOptions} options
 * @returns {<T extends {new(...args: any[]) => {}}>(Class: T) => T}
 * @constructor
 */
export declare function Store(options: StoreOptions): <T extends new (...args: any[]) => {}>(Class: T) => T;
