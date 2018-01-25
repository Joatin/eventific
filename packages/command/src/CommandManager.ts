import { CommandMessage, IAggregate, IStore, Injector, InternalLogger, Logger, IEventHandler, Bootstrapable, ITransport, Store } from '@eventific/core';
import * as emoji from 'node-emoji';
import chalk from 'chalk';
import * as Joi from 'joi';

const pascalCase = require('pascal-case');


/**
 * Defines params for the command manager
 *
 * @since 1.0
 */
export interface CommandManagerOptions {
  extensions?: any[];

  /**
   * The aggregate to issue commands against
   *
   * @since 1.0
   */
  aggregate: {
    _InstantiateAggregate(injector: Injector): IAggregate;
  };

  /**
   * The store that should be used to persist events
   *
   * @since 1.0
   */
  store: {
    _CreateStore(injector: Injector): IStore
  };

  /**
   * An array of transports that is used to receive commands
   *
   * @since 1.0
   */
  transports: Array<{
    _CreateTransport(injector: Injector): ITransport
  }>;

  /**
   * An array of providers to be used in Eventifics IOC
   *
   * @since 1.0
   */
  providers?: any[];
}

const commandManagerOptionsSchema = Joi.object().keys({
  extensions: Joi.array().items(Joi.any()).optional(),
  aggregate: (Joi as any).func().unknown().keys({
    Type: Joi.string().required(),
    Name: Joi.string().required(),
    _InstantiateAggregate: Joi.func().required()
  }).required(),
  store: (Joi as any).func().unknown().keys({
    _CreateStore: Joi.func().required()
  }).required(),
  transports: Joi.array().min(1).items((Joi as any).func().unknown().keys({
    _CreateTransport: Joi.func().required()
  })),
  providers: Joi.array().items(Joi.any()).optional()
});

export abstract class ICommandManager extends Bootstrapable{

}

/**
 *
 * @param {CommandManagerOptions} options
 * @returns T The decorated class
 * @Annotation
 */
export function CommandManager(options: CommandManagerOptions) {
  Joi.assert(options, commandManagerOptionsSchema);
  return <T extends {new(...args: any[]): {}}>(Class: T) => {
    return class extends Class {
      public static Type = 'CommandManager';
      public static _Instantiate(parentInjector: Injector): T {
        const injector = parentInjector.newChildInjector();
        const store = options.store._CreateStore(injector);
        injector.set({provide: Store, useConstant: store});
        injector.set({provide: Logger, useConstant: new InternalLogger(chalk.green(pascalCase('CommandManager')))});

        return new this({
          injector,
          store,
          transports: options.transports.map((t) => t._CreateTransport(injector)) || [],
          aggregate: options.aggregate._InstantiateAggregate(injector)
        }) as any;
      }

      readonly _injector: Injector;
      readonly _store: IStore;
      readonly _transports: ITransport[];
      readonly _aggregate: IAggregate;
      readonly _logger: Logger;

      constructor(...args: any[]) {
        super(...args[0].injector.args(Class));
        const params = args[0];
        this._injector = params.injector;
        this._store = params.store;
        this._transports = params.transports;
        this._aggregate = params.aggregate;
        this._logger = this._injector.get<Logger>(Logger);
      }

      public async _start() {
        if (this.onInit) {
          await this.onInit();
        }

        await this._store.start();

        for (const transport of this._transports) {
          await transport.start();
          if(transport.onCommand) {
            transport.onCommand(async (cmd: any) => {
              await this._handleCommand(cmd);
            });
          }
        }
        this._logger.info(`All setup and ready ${emoji.get('sparkles')}`)
      }

      public async _handleCommand(commandMessage: CommandMessage): Promise<void> {
        await this._aggregate.handleCommand(commandMessage);

        // const command = await this._aggregate.getCommand(commandMessage);
        // const stateDef = await this._aggregate.getState(command.aggregateId);
        // let events: IEvent[];
        // try {
        //   events = await command.handle(stateDef.state, stateDef.version);
        // } catch(ex) {
        //   this._logger.warn(`Command handler ${command.name} threw an error upon execution`, ex);
        //   throw ex;
        // }
        // if(!events || events.length <= 0) {
        //   this._logger.error(`Command handler ${command.name} did not return any events. A command has to return at least one event!`);
        //   throw Error('Internal Server Error');
        // }
        // await this._store.applyEvents(this._aggregate.name, events.map((e) => e.toMessage()));
      }

      public onInit?: () => void;

    };
  };
}
