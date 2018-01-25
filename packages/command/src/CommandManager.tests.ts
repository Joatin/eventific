import { CommandManager, ICommandManager } from './CommandManager';
import { Injector } from '@eventific/core';


let originalStdoutWrite;
let originalStderrWrite;
let injector: Injector;
let commandManagerClass;
let commandManagerInstance: ICommandManager;
let mockAggregateHandleCommand;
let mockStoreStart;
let mockTransportStart;
let mockTransport;
beforeEach(async () => {
  originalStderrWrite = process.stderr.write;
  process.stderr.write = jest.fn();
  originalStdoutWrite = process.stdout.write;
  process.stdout.write = jest.fn();

  const mockAggregate = jest.fn();
  mockAggregateHandleCommand = jest.fn();
  mockAggregate._InstantiateAggregate = jest.fn(() => ({
    handleCommand: mockAggregateHandleCommand
  }));
  mockAggregate.Type = 'Aggregate';
  mockAggregate.Name = 'Test';

  const mockStore = jest.fn();
  mockStoreStart = jest.fn(async () => {});
  mockStore._CreateStore = jest.fn(() => ({
    start: mockStoreStart
  }));

  mockTransport = jest.fn();
  mockTransportStart = jest.fn(async () => {});
  mockTransport._CreateTransport = jest.fn(() => ({
    start: mockTransportStart
  }));
  @CommandManager({
    aggregate: mockAggregate,
    store: mockStore,
    transports: [mockTransport]
  })
  class MockCommandManager extends ICommandManager {}

  commandManagerClass = MockCommandManager;
  injector = new Injector();
  commandManagerInstance = MockCommandManager._Instantiate(injector)
});

afterEach(async () => {
  process.stdout.write = originalStdoutWrite;
  process.stderr.write = originalStderrWrite;
});

test('It should be defined', async () => {
  expect(CommandManager).toBeDefined();
});

test('It should have a static prop Type defined', async () => {
  expect(commandManagerClass.Type).toEqual('CommandManager');
});

test('_start() should call onInit() if defined', async () => {
  commandManagerInstance.onInit = jest.fn();
  await expect(commandManagerInstance._start()).resolves.not.toBeDefined();
});

test('_start() should call start on store', async () => {
  await expect(commandManagerInstance._start()).resolves.not.toBeDefined();
  expect(mockStoreStart).toHaveBeenCalled();
});

test('_start() should call start on all transports', async () => {
  await expect(commandManagerInstance._start()).resolves.not.toBeDefined();
  expect(mockTransportStart).toHaveBeenCalled();
});

test('_start() should register an onCommand callback on all transports with method onCommand defined', async () => {
  const mockOnCommand = jest.fn();
  mockTransport._CreateTransport.mockImplementation(() => ({
    start: mockTransportStart,
    onCommand: mockOnCommand
  }));
  commandManagerInstance = commandManagerClass._Instantiate(injector);
  await expect(commandManagerInstance._start()).resolves.not.toBeDefined();
  expect(mockOnCommand).toHaveBeenCalled();
});

test('The onCommand callback should pass the message to the aggregate', async () => {
  const mockOnCommand = jest.fn();
  mockTransport._CreateTransport.mockImplementation(() => ({
    start: mockTransportStart,
    onCommand: mockOnCommand
  }));
  commandManagerInstance = commandManagerClass._Instantiate(injector);
  await commandManagerInstance._start();
  expect(mockOnCommand.mock.calls[0][0]).toBeDefined();
  const callback = mockOnCommand.mock.calls[0][0];
  await expect(callback({
    aggregateId: '0'
  })).resolves.not.toBeDefined();
  expect(mockAggregateHandleCommand).toHaveBeenCalledWith({
    aggregateId: '0'
  });
});
