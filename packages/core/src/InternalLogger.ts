import { Logger } from './Logger';
import chalk from 'chalk';

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
    let formattedName = '';
    if(this.name) {
      formattedName = ` [${this.name}]`
    }
    process.stderr.write(`${chalk.red('error')}:${formattedName} ${message}\n`);
  }

  public warn(message: string, ...meta: any[]): void {
    process.stderr.write(`${chalk.redBright('warn')}: ${message}\n`);
  }

  public info(message: string, ...meta: any[]): void {
    let formattedName = '';
    if(this.name) {
      formattedName = ` [${this.name}]`
    }
    process.stdout.write(`${chalk.cyan('info')}:${formattedName} ${message}\n`);
  }

  public verbose(message: string, ...meta: any[]): void {
    let formattedName = '';
    if(this.name) {
      formattedName = ` [${this.name}]`
    }
    process.stdout.write(`${chalk.yellow('verbose')}:${formattedName} ${message}\n`);
  }

  public debug(message: string, ...meta: any[]): void {
    process.stdout.write(`${chalk.green('debug')}: ${message}\n`);
  }

  public silly(message: string, ...meta: any[]): void {
    process.stdout.write(`${chalk.magenta('silly')}: ${message}\n`);
  }


  public getNamed(name: string): Logger {
    return this;
  }

}
