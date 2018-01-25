"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
/**
 * A interface for event stores.
 *
 * @since 1.0.0
 */
class IStore {
}
exports.IStore = IStore;
/**
 * Store decorator
 * @param {StoreOptions} options
 * @returns {<T extends {new(...args: any[]) => {}}>(Class: T) => T}
 * @constructor
 */
function Store(options) {
    return (Class) => {
        return _a = class extends Class {
                constructor() {
                    super(...arguments);
                    this.name = options.name;
                }
                static _CreateStore(injector) {
                    return new this(...injector.args(Class));
                }
            },
            _a.Name = options.name,
            _a;
        var _a;
    };
}
exports.Store = Store;
//# sourceMappingURL=Store.js.map