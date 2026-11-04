use std::sync::Mutex;

use crate::models::Car;

pub struct AppState {
    pub cars: Mutex<[Car; 2]>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            cars: Mutex::new([
                Car {
                    reg_num: 34323,
                    brand: String::from("Toyota"),
                    owner: String::from("Jimmy"),
                    is_luxury: true,
                },
                Car {
                    reg_num: 2342,
                    brand: String::from("Porsche"),
                    owner: String::from("Saber"),
                    is_luxury: true,
                },
            ]),
        }
    }
}
