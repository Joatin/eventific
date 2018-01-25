"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const Logger_1 = require("./Logger");
const chalk_1 = require("chalk");
class InternalLogger extends Logger_1.Logger {
    constructor(loggerName) {
        super();
        this.loggerName = loggerName;
        this.name = loggerName || '';
    }
    raw(message) {
        process.stdout.write(message + '\n');
    }
    error(message, ...meta) {
        let formattedName = '';
        if (this.name) {
            formattedName = ` [${this.name}]`;
        }
        process.stderr.write(`${chalk_1.default.red('error')}:${formattedName} ${message}\n`);
    }
    warn(message, ...meta) {
        process.stderr.write(`${chalk_1.default.redBright('warn')}: ${message}\n`);
    }
    info(message, ...meta) {
        let formattedName = '';
        if (this.name) {
            formattedName = ` [${this.name}]`;
        }
        process.stdout.write(`${chalk_1.default.cyan('info')}:${formattedName} ${message}\n`);
    }
    verbose(message, ...meta) {
        let formattedName = '';
        if (this.name) {
            formattedName = ` [${this.name}]`;
        }
        process.stdout.write(`${chalk_1.default.yellow('verbose')}:${formattedName} ${message}\n`);
    }
    debug(message, ...meta) {
        process.stdout.write(`${chalk_1.default.green('debug')}: ${message}\n`);
    }
    silly(message, ...meta) {
        process.stdout.write(`${chalk_1.default.magenta('silly')}: ${message}\n`);
    }
    getNamed(name) {
        return this;
    }
}
exports.InternalLogger = InternalLogger;
//# sourceMappingURL=InternalLogger.js.map