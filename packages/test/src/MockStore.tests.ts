import { Injector, EventMessage } from '@eventific/core';
import { MockStore } from '.';

let injector: Injector;
let mockStore: MockStore;
beforeEach(async () => {
  injector = new Injector();
  mockStore = MockStore._CreateStore(injector);
  await mockStore.start();
});

test('It should be defined', async () => {
  expect(MockStore).toBeDefined();
  expect(MockStore._CreateStore).toBeDefined();
  expect(MockStore.Name).toBeDefined();
});

test('applyEvents() should add events to the store', async () => {
  await mockStore.applyEvents('test', [<EventMessage>{
    aggregateId: '0',
    eventId: 0
  }]);
  const result = await mockStore.getEvents('test', '0');
  expect(result.events).toHaveLength(1);
  expect(result.events[0].eventId).toEqual(0);
});

test('It should be possible to get all events from current instance from static GetEvents()', async () => {
  await mockStore.applyEvents('test', [<EventMessage>{
    aggregateId: '0',
    eventId: 0
  }]);
  const result = await MockStore.GetEvents('test', '0');
  expect(result.events).toHaveLength(1);
  expect(result.events[0].eventId).toEqual(0);
});

test('It should be possible to insert events from static ApplyEvents()', async () => {
  await MockStore.ApplyEvents('test', [<EventMessage<any>>{
    aggregateId: '0',
    eventId: 0
  }]);
  const result = await mockStore.getEvents('test', '0');
  expect(result.events).toHaveLength(1);
  expect(result.events[0].eventId).toEqual(0);
});

test('static EmitEvents() should call registered callbacks given that the params match', async () => {
  const catchAll = jest.fn();
  const catchEvent = jest.fn();
  const catchNone = jest.fn();

  mockStore.onEvent('Test', '', catchAll);
  mockStore.onEvent('Test', 'TEST_EVENT', catchEvent);
  mockStore.onEvent('Unknown', '', catchNone);

  await MockStore.EmitEvents('Test',
    <EventMessage<any>>{
      aggregateId: '0',
      eventId: 0,
      event: 'TEST_EVENT'
    },
    <EventMessage<any>>{
      aggregateId: '0',
      eventId: 2,
      event: 'OTHER_EVENT'
    }
  );
  await MockStore.EmitEvents('Other',
    <EventMessage<any>>{
      aggregateId: '0',
      eventId: 0,
      event: 'TEST_EVENT'
    }
  );

  expect(catchAll).toHaveBeenCalledTimes(2);
  expect(catchEvent).toHaveBeenCalledTimes(1);
  expect(catchNone).toHaveBeenCalledTimes(0);
});

test('It should be possible to register a callback for events', async () => {
  const catchEvent = jest.fn();
  mockStore.onEvent('Test', 'TEST_EVENT', catchEvent);
  await MockStore.EmitEvents('Test',
    <EventMessage<any>>{
      aggregateId: '0',
      eventId: 0,
      event: 'TEST_EVENT'
    }
  );
  expect(catchEvent).toHaveBeenCalledTimes(1);
});

test('start() should throw if already started', async () => {
  await expect(mockStore.start()).rejects.toBeInstanceOf(Error);
});

test('getEvents() should throw if store is not started', async () => {
  mockStore = MockStore._CreateStore(injector);
  await expect(mockStore.getEvents('', '')).rejects.toBeInstanceOf(Error);
});

test('applyEvents() should throw if store is not started', async () => {
  mockStore = MockStore._CreateStore(injector);
  await expect(mockStore.applyEvents('', [])).rejects.toBeInstanceOf(Error);
});

test('purgeAllSnapshots() should throw if store is not started', async () => {
  mockStore = MockStore._CreateStore(injector);
  await expect(mockStore.purgeAllSnapshots('')).rejects.toBeInstanceOf(Error);
});

test('purgeAllSnapshots() should throw if the provided aggregateName is not a string', async () => {
  await expect(mockStore.purgeAllSnapshots([])).rejects.toBeInstanceOf(Error);
});

test('onEvent() should throw if store is not started', async () => {
  mockStore = MockStore._CreateStore(injector);
  await expect(() => {
    mockStore.onEvent('', '', () => {})
  }).toThrow();
});

test('getEvents() should return an empty list if no events exists', async () => {
  await expect(mockStore.getEvents('Test', '0000')).resolves.toEqual({events: []});
});
