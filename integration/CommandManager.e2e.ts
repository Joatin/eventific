import { CommandManager } from '@eventific/command';
import { bootstrap, IEvent } from '@eventific/core';
import { MockStore, MockTransport } from '@eventific/test';
import { Saga, ISaga, EventTrigger } from '@eventific/saga';
import { AccountAggregate } from './account/account.aggregate';
import { AddedEvent } from './account/added.event';

@Saga({
  aggregates: [
    AccountAggregate
  ],
  store: MockStore,
  transport: MockTransport
})
class TestSaga extends ISaga{
  @EventTrigger(AddedEvent)
  public async method(context: any) {
    console.log('RUNNING!!!')
  }
}

@CommandManager({
  aggregate: AccountAggregate,
  store: MockStore,
  transports: [
    MockTransport
  ]
})
class TestCommandManager {
}

test('When I send a command it should be inserted to the store', async () => {
  await bootstrap(TestCommandManager);
  await expect(MockTransport.Send({
    aggregateId: '27e7b187-5a11-41fe-afb7-a071c6c17b6d',
    command: 'CREATE',
    header: {
      createdDate: Date.now()
    }
  })).resolves.toEqual(undefined);
  const result = await MockStore.GetEvents(AccountAggregate.Name, '27e7b187-5a11-41fe-afb7-a071c6c17b6d');
  expect(result.events).toHaveLength(1);
});

test('It should not be possible to execute Create command twice on the same aggregate', async () => {
  await bootstrap(TestCommandManager);
  await expect(MockTransport.Send({
    aggregateId: '27e7b187-5a11-41fe-afb7-a071c6c17b6d',
    command: 'CREATE',
    header: {
      createdDate: Date.now()
    }
  })).resolves.toEqual(undefined);
  await expect(MockTransport.Send({
    aggregateId: '27e7b187-5a11-41fe-afb7-a071c6c17b6d',
    command: 'CREATE',
    header: {
      createdDate: Date.now()
    }
  })).rejects.toBeInstanceOf(Error);
});

test('Saga should trigger when a event is received', async () => {
  await expect(bootstrap(TestSaga)).resolves.not.toBeDefined();
  await MockStore.ApplyEvents(AccountAggregate.Name, [
    {
      aggregateId: '27e7b187-5a11-41fe-afb7-a071c6c17b6d',
      event: 'CREATED',
      eventId: 0,
      header: {
        createdDate: Date.now()
      },
      content: {}
    },
    {
      aggregateId: '27e7b187-5a11-41fe-afb7-a071c6c17b6d',
      event: 'ADDED',
      eventId: 1,
      header: {
        createdDate: Date.now()
      },
      content: {
        amount: 30
      }
    }
  ]);
  await MockStore.EmitEvents(AccountAggregate.Name, {
    aggregateId: '27e7b187-5a11-41fe-afb7-a071c6c17b6d',
    event: 'ADDED',
    eventId: 1,
    header: {
      createdDate: Date.now()
    },
    content: {
      amount: 30
    }
  })
});
