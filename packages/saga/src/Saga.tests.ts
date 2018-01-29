import { Injector } from '@eventific/core';
import { MockTransport, MockStore } from '@eventific/test';
import { ISaga } from './ISaga';
import { Saga } from './Saga';
import { Context } from './Context';

let injector: Injector;
let testSagaClass;
let testSagaInstance: ISaga;
beforeEach(async () => {
  injector = new Injector();

  @Saga({
    aggregates: [
      {
        _InstantiateAggregate: jest.fn()
      }
    ],
    store: MockStore,
    transport: MockTransport
  })
  class TestSaga {}

  testSagaClass = TestSaga;
  testSagaInstance = TestSaga._Instantiate(injector);
  await testSagaInstance._start();
});

test('It should be defined', async () => {
  expect(Saga).toBeDefined();
  expect(ISaga).toBeDefined();
});

test.skip('sendCommand() should throw if the transport does not support sending commands', async () => {
  @Saga({
    aggregates: [
      {
        _InstantiateAggregate: jest.fn()
      }
    ],
    store: MockStore,
    transport: {
      _CreateTransport: jest.fn(() => ({

      }))
    }
  })
  class FailSaga {}
  testSagaInstance = FailSaga._Instantiate(injector);

  await expect(testSagaInstance.sendCommand({
    aggregateId: '27e7b187-5a11-41fe-afb7-a071c6c17b6d',
    command: 'TEST',
    header: {
      createdDate: Date.now()
    },
    content: {}
  })).rejects.toEqual(new Error('Transport does not support sending commands'));
});

test.skip('sendCommand() should pass the message to the transport', async () => {
  const mockSendCommand = jest.fn(async () => {});
  @Saga({
    aggregates: [
      {
        _InstantiateAggregate: jest.fn()
      }
    ],
    store: MockStore,
    transport: {
      _CreateTransport: jest.fn(() => ({
        sendCommand: mockSendCommand
      }))
    }
  })
  class FailSaga {}
  testSagaInstance = FailSaga._Instantiate(injector);

  const message = {
    aggregateId: '27e7b187-5a11-41fe-afb7-a071c6c17b6d',
    command: 'TEST',
    header: {
      createdDate: Date.now()
    },
    content: {}
  };

  await expect(testSagaInstance.sendCommand('test',message)).resolves.not.toBeDefined();
  expect(mockSendCommand).toBeCalledWith('test', message);
});
