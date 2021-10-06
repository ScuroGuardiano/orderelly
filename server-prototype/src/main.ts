import { ServiceListElement } from './dtos';
import * as express from 'express';
import serviceList from './service-list';
import routerek from './demo';

const app = express();

app.get('/', (req, res) => {
  const services = serviceList.map<ServiceListElement>(service => {
    return {
      name: service.name,
      path: service.path,
      thumbnail: service.thumbailUrl || undefined
    }
  });

  res.status(200).json(services);
});

app.use('/demo', routerek);

app.listen(2137, () => {
  console.log("Service is listening on port 2137.");
});
