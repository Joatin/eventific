import { Transport, CommandMessage } from '@eventific/core';

export class RestTransport implements Transport {
  public async start(): Promise<any> {
    return undefined;
  }

  public onCommand(handler: (data: CommandMessage) => Promise<void>): void {
  }

}