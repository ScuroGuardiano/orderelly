#[allow(unused_imports)]
use actix_web::{get, web, App, HttpServer, HttpResponse};

mod common;
mod demo;
mod services;

use services::get_services;
use common::dtos;

#[get("/")]
async fn get_index() -> HttpResponse {
  let services: dtos::ServiceList = get_services()
  .iter()
  .map(move |service| {
    dtos::ServiceListElement {
      name: service.name.clone(),
      path: service.path.clone(),
      thumbnail: service.thumbnail_url.clone()
    }
  }).collect();

  return HttpResponse::Ok().json(services);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(move || {
    App::new()
      .service(get_index)
  })
  .bind("127.0.0.1:3002")?
  .run()
  .await
}
