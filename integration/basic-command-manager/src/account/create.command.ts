import { CommandHandler, ICommandHandler, CommandMessage, EventMessage, Logger } from '@eventific/core';
import { CreatedEvent } from './created.event';
import { AccountState } from './account.state';


@CommandHandler({
  command: 'CREATE'
})
export class CreateCommand extends ICommandHandler<any, any>{

  constructor(
    private logger: Logger
  ) {
    super();
  }

  async handle(command: CommandMessage<any>, state: AccountState, version: number): Promise<EventMessage<any>[]> {
    this.logger.debug('Handling create command');
    if(version === -1) {
      return [{
        event: CreatedEvent.Event,
        eventId: 0,
        aggregateId: command.aggregateId,
        header: {
          createdDate: Date.now()
        },
        content: {}
      }];
    } else {
      throw new Error('CREATE has to be the first event for the aggregate');
    }
  }

}
