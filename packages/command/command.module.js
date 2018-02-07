import { assert, object, func, string, array, any } from 'joi';
import { InternalLogger, Logger, Store, Bootstrapable } from '@eventific/core';
import chalk from 'chalk';
import { get } from 'node-emoji';

var commandManagerOptionsSchema = object().keys({
    aggregate: func().unknown().keys({
        Name: string().required(),
        Type: string().required(),
        _InstantiateAggregate: func().required()
    }).required(),
    extensions: array().items(any()).optional(),
    providers: array().items(any()).optional(),
    store: func().unknown().keys({
        _CreateStore: func().required()
    }).required(),
    transports: array().min(1).items(func().unknown().keys({
        _CreateTransport: func().required()
    })).required()
});

var __extends = (undefined && undefined.__extends) || (function () {
    var extendStatics = Object.setPrototypeOf ||
        ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
        function (d, b) { for (var p in b) if (b.hasOwnProperty(p)) d[p] = b[p]; };
    return function (d, b) {
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
var __awaiter = (undefined && undefined.__awaiter) || function (thisArg, _arguments, P, generator) {
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : new P(function (resolve) { resolve(result.value); }).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (undefined && undefined.__generator) || function (thisArg, body) {
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
// tslint:disable-next-line
var pascalCase = require('pascal-case');
/**
 *
 * @param {CommandManagerOptions} options
 * @returns T The decorated class
 * @Annotation
 */
function CommandManager(options) {
    assert(options, commandManagerOptionsSchema);
    return function (Class) {
        return _a = /** @class */ (function (_super) {
                __extends(class_1, _super);
                /* istanbul ignore next */
                function class_1() {
                    var args = [];
                    for (var _i = 0; _i < arguments.length; _i++) {
                        args[_i] = arguments[_i];
                    }
                    var _this = _super.apply(this, args[0].injector.args(Class)) || this;
                    var params = args[0];
                    _this._injector = params.injector;
                    _this._store = params.store;
                    _this._transports = params.transports;
                    _this._aggregate = params.aggregate;
                    _this._logger = _this._injector.get(Logger);
                    return _this;
                }
                class_1._Instantiate = function (parentInjector) {
                    var injector = parentInjector.newChildInjector();
                    var store = options.store._CreateStore(injector);
                    injector.set({ provide: Store, useConstant: store });
                    injector.set({ provide: Logger, useConstant: new InternalLogger(chalk.green(pascalCase('CommandManager'))) });
                    return new this({
                        aggregate: options.aggregate._InstantiateAggregate(injector),
                        injector: injector,
                        store: store,
                        transports: options.transports.map(function (t) { return t._CreateTransport(injector); })
                    });
                };
                class_1.prototype._start = function () {
                    return __awaiter(this, void 0, void 0, function () {
                        var _this = this;
                        var _i, _a, transport;
                        return __generator(this, function (_b) {
                            switch (_b.label) {
                                case 0:
                                    if (!this.onInit) return [3 /*break*/, 2];
                                    return [4 /*yield*/, this.onInit()];
                                case 1:
                                    _b.sent();
                                    _b.label = 2;
                                case 2: return [4 /*yield*/, this._store.start()];
                                case 3:
                                    _b.sent();
                                    _i = 0, _a = this._transports;
                                    _b.label = 4;
                                case 4:
                                    if (!(_i < _a.length)) return [3 /*break*/, 7];
                                    transport = _a[_i];
                                    return [4 /*yield*/, transport.start()];
                                case 5:
                                    _b.sent();
                                    if (transport.onCommand) {
                                        transport.onCommand(this._aggregate.name, function (cmd) { return __awaiter(_this, void 0, void 0, function () {
                                            return __generator(this, function (_a) {
                                                switch (_a.label) {
                                                    case 0: return [4 /*yield*/, this._handleCommand(cmd)];
                                                    case 1:
                                                        _a.sent();
                                                        return [2 /*return*/];
                                                }
                                            });
                                        }); });
                                    }
                                    _b.label = 6;
                                case 6:
                                    _i++;
                                    return [3 /*break*/, 4];
                                case 7: return [4 /*yield*/, this._aggregate._start()];
                                case 8:
                                    _b.sent();
                                    this._logger.info("All setup and ready " + get('sparkles'));
                                    return [2 /*return*/];
                            }
                        });
                    });
                };
                class_1.prototype._handleCommand = function (commandMessage) {
                    return __awaiter(this, void 0, void 0, function () {
                        return __generator(this, function (_a) {
                            switch (_a.label) {
                                case 0: return [4 /*yield*/, this._aggregate._handleCommand(commandMessage)];
                                case 1:
                                    _a.sent();
                                    return [2 /*return*/];
                            }
                        });
                    });
                };
                return class_1;
            }(Class)), _a.Type = 'CommandManager', _a;
        var _a;
    };
}

var __extends$1 = (undefined && undefined.__extends) || (function () {
    var extendStatics = Object.setPrototypeOf ||
        ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
        function (d, b) { for (var p in b) if (b.hasOwnProperty(p)) d[p] = b[p]; };
    return function (d, b) {
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
var ICommandManager = /** @class */ (function (_super) {
    __extends$1(ICommandManager, _super);
    function ICommandManager() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    return ICommandManager;
}(Bootstrapable));

export { CommandManager, ICommandManager };
