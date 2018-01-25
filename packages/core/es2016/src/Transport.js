"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
class ITransport {
}
exports.ITransport = ITransport;
function Transport(options) {
    return (Class) => {
        return _a = class extends Class {
                constructor() {
                    super(...arguments);
                    this.name = options.name;
                }
                static _CreateTransport(injector) {
                    return new this(...injector.args(Class));
                }
            },
            _a.Name = options.name,
            _a;
        var _a;
    };
}
exports.Transport = Transport;
//# sourceMappingURL=Transport.js.map