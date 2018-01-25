import { EventMessage } from './EventMessage';
import { Snapshot } from './Snapshot';
import { Injector } from './Injector';
export interface GetEventsResult<T> {
    events: EventMessage[];
    snapshot?: Snapshot<T>;
}
/**
 * A interface for event stores.
 *
 * @since 1.0.0
 */
export declare abstract class IStore {
    static _CreateStore: (injector: Injector) => IStore;
    static Settings: (settings: object) => {
        _CreateStore: (injector: Injector) => IStore;
    };
    /**
     * Starts this event store instance
     *
     * @since 1.0.0
     *
     * @returns {Promise<any>} A promise that returns when the store is ready
     */
    abstract start(): Promise<any>;
    /**
     * Returns the events for the particular aggregate. The result might also contain a snapshot.
     *
     * @since 1.0.0
     *
     * @param {string} aggregateName The name of the aggregate to access
     * @param {string} aggregateId The id of the particular aggregate
     * @returns {Promise<GetEventsResult>} A promise that resolves with events and perhaps a snapshot.
     */
    abstract getEvents<T>(aggregateName: string, aggregateId: string): Promise<GetEventsResult<T>>;
    abstract applyEvents<T>(aggregateName: string, events: EventMessage[], state?: T): Promise<void>;
    abstract purgeAllSnapshots(aggregateName: string): Promise<void>;
    abstract onEvent(aggregateName: string, eventName: string | null, callback: (event: EventMessage) => void): void;
}
export interface StoreOptions {
    name: string;
}
/**
 * Store decorator
 * @param {StoreOptions} options
 * @returns {<T extends {new(...args: any[]) => {}}>(Class: T) => T}
 * @constructor
 */
export declare function Store(options: StoreOptions): <T extends new (...args: any[]) => {}>(Class: T) => T;
