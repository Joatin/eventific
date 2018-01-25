"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
require("reflect-metadata");
function Inject(type) {
    return (target, key, index) => {
        const params = Reflect.getMetadata('injector:params', target) || [];
        params.push({
            type: type,
            required: true,
            index
        });
        Reflect.defineMetadata('injector:params', params, target);
    };
}
exports.Inject = Inject;
//# sourceMappingURL=Inject.js.map