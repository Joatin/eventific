// @public (undocumented)
interface AggregateOptions {
  commandHandlers: Array<{
      Command: string;
      new(...args: any[]): ICommandHandler<any, any>;
      _InstantiateCommandHandler(injector: Injector): ICommandHandler<any, any>;
    }>;
  eventHandlers: Array<{
      Event: string;
      new(...args: any[]): IEventHandler<any, any>;
      _InstantiateEventHandler(injector: Injector): IEventHandler<any, any>;
    }>;
  name: string;
  // (undocumented)
  providers?: any[];
}

// @public
export async function bootstrap<T>(type: {
  Type: string;
  _Instantiate: (injector: Injector) => Bootstrapable
}): Promise<void> {
  process.env.NODE_ENV = process.env.NODE_ENV || 'development';
  const injector = new Injector();
  injector.set({provide: Logger, useConstant: new InternalLogger()});
  const logger = injector.get<Logger>(Logger);
  logger.raw(chalk.green(banner));
  logger.info(`Launching Eventific ${emoji.get('rocket')}`);
  logger.info(`Version: ${process.env.npm_package_version || 'unknown'}`);
  logger.info(`Environment: ${process.env.NODE_ENV} ${emoji.get('eyes')}`);
  if (type._Instantiate) {
    logger.info(`Type: ${type.Type}`);
    const inst = type._Instantiate(injector);
    logger.info(`Starting application ${emoji.get('dancer')}`);
    await inst._start();
  } else {
    logger.error('The provided type does not seem to be a bootstrap able module');
    throw new Error('Failed to start');
  }
}

// @beta (undocumented)
class Bootstrapable {
  // (undocumented)
  public _Instantiate: (injector: Injector) => Bootstrapable;
  // (undocumented)
  public _start: () => Promise<void>;
  // (undocumented)
  public static Type: string;
}

// @beta (undocumented)
interface ClassDependencyDefinition {
  // (undocumented)
  dynamic?: true;
  // (undocumented)
  provide?: string | symbol | Function;
  // (undocumented)
  useClass: Function;
}

// @public (undocumented)
interface CommandHandlerOptions {
  // (undocumented)
  command: string;
}

// @public (undocumented)
interface CommandMessage<T = {}> {
  // (undocumented)
  aggregateId: string;
  // (undocumented)
  command: string;
  // (undocumented)
  content: T;
  // (undocumented)
  header: {
    createdBy?: string;
    createdDate: Date;
  };
}

// @public (undocumented)
interface EventHandlerOptions {
  // (undocumented)
  event: string;
  // (undocumented)
  schema?: Schema;
}

// @public (undocumented)
interface EventMessage<T = undefined> {
  // (undocumented)
  aggregateId: string;
  // (undocumented)
  content: T;
  // (undocumented)
  event: string;
  // (undocumented)
  eventId: number;
  // (undocumented)
  header: {
    createdDate: Date;
  };
}

// @public (undocumented)
interface GetEventsResult<T> {
  // (undocumented)
  events: EventMessage[];
  // (undocumented)
  snapshot?: Snapshot<T>;
}

// @public
class IAggregate {
  // (undocumented)
  public static _InstantiateAggregate: (parentInjector: Injector) => IAggregate;
  // (undocumented)
  public getCommandNames: () => string[];
  // (undocumented)
  public getEventNames: () => string[];
  // (undocumented)
  public getState: (aggregateId: string) => Promise<{version: number, state: any}>;
  public handleCommand: (commandMessage: CommandMessage) => Promise<void>;
  public readonly name: string;
  // (undocumented)
  public static Name: string;
  // (undocumented)
  public static Type: string;
}

// @public (undocumented)
class ICommandHandler<T, R> {
  // (undocumented)
  public static _InstantiateCommandHandler: (injector: Injector) => ICommandHandler<any, any>;
  // (undocumented)
  public readonly command: string;
  // (undocumented)
  public static Command: string;
  // (undocumented)
  public abstract handle(message: CommandMessage<T>, state: R, version: number): Promise<EventMessage[]>;
}

// @public
class IEventHandler<T, R> {
  // (undocumented)
  public static _InstantiateEventHandler: (injector: Injector) => IEventHandler<any, any>;
  // (undocumented)
  public _validateAndHandle: (event: EventMessage<T>, state: R) => Promise<R>;
  // (undocumented)
  public readonly event: string;
  // (undocumented)
  public static Event: string;
  // (undocumented)
  public abstract handle(event: EventMessage<T>, state: R): Promise<R>;
  // (undocumented)
  public static Type: string;
}

