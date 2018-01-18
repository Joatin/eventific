
export function bootstrap(type: any) {
  if (type.Type && type._Instantiate && type.Type === 'CommandManager') {
    if (type._Instantiate) {
      const inst = type._Instantiate();
      inst._start();
    }
  }

}
