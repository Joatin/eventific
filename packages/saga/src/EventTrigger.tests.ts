import { EventTrigger } from './EventTrigger';

test('It should be defined', async () => {
  expect(EventTrigger).toBeDefined();
});

test('It should throw if no triggers are passed', async () => {
  expect(() => {
    EventTrigger();
  }).toThrow()
});

test('It should add a prop with definitions about the triggers', () => {
  const testTrigger = 'TRIGGER';
  const fakeClass = {};
  const method = 'superFancyTriggerHandler';
  EventTrigger(testTrigger)(fakeClass, method, {});
  expect(fakeClass._triggerDefinitions).toBeDefined();
  expect(fakeClass._triggerDefinitions).toHaveLength(1);
  expect(fakeClass._triggerDefinitions[0].triggers).toHaveLength(1);
  expect(fakeClass._triggerDefinitions[0].triggers[0]).toEqual(testTrigger);
  expect(fakeClass._triggerDefinitions[0].propertyKey).toEqual(method);
});
