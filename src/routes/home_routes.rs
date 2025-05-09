use actix_web::web;

use super::handlers;


pub fn config(config: &mut web::ServiceConfig){
    config
    .service(
        web::scope("/home") //adds /home prefix to all routes
        .service(handlers::home_handlers::greet)
        .service(handlers::home_handlers::test)
        .route("/hi",web::get().to(handlers::home_handlers::manual_hello)) //mine


    );
    
}