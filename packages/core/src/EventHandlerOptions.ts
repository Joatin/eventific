import { Schema } from 'joi';


export interface EventHandlerOptions {
  event: string;
  schema?: Schema;
}
