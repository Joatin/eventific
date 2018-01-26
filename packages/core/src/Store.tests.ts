import { Store } from './Store';


test('It should be defined', async () => {
  expect(Store).toBeDefined();
});

test('store decorator should set static Name and property name', async () => {
  const decorated = Store({name: 'TEST123'})(class {});
  expect(decorated.Name).toEqual('TEST123');
  expect(decorated._CreateStore).toBeDefined();
  const decoratedInstance = decorated._CreateStore({
    newChildInjector: jest.fn(() => ({
      args: jest.fn(() => []),
      set: jest.fn()
    }))
  });
  expect(decoratedInstance.name).toEqual('TEST123');
});
