
const mockUse = jest.fn();
const mockListenOn = jest.fn((name: string, callback: () => void) => {callback()});
const mockListen = jest.fn(() => ({
  on: mockListenOn
}));

const mock = jest.fn(() => ({
  use: mockUse,
  listen: mockListen
}));

mock.mockUse = mockUse;
mock.mockListenOn = mockListenOn;
mock.mockListen = mockListen;
export = mock;
