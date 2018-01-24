import { EventHandler, IEventHandler } from '@eventific/core';

@EventHandler({
  event: 'SUBTRACTED'
})
export class SubtractedEvent extends IEventHandler {

}
