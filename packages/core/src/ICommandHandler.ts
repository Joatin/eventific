import { CommandMessage } from './CommandMessage';
import { EventMessage } from './EventMessage';
import { Injector } from './Injector';

/**
 * @public
 */
export abstract class ICommandHandler<T, R> {
  public static _InstantiateCommandHandler: (injector: Injector) => ICommandHandler<any, any>;
  public static Command: string;
  public readonly command: string;
  public abstract handle(message: CommandMessage<T>, state: R, version: number): Promise<EventMessage[]>;
}
