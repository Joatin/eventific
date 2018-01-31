import { EventMessage } from './EventMessage';
import { GetEventsResult } from './GetEventsResult';
import { Injector } from './Injector';


/**
 * A interface for event stores.
 * @public
 */
export abstract class IStore {
  public static _CreateStore: (injector: Injector) => IStore;
  public static Settings: (settings: object) => { _CreateStore: (injector: Injector) => IStore };

  /**
   * Starts this event store instance
   *
   *
   */
  public abstract start(): Promise<any>;

  /**
   * Returns the events for the particular aggregate. The result might also contain a snapshot.
   *
   *
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
