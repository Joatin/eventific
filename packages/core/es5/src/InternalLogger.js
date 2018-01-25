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
Object.defineProperty(exports, "__esModule", { value: true });
var Logger_1 = require("./Logger");
var chalk_1 = require("chalk");
var InternalLogger = /** @class */ (function (_super) {
    __extends(InternalLogger, _super);
    function InternalLogger(loggerName) {
        var _this = _super.call(this) || this;
        _this.loggerName = loggerName;
        _this.name = loggerName || '';
        return _this;
    }
    InternalLogger.prototype.raw = function (message) {
        process.stdout.write(message + '\n');
    };
    InternalLogger.prototype.error = function (message) {
        var meta = [];
        for (var _i = 1; _i < arguments.length; _i++) {
            meta[_i - 1] = arguments[_i];
        }
        var formattedName = '';
        if (this.name) {
            formattedName = " [" + this.name + "]";
        }
        process.stderr.write(chalk_1.default.red('error') + ":" + formattedName + " " + message + "\n");
    };
    InternalLogger.prototype.warn = function (message) {
        var meta = [];
        for (var _i = 1; _i < arguments.length; _i++) {
            meta[_i - 1] = arguments[_i];
        }
        process.stderr.write(chalk_1.default.redBright('warn') + ": " + message + "\n");
    };
    InternalLogger.prototype.info = function (message) {
        var meta = [];
        for (var _i = 1; _i < arguments.length; _i++) {
            meta[_i - 1] = arguments[_i];
        }
        var formattedName = '';
        if (this.name) {
            formattedName = " [" + this.name + "]";
        }
        process.stdout.write(chalk_1.default.cyan('info') + ":" + formattedName + " " + message + "\n");
    };
    InternalLogger.prototype.verbose = function (message) {
        var meta = [];
        for (var _i = 1; _i < arguments.length; _i++) {
            meta[_i - 1] = arguments[_i];
        }
        var formattedName = '';
        if (this.name) {
            formattedName = " [" + this.name + "]";
        }
        process.stdout.write(chalk_1.default.yellow('verbose') + ":" + formattedName + " " + message + "\n");
    };
    InternalLogger.prototype.debug = function (message) {
        var meta = [];
        for (var _i = 1; _i < arguments.length; _i++) {
            meta[_i - 1] = arguments[_i];
        }
        process.stdout.write(chalk_1.default.green('debug') + ": " + message + "\n");
    };
    InternalLogger.prototype.silly = function (message) {
        var meta = [];
        for (var _i = 1; _i < arguments.length; _i++) {
            meta[_i - 1] = arguments[_i];
        }
        process.stdout.write(chalk_1.default.magenta('silly') + ": " + message + "\n");
    };
    InternalLogger.prototype.getNamed = function (name) {
        return this;
    };
    return InternalLogger;
}(Logger_1.Logger));
exports.InternalLogger = InternalLogger;
//# sourceMappingURL=InternalLogger.js.map