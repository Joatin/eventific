
export interface SagaOptions {
  aggregates: any[];
  store: any;
}

export function Saga(options: SagaOptions) {
  return <T extends {new(...args: any[]): {}}>(constructor: T) => {
    return class extends constructor {
      public static Name = options.name;
      public name = options.name;
    };
  };
}
