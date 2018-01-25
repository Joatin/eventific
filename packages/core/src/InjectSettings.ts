import "reflect-metadata";
import { SettingSymbol } from './Injector';

export function InjectSettings() {
  return (target: Function, key: string, index: any) => {
    const params = Reflect.getMetadata('injector:params', target) || [];
    params.push({
      type: SettingSymbol,
      required: false,
      index
    });
    Reflect.defineMetadata('injector:params', params, target);
  }
}
