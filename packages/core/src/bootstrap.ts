/**
 * Bootstraps a CommandManager, ReadManager, or Saga.
 *
 * @since 1.0.0
 *
 * @param type The type to instantiate
 * @returns {Promise<void>} A promise that resolves once the app is started
 */
export async function bootstrap(type: any): Promise<void> {
  console.log(banner);
  if (type.Type && type._Instantiate && type.Type === 'CommandManager') {
    if (type._Instantiate) {
      const inst = type._Instantiate();
      await inst._start();
    }
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
