'use strict';

Object.defineProperty(exports, '__esModule', { value: true });

function _interopDefault (ex) { return (ex && (typeof ex === 'object') && 'default' in ex) ? ex['default'] : ex; }

var Joi = require('joi');
var core = require('@eventific/core');
var chalk = _interopDefault(require('chalk'));
var emoji = require('node-emoji');

var queryManagerOptionsSchema = Joi.object().keys({
    aggregates: Joi.array().min(1).required(),
    providers: Joi.array().optional(),
    store: Joi.any().required(),
    viewHandlers: Joi.array().min(1).required()
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
function QueryManager(options) {
    Joi.assert(options, queryManagerOptionsSchema);
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
                    _this._aggregates = params.aggregates;
                    _this._viewHandlers = params.viewHandlers;
                    _this._logger = _this._injector.get(core.Logger);
                    return _this;
                }
                class_1._Instantiate = function (parentInjector) {
                    var injector = parentInjector.newChildInjector();
                    var store = options.store._CreateStore(injector);
                    for (var _i = 0, _a = (options.providers || []); _i < _a.length; _i++) {
                        var prov = _a[_i];
                        injector.set(prov);
                    }
                    injector.set({ provide: core.Store, useConstant: store });
                    injector.set({ provide: core.Logger, useConstant: new core.InternalLogger(chalk.green(pascalCase('QueryManager'))) });
                    return new this({
                        aggregates: options.aggregates.map(function (a) { return a._InstantiateAggregate(injector); }),
                        injector: injector,
                        store: store,
                        viewHandlers: options.viewHandlers.map(function (v) { return v._InstantiateViewHandler(injector); })
                    });
                };
                class_1.prototype._start = function () {
                    return __awaiter(this, void 0, void 0, function () {
                        var _this = this;
                        var _i, _a, handler, _loop_1, _b, _c, agg;
                        return __generator(this, function (_d) {
                            switch (_d.label) {
                                case 0:
                                    if (!this.onInit) return [3 /*break*/, 2];
                                    return [4 /*yield*/, this.onInit({ injector: this._injector })];
                                case 1:
                                    _d.sent();
                                    _d.label = 2;
                                case 2: return [4 /*yield*/, this._store.start()];
                                case 3:
                                    _d.sent();
                                    _i = 0, _a = this._viewHandlers;
                                    _d.label = 4;
                                case 4:
                                    if (!(_i < _a.length)) return [3 /*break*/, 7];
                                    handler = _a[_i];
                                    return [4 /*yield*/, handler.start()];
                                case 5:
                                    _d.sent();
                                    _d.label = 6;
                                case 6:
                                    _i++;
                                    return [3 /*break*/, 4];
                                case 7:
                                    _loop_1 = function (agg) {
                                        return __generator(this, function (_a) {
                                            switch (_a.label) {
                                                case 0:
                                                    agg.changes$.subscribe(function (event) { return __awaiter(_this, void 0, void 0, function () {
                                                        var stateResult, _i, _a, handler;
                                                        return __generator(this, function (_b) {
                                                            switch (_b.label) {
                                                                case 0: return [4 /*yield*/, agg.getState(event.aggregateId)];
                                                                case 1:
                                                                    stateResult = _b.sent();
                                                                    _i = 0, _a = this._viewHandlers;
                                                                    _b.label = 2;
                                                                case 2:
                                                                    if (!(_i < _a.length)) return [3 /*break*/, 5];
                                                                    handler = _a[_i];
                                                                    return [4 /*yield*/, handler.buildAndPersistView(event.aggregateId, stateResult.state, stateResult.version)];
                                                                case 3:
                                                                    _b.sent();
                                                                    _b.label = 4;
                                                                case 4:
                                                                    _i++;
                                                                    return [3 /*break*/, 2];
                                                                case 5: return [2 /*return*/];
                                                            }
                                                        });
                                                    }); });
                                                    return [4 /*yield*/, agg._start()];
                                                case 1:
                                                    _a.sent();
                                                    return [2 /*return*/];
                                            }
                                        });
                                    };
                                    _b = 0, _c = this._aggregates;
                                    _d.label = 8;
                                case 8:
                                    if (!(_b < _c.length)) return [3 /*break*/, 11];
                                    agg = _c[_b];
                                    return [5 /*yield**/, _loop_1(agg)];
                                case 9:
                                    _d.sent();
                                    _d.label = 10;
                                case 10:
                                    _b++;
                                    return [3 /*break*/, 8];
                                case 11:
                                    this._logger.info("All setup and ready " + emoji.get('sparkles'));
                                    return [2 /*return*/];
                            }
                        });
                    });
                };
                return class_1;
            }(Class)), _a.Type = 'QueryManager', _a;
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
var IQueryManager = /** @class */ (function (_super) {
    __extends$1(IQueryManager, _super);
    function IQueryManager() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    return IQueryManager;
}(core.Bootstrapable));

var __extends$2 = (undefined && undefined.__extends) || (function () {
    var extendStatics = Object.setPrototypeOf ||
        ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
        function (d, b) { for (var p in b) if (b.hasOwnProperty(p)) d[p] = b[p]; };
    return function (d, b) {
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
// tslint:disable-next-line
var pascalCase$1 = require('pascal-case');
function ViewHandler(options) {
    Joi.assert(options, Joi.object());
    return function (Class) {
        return _a = /** @class */ (function (_super) {
                __extends$2(class_1, _super);
                function class_1() {
                    var args = [];
                    for (var _i = 0; _i < arguments.length; _i++) {
                        args[_i] = arguments[_i];
                    }
                    var _this = _super.apply(this, args[0].args(Class)) || this;
                    _this.name = options.name;
                    return _this;
                }
                class_1._InstantiateViewHandler = function (parentInjector) {
                    var injector = parentInjector.newChildInjector();
                    injector.set({
                        provide: core.Logger,
                        useConstant: new core.InternalLogger(chalk.blue(pascalCase$1(options.name) + "CommandHandler"))
                    });
                    return new this(injector);
                };
                return class_1;
            }(Class)), _a.Name = options.name, _a;
        var _a;
    };
}

var IViewHandler = /** @class */ (function () {
    function IViewHandler() {
    }
    return IViewHandler;
}());

exports.QueryManager = QueryManager;
exports.IQueryManager = IQueryManager;
exports.ViewHandler = ViewHandler;
exports.IViewHandler = IViewHandler;
