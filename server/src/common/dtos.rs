use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use derive_builder::Builder;

#[derive(Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
pub struct ServiceListElement {
  pub name: String,
  pub path: String,
  #[builder(default)]
  pub thumbnail: Option<String>
}

#[allow(dead_code)]
pub type ServiceList = Vec<ServiceListElement>;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ParamType {
  String,
  Number,
  Boolean,
  Array
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum DefaultQueryParam {
  String(String),
  Number(f64),
  Boolean(bool),
  Null
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum ExtendedDefaultQueryParam {
  String(String),
  Number(f64),
  Boolean(bool),
  Array(Vec<DefaultQueryParam>)
}

#[derive(Serialize, Deserialize, Builder, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
pub struct QueryParamInfo {
  pub param: String,
  pub param_name: String,
  #[builder(default)]
  pub required: Option<bool>,
  #[builder(default)]
  pub default: Option<ExtendedDefaultQueryParam>,
  pub param_type: ParamType,
  #[builder(default)]
  pub hidden: Option<bool>,
  pub description: String
}

#[derive(Serialize, Deserialize, Builder, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryInfo {
  pub params: Vec<QueryParamInfo>
}

#[derive(Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
pub struct ServiceInfo {
  pub name: String,
  pub path: String,
  pub url: String,
  pub description: String,
  #[builder(default)]
  pub thumbnail: Option<String>,
  #[builder(default)]
  pub color: Option<String>,
  #[builder(default)]
  pub logo: Option<String>,
  #[builder(default)]
  pub query: Option<QueryInfo>
}

#[derive(Serialize, Deserialize, Builder, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
pub struct SingleFileMetadata {
  #[serde(rename = "type")]
  pub content_type: String,
  #[builder(default)]
  pub width: Option<u32>,
  #[builder(default)]
  pub height: Option<u32>,
  #[builder(default)]
  pub size: Option<u32>,
  #[builder(default)]
  pub quality: Option<f32>
}

#[derive(Serialize, Deserialize, Builder, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
  #[serde(flatten)]
  pub metadata: SingleFileMetadata,
  pub thumbnail: SingleFileMetadata
}

#[derive(Serialize, Deserialize, Builder, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
pub struct Tag {
  #[builder(default)]
  pub url: Option<String>,
  pub text: String,
  #[builder(default)]
  pub description: Option<String>
}

#[allow(dead_code)]
pub type TagStyleInformation = HashMap<String, Vec<Tag>>;

#[derive(Serialize, Deserialize, Builder, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
pub struct Image {
  pub title: String,
  pub url: String,
  pub image_id: String,
  pub thumbnail: String,
  pub direct_image_url: String,
  pub author_name: String,
  #[builder(default)]
  pub author_url: Option<String>,
  #[builder(default)]
  pub text: Option<String>,
  pub metadata: Metadata,
  pub tag_style_information: TagStyleInformation
}

#[derive(Serialize, Deserialize, Builder, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
pub struct PaginatedImageList {
  #[builder(default)]
  pub total_images: Option<u32>,
  pub page: u32,
  pub on_page: u32,
  #[builder(default)]
  pub next_page: Option<String>,
  pub images: Vec<Image>
}

/// No builder for ForeingError, use ::new like that:
/// ```
///   let error = ForeignError::new("Internal Error", 500, "{ \"xd\": \"Sheeeeeeeeesh\" }");
/// ```
/// 
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ForeignError {
  pub error: String,
  pub code: u32,
  /// The body in APIv3 is of any type, since any type isn't that easy in Rust.
  /// Just pass here serialized JSON. 
  pub body: String
}

#[allow(dead_code)]
impl ForeignError {
  pub fn new(error: &str, code: u32, body: &str) -> Self {
    return ForeignError {
      error: String::from(error),
      code,
      body: String::from(body)
    };
  }
}

/// No builder for ForeignServiceError, use ::new like that:
/// ```
///   let error = ForeignServiceError::new(
///     ForeignError::new("Internal Error", 500, "{ \"xd\": \"Sheeeeeeeeesh\" }")
///   );
/// ```
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ForeignServiceError {
  error: String,
  foreign_error: ForeignError
}

#[allow(dead_code)]
impl ForeignServiceError {
  pub fn new(foreign_error: ForeignError) -> Self {
    ForeignServiceError {
      error: String::from("ForeignServiceError"),
      foreign_error
    }
  }
}

/// No builder for ForeignServiceTimeoutError, use ::new like that:
/// ```
///   let error = ForeignServiceTimeoutError::new();
/// ```
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ForeignServiceTimeoutError {
  error: String
}

#[allow(dead_code)]
impl ForeignServiceTimeoutError {
  pub fn new() -> Self {
    ForeignServiceTimeoutError {
      error: String::from("ForeignServiceTimeoutError")
    }
  }
}
