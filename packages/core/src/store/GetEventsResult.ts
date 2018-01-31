import { EventMessage } from '../event/EventMessage';
import { Snapshot } from './Snapshot';

/**
 * @public
 */
export interface GetEventsResult<T> {
  events: EventMessage[];
  snapshot?: Snapshot<T>;
}
