import chalk from 'chalk';
import * as emoji from 'node-emoji';
import { Injector } from './injector/Injector';
import { InternalLogger } from './logger/InternalLogger';
import { Logger } from './logger/Logger';

/**
 * Bootstraps a CommandManager, ReadManager, or Saga.
 *
 * @public
 *
 * @param type - The type to instantiate
 */
export async function bootstrap<T>(type: {
  Type: string;
  _Instantiate: (injector: Injector) => Bootstrapable
}): Promise<void> {
  process.env.NODE_ENV = process.env.NODE_ENV || 'development';
  const injector = new Injector();
  injector.set({provide: Logger, useConstant: new InternalLogger()});
  const logger = injector.get<Logger>(Logger);
  logger.raw(chalk.green(banner));
  logger.info(`Launching Eventific ${emoji.get('rocket')}`);
  logger.info(`Version: ${process.env.npm_package_version || 'unknown'}`);
  logger.info(`Environment: ${process.env.NODE_ENV} ${emoji.get('eyes')}`);
  if (type._Instantiate) {
    logger.info(`Type: ${type.Type}`);
    const inst = type._Instantiate(injector);
    logger.info(`Starting application ${emoji.get('dancer')}`);
    await inst._start();
  } else {
    logger.error('The provided type does not seem to be a bootstrap able module');
    throw new Error('Failed to start');
  }
}

/**
 * @beta
 */
export abstract class Bootstrapable {
  public static Type: string;
  public _Instantiate: (injector: Injector) => Bootstrapable;
  public _start: () => Promise<void>;
}

/**
 * @internal
 */
const banner = `

  ███████╗██╗   ██╗███████╗███╗   ██╗████████╗██╗███████╗██╗ ██████╗
  ██╔════╝██║   ██║██╔════╝████╗  ██║╚══██╔══╝██║██╔════╝██║██╔════╝
  █████╗  ██║   ██║█████╗  ██╔██╗ ██║   ██║   ██║█████╗  ██║██║
  ██╔══╝  ╚██╗ ██╔╝██╔══╝  ██║╚██╗██║   ██║   ██║██╔══╝  ██║██║
  ███████╗ ╚████╔╝ ███████╗██║ ╚████║   ██║   ██║██║     ██║╚██████╗
  ╚══════╝  ╚═══╝  ╚══════╝╚═╝  ╚═══╝   ╚═╝   ╚═╝╚═╝     ╚═╝ ╚═════╝

`;
