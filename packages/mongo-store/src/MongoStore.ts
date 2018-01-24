import { EventMessage, IStore, Store, GetEventsResult } from '@eventific/core';
import { Db, MongoClient } from 'mongodb';
const promiseRetry = require('promise-retry');

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
  url?: string;

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
@Store({
  name: 'Mongo'
})
export class MongoStore extends IStore {

  public readonly url: string;
  public readonly database: string;

  private _client: MongoClient;
  private _db: Db;

  /**
   * Creates a new store instance
   *
   * @since 1.0.0
   *
   * @returns {MongoStore} A new store instance
   * @constructor
   */
  public static CreateStore(): MongoStore {
    return new MongoStore({});
  }

  /* istanbul ignore next */
  constructor(
    options: MongoStoreOptions
  ) {
    super();
    this.url = options.url || process.env.MONGO_URL || 'mongodb://localhost:27017';
    this.database = options.database || process.env.MONGO_DATABASE || 'eventific-test';
  }

  /**
   * @inheritDoc
   */
  public async start(): Promise<void> {
    try {
      this._client = await promiseRetry((retry: any) => {
        return MongoClient.connect(this.url)
          .catch(retry);
      });
    } catch(ex) {
      throw new Error('Could not connect to the mongo database');
    }
    this._db = this._client.db(this.database);

    // await this._createEventCollection(db, this.options.collection);
  }

  /**
   * @inheritDoc
   */
  public async getEvents<T>(aggregateName: string, aggregateId: string): Promise<GetEventsResult<T>> {
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

  public onEvent(aggregateName: string, eventName: string | null, callback: (event: EventMessage) => void): void {

  }

  private _getCollection(aggregateName: string) {
    return this._db.collection(aggregateName.toLowerCase());
  }

}
