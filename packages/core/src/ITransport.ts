import { CommandMessage } from './CommandMessage';
import { Injector } from './Injector';


export abstract class ITransport {
  public static _CreateTransport: (injector: Injector) => ITransport;
  public static Settings: (settings: object) => { _CreateTransport: (injector: Injector) => ITransport };
  public abstract start(): Promise<void>;
  public onCommand?(aggregateName: string, handler: (data: CommandMessage) => Promise<void>): void;
  public sendCommand?(aggregateName: string, data: CommandMessage): Promise<void>;
}
