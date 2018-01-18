import { EventMessage, GetEventsResultWithSnapshot, SnapshotStore } from '@eventific/core';
import { Db, MongoClient } from 'mongodb';

/**
 * The options that can be passed to this store
 *
 * @since 1.0.0
 */
export interface MongoStoreOptions {
  /**
   * The url to the mongo db server.
   *
   * If the param is not provided the store will read the MONGO_URL env variable. If that is not set it will default to "mongodb://localhost:27017"
   *
   * @since 1.0.0
   */
  mongoUrl?: string;

  /**
   * The name of the database to use. If does not exist it will be created
   *
   * If the param is not provided the env variable MONGO_DATABASE will be used. If the variable is empty this param will default to "eventific-test"
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
export class MongoStore extends SnapshotStore {

  private _options: {
    mongoUrl: string;
    database: string;
  } & MongoStoreOptions;
  private _client: MongoClient;
  private _db: Db;

  /**
   * Adds settings to the store
   *
   * @since 1.0.0
   *
   * @param {MongoStoreOptions} options Options provided to the store
   * @returns {{CreateStore: (() => SnapshotStore)}} A function to call in order to instantiate this store
   * @constructor
   */
  public static Settings(options: MongoStoreOptions): {
    CreateStore: () => SnapshotStore
  } {
    return {
      CreateStore(): SnapshotStore {
        return new MongoStore(options);
      }
    };
  }

  /**
   * Creates a new store instance
   *
   * @since 1.0.0
   *
   * @returns {SnapshotStore} A new store instance
   * @constructor
   */
  public static CreateStore(): SnapshotStore {
    return new MongoStore({});
  }

  constructor(
    options: MongoStoreOptions
  ) {
    super();
    this._options = {
      mongoUrl: process.env.MONGO_URL || 'mongodb://localhost:27017',
      database: process.env.MONGO_DATABASE || 'eventific-test',
      ...options
    };
  }

  /**
   * @inheritDoc
   */
  public async start(): Promise<void> {
    this._client = await MongoClient.connect(this._options.mongoUrl);

    this._db = this._client.db(this._options.database);
    // await this._createEventCollection(db, this.options.collection);
  }

  /**
   * @inheritDoc
   */
  public async getEvents<T extends object>(aggregateName: string, aggregateId: string): Promise<GetEventsResultWithSnapshot<T>> {
    const collection = this._getCollection(aggregateName);
    const events = await collection.find<EventMessage>({aggregateId}).toArray();
    return { events };
  }

  /**
   * @inheritDoc
   */
  public async applyEvents<T>(aggregateName: string, events: any[], state?: T): Promise<void> {
    const collection = this._getCollection(aggregateName);
    await collection.insertMany(events);
  }

  public async purgeAllSnapshots(aggregateName: string): Promise<void> {

  }

  private _getCollection(aggregateName: string) {
    return this._db.collection(aggregateName.toLowerCase());
  }

}
