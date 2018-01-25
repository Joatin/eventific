"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
require("reflect-metadata");
var assert = require("assert");
var Injector = /** @class */ (function () {
    function Injector(parent) {
        this._dependencies = new Map();
        this._parent = parent;
    }
    Injector.prototype.set = function (dependency) {
        if (dependency.useClass) {
            assert(isClass(dependency.useClass), 'The provided class has to actually be a class');
            if (dependency.provide) {
                var key = this._getProvideKey(dependency.provide);
                this._dependencies.set(key, dependency);
            }
            else {
                this._dependencies.set(dependency.useClass.name, dependency);
            }
        }
        else if (dependency.useConstant) {
            var key = this._getProvideKey(dependency.provide);
            this._dependencies.set(key, dependency);
        }
        else {
            assert(isClass(dependency), 'The provided class has to actually be a class');
            this._dependencies.set(dependency.name, { useClass: dependency });
        }
    };
    Injector.prototype.get = function (type) {
        var result = this.getOptional(type);
        if (result) {
            return result;
        }
        else {
            throw new Error('InjectionError: No provider for type: ' + type.toString());
        }
    };
    Injector.prototype.getOptional = function (type) {
        var key = this._getProvideKey(type);
        var result = this._dependencies.get(key);
        if (!result && this._parent) {
            return this._parent.getOptional(type);
        }
        else if (result) {
            if (result.useClass) {
                return new ((_a = result.useClass).bind.apply(_a, [void 0].concat(this.args(result.useClass))))();
            }
            else {
                return result.useConstant;
            }
        }
        else {
            return undefined;
        }
        var _a;
    };
    Injector.prototype.args = function (type) {
        var _this = this;
        var types = this._getTypes(type);
        var args = [];
        types.forEach(function (param) {
            if (param.required) {
                args.push(_this.get(param.type));
            }
            else {
                args.push(_this.getOptional(param.type));
            }
        });
        return args;
    };
    Injector.prototype._getTypes = function (type) {
        assert.notEqual(type, undefined);
        var definedTypes = Reflect.getMetadata('injector:params', type) || [];
        var params = Reflect.getMetadata("design:paramtypes", type) || [];
        var types = new Array(params.length);
        params.forEach(function (param, index) {
            types[index] = {
                required: true,
                type: param.name
            };
        });
        definedTypes.forEach(function (def) {
            if (def.index >= params.length) {
                throw new Error('InjectionError: injector:params has defined a param that has a greater index than the total amount of params');
            }
            types[def.index] = {
                required: def.required,
                type: def.type
            };
        });
        types.forEach(function (param, index) {
            if (param.type === 'Number' ||
                param.type === 'String' ||
                param.type === 'Boolean' ||
                param.type === 'Object' ||
                param.type === 'Array' ||
                param.type === 'Function' ||
                param.type === 'Object') {
                throw new Error("InjectionError: param at index: " + index + " on type " + type.name + " is of a basic type and does not have a @Inject annotation");
            }
        });
        return types;
    };
    Injector.prototype._getProvideKey = function (provide) {
        var type = typeof provide;
        switch (type) {
            case 'string': {
                return provide;
            }
            case 'number': {
                throw new Error('InjectionError: Numbers are not a supported provide type');
            }
            case 'boolean': {
                throw new Error('InjectionError: Booleans are not a supported provide type');
            }
            case 'function': {
                return provide.name;
            }
            default: {
                throw new Error("InjectionError: " + type + " are not a supported provide type");
            }
        }
    };
    Injector.prototype.newChildInjector = function () {
        return new Injector(this);
    };
    return Injector;
}());
exports.Injector = Injector;
function isClass(v) {
    // return typeof v === 'function' && /^\s*class\s+/.test(v.toString());
    return typeof v === 'function';
}
//# sourceMappingURL=Injector.js.map