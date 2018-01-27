import { CommandMessage, InjectSettings, ITransport, Logger, Transport } from '@eventific/core';
import { Channel, connect } from 'amqplib';
import promiseRetry = require('promise-retry');

@Transport({
  name: 'Rabbit'
})
export class RabbitTransport extends ITransport {

  private _channel: Channel;

  constructor(
    @InjectSettings() settings: any | undefined,
    private _logger: Logger
  ) {
    super();
  }

  /**
   * @inheritDoc
   */
  public async start(): Promise<void> {
    try {
      const connection = await promiseRetry({
        maxTimeout: 10000
      }, (retry: any, count: number) => {
        return connect('amqp://localhost:5672')
          .catch((err) => {
            this._logger.warn(
              `Failed to connect with rabbitmq, current attempt: ${count}`,
              err
            );
            retry(err);
          }) as any;
      });
      this._channel = await connection.createChannel();
    } catch (ex) {
      throw new Error('Could not connect to the rabbitmq');
    }
  }

  /**
   * @inheritDoc
   */
  public onCommand(aggregateName: string, handler: (data: CommandMessage) => Promise<void>): void {
    if (this._channel) {
      this._listenToQueue(aggregateName, handler).catch((ex) => {
        throw ex;
      });
    } else {
      throw new Error('You have to start the transport first');
    }
  }

  /**
   * @inheritDoc
   */
  public async sendCommand(aggregateName: string, data: CommandMessage): Promise<void> {
    if (this._channel) {
      const queue = `aggregate.${aggregateName}`;
      await this._channel.sendToQueue(queue, Buffer.from(JSON.stringify(data)));
    } else {
      throw new Error('You have to start the transport first');
    }
  }

  private async _listenToQueue(aggregateName: string, handler: (data: CommandMessage) => Promise<void>) {
    const queue = `aggregate.${aggregateName}`;
    await this._channel.assertQueue(queue);
    await this._channel.consume(queue, (msg) => {
      if (msg) {
        const command = JSON.parse(msg.content.toString());
        this._logger.verbose(`Received command`, command);
        handler(command).then(() => {
          this._logger.verbose(`Successfully handled command`, command);
          this._channel.ack(msg);
        }, (ex) => {
          this._logger.verbose(`Failed to handle message`, command, ex);
          this._channel.nack(msg);
        });
      }
    });
  }
}
