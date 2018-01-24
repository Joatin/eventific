

export abstract class Logger {
  abstract readonly name: string;
  abstract raw(message: string): void;
  abstract error(message: string, ...meta: any[]): void;
  abstract warn(message: string, ...meta: any[]): void;
  abstract info(message: string, ...meta: any[]): void;
  abstract verbose(message: string, ...meta: any[]): void;
  abstract debug(message: string, ...meta: any[]): void;
  abstract silly(message: string, ...meta: any[]): void;
  abstract getNamed(name: string): Logger;
}
