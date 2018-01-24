import "reflect-metadata";

export function Inject(type: string | symbol) {
  return (target: Function, key: string, index: any) => {
    const params = Reflect.getMetadata('injector:params', target) || [];
    params.push({
      type: type,
      required: true,
      index
    });
    Reflect.defineMetadata('injector:params', params, target);
  }
}