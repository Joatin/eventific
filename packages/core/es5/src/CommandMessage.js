"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var Joi = require("joi");
exports.commandMessageSchema = Joi.object().keys({
    aggregateId: Joi.string().uuid().required(),
    command: Joi.string().required(),
    header: Joi.object().keys({
        createdDate: Joi.date().required(),
        createdBy: Joi.string().optional()
    }).required(),
    content: Joi.any().optional()
});
//# sourceMappingURL=CommandMessage.js.map