import { EventMessage, GetEventsResult, IStore, Store } from '@eventific/core';
import * as Joi from 'joi';

/**
 * A test utility that simulates a store. This construct uses an in memory storage mechanism for events and can
 * therefor not be shared between separate processes.
 *
 * @since 1.0.0
 */
@Store({
  name: 'Mock'
})
export class MockStore extends IStore {
  public static _instance: MockStore;

  public static async GetEvents<T>(aggregateName: string, aggregateId: string): Promise<GetEventsResult<T>> {
    return await MockStore._instance.getEvents<T>(aggregateName, aggregateId);
  }

  public static async EmitEvents(aggregateName: string, ...events: EventMessage[]): Promise<void> {
    const eventMap = MockStore._instance._callbacks.get(aggregateName);
    if (eventMap) {
      for (const event of events) {
        const callback1 = eventMap.get('');
        if (callback1) {
          callback1(event);
        }
        const callback2 = eventMap.get(event.event);
        if (callback2) {
          callback2(event);
        }
      }
    }
  }

  public static async ApplyEvents(aggregateName: string, events: EventMessage[]): Promise<void> {
    return await MockStore._instance.applyEvents(aggregateName, events);
  }

  private _started = false;
  private _events = new Map<string, EventMessage[]>();
  private _callbacks = new Map<string, Map<string, (event: EventMessage) => void>>();

  constructor() {
    super();
    MockStore._instance = this;
  }
  /**
   * @inheritDoc
   */
  public async start(): Promise<void> {
    if (!this._started) {
      this._started = true;
    } else {
      throw new Error('A store can not be started twice');
    }
  }

  public async getEvents<T>(aggregateName: string, aggregateId: string): Promise<GetEventsResult<T>> {
    if (this._started) {
      Joi.assert(aggregateName, Joi.string());
      Joi.assert(aggregateId, Joi.string());
      const events = (this._events.get(aggregateName) || []).filter((x) => x.aggregateId === aggregateId);
      return {
        events
      };
    } else {
      throw new Error('Not started');
    }
  }

  public async applyEvents(aggregateName: string, events: EventMessage[]): Promise<void> {
    if (this._started) {
      Joi.assert(aggregateName, Joi.string());
      Joi.assert(events, Joi.array().min(1));

      const list = this._events.get(aggregateName) || [];
      list.push(...events);
      this._events.set(aggregateName, list);
    } else {
      throw new Error('Not started');
    }
  }

  public async purgeAllSnapshots(aggregateName: string): Promise<void> {
    if (this._started) {
      Joi.assert(aggregateName, Joi.string());
    } else {
      throw new Error('Not started');
    }
  }

  public onEvent(aggregateName: string, eventName: string, callback: (event: EventMessage) => void): void {
    if (this._started) {
      const eventMap = this._callbacks.get(aggregateName) || new Map();
      eventMap.set(eventName, callback);
      this._callbacks.set(aggregateName, eventMap);
    } else {
      throw new Error('Not started');
    }
  }
}
