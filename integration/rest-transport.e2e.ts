import { RestTransport } from '@eventific/rest-transport';
import { Injector, InternalLogger, Logger, CommandMessage } from '@eventific/core';
import axios from 'axios';

test('It should receive commands', async (done) => {
  const injector = new Injector();
  injector.set({provide: Logger, useConstant: new InternalLogger()});
  const transport = RestTransport._CreateTransport(injector);
  await transport.start();
  transport.onCommand(async (command: CommandMessage) => {
    expect(command.aggregateId).toEqual('0000');
    done();
  });

  await axios.post('http://localhost:1337/commands', {
    aggregateId: '0000',
    command: 'TEST'
  })
});
