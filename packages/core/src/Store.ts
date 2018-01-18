
import { EventMessage } from './EventMessage';
import { Snapshot } from './Snapshot';

export interface StoreParams {

}

export interface GetEventsResult {
  events: EventMessage[];
}

export interface GetEventsResultWithSnapshot<T extends object> extends GetEventsResult {
  snapshot?: Snapshot<T>;
  events: EventMessage[];
}

/**
 * A interface for event stores.
 *
 * @since 1.0.0
 */
export abstract class Store {

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
  public abstract getEvents(aggregateName: string, aggregateId: string): Promise<GetEventsResult>;

  public abstract applyEvents(aggregateName: string, events: any[]): Promise<void>;
}

export abstract class SnapshotStore extends Store {

  public abstract getEvents<T extends object>(aggregateName: string, aggregateId: string): Promise<GetEventsResultWithSnapshot<T>>;

  public abstract applyEvents<T extends object>(aggregateName: string, events: any[], state?: T): Promise<void>;
  public abstract purgeAllSnapshots(aggregateName: string): Promise<void>;
}
