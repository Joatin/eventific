import { MongoStore } from '.';
import { MongoClient } from 'mongodb';
import { CollectionInstance, DbInstance } from '../__mocks__/mongodb';

test('MongoStore should be defined', async () => {
  expect(MongoStore).toBeDefined();
});

test('static CreateStore() should return a MongoStore instance', async () => {
  expect(MongoStore.CreateStore).toBeDefined();
  expect(MongoStore.CreateStore()).toBeInstanceOf(MongoStore);
});

test('start() should connect to the database', async () => {
  const store = MongoStore.CreateStore();
  expect(MongoClient.connect).not.toBeCalled();
  await store.start();
  expect(MongoClient.connect).toBeCalled();
});

test('start() should reject with an error if it fails to connect to the database', async () => {
  // t.context.sandbox.stub(MongoClient, 'connect').rejects(new Error('fail'));
  const originalMockImp = MongoClient.connect;
  MongoClient.connect = jest.fn(async () => {
    throw new Error('Error');
  });

  const store = MongoStore.CreateStore();
  expect(store.start()).rejects.toEqual(new Error('Could not connect to the mongo database'));
  MongoClient.connect = originalMockImp;
});

test('getEvents() should return a list of events', async () => {
  const store = MongoStore.CreateStore();
  await store.start();
  const result = await store.getEvents<any>('Test', '0000');
  expect(result.events).toHaveLength(3);
});

test('getEvents() should query a collection with correct name', async () => {
  const store = MongoStore.CreateStore();
  await store.start();
  await store.getEvents<any>('Test', '0000');
  expect(DbInstance.collection).toBeCalledWith('test');
});

test('applyEvents() should insert the events to the collection', async () => {
  const store = MongoStore.CreateStore();
  await store.start();
  await store.applyEvents('Test', [{eventId: 1}]);
  expect(CollectionInstance.insertMany).toBeCalledWith([{eventId: 1}]);

});
