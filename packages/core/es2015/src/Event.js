"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : new P(function (resolve) { resolve(result.value); }).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
const EventMessage_1 = require("./EventMessage");
const Joi = require("joi");
const Logger_1 = require("./Logger");
/**
 * OBS: Needed until typescript supports decorator type extensions.
 *
 * @since 1.0.0
 */
class IEventHandler {
}
exports.IEventHandler = IEventHandler;
/**
 * Creates a new event.
 *
 * @since 1.0.0
 * @returns {IEventHandler<any>} A decorated class that implements IEvent
 */
function EventHandler(options) {
    return (Class) => {
        return _a = class extends Class {
                constructor(...args) {
                    super(...args[0].args(Class));
                    this.event = options.event;
                    this._logger = args[0].get(Logger_1.Logger);
                }
                static _InstantiateEventHandler(parentInjector) {
                    const injector = parentInjector.newChildInjector();
                    return new this(injector);
                }
                _validateAndHandle(event, state) {
                    return __awaiter(this, void 0, void 0, function* () {
                        let schema = EventMessage_1.eventMessageSchema;
                        if (options.schema) {
                            schema = EventMessage_1.eventMessageSchema.keys({
                                content: options.schema.required()
                            });
                        }
                        else {
                            schema = EventMessage_1.eventMessageSchema.keys({
                                content: Joi.any()
                            });
                        }
                        Joi.assert(event, schema);
                        if (this.handle) {
                            return yield this.handle(event, state);
                        }
                        else {
                            this._logger.error(`The event handler "${this.event}" has no handle method`);
                            throw new Error('No handle method');
                        }
                    });
                }
            },
            _a.Type = 'Event',
            _a.Event = options.event,
            _a;
        var _a;
    };
}
exports.EventHandler = EventHandler;
//# sourceMappingURL=Event.js.map