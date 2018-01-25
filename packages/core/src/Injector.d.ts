import 'reflect-metadata';
export interface ClassDependencyDefinition {
    provide?: string | symbol | Function;
    useClass: Function;
    dynamic?: true;
}
export interface ValueDependencyDefinition {
    provide: string | symbol | Function;
    useConstant: any;
}
export declare class Injector {
    private _parent?;
    private _dependencies;
    constructor(parent?: Injector);
    set(dependency: ClassDependencyDefinition | ValueDependencyDefinition | {
        new (...args: any[]): {};
    }): void;
    get<T = any>(type: string | symbol | Function): T;
    getOptional<T = any>(type: string | symbol | Function): T | undefined;
    args(type: Function): any[];
    private _getTypes(type);
    private _getProvideKey(provide);
    newChildInjector(): Injector;
}