// @public (undocumented)
class Injector {
  constructor(parent?: Injector) {
      this._parent = parent;
    }
  // (undocumented)
  public args(type: Function, setting?: object): any[] {
      const types = this._getTypes(type);
      const args: any[] = [];
      types.forEach((param) => {
        if (param.type === SettingSymbol) {
          args.push(setting);
        } else {
          if (param.required) {
            args.push(this.get(param.type));
          } else {
            args.push(this.getOptional(param.type));
          }
        }

      });
      return args;
    }
  // (undocumented)
  public get<T = any>(type: string | symbol | Function): T {
      const result = this.getOptional<T>(type);
      if (result) {
        return result;
      } else {
        throw new Error('InjectionError: No provider for type: ' + type.toString());
      }
    }
  // (undocumented)
  public getOptional<T = any>(type: string | symbol | Function): T | undefined {
      const key = this._getProvideKey(type);
      const result = this._dependencies.get(key);
      if (!result && this._parent) {
        return this._parent.getOptional<T>(type);
      } else if (result) {
        if ((result as ClassDependencyDefinition).useClass) {
          return new ((result as ClassDependencyDefinition).useClass as any)
          (...this.args((result as ClassDependencyDefinition).useClass)) as T;
        } else {
          return (result as ValueDependencyDefinition).useConstant as any;
        }
      } else {
        return undefined;
      }
    }
  // (undocumented)
  public newChildInjector(): Injector {
      return new Injector(this);
    }
  // (undocumented)
  public set(dependency: ClassDependencyDefinition | ValueDependencyDefinition | { new(...args: any[]): {} }): void {
      if ((dependency as ClassDependencyDefinition).useClass) {
        assert(
          isClass((dependency as ClassDependencyDefinition).useClass),
          'The provided class has to actually be a class'
        );
        if ((dependency as ClassDependencyDefinition).provide) {
          const key = this._getProvideKey((dependency as ClassDependencyDefinition).provide);
          this._dependencies.set(key, (dependency as ClassDependencyDefinition));
        } else {
          this._dependencies.set(
            (dependency as ClassDependencyDefinition).useClass.name,
            (dependency as ClassDependencyDefinition)
          );
        }
      } else if ((dependency as ValueDependencyDefinition).useConstant) {
        const key = this._getProvideKey((dependency as ValueDependencyDefinition).provide);
        this._dependencies.set(key, (dependency as ValueDependencyDefinition));
      } else {
        assert(isClass((dependency as { new(...args: any[]): {} })), 'The provided class has to actually be a class');
        this._dependencies.set(
          (dependency as { new(...args: any[]): {} }).name,
          { useClass: (dependency as { new(...args: any[]): {} }) }
        );
      }
    }
}

// @public (undocumented)
class InternalLogger extends Logger {
  constructor(
      readonly loggerName?: string
    ) {
      super();
      this.name = loggerName || '';
    }
  // (undocumented)
  public debug(message: string, ...meta: any[]): void {
      this._doLog(chalk.green('debug'), message, meta);
    }
  // (undocumented)
  public error(message: string, ...meta: any[]): void {
      this._doLog(chalk.red('error'), message, meta, true);
    }
  // (undocumented)
  public getNamed(name: string): Logger {
      return this;
    }
  // (undocumented)
  public info(message: string, ...meta: any[]): void {
      this._doLog(chalk.cyan('info'), message, meta);
    }
  // (undocumented)
  public readonly name: string;
  // (undocumented)
  public raw(message: string): void {
      process.stdout.write(message + '\n');
    }
  // (undocumented)
  public silly(message: string, ...meta: any[]): void {
      this._doLog(chalk.gray('silly'), message, meta);
    }
  // (undocumented)
  public verbose(message: string, ...meta: any[]): void {
      this._doLog(chalk.magenta('verbose'), message, meta);
    }
  // (undocumented)
  public warn(message: string, ...meta: any[]): void {
      this._doLog(chalk.yellow('warn'), message, meta, true);
    }
}

// @public
class IStore {
  // (undocumented)
  public static _CreateStore: (injector: Injector) => IStore;
  // (undocumented)
  public abstract applyEvents<T>(aggregateName: string, events: EventMessage[], state?: T): Promise<void>;
  public abstract getEvents<T>(aggregateName: string, aggregateId: string): Promise<GetEventsResult<T>>;
  // (undocumented)
  public abstract onEvent(
      aggregateName: string,
      eventName: string | null,
      callback: (event: EventMessage) => Promise<void>
    ): void;
  // (undocumented)
  public abstract purgeAllSnapshots(aggregateName: string): Promise<void>;
  // (undocumented)
  public static Settings: (settings: object) => { _CreateStore: (injector: Injector) => IStore };
  public abstract start(): Promise<any>;
}

// @public (undocumented)
class ITransport {
  // (undocumented)
  public static _CreateTransport: (injector: Injector) => ITransport;
  // (undocumented)
  public onCommand?(aggregateName: string, handler: (data: CommandMessage) => Promise<void>): void;
  // (undocumented)
  public sendCommand?(aggregateName: string, data: CommandMessage): Promise<void>;
  // (undocumented)
  public static Settings: (settings: object) => { _CreateTransport: (injector: Injector) => ITransport };
  // (undocumented)
  public abstract start(): Promise<void>;
}

// @public (undocumented)
class Logger {
  // (undocumented)
  public abstract debug(message: string, ...meta: any[]): void;
  // (undocumented)
  public abstract error(message: string, ...meta: any[]): void;
  // (undocumented)
  public abstract getNamed(name: string): Logger;
  // (undocumented)
  public abstract info(message: string, ...meta: any[]): void;
  // (undocumented)
  public readonly name: string;
  // (undocumented)
  public abstract raw(message: string): void;
  // (undocumented)
  public abstract silly(message: string, ...meta: any[]): void;
  // (undocumented)
  public abstract verbose(message: string, ...meta: any[]): void;
  // (undocumented)
  public abstract warn(message: string, ...meta: any[]): void;
}

// @public (undocumented)
interface StoreOptions {
  // (undocumented)
  name: string;
}

// @public (undocumented)
interface TransportOptions {
  // (undocumented)
  name: string;
}

// @beta (undocumented)
interface ValueDependencyDefinition {
  // (undocumented)
  provide: string | symbol | Function;
  // (undocumented)
  useConstant: any;
}

// WARNING: Store has incomplete type information
// WARNING: Transport has incomplete type information
// WARNING: Aggregate has incomplete type information
// WARNING: Unsupported export: commandMessageSchema
// WARNING: Unsupported export: eventMessageSchema
// WARNING: Inject has incomplete type information
// WARNING: CommandHandler has incomplete type information
// WARNING: EventHandler has incomplete type information
// WARNING: InjectSettings has incomplete type information
// (No @packagedocumentation comment for this package)
