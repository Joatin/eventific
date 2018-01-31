import 'reflect-metadata';
import { SettingSymbol } from './Injector';

export function InjectSettings() {
  return (target: Function, key: string, index: any) => {
    const params = Reflect.getMetadata('injector:params', target) || [];
    params.push({
      index,
      required: false,
      type: SettingSymbol
    });
    Reflect.defineMetadata('injector:params', params, target);
  };
}
