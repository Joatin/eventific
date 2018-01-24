import { CommandHandler, ICommandHandler, IEvent } from '@eventific/core';
import { AddedEvent } from './added.event';
import { AccountState } from './account.state';


@CommandHandler({
  command: 'ADD'
})
export class AddCommand implements ICommandHandler {

  async handle(state: AccountState, version: number): Promise<IEvent[]> {
    return [AddedEvent.Create(version + 1, '27e7b187-5a11-41fe-afb7-a071c6c17b6d')];
  }

}
