import { EventHandler } from './EventHandler';
import { IEventHandler } from './IEventHandler';

@EventHandler({
  name: 'TEST123'
})
class TestEvent extends IEventHandler<string> {
  async handle(): Promise<any> {
    return null;
  }
}

test('It should be defined', async () => {
  expect(IEventHandler).toBeDefined();
  expect(EventHandler).toBeDefined();
});
