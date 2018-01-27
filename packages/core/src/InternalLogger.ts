import chalk from 'chalk';
import * as util from 'util';
import { Logger } from './Logger';

export class InternalLogger extends Logger {
  public readonly name: string;

  constructor(
    readonly loggerName?: string
  ) {
    super();
    this.name = loggerName || '';
  }

  public raw(message: string): void {
    process.stdout.write(message + '\n');
  }

  public error(message: string, ...meta: any[]): void {
    this._doLog(chalk.red('error'), message, meta, true);
  }

  public warn(message: string, ...meta: any[]): void {
    this._doLog(chalk.yellow('warn'), message, meta, true);
  }

  public info(message: string, ...meta: any[]): void {
    this._doLog(chalk.cyan('info'), message, meta);
  }

  public verbose(message: string, ...meta: any[]): void {
    this._doLog(chalk.magenta('verbose'), message, meta);
  }

  public debug(message: string, ...meta: any[]): void {
    this._doLog(chalk.green('debug'), message, meta);
  }

  public silly(message: string, ...meta: any[]): void {
    this._doLog(chalk.gray('silly'), message, meta);
  }


  public getNamed(name: string): Logger {
    return this;
  }

  private _doLog(level: string, message: string, meta: any[], error?: boolean) {
    let stream = process.stdout;
    if (error) {
      stream = process.stderr;
    }
    let formattedName = '';
    if (this.name) {
      formattedName = ` [${this.name}]`;
    }
    stream.write(`${level}:${formattedName} ${message}\n`);
    for (const item of meta) {
      const data = util.inspect(item, {
        colors: true,
        depth: 5
      });
      const lines = data.split('\n');
      for (const line of lines) {
        stream.write(`${level}:${formattedName} ${line}\n`);
      }
      stream.write(`${level}:${formattedName}\n`);
    }

  }

}
