
export const ClientInstance = {
  db: jest.fn((name: string) => {
    return DbInstance;
  })
};

export const DbInstance = {
  collection: jest.fn((name: string) => {
    return CollectionInstance;
  }),
  createCollection: jest.fn(async () => {}),
  createIndex: jest.fn(async () => {})
};

export const CollectionInstance = {
  find: jest.fn((query: object) => {
    return CursorInstance;
  }),
  insertMany: jest.fn((docs: object[]) => {

  })
};

export const CursorInstance = {
  toArray: jest.fn(async () => {
    return [
      {
        eventId: 1,
        aggregateId: '1234'
      },
      {
        eventId: 2,
        aggregateId: '1234'
      },
      {
        eventId: 3,
        aggregateId: '1234'
      }
      ]
  })
};

export class MongoClient {
  static connect = jest.fn(async (url) => {
    return ClientInstance;
  });
}
