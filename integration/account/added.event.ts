import { EventHandler, IEventHandler, EventMessage } from '@eventific/core';
import { AccountState } from './account.state';

export interface AddedEventContent {
  amount: number;
}

@EventHandler({
  event: 'ADDED'
})
export class AddedEvent extends IEventHandler<any, any> {
  async handle(event: EventMessage<AddedEventContent>, state: AccountState): Promise<AccountState> {
    return {
      ...state,
      balance: state.balance + event.content.amount
    }
  }
}
