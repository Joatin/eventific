import { BaseEvent, Event } from './Event';

@Event({
  name: 'TEST123'
})
class TestEvent extends BaseEvent<string> {
  apply() {}
}

test('It should be defined', async () => {
  expect(TestEvent).toBeDefined();
  expect(TestEvent.Name).toBeDefined();
});