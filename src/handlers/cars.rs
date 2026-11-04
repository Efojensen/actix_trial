use actix_web::{get, web, Responder};

use crate::models::Car;

#[get("/cars")]
pub async fn list_cars() -> impl Responder {
    let cars: [Car; 2] = [
        Car{
            reg_num: 34323,
            brand: String::from("Toyota"),
            owner: String::from("Jimmy"),
            is_luxury: true
        },
        Car{
            reg_num: 2342,
            brand: String::from("Porsche"),
            owner: String::from("Saber"),
            is_luxury: true
        },
    ];

    web::Json(cars)
}