import { EventHandler, IEventHandler, EventMessage } from '@eventific/core';
import { AccountState } from './account.state';

@EventHandler({
  event: 'SUBTRACTED'
})
export class SubtractedEvent extends IEventHandler<any, any> {
  async handle(event: EventMessage<undefined>, state: AccountState): Promise<AccountState> {
    return {
      balance: 0
    }
  }
}
