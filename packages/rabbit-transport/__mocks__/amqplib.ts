
export const mockSendToQueue = jest.fn();

export const mockCreateChannel = jest.fn(async () => ({
  sendToQueue: mockSendToQueue
}));

export const connect = jest.fn(async () => ({
  createChannel: mockCreateChannel
}));
