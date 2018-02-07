'use strict';

Object.defineProperty(exports, '__esModule', { value: true });

var core = require('@eventific/core');
var Joi = require('joi');
var mongodb = require('mongodb');
var rxjs = require('rxjs');

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
var MongoEventsWithSnapshotIterator = /** @class */ (function () {
    function MongoEventsWithSnapshotIterator(_cursor, _aproxVersion, _snapshot) {
        this._cursor = _cursor;
        this.approximateVersion = _aproxVersion;
        if (_snapshot) {
            this.snapshotVersion = _snapshot.version;
            this.snapshotState = _snapshot.state;
        }
    }
    MongoEventsWithSnapshotIterator.prototype.next = function () {
        return __awaiter(this, void 0, void 0, function () {
            var event;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0: return [4 /*yield*/, this._cursor.next()];
                    case 1:
                        event = _a.sent();
                        if (event) {
                            delete event._id;
                            return [2 /*return*/, {
                                    done: false,
                                    value: event
                                }];
                        }
                        else {
                            return [2 /*return*/, {
                                    done: true,
                                    value: null
                                }];
                        }
                        return [2 /*return*/];
                }
            });
        });
    };
    MongoEventsWithSnapshotIterator.prototype[Symbol.asyncIterator] = function () {
        return this;
    };
    return MongoEventsWithSnapshotIterator;
}());

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
var __decorate = (undefined && undefined.__decorate) || function (decorators, target, key, desc) {
    var c = arguments.length, r = c < 3 ? target : desc === null ? desc = Object.getOwnPropertyDescriptor(target, key) : desc, d;
    if (typeof Reflect === "object" && typeof Reflect.decorate === "function") r = Reflect.decorate(decorators, target, key, desc);
    else for (var i = decorators.length - 1; i >= 0; i--) if (d = decorators[i]) r = (c < 3 ? d(r) : c > 3 ? d(target, key, r) : d(target, key)) || r;
    return c > 3 && r && Object.defineProperty(target, key, r), r;
};
var __metadata = (undefined && undefined.__metadata) || function (k, v) {
    if (typeof Reflect === "object" && typeof Reflect.metadata === "function") return Reflect.metadata(k, v);
};
var __param = (undefined && undefined.__param) || function (paramIndex, decorator) {
    return function (target, key) { decorator(target, key, paramIndex); }
};
var __awaiter$1 = (undefined && undefined.__awaiter) || function (thisArg, _arguments, P, generator) {
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : new P(function (resolve) { resolve(result.value); }).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator$1 = (undefined && undefined.__generator) || function (thisArg, body) {
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
var promiseRetry = require('promise-retry');
/**
 * Mongo store
 *
 * @since 1.0.0
 */
var MongoStore = /** @class */ (function (_super) {
    __extends(MongoStore, _super);
    /* istanbul ignore next */
    function MongoStore(options, _logger) {
        var _this = _super.call(this) || this;
        _this._logger = _logger;
        _this.url = options && options.url || process.env.MONGO_URL || 'mongodb://localhost:27017';
        _this.database = options && options.database || process.env.MONGO_DATABASE || 'eventific-test';
        return _this;
    }
    /**
     * @inheritDoc
     */
    MongoStore.prototype.start = function () {
        return __awaiter$1(this, void 0, void 0, function () {
            var _this = this;
            var _a, ex_1;
            return __generator$1(this, function (_b) {
                switch (_b.label) {
                    case 0:
                        _b.trys.push([0, 2, , 3]);
                        _a = this;
                        return [4 /*yield*/, promiseRetry({
                                maxTimeout: 10000
                            }, function (retry, count) {
                                return mongodb.MongoClient.connect(_this.url)
                                    .catch(function (err) {
                                    _this._logger.warn("Failed to connect with mongodb, current attempt: " + count);
                                    retry(err);
                                });
                            })];
                    case 1:
                        _a._client = _b.sent();
                        return [3 /*break*/, 3];
                    case 2:
                        ex_1 = _b.sent();
                        throw new Error('Could not connect to the mongo database');
                    case 3:
                        this._db = this._client.db(this.database);
                        return [2 /*return*/];
                }
            });
        });
    };
    /**
     * @inheritDoc
     */
    MongoStore.prototype.getEvents = function (aggregateName, aggregateId, options) {
        return __awaiter$1(this, void 0, void 0, function () {
            var collection, snapshotCollection, approxVersion, snapshot;
            return __generator$1(this, function (_a) {
                switch (_a.label) {
                    case 0: return [4 /*yield*/, this._getCollection(aggregateName)];
                    case 1:
                        collection = _a.sent();
                        return [4 /*yield*/, this._getSnapshotCollection(aggregateName)];
                    case 2:
                        snapshotCollection = _a.sent();
                        return [4 /*yield*/, collection.count({ aggregateId: aggregateId })];
                    case 3:
                        approxVersion = (_a.sent()) - 1;
                        if (!(options && !options.skipSnapshot)) return [3 /*break*/, 5];
                        return [4 /*yield*/, snapshotCollection.findOne({ aggregateId: aggregateId })];
                    case 4:
                        snapshot = _a.sent();
                        _a.label = 5;
                    case 5: return [2 /*return*/, new MongoEventsWithSnapshotIterator(collection.find({ aggregateId: aggregateId }), approxVersion, snapshot || undefined)];
                }
            });
        });
    };
    MongoStore.prototype.getAllIds = function (aggregateName) {
        return __awaiter$1(this, void 0, void 0, function () {
            var collection, ids;
            return __generator$1(this, function (_a) {
                switch (_a.label) {
                    case 0: return [4 /*yield*/, this._getCollection(aggregateName)];
                    case 1:
                        collection = _a.sent();
                        return [4 /*yield*/, collection.distinct('aggregateId', {})];
                    case 2:
                        ids = _a.sent();
                        return [2 /*return*/, ids];
                }
            });
        });
    };
    /**
     * @inheritDoc
     */
    MongoStore.prototype.applyEvents = function (aggregateName, events) {
        return __awaiter$1(this, void 0, void 0, function () {
            var collection;
            return __generator$1(this, function (_a) {
                switch (_a.label) {
                    case 0: return [4 /*yield*/, this._getCollection(aggregateName)];
                    case 1:
                        collection = _a.sent();
                        return [4 /*yield*/, collection.insertMany(events)];
                    case 2:
                        _a.sent();
                        return [2 /*return*/];
                }
            });
        });
    };
    MongoStore.prototype.purgeAllSnapshots = function (aggregateName) {
        return __awaiter$1(this, void 0, void 0, function () {
            return __generator$1(this, function (_a) {
                return [2 /*return*/];
            });
        });
    };
    MongoStore.prototype.saveSnapshots = function (aggregateName, aggregateId, version, state) {
        return __awaiter$1(this, void 0, void 0, function () {
            var snapshotCollection;
            return __generator$1(this, function (_a) {
                switch (_a.label) {
                    case 0: return [4 /*yield*/, this._getSnapshotCollection(aggregateName)];
                    case 1:
                        snapshotCollection = _a.sent();
                        return [2 /*return*/];
                }
            });
        });
    };
    MongoStore.prototype.listenForEvents$ = function (aggregateName) {
        var _this = this;
        return rxjs.Observable.create(function (observer) { return __awaiter$1(_this, void 0, void 0, function () {
            var _this = this;
            var collection, lastEvent, query, cursor, next;
            return __generator$1(this, function (_a) {
                switch (_a.label) {
                    case 0: return [4 /*yield*/, this._getCollection(aggregateName)];
                    case 1:
                        collection = _a.sent();
                        return [4 /*yield*/, collection.findOne({}, {
                                sort: { $natural: -1 }
                            })];
                    case 2:
                        lastEvent = _a.sent();
                        query = {};
                        if (lastEvent) {
                            query = { eventId: { $gt: lastEvent.eventId } };
                        }
                        cursor = collection
                            .find(query)
                            .addCursorFlag('tailable', true)
                            .addCursorFlag('awaitData', true)
                            .setCursorOption('numberOfRetries', Number.MAX_VALUE);
                        _a.label = 3;
                    case 3:
                        if (!!cursor.isClosed()) return [3 /*break*/, 5];
                        return [4 /*yield*/, cursor.next()];
                    case 4:
                        next = _a.sent();
                        if (next) {
                            observer.next(next);
                        }
                        else {
                            observer.error(new Error('No data'));
                            return [3 /*break*/, 5];
                        }
                        return [3 /*break*/, 3];
                    case 5: return [2 /*return*/, function () { return __awaiter$1(_this, void 0, void 0, function () {
                            return __generator$1(this, function (_a) {
                                switch (_a.label) {
                                    case 0: return [4 /*yield*/, cursor.close()];
                                    case 1:
                                        _a.sent();
                                        return [2 /*return*/];
                                }
                            });
                        }); }];
                }
            });
        }); });
    };
    MongoStore.prototype.onEvent = function (aggregateName, eventName, callback) {
        var _this = this;
        Joi.assert(aggregateName, Joi.string(), 'Aggregate name has to be a string and cannot be empty');
        Joi.assert(callback, Joi.func(), 'callback must be a function');
        var query = eventName ? { event: eventName } : undefined;
        // .sort({$natural: -1}).limit(1)
        this._getCollection(aggregateName).then(function (collection) { return __awaiter$1(_this, void 0, void 0, function () {
            var _this = this;
            return __generator$1(this, function (_a) {
                switch (_a.label) {
                    case 0: return [4 /*yield*/, promiseRetry({
                            maxTimeout: 3000
                        }, function (retry, count) {
                            return new Promise(function (resolve, reject) {
                                var stream = collection
                                    .find(query)
                                    .addCursorFlag('tailable', true)
                                    .addCursorFlag('awaitData', true)
                                    .setCursorOption('numberOfRetries', Number.MAX_VALUE)
                                    .stream();
                                stream.on('data', function (data) { return __awaiter$1(_this, void 0, void 0, function () {
                                    var ex_2;
                                    return __generator$1(this, function (_a) {
                                        switch (_a.label) {
                                            case 0:
                                                _a.trys.push([0, 2, , 3]);
                                                delete data._id;
                                                return [4 /*yield*/, callback(data)];
                                            case 1:
                                                _a.sent();
                                                return [3 /*break*/, 3];
                                            case 2:
                                                ex_2 = _a.sent();
                                                this._logger.error('Error occurred when passing event on to handler', ex_2);
                                                return [3 /*break*/, 3];
                                            case 3: return [2 /*return*/];
                                        }
                                    });
                                }); });
                                stream.on('error', reject);
                                stream.on('close', function () { return __awaiter$1(_this, void 0, void 0, function () {
                                    return __generator$1(this, function (_a) {
                                        reject();
                                        return [2 /*return*/];
                                    });
                                }); });
                            }).catch(retry);
                        })];
                    case 1:
                        _a.sent();
                        return [2 /*return*/];
                }
            });
        }); });
    };
    MongoStore.prototype._getCollection = function (aggregateName) {
        return __awaiter$1(this, void 0, void 0, function () {
            var ex_3;
            return __generator$1(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        _a.trys.push([0, 3, , 4]);
                        return [4 /*yield*/, this._db.createCollection(aggregateName.toLowerCase(), { capped: true, size: 1000000000, max: 50000000 })];
                    case 1:
                        _a.sent();
                        return [4 /*yield*/, this._db.createIndex(aggregateName.toLowerCase(), { aggregateId: 1, eventId: 1 }, { unique: true })];
                    case 2:
                        _a.sent();
                        return [3 /*break*/, 4];
                    case 3:
                        ex_3 = _a.sent();
                        return [3 /*break*/, 4];
                    case 4: return [2 /*return*/, this._db.collection(aggregateName.toLowerCase())];
                }
            });
        });
    };
    MongoStore.prototype._getSnapshotCollection = function (aggregateName) {
        return __awaiter$1(this, void 0, void 0, function () {
            var ex_4;
            return __generator$1(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        _a.trys.push([0, 3, , 4]);
                        return [4 /*yield*/, this._db.createCollection(aggregateName.toLowerCase() + '-snapshots')];
                    case 1:
                        _a.sent();
                        return [4 /*yield*/, this._db.createIndex(aggregateName.toLowerCase() + '-snapshots', { aggregateId: 1 }, { unique: true })];
                    case 2:
                        _a.sent();
                        return [3 /*break*/, 4];
                    case 3:
                        ex_4 = _a.sent();
                        return [3 /*break*/, 4];
                    case 4: return [2 /*return*/, this._db.collection(aggregateName.toLowerCase() + '-snapshots')];
                }
            });
        });
    };
    MongoStore = __decorate([
        core.Store({
            name: 'Mongo'
        }),
        __param(0, core.InjectSettings()),
        __metadata("design:paramtypes", [Object, core.Logger])
    ], MongoStore);
    return MongoStore;
}(core.IStore));

exports.MongoStore = MongoStore;
