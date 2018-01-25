import { EventMessage } from './EventMessage';
import { Snapshot } from './Snapshot';


export interface GetEventsResult<T> {
  events: EventMessage[];
  snapshot?: Snapshot<T>;
}
