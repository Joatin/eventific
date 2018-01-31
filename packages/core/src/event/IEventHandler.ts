import { Injector } from '../injector/Injector';
import { EventMessage } from './EventMessage';


/**
 * OBS: Needed until typescript supports decorator type extensions.
 *
 * @public
 */
export abstract class IEventHandler<T, R> {
  public static _InstantiateEventHandler: (injector: Injector) => IEventHandler<any, any>;
  public static Type: string;
  public static Event: string;
  public readonly event: string;
  public _validateAndHandle: (event: EventMessage<T>, state: R) => Promise<R>;
  public abstract handle(event: EventMessage<T>, state: R): Promise<R>;
}
