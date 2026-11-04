use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Car {
    pub reg_num: u32,
    pub brand: String,
    pub owner: String,
    pub is_luxury: bool
}

