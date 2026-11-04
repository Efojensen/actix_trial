use serde::Serialize;

#[derive(Serialize)]
pub struct Car {
    pub reg_num: u32,
    pub brand: String,
    pub owner: String,
    pub is_luxury: bool
}

