import { CommandManager } from './CommandManager';


let originalStdoutWrite;
let originalStderrWrite;
beforeEach(async () => {
  originalStderrWrite = process.stderr.write;
  process.stderr.write = jest.fn();
  originalStdoutWrite = process.stdout.write;
  process.stdout.write = jest.fn();
});

afterEach(async () => {
  process.stdout.write = originalStdoutWrite;
  process.stderr.write = originalStderrWrite;
});

test('It should be defined', async () => {
  expect(CommandManager).toBeDefined();
});
