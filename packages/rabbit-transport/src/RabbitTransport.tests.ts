import { RabbitTransport } from './RabbitTransport';
import { Injector, Logger, InternalLogger } from '@eventific/core';


test('It should be defined', async () => {
  expect(RabbitTransport).toBeDefined();
});

test('sendCommand() should add command to queue', async () => {
  const injector = new Injector();
  injector.set({ provide: Logger, useConstant: new InternalLogger()});
  const transport = RabbitTransport._CreateTransport(injector);
  await transport.start();

  await transport.sendCommand('Test', {
    aggregateId: '1234'
  })
});
