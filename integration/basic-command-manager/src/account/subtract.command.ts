import { CommandHandler, ICommandHandler } from '@eventific/core';


@CommandHandler({
  command: 'SUBTRACT'
})
export class SubtractCommand implements ICommandHandler {

}
