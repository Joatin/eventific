import { RestTransport } from './RestTransport';
import { Injector } from '@eventific/core';
import { mockListen } from '../__mocks__/koa';


let injector: Injector;
let instance: RestTransport;
beforeEach(async () => {
  injector = new Injector();
  instance = RestTransport._CreateTransport(injector);
});

test('It should be defined', async () => {
  expect(RestTransport).toBeDefined();
});

test('start() should make it listen to the provided port', async () => {
  expect(mockListen).not.toBeCalled();
  await instance.start();
  expect(mockListen).toBeCalled();
  expect(mockListen).toBeCalledWith(1337);
  instance = RestTransport.Settings({
    port: 3000
  })._CreateTransport(injector);
  mockListen.mockClear();
  expect(mockListen).not.toBeCalled();
  await instance.start();
  expect(mockListen).toBeCalled();
  expect(mockListen).toBeCalledWith(3000);
});
