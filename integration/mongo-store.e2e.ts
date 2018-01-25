import { MongoStore } from '@eventific/mongo-store';
import { Injector } from '@eventific/core';

test('MongoStore should add events to the database', async () => {
  const injector = new Injector();
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
  const injector = new Injector();
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
