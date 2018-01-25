"use strict";
var __extends = (this && this.__extends) || (function () {
    var extendStatics = Object.setPrototypeOf ||
        ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
        function (d, b) { for (var p in b) if (b.hasOwnProperty(p)) d[p] = b[p]; };
    return function (d, b) {
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
var __assign = (this && this.__assign) || Object.assign || function(t) {
    for (var s, i = 1, n = arguments.length; i < n; i++) {
        s = arguments[i];
        for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p))
            t[p] = s[p];
    }
    return t;
};
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : new P(function (resolve) { resolve(result.value); }).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = y[op[0] & 2 ? "return" : op[0] ? "throw" : "next"]) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [0, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
Object.defineProperty(exports, "__esModule", { value: true });
var Joi = require("joi");
var assert = require("assert");
var chalk_1 = require("chalk");
var CommandMessage_1 = require("./CommandMessage");
var Store_1 = require("./Store");
var Logger_1 = require("./Logger");
var InternalLogger_1 = require("./InternalLogger");
var EventMessage_1 = require("./EventMessage");
var pascalCase = require('pascal-case');
/**
 * Represents a aggregate instance
 *
 * @since 1.0.0
 */
var IAggregate = /** @class */ (function () {
    function IAggregate() {
    }
    return IAggregate;
}());
exports.IAggregate = IAggregate;
function Aggregate(options) {
    return function (Class) {
        return _a = /** @class */ (function (_super) {
                __extends(class_1, _super);
                function class_1() {
                    var args = [];
                    for (var _i = 0; _i < arguments.length; _i++) {
                        args[_i] = arguments[_i];
                    }
                    var _this = _super.apply(this, args[0].args(Class)) || this;
                    _this.name = options.name;
                    _this._injector = args[0];
                    _this._injector.set({ provide: Aggregate, useConstant: _this });
                    _this._commandHandlers = new Map(options.commandHandlers.map(function (cmd) { return [cmd.Command, cmd._InstantiateCommandHandler(_this._injector)]; }));
                    _this._eventHandlers = new Map(options.eventHandlers.map(function (cmd) { return [cmd.Event, cmd._InstantiateEventHandler(_this._injector)]; }));
                    _this._logger = _this._injector.get(Logger_1.Logger);
                    _this._store = _this._injector.get(Store_1.Store);
                    _this._logger.verbose("Registered events:\n  - " + Array.from(_this._eventHandlers.keys()).join(',\n  - '));
                    _this._logger.verbose("Registered commands:\n  - " + Array.from(_this._commandHandlers.keys()).join(',\n  - '));
                    return _this;
                }
                class_1._InstantiateAggregate = function (parentInjector) {
                    assert(parentInjector);
                    var injector = parentInjector.newChildInjector();
                    injector.set({ provide: Logger_1.Logger, useConstant: new InternalLogger_1.InternalLogger(chalk_1.default.yellow(pascalCase(options.name) + "Aggregate")) });
                    return new this(injector);
                };
                class_1.prototype.handleCommand = function (commandMessage) {
                    return __awaiter(this, void 0, void 0, function () {
                        var validatedCommandMessage, handler, stateResult, events;
                        return __generator(this, function (_a) {
                            switch (_a.label) {
                                case 0: return [4 /*yield*/, this._validateCommand(commandMessage)];
                                case 1:
                                    validatedCommandMessage = _a.sent();
                                    handler = this._commandHandlers.get(validatedCommandMessage.command);
                                    return [4 /*yield*/, this.getState(validatedCommandMessage.aggregateId)];
                                case 2:
                                    stateResult = _a.sent();
                                    if (!handler) return [3 /*break*/, 6];
                                    return [4 /*yield*/, handler.handle(validatedCommandMessage, stateResult.state, stateResult.version)];
                                case 3:
                                    events = _a.sent();
                                    if (!events || events.length <= 0) {
                                        this._logger.error("Command handler for command " + validatedCommandMessage.command + " did not return any events. A command has to return at least one event!");
                                        throw Error('Internal Server Error');
                                    }
                                    // TODO: retry insert to store
                                    return [4 /*yield*/, this.applyToState({ state: stateResult.state, version: stateResult.version }, events)];
                                case 4:
                                    // TODO: retry insert to store
                                    _a.sent();
                                    return [4 /*yield*/, this._store.applyEvents(this.name, events)];
                                case 5:
                                    _a.sent();
                                    return [3 /*break*/, 7];
                                case 6:
                                    this._logger.error("Received a unknown command \"" + validatedCommandMessage.command + "\"");
                                    throw Error("UnknownCommand: " + validatedCommandMessage.command);
                                case 7: return [2 /*return*/];
                            }
                        });
                    });
                };
                class_1.prototype.applyToState = function (stateDef, events) {
                    return __awaiter(this, void 0, void 0, function () {
                        var sortedEvents, state, version, _i, sortedEvents_1, event, handler, _a;
                        return __generator(this, function (_b) {
                            switch (_b.label) {
                                case 0:
                                    sortedEvents = events.sort(function (e1, e2) { return e1.eventId - e2.eventId; });
                                    state = stateDef.state;
                                    version = stateDef.version;
                                    _i = 0, sortedEvents_1 = sortedEvents;
                                    _b.label = 1;
                                case 1:
                                    if (!(_i < sortedEvents_1.length)) return [3 /*break*/, 5];
                                    event = sortedEvents_1[_i];
                                    Joi.assert(event, EventMessage_1.eventMessageSchema);
                                    if (state === null && event.eventId != 0) {
                                        throw new Error('State can not be null if this is not the initial event');
                                    }
                                    if (event.eventId != version + 1) {
                                        throw new Error('Events are not applied in sequential order');
                                    }
                                    handler = this._eventHandlers.get(event.event);
                                    if (!handler) return [3 /*break*/, 3];
                                    _a = [{}];
                                    return [4 /*yield*/, handler._validateAndHandle(event, state)];
                                case 2:
                                    state = __assign.apply(void 0, _a.concat([_b.sent()]));
                                    version = event.eventId;
                                    return [3 /*break*/, 4];
                                case 3: throw new Error("Handler missing for event " + event.event);
                                case 4:
                                    _i++;
                                    return [3 /*break*/, 1];
                                case 5: return [2 /*return*/, { version: version, state: state }];
                            }
                        });
                    });
                };
                class_1.prototype.getState = function (aggregateId) {
                    return __awaiter(this, void 0, void 0, function () {
                        var eventResult, state, version;
                        return __generator(this, function (_a) {
                            switch (_a.label) {
                                case 0: return [4 /*yield*/, this._store.getEvents(this.name, aggregateId)];
                                case 1:
                                    eventResult = _a.sent();
                                    state = null;
                                    version = -1;
                                    if (eventResult.snapshot) {
                                        state = eventResult.snapshot.state || state;
                                        version = eventResult.snapshot.version || version;
                                    }
                                    return [4 /*yield*/, this.applyToState({ state: state, version: version }, eventResult.events)];
                                case 2: return [2 /*return*/, _a.sent()];
                            }
                        });
                    });
                };
                class_1.prototype.getEventNames = function () {
                    return Array.from(this._eventHandlers.keys());
                };
                class_1.prototype._validateCommand = function (cmd) {
                    return __awaiter(this, void 0, void 0, function () {
                        return __generator(this, function (_a) {
                            return [2 /*return*/, new Promise(function (resolve, reject) {
                                    Joi.validate(cmd, CommandMessage_1.commandMessageSchema, {}, function (error, command) {
                                        if (error) {
                                            reject(error);
                                        }
                                        else {
                                            resolve(command);
                                        }
                                    });
                                })];
                        });
                    });
                };
                return class_1;
            }(Class)),
            _a.Type = 'Aggregate',
            _a.Name = options.name,
            _a;
        var _a;
    };
}
exports.Aggregate = Aggregate;
//# sourceMappingURL=Aggregate.js.map