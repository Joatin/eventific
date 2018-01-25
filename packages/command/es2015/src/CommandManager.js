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
const core_1 = require("@eventific/core");
const emoji = require("node-emoji");
const chalk_1 = require("chalk");
const pascalCase = require('pascal-case');
class ICommandManager extends core_1.Bootstrapable {
}
exports.ICommandManager = ICommandManager;
/**
 *
 * @param {CommandManagerOptions} options
 * @returns T The decorated class
 * @Annotation
 */
function CommandManager(options) {
    return (Class) => {
        return _a = class extends Class {
                constructor(...args) {
                    super(...args[0].injector.args(Class));
                    const params = args[0];
                    this._injector = params.injector;
                    this._store = params.store;
                    this._transports = params.transports;
                    this._aggregate = params.aggregate;
                    this._logger = this._injector.get(core_1.Logger);
                }
                static _Instantiate(parentInjector) {
                    const injector = parentInjector.newChildInjector();
                    const store = options.store._CreateStore(injector);
                    injector.set({ provide: core_1.Store, useConstant: store });
                    injector.set({ provide: core_1.Logger, useConstant: new core_1.InternalLogger(chalk_1.default.green(pascalCase('CommandManager'))) });
                    return new this({
                        injector,
                        store,
                        transports: options.transports.map((t) => t._CreateTransport(injector)) || [],
                        aggregate: options.aggregate._InstantiateAggregate(injector)
                    });
                }
                _start() {
                    return __awaiter(this, void 0, void 0, function* () {
                        if (this.onInit) {
                            yield this.onInit();
                        }
                        yield this._store.start();
                        for (const transport of this._transports) {
                            yield transport.start();
                            if (transport.onCommand) {
                                transport.onCommand((cmd) => __awaiter(this, void 0, void 0, function* () {
                                    yield this._handleCommand(cmd);
                                }));
                            }
                        }
                        this._logger.info(`All setup and ready ${emoji.get('sparkles')}`);
                    });
                }
                _handleCommand(commandMessage) {
                    return __awaiter(this, void 0, void 0, function* () {
                        yield this._aggregate.handleCommand(commandMessage);
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
                    });
                }
            },
            _a.Type = 'CommandManager',
            _a;
        var _a;
    };
}
exports.CommandManager = CommandManager;
//# sourceMappingURL=CommandManager.js.map