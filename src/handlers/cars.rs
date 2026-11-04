use actix_web::{get, web, Responder};

use crate::state::AppState;

#[get("/cars")]
pub async fn list_cars(state: web::Data<AppState>) -> impl Responder {
    let cars = state.cars.lock().unwrap();

    web::Json(cars.clone())
}