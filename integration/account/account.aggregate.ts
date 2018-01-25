import { Aggregate, AggregateOptions, IAggregate } from '@eventific/core';
import { AddCommand } from './add.command';
import { CreateCommand } from './create.command';
import { SubtractCommand } from './subtract.command';
import { CreatedEvent } from './created.event';
import { SubtractedEvent } from './subtracted.event';
import { AddedEvent } from './added.event';

@Aggregate(<AggregateOptions>{
  name: 'Account',
  commandHandlers: [
    AddCommand,
    CreateCommand,
    SubtractCommand
  ],
  eventHandlers: [
    AddedEvent,
    CreatedEvent,
    SubtractedEvent
  ]
})
export class AccountAggregate extends IAggregate{

}
