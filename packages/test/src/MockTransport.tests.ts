import { Injector } from '@eventific/core';
import { MockTransport } from '.';


let injector: Injector;
let mockTransport: MockTransport;
beforeEach(async () => {
  injector = new Injector();
  mockTransport = MockTransport._CreateTransport(injector);
  await mockTransport.start();
});

test('It should be defined', async () => {
  expect(MockTransport).toBeDefined();
});

test('start() should throw if called twice', async () => {
  await expect(mockTransport.start()).rejects.toBeInstanceOf(Error);
});

test('sendMessage() should pass the message to the registered handler', async () => {
  const message = {
    aggregateId: '0000'
  };
  const handler = jest.fn(async () => {});
  mockTransport.onCommand(handler);
  await mockTransport.sendMessage(message);

  expect(handler).toBeCalledWith(message);
});

test('It should be possible to send a message through static Send() to local instance', async () => {
  const message = {
    aggregateId: '0000'
  };
  const handler = jest.fn(async () => {});
  mockTransport.onCommand(handler);
  await MockTransport.Send(message);

  expect(handler).toBeCalledWith(message);
});

test('sendMessage() should throw if transport is not started', async () => {
  mockTransport = MockTransport._CreateTransport(injector);
  await expect(mockTransport.sendMessage({})).rejects.toBeInstanceOf(Error);
});

test('onCommand() should throw if transport is not started', async () => {
  mockTransport = MockTransport._CreateTransport(injector);
  await expect(() => {
    mockTransport.onCommand(() => {})
  }).toThrow();
});
