"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : new P(function (resolve) { resolve(result.value); }).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
const chalk_1 = require("chalk");
const Injector_1 = require("./Injector");
const InternalLogger_1 = require("./InternalLogger");
const Logger_1 = require("./Logger");
const emoji = require("node-emoji");
/**
 * Bootstraps a CommandManager, ReadManager, or Saga.
 *
 * @since 1.0.0
 *
 * @param type The type to instantiate
 * @returns {Promise<void>} A promise that resolves once the app is started
 */
function bootstrap(type) {
    return __awaiter(this, void 0, void 0, function* () {
        process.env.NODE_ENV = process.env.NODE_ENV || 'development';
        const injector = new Injector_1.Injector();
        injector.set({ provide: Logger_1.Logger, useConstant: new InternalLogger_1.InternalLogger() });
        const logger = injector.get(Logger_1.Logger);
        logger.raw(chalk_1.default.green(banner));
        logger.info(`Launching Eventific ${emoji.get('rocket')}`);
        logger.info(`Version: ${require('../package.json').version}`);
        logger.info(`Environment: ${process.env.NODE_ENV} ${emoji.get('eyes')}`);
        if (type._Instantiate) {
            logger.info(`Type: ${type.Type}`);
            const inst = type._Instantiate(injector);
            logger.info(`Starting application ${emoji.get('dancer')}`);
            yield inst._start();
        }
        else {
            logger.error('The provided type does not seem to be a bootstrap able module');
            throw new Error('Failed to start');
        }
    });
}
exports.bootstrap = bootstrap;
class Bootstrapable {
}
exports.Bootstrapable = Bootstrapable;
const banner = `

  ███████╗██╗   ██╗███████╗███╗   ██╗████████╗██╗███████╗██╗ ██████╗
  ██╔════╝██║   ██║██╔════╝████╗  ██║╚══██╔══╝██║██╔════╝██║██╔════╝
  █████╗  ██║   ██║█████╗  ██╔██╗ ██║   ██║   ██║█████╗  ██║██║     
  ██╔══╝  ╚██╗ ██╔╝██╔══╝  ██║╚██╗██║   ██║   ██║██╔══╝  ██║██║     
  ███████╗ ╚████╔╝ ███████╗██║ ╚████║   ██║   ██║██║     ██║╚██████╗
  ╚══════╝  ╚═══╝  ╚══════╝╚═╝  ╚═══╝   ╚═╝   ╚═╝╚═╝     ╚═╝ ╚═════╝
                                                                  
`;
//# sourceMappingURL=bootstrap.js.map