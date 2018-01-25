import chalk from 'chalk';
import { Injector } from './Injector';
import { InternalLogger } from './InternalLogger';
import { Logger } from './Logger';
import * as emoji from 'node-emoji';

/**
 * Bootstraps a CommandManager, ReadManager, or Saga.
 *
 * @since 1.0.0
 *
 * @param type The type to instantiate
 * @returns {Promise<void>} A promise that resolves once the app is started
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

export abstract class Bootstrapable {
  static Type: string;
  _Instantiate: (injector: Injector) => Bootstrapable;
  _start: () => Promise<void>;
}

const banner = `

  ███████╗██╗   ██╗███████╗███╗   ██╗████████╗██╗███████╗██╗ ██████╗
  ██╔════╝██║   ██║██╔════╝████╗  ██║╚══██╔══╝██║██╔════╝██║██╔════╝
  █████╗  ██║   ██║█████╗  ██╔██╗ ██║   ██║   ██║█████╗  ██║██║     
  ██╔══╝  ╚██╗ ██╔╝██╔══╝  ██║╚██╗██║   ██║   ██║██╔══╝  ██║██║     
  ███████╗ ╚████╔╝ ███████╗██║ ╚████║   ██║   ██║██║     ██║╚██████╗
  ╚══════╝  ╚═══╝  ╚══════╝╚═╝  ╚═══╝   ╚═╝   ╚═╝╚═╝     ╚═╝ ╚═════╝
                                                                  
`;
