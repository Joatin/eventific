import { EventMessage } from '../event/EventMessage';
import { Injector } from '../injector/Injector';
import { CommandMessage } from './CommandMessage';

/**
 * @public
 */
export abstract class ICommandHandler<T, R> {
  public static _InstantiateCommandHandler: (injector: Injector) => ICommandHandler<any, any>;
  public static Command: string;
  public readonly command: string;
  public abstract handle(message: CommandMessage<T>, state: R, version: number): Promise<EventMessage[]>;
}
