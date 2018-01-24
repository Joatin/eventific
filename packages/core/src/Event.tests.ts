import { IEventHandler, EventHandler } from './Event';

@EventHandler({
  name: 'TEST123'
})
class TestEvent extends IEventHandler<string> {
  async apply(): Promise<any> {
    return null;
  }
}

test('It should be defined', async () => {
  expect(IEventHandler).toBeDefined();
  expect(EventHandler).toBeDefined();
});
