# API Definitions
Okey we must define some consistent and universal API rules.

## `GET /`
Returns list of available services.

* ### Response
  `ServiceList` - list containing all supported services

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
  `ServiceInfo` - contains information about service like name, url to it and information how to query it.
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
  
# Types
```ts
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
```
