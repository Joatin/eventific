
import { Aggregate } from './Aggregate';

@Aggregate({
  name: 'Test',
  commands: ''
})
class TestAggregate {

}

test('It should be defined', async () => {
  expect(Aggregate).toBeDefined();
});