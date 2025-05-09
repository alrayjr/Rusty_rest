use actix_web::{ get, web, HttpResponse, Responder};

use crate::utils::api_response;


#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    api_response::ApiResponse::new(200, format!("Hello {name}!"))
}

#[get("/test")]
pub async fn test() -> impl Responder {
    api_response::ApiResponse::new(200, "test".to_string())
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there cool guy")
}


