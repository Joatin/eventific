import { IQueryManager, QueryManager, IViewHandler, ViewHandler } from '@eventific/query';
import { AccountAggregate } from './account/account.aggregate';
import { MockStore } from '@eventific/test';
import axios from 'axios';
import * as Koa from 'koa';
import { bootstrap, Inject } from '@eventific/core';
import * as _ from 'koa-route';
import { CreatedEvent } from './account/created.event';


test('It should be able to receive requests and return them', async () => {

  @ViewHandler({
    name: 'Test'
  })
  class TestViewHandler extends IViewHandler {
    private views = new Map<string, any>();

    constructor(
      @Inject('KOA') private _app: any
    ) {}

    async start() {
      this._app.use(_.get('/account/:id', async (ctx, id) => {
        ctx.body = JSON.stringify(this.views.get(id));
      }))
    }

    async buildAndPersistView(aggregateId: string, state: any, version: number): Promise<void> {
      this.views.set(aggregateId, state);
    }
  }
  const app = new Koa();

  @QueryManager({
    aggregates: [AccountAggregate],
    store: MockStore,
    viewHandlers: [
      TestViewHandler
    ],
    providers: [
      {provide: 'KOA', useConstant: app}
    ]
  })
  class TestQueryManager extends IQueryManager {
    async onInit({injector}) {
      app.listen(3000);
    }
  }

  await bootstrap(TestQueryManager);

  await MockStore.ApplyEvents('Account', [{
    event: CreatedEvent.Event,
    eventId: 0,
    aggregateId: '27e7b187-5a11-41fe-afb7-a071c6c17b6d',
    header: {
      createdDate: new Date()
    },
    content: {}
  },
    {
      event: 'ADDED',
      eventId: 1,
      aggregateId: '27e7b187-5a11-41fe-afb7-a071c6c17b6d',
      header: {
        createdDate: new Date()
      },
      content: {
        amount: 30
      }
    }]);
  const result = await axios.get('http://localhost:3000/account/27e7b187-5a11-41fe-afb7-a071c6c17b6d');
  console.dir(result.data);
  expect(result.data.balance).toEqual(30);

});
