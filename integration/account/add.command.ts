import { CommandHandler, ICommandHandler, EventMessage, CommandMessage } from '@eventific/core';
import { AddedEvent } from './added.event';
import { AccountState } from './account.state';


@CommandHandler({
  command: 'ADD'
})
export class AddCommand extends ICommandHandler<any, any> {

  async handle(message: CommandMessage<{amount: number}>, state: AccountState, version: number): Promise<EventMessage<any>[]> {
    return [{
      event: 'ADDED',
      eventId: version + 1,
      aggregateId: message.aggregateId,
      header: {
        createdDate: new Date()
      },
      content: {
        amount: message.content.amount
      }
    }];
  }

}
