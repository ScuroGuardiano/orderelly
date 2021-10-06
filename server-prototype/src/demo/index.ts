import { Router } from "express";
import { IImage, PaginatedImageList, ServiceInfo } from "../dtos";
import { promises as fs } from 'fs';
import { last } from "../util";
import { Response } from 'express';

const routerek = Router();
export default routerek;

routerek.get('/', (req, res: Response<ServiceInfo>) => {
  const serviceInfo: ServiceInfo = {
    name: "Demo",
    path: "/demo",
    url: "/demo",
    description: "Demo service serving static data to show how should API work and test if shit is working.",
    color: "black"
  }

  res.status(200).json(serviceInfo);
});

routerek.get('/query', async (req, res: Response<PaginatedImageList>) => {
  const imagesDir = __dirname + "/images";
  let fileNames = await fs.readdir(imagesDir);
  const fileUrls = fileNames
    .filter(file => !file.endsWith(".md"))
    .map(file => `/demo/images/${file}`);
  
  let page = 1;
  let onPage = 20;
  const totalImages = fileUrls.length;

  if (req.query.page && +req.query.page !== NaN) {
    page = +req.query.page;
  }
  if (req.query.onPage && +req.query.onPage !== NaN) {
    onPage = +req.query.onPage;
  }

  const offset = (page - 1) * onPage;
  if (totalImages <= offset) {
    return res.status(404).json(404 as any);
  }

  const imagesToReturn: IImage[] = fileUrls
  .slice(offset, offset + onPage)
  .map<IImage>(url => {
    return {
      url,
      title: url,
      thumbnail: `${url}/transform?w=256&q=0.7`,
      imageId: last(url.split('/')),
      resourceUrl: "https://check.server/src/images/readme.md?for=source-image-urls",
      directImageUrl: "https://check.server/src/images/readme.md?for=source-image-urls",
      authorName: "check server/src/images/readme.md for authors",
      metadata: {
        type: "image/jpeg",
        thumbnail: {
          type: "image/jpeg"
        }
      },
      tagStyleInformation: {
        author: [{
          text: "check server/src/images/readme.md for authors"
        }],
        general: [
          { text: "demo", descrition: "The image came from demo gallery" }
        ]
      }
    }
  });

  // Is there next page
  let nextPage = 
    (offset + onPage) < totalImages ?
    `/demo/query?page=${page + 1}&onPage=${onPage}`
    : undefined;
  
  res.json({
    totalImages,
    page,
    onPage,
    nextPage,
    images: imagesToReturn
  });

});
