
export const mockSendToQueue = jest.fn();

export const mockCreateChannel = jest.fn(async () => ({
  sendToQueue: mockSendToQueue,
  prefetch: jest.fn(async () => {}),
  assertQueue: jest.fn(async () => {})

}));

export const connect = jest.fn(async () => ({
  createChannel: mockCreateChannel
}));
