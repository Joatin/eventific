import 'reflect-metadata';
import { Injector } from './Injector';
import { InternalLogger } from './InternalLogger';
import { Inject } from './Inject';

class TestClass {}

test('It should be defined', async () => {
  expect(Injector).toBeDefined();
});

test('newChildInjector should return a new injector', async () => {
  const parent = new Injector();
  const child = parent.newChildInjector();
  expect(child).toBeInstanceOf(Injector);
  parent.set({provide: 'PARENT_TEST', useConstant: 1337});
  child.set({provide: 'CHILD_TEST', useConstant: 7331});
  expect(child.getOptional('PARENT_TEST')).toEqual(1337);
  expect(child.getOptional('CHILD_TEST')).toEqual(7331);
  expect(parent.getOptional('CHILD_TEST')).toEqual(undefined);
});

test('get() should throw an error if the requested type does not exist', async () => {
  const injector = new Injector();
  expect(() => {
    injector.get('UNDEFINED')
  }).toThrow()
});

test('if you provide a class to set() it should use that class type as key', async () => {
  const injector = new Injector();
  injector.set(TestClass);
  expect(injector.get(TestClass)).toBeInstanceOf(TestClass);
});

test('it should be possible to provide a class with provide key', async () => {
  const injector = new Injector();
  injector.set({ provide: 'TEST', useClass: TestClass});
  expect(injector.get('TEST')).toBeInstanceOf(TestClass);
});

test('it should be possible to provide a class without provide key, in that case the class should be used as key', async () => {
  const injector = new Injector();
  injector.set({ useClass: TestClass});
  expect(injector.get(TestClass)).toBeInstanceOf(TestClass);
});

test('it should not be possible to use numbers as provide key', async () => {
  const injector = new Injector();
  expect(() => {
    injector.set({ provide: 1337, useConstant: 'TEST'});
  }).toThrow();
});

test('it should not be possible to use booleans as provide key', async () => {
  const injector = new Injector();
  expect(() => {
    injector.set({ provide: true, useConstant: 'TEST'});
  }).toThrow();
});

test('it should not be possible to use null as provide key', async () => {
  const injector = new Injector();
  expect(() => {
    injector.set({ provide: null, useConstant: 'TEST'});
  }).toThrow();
});

test('it should be possible to use symbols as provide key', async () => {
  const injector = new Injector();
  expect(() => {
    injector.set({ provide: Symbol('TEST'), useConstant: 'TEST'});
  }).not.toThrow();
});

test('args() should return the constructor params for a class', async () => {
  const injector = new Injector();
  injector.set(TestClass);
  class SubTestClass {
    constructor(@Inject(TestClass) test: TestClass) {}
  }
  const args = injector.args(SubTestClass);
  expect(args).toHaveLength(1);
  expect(args[0]).toBeInstanceOf(TestClass);
});
