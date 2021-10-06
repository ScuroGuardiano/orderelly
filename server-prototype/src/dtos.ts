export type u32 = number;

export interface ServiceListElement {
  name: string
  path: string
  thumbnail?: string
}

export type ServiceList = ServiceListElement[]

export interface QueryParamInfo {
  param: string
  paramName: string
  required?: boolean // By default it shouldn't be required
  default?: any // By default should be null
  paramType: string
  hidden?: boolean // By default false. It tells frontend to not show this field. Useful for tags for example.
  description: string
}

export interface QueryInfo {
  params: QueryParamInfo[]
}

export interface ServiceInfo {
  name: string
  path: string
  url: string // URL to original service, eg. https://instagram.com
  description: string
  thumbnail?: string
  color?: string
  logo?: string
  query?: QueryInfo // If empty then there's no query parameters and service should respond to request without params
}

export interface SingleFileMetadata {
  type: string
  width?: u32
  height?: u32
  size?: u32
  quality?: u32
}

export interface Metadata extends SingleFileMetadata {
  thumbnail: SingleFileMetadata
}

export interface Tag {
  url?: string
  text: string
  descrition?: string
}

export interface TagStyleInformation {
  [key: string]: Tag[]
}

export interface IImage {
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

export interface PaginatedImageList {
  totalImages?: u32 // Undefined or null if can't be determined or calculating it would be too expensive
  page: u32
  onPage: u32
  nextPage?: string // URL to the next page, undefined or null if that's end of results.
  images: IImage[]
}

export interface ForeignError {
  error: string
  code: u32
  body: any
}

export interface ForeignServiceError {
  error: "ForeignServiceError",
  foreignError: ForeignError
}

export interface ForeignServiceTimeoutError {
  error: "ForeignServiceTimeoutError"
}
