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
const Joi = require("joi");
const assert = require("assert");
const chalk_1 = require("chalk");
const CommandMessage_1 = require("./CommandMessage");
const Store_1 = require("./Store");
const Logger_1 = require("./Logger");
const InternalLogger_1 = require("./InternalLogger");
const EventMessage_1 = require("./EventMessage");
const pascalCase = require('pascal-case');
/**
 * Represents a aggregate instance
 *
 * @since 1.0.0
 */
class IAggregate {
}
exports.IAggregate = IAggregate;
function Aggregate(options) {
    return (Class) => {
        return _a = class extends Class {
                constructor(...args) {
                    super(...args[0].args(Class));
                    this.name = options.name;
                    this._injector = args[0];
                    this._injector.set({ provide: Aggregate, useConstant: this });
                    this._commandHandlers = new Map(options.commandHandlers.map((cmd) => [cmd.Command, cmd._InstantiateCommandHandler(this._injector)]));
                    this._eventHandlers = new Map(options.eventHandlers.map((cmd) => [cmd.Event, cmd._InstantiateEventHandler(this._injector)]));
                    this._logger = this._injector.get(Logger_1.Logger);
                    this._store = this._injector.get(Store_1.Store);
                    this._logger.verbose(`Registered events:\n  - ${Array.from(this._eventHandlers.keys()).join(',\n  - ')}`);
                    this._logger.verbose(`Registered commands:\n  - ${Array.from(this._commandHandlers.keys()).join(',\n  - ')}`);
                }
                static _InstantiateAggregate(parentInjector) {
                    assert(parentInjector);
                    const injector = parentInjector.newChildInjector();
                    injector.set({ provide: Logger_1.Logger, useConstant: new InternalLogger_1.InternalLogger(chalk_1.default.yellow(`${pascalCase(options.name)}Aggregate`)) });
                    return new this(injector);
                }
                handleCommand(commandMessage) {
                    return __awaiter(this, void 0, void 0, function* () {
                        const validatedCommandMessage = yield this._validateCommand(commandMessage);
                        const handler = this._commandHandlers.get(validatedCommandMessage.command);
                        const stateResult = yield this.getState(validatedCommandMessage.aggregateId);
                        if (handler) {
                            const events = yield handler.handle(validatedCommandMessage, stateResult.state, stateResult.version);
                            if (!events || events.length <= 0) {
                                this._logger.error(`Command handler for command ${validatedCommandMessage.command} did not return any events. A command has to return at least one event!`);
                                throw Error('Internal Server Error');
                            }
                            // TODO: retry insert to store
                            yield this.applyToState({ state: stateResult.state, version: stateResult.version }, events);
                            yield this._store.applyEvents(this.name, events);
                        }
                        else {
                            this._logger.error(`Received a unknown command "${validatedCommandMessage.command}"`);
                            throw Error(`UnknownCommand: ${validatedCommandMessage.command}`);
                        }
                    });
                }
                applyToState(stateDef, events) {
                    return __awaiter(this, void 0, void 0, function* () {
                        const sortedEvents = events.sort((e1, e2) => e1.eventId - e2.eventId);
                        let state = stateDef.state;
                        let version = stateDef.version;
                        for (const event of sortedEvents) {
                            Joi.assert(event, EventMessage_1.eventMessageSchema);
                            if (state === null && event.eventId != 0) {
                                throw new Error('State can not be null if this is not the initial event');
                            }
                            if (event.eventId != version + 1) {
                                throw new Error('Events are not applied in sequential order');
                            }
                            const handler = this._eventHandlers.get(event.event);
                            if (handler) {
                                state = Object.assign({}, yield handler._validateAndHandle(event, state));
                                version = event.eventId;
                            }
                            else {
                                throw new Error(`Handler missing for event ${event.event}`);
                            }
                        }
                        return { version, state };
                    });
                }
                getState(aggregateId) {
                    return __awaiter(this, void 0, void 0, function* () {
                        const eventResult = yield this._store.getEvents(this.name, aggregateId);
                        let state = null;
                        let version = -1;
                        if (eventResult.snapshot) {
                            state = eventResult.snapshot.state || state;
                            version = eventResult.snapshot.version || version;
                        }
                        return yield this.applyToState({ state, version }, eventResult.events);
                    });
                }
                getEventNames() {
                    return Array.from(this._eventHandlers.keys());
                }
                _validateCommand(cmd) {
                    return __awaiter(this, void 0, void 0, function* () {
                        return new Promise((resolve, reject) => {
                            Joi.validate(cmd, CommandMessage_1.commandMessageSchema, {}, (error, command) => {
                                if (error) {
                                    reject(error);
                                }
                                else {
                                    resolve(command);
                                }
                            });
                        });
                    });
                }
            },
            _a.Type = 'Aggregate',
            _a.Name = options.name,
            _a;
        var _a;
    };
}
exports.Aggregate = Aggregate;
//# sourceMappingURL=Aggregate.js.map