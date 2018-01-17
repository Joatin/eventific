import { CommandMessage } from './CommandMessage';

export interface Transport {
  start(): Promise<any>
  onCommand(handler: (data: CommandMessage) => Promise<void>): void
}
