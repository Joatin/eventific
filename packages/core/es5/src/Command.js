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
var InternalLogger_1 = require("./InternalLogger");
var chalk_1 = require("chalk");
var Joi = require("joi");
var pascalCase = require('pascal-case');
var ICommandHandler = /** @class */ (function () {
    function ICommandHandler() {
    }
    return ICommandHandler;
}());
exports.ICommandHandler = ICommandHandler;
var commandHandlerOptionsSchema = Joi.object().keys({
    command: Joi.string().min(3).required()
});
function CommandHandler(options) {
    Joi.assert(options, commandHandlerOptionsSchema);
    return function (Class) {
        return _a = /** @class */ (function (_super) {
                __extends(class_1, _super);
                function class_1() {
                    var args = [];
                    for (var _i = 0; _i < arguments.length; _i++) {
                        args[_i] = arguments[_i];
                    }
                    var _this = _super.apply(this, args[0].args(Class)) || this;
                    _this.command = options.command;
                    return _this;
                }
                class_1._InstantiateCommandHandler = function (parentInjector) {
                    var injector = parentInjector.newChildInjector();
                    injector.set({ provide: Logger_1.Logger, useConstant: new InternalLogger_1.InternalLogger(chalk_1.default.bgGreen(pascalCase(options.command) + "Handler")) });
                    return new this(injector);
                };
                return class_1;
            }(Class)),
            _a.Command = options.command,
            _a;
        var _a;
    };
}
exports.CommandHandler = CommandHandler;
//# sourceMappingURL=Command.js.map