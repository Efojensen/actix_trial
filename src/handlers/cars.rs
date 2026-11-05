use actix_web::{get, web, Responder, HttpResponse};

use crate::{models::{BrandFilter, Car}, state::AppState};

#[get("/cars")]
pub async fn list_cars(state: web::Data<AppState>, brand: web::Query<BrandFilter>) -> impl Responder {
    let the_brand = brand.into_inner();
    let cars = state.cars.lock().unwrap();

    let res: Vec<Car> = cars.iter().filter(|c| match &the_brand.brand {
        Some(brand) => &c.brand == brand,
        None => true
    }).cloned().collect();

    web::Json(res)
}

#[get("/cars/{id}")]
pub async fn get_reg_num(state: web::Data<AppState>, path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    let cars = state.cars.lock().unwrap();

    match cars.iter().find(|car| car.reg_num == id ) {
        Some(car) => HttpResponse::Ok().json(car),
        None => HttpResponse::NotFound().body("car not found")
    }
}