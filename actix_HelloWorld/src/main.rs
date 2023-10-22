/*use actix_web::{web, App, HttpRequest, HttpServer, Responder};
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}*/
#[tokio::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new().route("/{name}",web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
/*//reutnr actix_Web::Result type
use actix_web::{web, App, HttpRequest, HttpServer, Responder, Result, HttpResponse};
async fn greet(req: HttpRequest) -> Result<impl Responder> {
    let name = req.match_info().get("name").unwrap_or("World");
    if name == "Error" {
        Err(actix_web::error::ErrorInternalServerError("an error"))
    } else {
        Ok(HttpResponse::Ok())
    }
}*/
//reutnr Json type
use actix_web::{web, App, HttpRequest, HttpServer, Responder, Result};
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: i32
}
async fn greet(req: HttpRequest) -> Result<impl Responder> {
    let name = req.match_info().get("name").unwrap_or("World");
    if name == "Error" {
        Err(actix_web::error::ErrorInternalServerError("an error"))
    } else {
        Ok(web::Json(Person {name: "Peter Parker".to_string(), age: 32}))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use actix_web::web::Bytes;
    
    #[actix_rt::test]
    async fn test_index_get(){
        let mut app = test::init_service(
            App::new()
            .route("/{name}", web::get().to(greet)),
        )
        .await;
        let req = test::TestRequest::get().uri("/mundo").to_request();
        let resp = test::read_response(&mut app, req).await;
        assert_eq!(resp, Bytes::from_static(b"welcom"));
    }
}