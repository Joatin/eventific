import { BaseEvent } from './Event';


export abstract class BaseCommand<T = undefined> {
    static Name: string;
    readonly name: string;
    readonly data: T;

    abstract handle(state: any, version: number): Promise<BaseEvent[]>
}