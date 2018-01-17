
import { Aggregate } from './Aggregate';

@Aggregate({
  name: 'Test',
  commands: [],
  events: []
})
class TestAggregate {

}

const test1 = new TestAggregate();

test('It should be defined', async () => {
  expect(Aggregate).toBeDefined();
});