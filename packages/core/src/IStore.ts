import { EventMessage } from './EventMessage';
import { GetEventsResult } from './GetEventsResult';
import { Injector } from './Injector';


/**
 * A interface for event stores.
 *
 * @since 1.0.0
 */
export abstract class IStore {
  public static _CreateStore: (injector: Injector) => IStore;
  public static Settings: (settings: object) => { _CreateStore: (injector: Injector) => IStore };

  /**
   * Starts this event store instance
   *
   * @since 1.0.0
   *
   * @returns {Promise<any>} A promise that returns when the store is ready
   */
  public abstract start(): Promise<any>;

  /**
   * Returns the events for the particular aggregate. The result might also contain a snapshot.
   *
   * @since 1.0.0
   *
   * @param {string} aggregateName The name of the aggregate to access
   * @param {string} aggregateId The id of the particular aggregate
   * @returns {Promise<GetEventsResult>} A promise that resolves with events and perhaps a snapshot.
   */
  public abstract getEvents<T>(aggregateName: string, aggregateId: string): Promise<GetEventsResult<T>>;

  public abstract applyEvents<T>(aggregateName: string, events: EventMessage[], state?: T): Promise<void>;
  public abstract purgeAllSnapshots(aggregateName: string): Promise<void>;
  public abstract onEvent(
    aggregateName: string,
    eventName: string | null,
    callback: (event: EventMessage) => Promise<void>
  ): void;
}
