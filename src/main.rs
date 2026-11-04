mod handlers;

use handlers::general;

use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(general::hello)
        .service(general::about)
        .route("/health", web::get().to(general::health))
    )
    .bind(("localhost", 8080))?
    .run()
    .await
}
