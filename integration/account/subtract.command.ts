import { CommandHandler, ICommandHandler, EventMessage, CommandMessage } from '@eventific/core';
import { AccountState } from './account.state';


@CommandHandler({
  command: 'SUBTRACT'
})
export class SubtractCommand extends ICommandHandler<any, any> {
  async handle(message: CommandMessage<any>, state: AccountState, version: number): Promise<EventMessage[]> {
    return [{
      event: 'ADDED',
      eventId: 0,
      aggregateId: message.aggregateId,
      header: {
        createdDate: new Date()
      },
      content: undefined
    }];
  }
}
