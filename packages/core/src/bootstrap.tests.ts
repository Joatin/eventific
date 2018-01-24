import { bootstrap } from './bootstrap';

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
  expect(bootstrap).toBeDefined();
});

test('It should call _Instantiate() and then start', async () => {
  const mockStart = jest.fn(async () => {

  });
  const mock = {
    _Instantiate: jest.fn(() => ({
      _start: mockStart
    }))
  };
  await bootstrap(mock);
  expect(mock._Instantiate).toBeCalled();
  expect(mockStart).toBeCalled();
});

test('It should throw error if _Instantiate() does not exists', async () => {
  await expect(bootstrap({} as any)).rejects.toBeInstanceOf(Error);
  expect(process.stderr.write).toBeCalled();
});
