import { RabbitTransport } from '@eventific/rabbit-transport';
import { Injector, InternalLogger, Logger, CommandMessage } from '@eventific/core';


test('It should be possible to send and receive commands', async (done) => {
  jest.setTimeout(30000);
  const injector = new Injector();
  injector.set({provide: Logger, useConstant: new InternalLogger()});
  const transport = RabbitTransport._CreateTransport(injector);
  await transport.start();
  transport.onCommand('test', async (command: CommandMessage) => {
    expect(command.aggregateId).toEqual('0000');
    done();
  });

  await transport.sendCommand('test', {
    aggregateId: '0000',
    command: 'TEST'
  })
});
