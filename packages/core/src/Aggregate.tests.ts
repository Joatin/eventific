
import { Aggregate, IAggregate } from './Aggregate';

interface TestAggregate extends IAggregate{}

@Aggregate({
  name: 'Test',
  commands: [],
  events: []
})
class TestAggregate {

}


test('It should be defined', async () => {
  expect(Aggregate).toBeDefined();
});

test('It should have its name as a static property', async () => {
  expect(TestAggregate.Name).toEqual('Test');
});