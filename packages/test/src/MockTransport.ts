import { CommandMessage, ITransport, Transport } from '@eventific/core';

@Transport({
  name: 'MockTransport'
})
export class MockTransport implements ITransport {
  public static _instance: MockTransport;

  public static async Send(message: CommandMessage) {
    await MockTransport._instance.sendMessage(message);
  }

  private _handler: (data: CommandMessage) => Promise<void>;
  private _started = false;

  constructor() {
    MockTransport._instance = this;
  }

  public async start(): Promise<void> {
    if (!this._started) {
      this._started = true;
    } else {
      throw new Error('A transport can not be started twice');
    }
  }

  public async sendMessage(message: CommandMessage): Promise<void> {
    if (this._started) {
      await this._handler(message);
    } else {
      throw new Error('Not started');
    }
  }

  public onCommand(handler: (data: CommandMessage) => Promise<void>): void {
    if (this._started) {
      this._handler = handler;
    } else {
      throw new Error('Not started');
    }
  }

}
