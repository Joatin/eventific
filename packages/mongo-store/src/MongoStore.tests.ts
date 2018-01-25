import { MongoStore } from './MongoStore';
import { MongoClient } from 'mongodb';
import { CollectionInstance, DbInstance } from '../__mocks__/mongodb';
import { Injector, Logger, InternalLogger } from '@eventific/core';

test('MongoStore should be defined', async () => {
  expect(MongoStore).toBeDefined();
});

test('static CreateStore() should return a MongoStore instance', async () => {
  const injector = new Injector();
  injector.set({provide: Logger, useConstant: new InternalLogger()});
  expect(MongoStore._CreateStore).toBeDefined();
  expect(MongoStore._CreateStore(injector)).toBeInstanceOf(MongoStore);
});

test('start() should connect to the database', async () => {
  const injector = new Injector();
  injector.set({provide: Logger, useConstant: new InternalLogger()});
  const store = MongoStore._CreateStore(injector);
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
  const injector = new Injector();
  injector.set({provide: Logger, useConstant: new InternalLogger()});

  const store = MongoStore._CreateStore(injector);
  await expect(store.start()).rejects.toEqual(new Error('Could not connect to the mongo database'));
  MongoClient.connect = originalMockImp;
});

test('getEvents() should return a list of events', async () => {
  const injector = new Injector();
  injector.set({provide: Logger, useConstant: new InternalLogger()});
  const store = MongoStore._CreateStore(injector);
  await store.start();
  const result = await store.getEvents<any>('Test', '0000');
  expect(result.events).toHaveLength(3);
});

test('getEvents() should query a collection with correct name', async () => {
  const injector = new Injector();
  injector.set({provide: Logger, useConstant: new InternalLogger()});
  const store = MongoStore._CreateStore(injector);
  await store.start();
  await store.getEvents<any>('Test', '0000');
  expect(DbInstance.collection).toBeCalledWith('test');
});

test('applyEvents() should insert the events to the collection', async () => {
  const injector = new Injector();
  injector.set({provide: Logger, useConstant: new InternalLogger()});
  const store = MongoStore._CreateStore(injector);
  await store.start();
  await store.applyEvents('Test', [{eventId: 1}]);
  expect(CollectionInstance.insertMany).toBeCalledWith([{eventId: 1}]);

});
