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
/**
 * A interface for event stores.
 *
 * @since 1.0.0
 */
var IStore = /** @class */ (function () {
    function IStore() {
    }
    return IStore;
}());
exports.IStore = IStore;
/**
 * Store decorator
 * @param {StoreOptions} options
 * @returns {<T extends {new(...args: any[]) => {}}>(Class: T) => T}
 * @constructor
 */
function Store(options) {
    return function (Class) {
        return _a = /** @class */ (function (_super) {
                __extends(class_1, _super);
                function class_1() {
                    var _this = _super !== null && _super.apply(this, arguments) || this;
                    _this.name = options.name;
                    return _this;
                }
                class_1._CreateStore = function (injector) {
                    return new (this.bind.apply(this, [void 0].concat(injector.args(Class))))();
                };
                return class_1;
            }(Class)),
            _a.Name = options.name,
            _a;
        var _a;
    };
}
exports.Store = Store;
//# sourceMappingURL=Store.js.map