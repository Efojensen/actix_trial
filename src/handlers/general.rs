use actix_web::{Responder, get};

#[get("/")]
pub async fn hello() -> impl Responder {
    "Hello world"
}

#[get("/about")]
pub async fn about() -> impl Responder {
    "Actix trial is my trial project to learning actix_web"
}

pub async fn health() -> impl Responder {
    "The api is running"
}