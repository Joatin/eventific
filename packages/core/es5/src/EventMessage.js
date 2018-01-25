"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var Joi = require("joi");
exports.eventMessageSchema = Joi.object().keys({
    event: Joi.string().min(3).required(),
    eventId: Joi.number().min(0).required(),
    aggregateId: Joi.string().guid().required(),
    header: Joi.object().keys({
        createdDate: Joi.date().required()
    }).required(),
    content: Joi.any().optional()
});
//# sourceMappingURL=EventMessage.js.map