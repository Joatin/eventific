import { Bootstrapable, CommandMessage } from '@eventific/core';


export abstract class ISaga extends Bootstrapable {
  public _triggerDefinitions: Array<{
    triggers: any[],
    propertyKey: string;
  }>;
  public sendCommand: (message: CommandMessage) => Promise<void>;
}
