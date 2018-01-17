import { BaseEvent, Event } from './Event';

@Event({
  name: 'TEST123'
})
class TestEvent extends BaseEvent<string> {
  apply() {}
}

const mock = new TestEvent();

test('It should be defined', async () => {
  expect(TestEvent).toBeDefined();
  expect(mock.name).toBeDefined();
  expect(TestEvent.Name).toBeDefined();
});