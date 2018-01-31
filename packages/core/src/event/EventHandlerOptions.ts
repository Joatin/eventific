import { Schema } from 'joi';

/**
 * @public
 */
export interface EventHandlerOptions {
  event: string;
  schema?: Schema;
}
