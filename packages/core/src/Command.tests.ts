import { BaseCommand } from './Command';


class MockCommand extends BaseCommand {
  handle(state: any, version: number): Promise<BaseEvent[]> {

  }
}

test('It should be defined', async () => {
  expect(BaseCommand).toBeDefined();
});