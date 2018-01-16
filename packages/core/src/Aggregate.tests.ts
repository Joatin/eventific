
import { Aggregate } from './Aggregate';

@Aggregate({
  name: 'Test',
  commands: ''
})
class TestAggregate {

}

const test: TestAggregate = TestAggregate;
console.log(test);

test('It should be defined', async () => {
  expect(test).toBeDefined();
});