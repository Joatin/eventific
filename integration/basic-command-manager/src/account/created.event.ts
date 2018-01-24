import { EventHandler, IEventHandler, EventMessage } from '@eventific/core';
import { AccountState } from './account.state';

@EventHandler({
  event: 'CREATED'
})
export class CreatedEvent extends IEventHandler<undefined, AccountState> {
  async handle(event: EventMessage<undefined>, state: AccountState): Promise<AccountState> {
    return {
      balance: 0
    }
  }
}
