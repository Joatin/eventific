

export abstract class Logger {
  public abstract readonly name: string;
  public abstract raw(message: string): void;
  public abstract error(message: string, ...meta: any[]): void;
  public abstract warn(message: string, ...meta: any[]): void;
  public abstract info(message: string, ...meta: any[]): void;
  public abstract verbose(message: string, ...meta: any[]): void;
  public abstract debug(message: string, ...meta: any[]): void;
  public abstract silly(message: string, ...meta: any[]): void;
  public abstract getNamed(name: string): Logger;
}
