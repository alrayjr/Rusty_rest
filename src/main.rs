use actix_web::{ web, App, HttpServer};
use std::sync::Mutex;

mod utils;
mod routes;

// This struct represents state
struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    unsafe{ //logs incoming request info
        if std::env::var_os("RUST_LOG").is_none() {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }

    dotenv::dotenv().ok();  //dot env init
    env_logger::init();     //env var logger init

    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    let port = (*utils::constants::PORT).clone();
    let address = (*utils::constants::ADDRESS).clone();

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .configure(routes::home_routes::config)

            .app_data(counter.clone()) // <- register the created data
            .route("/", web::get().to(index))
            
    })
    .bind((address, port))?
    .run()
    .await
}