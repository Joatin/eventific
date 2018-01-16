
export interface Store {
  getEvents(aggregateId: string): Promise<any[]>;
  applyEvents(events: any[]): Promise<void>;
}
