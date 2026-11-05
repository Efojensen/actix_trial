mod handlers;
mod models;
mod state;

use handlers::{cars, general};

use actix_web::{App, HttpServer, web};

use crate::state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let state = web::Data::new(AppState::new());
        App::new()
            .app_data(state)
            .service(general::hello)
            .service(general::about)
            .service(cars::list_cars)
            .service(cars::get_reg_num)
            .route("/health", web::get().to(general::health))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
