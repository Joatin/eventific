export interface Snapshot<T> {
    version: number;
    aggregateId: string;
    state: T;
}
