import { ICommandHandler, CommandHandler } from './Command';


class MockCommand extends ICommandHandler {
  async handle(state: any, version: number): Promise<[]> {
    return [];
  }
}

test('It should be defined', async () => {
  expect(ICommandHandler).toBeDefined();
  expect(CommandHandler).toBeDefined();
});
