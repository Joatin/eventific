import { CommandMessage } from './CommandMessage';
import { Injector } from './Injector';


export abstract class ITransport {
  public static _CreateTransport: (injector: Injector) => ITransport;
  public static Settings: (settings: object) => { _CreateTransport: (injector: Injector) => ITransport };
  public abstract start(): Promise<void>;
  public onCommand?(handler: (data: CommandMessage) => Promise<void>): void;
  public sendCommand?(data: CommandMessage): Promise<void>;
}
