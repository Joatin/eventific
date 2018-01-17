
export interface Store {
  start(): Promise<any>
  getEvents(aggregateId: string): Promise<any[]>;
  applyEvents(events: any[]): Promise<void>;
}
