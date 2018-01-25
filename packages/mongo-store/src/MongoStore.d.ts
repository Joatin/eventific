import { EventMessage, GetEventsResult, IStore, Logger } from '@eventific/core';
/**
 * The options that can be passed to this store
 *
 * @since 1.0.0
 */
export interface MongoStoreOptions {
    /**
     * The url to the mongo db server.
     *
     * If the param is not provided the store will read the MONGO_URL env variable.
     * If that is not set it will default to "mongodb://localhost:27017"
     *
     * @since 1.0.0
     */
    url?: string;
    /**
     * The name of the database to use. If does not exist it will be created
     *
     * If the param is not provided the env variable MONGO_DATABASE will be used.
     * If the variable is empty this param will default to "eventific-test"
     *
     * @since 1.0.0
     */
    database?: string;
}
/**
 * Mongo store
 *
 * @since 1.0.0
 */
export declare class MongoStore extends IStore {
    private _logger;
    readonly url: string;
    readonly database: string;
    private _client;
    private _db;
    constructor(options: MongoStoreOptions | undefined, _logger: Logger);
    /**
     * @inheritDoc
     */
    start(): Promise<void>;
    /**
     * @inheritDoc
     */
    getEvents<T>(aggregateName: string, aggregateId: string): Promise<GetEventsResult<T>>;
    /**
     * @inheritDoc
     */
    applyEvents<T>(aggregateName: string, events: any[], state?: T): Promise<void>;
    purgeAllSnapshots(aggregateName: string): Promise<void>;
    onEvent(aggregateName: string, eventName: string | null, callback: (event: EventMessage) => void): void;
    private _getCollection(aggregateName);
}
