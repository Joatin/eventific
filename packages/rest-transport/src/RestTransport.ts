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

    this._app.use(_.post('/commands', async (ctx) => {
      const body = ctx.request.body;
      try {
        await this._handler(body);
        ctx.body = JSON.stringify({status: 'success'});
      } catch (ex) {
        this._logger.error('Error occurred', ex);
        if (ex.message && ex.message.includes('DuplicateAggregate')) {
          ctx.throw(JSON.stringify({
            error: ex.message
          }), 400);
        } else if (ex.name && ex.name.includes('ValidationError')) {
          ctx.throw(JSON.stringify({
            error: ex.message
          }), 400);
        } else {
          this._logger.error('Error occurred', ex);
          ctx.throw(JSON.stringify('Internal Server Error'), 500);
        }
      }
    }));
  }

  public async start(): Promise<void> {
    this._app.listen(this._port);
  }

  public onCommand(handler: (data: CommandMessage) => Promise<void>): void {
    this._handler = handler;
  }

}
