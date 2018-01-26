import { MongoStore } from '@eventific/mongo-store';
import { Injector, InternalLogger, Logger } from '@eventific/core';

test('MongoStore should add events to the database', async () => {
  jest.setTimeout(30000);
  const injector = new Injector();
  injector.set({provide: Logger, useConstant: new InternalLogger()});
  const store = MongoStore._CreateStore(injector);
  await store.start();
  const event = {
    event: 'TEST',
    eventId: 0,
    aggregateId: '0'
  };
  await store.applyEvents('Test', [
    {...event}
  ]);
  const result = await store.getEvents('Test', '0');
  await expect(result.events).toEqual([event]);
});

test('MongoStore should handle a bunch of events', async () => {
  jest.setTimeout(30000);
  const injector = new Injector();
  injector.set({provide: Logger, useConstant: new InternalLogger()});
  const store = MongoStore._CreateStore(injector);
  await store.start();
  const events = [];
  for(let i = 0; i <10000; i++){
    events.push({
      event: 'TEST',
      eventId: i,
      aggregateId: '1234',
      header: {
        createdDate: new Date()
      },
      content: {
        someData: 'dgsdfgfdgsjkfdgkjdfsghkjdfghkjfdhgjksfdghkjdfghskdfljghfkjdsghdfjskghfdkjghkjfsd'
      }
    })
  }
  await store.applyEvents('Test', events);
  const result = await store.getEvents('Test', '1234');
  await expect(result.events).toHaveLength(10000);
});

test('onEvent() should pass messages when inserted to the db, to the provided callback', async (done) => {
  jest.setTimeout(30000);
  const injector = new Injector();
  injector.set({provide: Logger, useConstant: new InternalLogger()});
  const store = MongoStore._CreateStore(injector);
  await store.start();
  store.onEvent('Test', 'TEST', (event) => {
    console.log(event);
    if(event.aggregateId === '123') {
      done();
    }
  });
  const event = {
    event: 'TEST',
    eventId: 0,
    aggregateId: '123'
  };
  await store.applyEvents('Test', [
    {...event}
  ]);
});
