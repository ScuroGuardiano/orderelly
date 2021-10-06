export class Service {
  constructor(
    public name: string,
    public path: string,
    public thumbailUrl: string = null
  ) {}
}

const serviceList = [
  new Service('Demo', '/demo')
];

export default serviceList;
