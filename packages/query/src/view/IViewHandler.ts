

export abstract class IViewHandler {
  public abstract start(): Promise<string>;
  public abstract buildAndPersistView(aggregateId: string, state: any, version: number): Promise<void>;
}
