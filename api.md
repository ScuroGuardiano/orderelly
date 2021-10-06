# API V1 Definitions
Okey we must define some consistent and universal API rules.

## `GET /`
Returns list of available services.

* ### Response
  - `200 OK` `ServiceList` - list containing all supported services
* ### Example Response
  ```json
    [
      {
        "name": "Twitter",
        "path": "/twitter",
        "thumbnail": "/path/to/image.jpg"
      },
      {
        "name": "Instagram",
        "path": "/instagram"
      }
    ]
  ```

## `GET /{service}`
Returns information about service and how to query it.

* ### Exampe Request
  `GET /twitter`
* ### Response
  - `200 OK` `ServiceInfo` - contains information about service like name, url to it and information how to query it.
  - `403 Forbidden` `void` - returned when service is disabled
  - `404 Not Found` `void` - returned when service doesn't exist
* ### Example Response
  ```json
    {
      "name": "Twitter",
      "path": "/twitter",
      "url": "https://twitter.com",
      "description": "Twitter is an American microblogging and social networking service on which users post and interact with messages known as \"tweets\".",
      "thumbnail": "/twitter/static/thumbnail.jpg",
      "color": "#1DA1F2",
      "logo": "/twitter/static/logo-white-32x32.png",
      "query": {
        "params": [{        
          "param": "user",
          "paramName": "Username or URL",
          "required": true,
          "paramType": "string",
          "description": "Type username without @ or link to his profile. Examples: 'scuroguardiano', 'https://twitter.com/scuroguardiano', 'twitter.com/scuroguardiano'"
        }, {
          "param": "retweets",
          "paramName": "Include retweets",
          "required": false,
          "default": false,
          "paramType": "boolean",
          "description": "Should include retweets aswell?"
        }]
      }
    }
  ```
  
## `GET /{service}/query`
* ### Request
  - `param page: number` `optional` - index of results' page. Check out pagination section of this request for more information.
  - `param onPage: number` `optional` - how many results on page. `Default: 20`
  - Params that are accepted by service. Request `/{service}` to get param list.
* ### Example Request
  `/reddit/query?r=unixporn&page=3`
