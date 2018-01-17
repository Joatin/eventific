

export interface SagaOptions {
  aggregates: any[],
  store: any
}

export function Saga(options: SagaOptions) {
  return <T extends {new(...args: any[]): {}}>(constructor: T) => {
    return class extends constructor {
      static Name = options.name;
      name = options.name;
    };
  };
}

