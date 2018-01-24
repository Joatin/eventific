import 'reflect-metadata';
import { Aggregate, IAggregate } from './Aggregate';
import { Injector } from './Injector';
import { Store } from './Store';



let originalStdoutWrite;
let originalStderrWrite;
let testAggregateClass;
beforeEach(async () => {
  originalStderrWrite = process.stderr.write;
  process.stderr.write = jest.fn();
  originalStdoutWrite = process.stdout.write;
  process.stdout.write = jest.fn();

  @Aggregate({
    name: 'Test',
    commandHandlers: [],
    eventHandlers: []
  })
  class TestAggregate {
    name = 'test';
    constructor() {

    }
  }

  testAggregateClass = TestAggregate;

  class MockClass {

  }

  const injector = new Injector();
  injector.set({provide: Store, useClass: MockClass});
  injector.set({provide: 'Test', useClass: MockClass});
  const agg = (TestAggregate as any)._InstantiateAggregate(injector);
});

afterEach(async () => {
  process.stdout.write = originalStdoutWrite;
  process.stderr.write = originalStderrWrite;
});

test('It should be defined', async () => {
  expect(Aggregate).toBeDefined();
});

test('It should have its name as a static property', async () => {
  expect((testAggregateClass as any).Name).toEqual('Test');
});