* ### Response
  - `200 OK` `PaginatedImageList` - Contains image list and information about pagination.
  - `400 Bad Request` `void` - returned when there are missing params or they have wrong format.
  - `403 Forbidden` `void` - returned when service is disabled
  - `404 Not Found` `void` - returned when service doesn't exist or foreing service returned 404
  - `502 Bad Gateway` `ForeignServiceError` - returned when service that is queried by backend of is unavailable or returned unexpected response. Eg. instagram went down or changed API so backend returns error. Notice that Bad Gateway can be also returned by NGINX, to determine whenever error came from NGINX or Orderelly the reponse body should be checked. Orderelly should return `ForeginServiceError`. Check [Types](#types) section and Example responses to get more information.
  - `504 Gateway Timeout` `ForeignServiceTimeoutError` - return when service that is queried by backend timeouted. Like in `502 Bad Gateway` error this could also be returned from NGINX. Now body should contain `ForeignServiceTimeoutError`. Check [Types](#types) section for more.
* ### Example responses
  - `200 OK`
    ```json
      {
        "totalImages": 2143,
        "page": 2,
        "onPage": 20,
        "nextPage": "/reddit/query?r=unixporn&page=4&onPage=20",
        "images": [
          {
            "title": "[Plasma] Just a simple panel-less Plasma...",
            "imageId": "f8lz84a3bcr71.png",
            "url": "/reddit/image/f8lz84a3bcr71.png",
            "thumbnail": "/reddit/image/f8lz84a3bcr71.png/transform?w=256&q=0.7",
            "resourceUrl": "https://www.reddit.com/r/unixporn/comments/q0vvf7/plasma_just_a_simple_panelless_plasma/",
            "directImageUrl": "https://i.redd.it/f8lz84a3bcr71.png",
            "authorName": "KeivnL",
            "authorUrl": "https://www.reddit.com/user/KeivnL/",
            "text": "Generally here should be text associated with image. In case of Reddit I guess it should be first OP comment. The problem comes with formatting, if it's needed it should be done on markdown without any images or HTML tags.",
            "metadata": {
              "type": "image/png",
              "width": 1920,
              "height": 1080,
              "size": 133742,
              "thumbnail": {
                "type": "image/jpeg",
                "width": 256,
                "height": 144,
                "size": 4242,
                "quality": 0.7
              }
            },
            "tagStyleInformation": {
              "author": [
                {
                  "url": "/reddit/query?r=unixporn&user=KeivnL",
                  "text": "KeivnL"
                }
              ],
              "general": [
                {
                  "url": "https://github.com/danbooru/danbooru",
                  "text": "Danbooru",
                  "description": "This tagging information idea I've seen on danbooru and I really like it. Easy way to provide more image information."
                }
                {
                  "url": "/reddit/query?r=unixporn&general=exampleTag",
                  "text": "exampleTag"
                }
              ]
            }
          }
        ]
      }
    ```
  - `502 Bad Gateway`
    ```json
      {
        "error": "ForeignServiceError",
        "foreignError": {
          "error": "Bad Request",
          "code": 400,
          "body": {}
        }
      }
    ```

* ### Pagination
  Results must be paginated. There should be given total items number in order to calculate total pages. Unfortunately in certain situations it can be not possible or too expensive to return number of total images. In that situation the backend should always know if there's nextPage and return nextPage url if so. **NEVER** include nextPage in the response if there's no more results, backend must check for example if there's at least one more item after page limit. Also **ALWAYS** include nextPage if there's more results.

## `GET /{service}/image/{imageId}`
Returns image XD That's it
* ### Request
  `pathparam imageId: string` - imageId. It must be imageId returned from `GET /{service}/query`.
* ### Response
  - `200 OK` response with image
  - all errors from `GET /{service}/query`

## `GET /[service]/image/{imageId}/transform`
Returns transformed image. Currently it's only width `w` and quality `q`. If `q` is set then image/jpeg is returned, otherwise original image type is returned - if it's possible for backend. If backend can't generate original image type, then image/jpeg is returned. Set `q` to 1 if you want just conversion to jpeg. In case when no witdh nor quality is set, original image will be returned.
* ### Request
  - `pathparam imageId: string` - imageId. It must be imageId returned from `GET /{service}/query`.
  - `param w: number` `optional` - desired width
  - `param q: number` `optional` - desired quality


# Types
```ts
  type u32 = number;

  interface ServiceListElement {
    name: string
    path: string
    thumbnail?: string
  }
  
  type ServiceList = ServiceListElement[]

  interface QueryParamInfo {
    param: string
    paramName: string
    required?: boolean // By default it shouldn't be required
    default?: QueryParamInfo.paramType // By default should be null
    paramType: string
    hidden?: boolean // By default false. It tells frontend to not show this field. Useful for tags for example.
    description: string
  }

  interface QueryInfo {
    params: QueryParamInfo[]
  }

  interface ServiceInfo {
    name: string
    path: string
    url: string // URL to original service, eg. https://instagram.com
    description: string
    thumbnail?: string
    color?: string
    logo?: string
    query?: QueryInfo // If empty then there's no query parameters and service should respond to request without params
  }

  interface SingleFileMetadata {
    type: string
    width?: u32
    height?: u32
    size?: u32
    quality?: u32
  }
  
  interface Metadata extends SingleFileMetadata {
    thumbnail: SingleFileMetadata
  }

  interface Tag {
    url?: string
    text: string
    descrition?: string
  }

  interface TagStyleInformation {
    [key: string]: Tag[]
  }

  interface Image {
    title?: string
    url: string
    imageId: string
    thumbnail: string
    resourceUrl: string
    directImageUrl: string
    authorName: string
    authorUrl?: string
    text?: string
    metadata: Metadata
    tagStyleInformation: TagStyleInformation
  }

  interface PaginatedImageList {
    totalImages?: u32 // Undefined or null if can't be determined or calculating it would be too expensive
    page: u32
    onPage: u32
    nextPage?: string // URL to the next page, undefined or null if that's end of results.
    images: Image[]
  }

  interface ForeignError {
    error: string
    code: u32
    body: any
  }

  interface ForeignServiceError {
    error: "ForeignServiceError",
    foreignError: ForeignError
  }

  interface ForeignServiceTimeoutError {
    error: "ForeignServiceTimeoutError"
  }
```
