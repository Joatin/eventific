
export interface Transport {
  start(): Promise<any>
  onCommand(handler: (data: any) => Promise<void>): void
}
