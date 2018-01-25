"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const Logger_1 = require("./Logger");
const InternalLogger_1 = require("./InternalLogger");
const chalk_1 = require("chalk");
const Joi = require("joi");
const pascalCase = require('pascal-case');
class ICommandHandler {
}
exports.ICommandHandler = ICommandHandler;
const commandHandlerOptionsSchema = Joi.object().keys({
    command: Joi.string().min(3).required()
});
function CommandHandler(options) {
    Joi.assert(options, commandHandlerOptionsSchema);
    return (Class) => {
        return _a = class extends Class {
                constructor(...args) {
                    super(...args[0].args(Class));
                    this.command = options.command;
                }
                static _InstantiateCommandHandler(parentInjector) {
                    const injector = parentInjector.newChildInjector();
                    injector.set({ provide: Logger_1.Logger, useConstant: new InternalLogger_1.InternalLogger(chalk_1.default.bgGreen(`${pascalCase(options.command)}Handler`)) });
                    return new this(injector);
                }
            },
            _a.Command = options.command,
            _a;
        var _a;
    };
}
exports.CommandHandler = CommandHandler;
//# sourceMappingURL=Command.js.map