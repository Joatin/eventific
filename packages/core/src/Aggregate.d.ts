import { AggregateOptions } from './AggregateOptions';
export declare function Aggregate(options: AggregateOptions): <T extends new (...args: any[]) => {}>(Class: T) => T;
