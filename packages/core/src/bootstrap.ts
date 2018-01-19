import * as chalk from 'chalk';

/**
 * Bootstraps a CommandManager, ReadManager, or Saga.
 *
 * @since 1.0.0
 *
 * @param type The type to instantiate
 * @returns {Promise<void>} A promise that resolves once the app is started
 */
export async function bootstrap(type: {
  _Instantiate: () => {
    _start(): Promise<void>
  }
}): Promise<void> {
  console.log(chalk.greenBright(banner));
  if (type._Instantiate) {
    const inst = type._Instantiate();
    await inst._start();
  } else {
    throw Error('The provided type does not seem to be a bootstrap able module');
  }
}

const banner = `

  ███████╗██╗   ██╗███████╗███╗   ██╗████████╗██╗███████╗██╗ ██████╗
  ██╔════╝██║   ██║██╔════╝████╗  ██║╚══██╔══╝██║██╔════╝██║██╔════╝
  █████╗  ██║   ██║█████╗  ██╔██╗ ██║   ██║   ██║█████╗  ██║██║     
  ██╔══╝  ╚██╗ ██╔╝██╔══╝  ██║╚██╗██║   ██║   ██║██╔══╝  ██║██║     
  ███████╗ ╚████╔╝ ███████╗██║ ╚████║   ██║   ██║██║     ██║╚██████╗
  ╚══════╝  ╚═══╝  ╚══════╝╚═╝  ╚═══╝   ╚═╝   ╚═╝╚═╝     ╚═╝ ╚═════╝
                                                                  
`;