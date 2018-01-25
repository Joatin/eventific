import { Injector } from './Injector';
/**
 * Bootstraps a CommandManager, ReadManager, or Saga.
 *
 * @since 1.0.0
 *
 * @param type The type to instantiate
 * @returns {Promise<void>} A promise that resolves once the app is started
 */
export declare function bootstrap<T>(type: {
    Type: string;
    _Instantiate: (injector: Injector) => Bootstrapable;
}): Promise<void>;
export declare abstract class Bootstrapable {
    static Type: string;
    _Instantiate: (injector: Injector) => Bootstrapable;
    _start: () => Promise<void>;
}
