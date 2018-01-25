import 'reflect-metadata';

export function Inject(type: string | symbol) {
  return (target: Function, key: string, index: any) => {
    const params = Reflect.getMetadata('injector:params', target) || [];
    params.push({
      index,
      required: true,
      type
    });
    Reflect.defineMetadata('injector:params', params, target);
  };
}
