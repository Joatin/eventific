import { CommandMessage, InjectSettings, ITransport, Logger, Transport } from '@eventific/core';
import * as Koa from 'koa';
import * as bodyparser from 'koa-bodyparser';
import * as _ from 'koa-route';

@Transport({
  name: 'RestTransport'
})
export class RestTransport extends ITransport {
  private _app = new Koa();
  private _handler: (data: CommandMessage) => Promise<void>;
  private _port: number;

  constructor(
    @InjectSettings() options: {
      port?: number
    },
    private _logger: Logger
  ) {
    super();
    this._port = options && options.port || 1337;
    this._app.use(bodyparser());
  }

  public async start(): Promise<void> {
    await new Promise((resolve) => {
      this._app.listen(this._port).on('listening', () => {
        resolve();
      });
    });

    this._logger.info(`Listening on port: ${this._port}`);
  }

  public onCommand(aggregateName: string, handler: (data: CommandMessage) => Promise<void>): void {
    this._app.use(_.post(`/${aggregateName}`, async (ctx) => {
      const body = ctx.request.body;
      if (handler) {
        try {
          handler(body);
          ctx.body = JSON.stringify({ status: 'success' });
        } catch (ex) {
          this._logger.error(
            'Error thrown when calling the onCommand handler, the error was: ' + ex,
            ex
          );
          if (ex.message && ex.message.includes('DuplicateAggregate')) {
            ctx.throw(JSON.stringify({
              error: ex.message
            }), 400);
          } else if (ex.name && ex.name.includes('ValidationError')) {
            ctx.throw(JSON.stringify({
              error: ex.message
            }), 400);
          } else {
            this._logger.error('Unknown error, returning "Internal Server Error" to client' + ex, ex);
            ctx.throw(JSON.stringify('Internal Server Error'), 500);
          }
        }
      } else {
        this._logger.error('Command received but no on command handler was registered');
        ctx.throw(JSON.stringify('Service Unavailable'), 503);
      }
    }));
  }

}
