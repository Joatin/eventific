import { EventMessage, GetEventsResult, InjectSettings, IStore, Logger, Store } from '@eventific/core';
import { Db, MongoClient } from 'mongodb';
import promiseRetry = require('promise-retry');

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
@Store({
  name: 'Mongo'
})
export class MongoStore extends IStore {

  public readonly url: string;
  public readonly database: string;

  private _client: MongoClient;
  private _db: Db;

  /* istanbul ignore next */
  constructor(
    @InjectSettings() options: MongoStoreOptions | undefined,
    private _logger: Logger
  ) {
    super();
    this.url = options && options.url || process.env.MONGO_URL || 'mongodb://localhost:27017';
    this.database = options && options.database || process.env.MONGO_DATABASE || 'eventific-test';
  }

  /**
   * @inheritDoc
   */
  public async start(): Promise<void> {
    try {
      this._client = await promiseRetry({
        maxTimeout: 10000
      }, (retry: any, count: number) => {
        return MongoClient.connect(this.url)
          .catch((err) => {
            this._logger.warn(
              `Failed to connect with mongodb, prevoius attempts: ${count}, error was ${err}`
            );
            retry(err);
          });
      });
    } catch (ex) {
      throw new Error('Could not connect to the mongo database');
    }
    this._db = this._client.db(this.database);

    // await this._createEventCollection(db, this.options.collection);
  }

  /**
   * @inheritDoc
   */
  public async getEvents<T>(aggregateName: string, aggregateId: string): Promise<GetEventsResult<T>> {
    const collection = await this._getCollection(aggregateName);
    const events = await collection.find<EventMessage>({ aggregateId }).toArray();
    for (const event of events) {
      delete (event as any)._id;
    }
    return { events };
  }

  /**
   * @inheritDoc
   */
  public async applyEvents<T>(aggregateName: string, events: any[], state?: T): Promise<void> {
    const collection = await this._getCollection(aggregateName);
    await collection.insertMany(events);
  }

  public async purgeAllSnapshots(aggregateName: string): Promise<void> {
    // TODO: Add snapshot functionality
  }

  public onEvent(aggregateName: string, eventName: string | null, callback: (event: EventMessage) => void): void {
    // TODO: Start listening on events, probably with a tailable cursor
  }

  private async _getCollection(aggregateName: string) {
    await this._db.createCollection(
      aggregateName.toLowerCase(),
      { capped: true, size: 1000000000, max: 50000000 }
    );
    await this._db.createIndex(
      aggregateName.toLowerCase(),
      { aggregateId: 1, eventId: 1 },
      { unique: true }
    );
    return this._db.collection(aggregateName.toLowerCase());
  }

}
