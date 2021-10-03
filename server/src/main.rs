use actix_web::{get, web, App, HttpServer, HttpResponse};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

struct CounterState {
  counter: Mutex<u32>
}

#[derive(Serialize, Deserialize)]
struct CounterDTO {
  counter: u32
}

#[get("/counter")]
async fn get_counter(data: web::Data<CounterState>) -> HttpResponse {
  let counter = data.counter.lock().unwrap();
  return HttpResponse::Ok().json(CounterDTO {
    counter: *counter
  });
}

#[get("/counter/increment")]
async fn increment_counter(data: web::Data<CounterState>) -> HttpResponse {
  let mut counter = data.counter.lock().unwrap();
  *counter += 1;

  return HttpResponse::Ok().json(CounterDTO {
    counter: *counter
  });
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let counter = web::Data::new(CounterState {
    counter: Mutex::new(0)
  });

  HttpServer::new(move || {
    App::new()
      .app_data(counter.clone())
      .service(get_counter)
      .service(increment_counter)
  })
  .bind("127.0.0.1:3002")?
  .run()
  .await
}
