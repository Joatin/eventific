
export interface Snapshot<T extends object = {}> {
  version: number;
  aggregateId: string;
  state: T;
}
