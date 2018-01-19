import { bootstrap } from './bootstrap';


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

test('It should throw an error if _Instantiate() does not exists', async () => {
  expect(bootstrap({})).rejects.toBeInstanceOf(Error);
});