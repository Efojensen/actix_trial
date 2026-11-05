use serde::{Serialize, Deserialize};

#[derive(Serialize, Clone)]
pub struct Car {
    pub reg_num: u32,
    pub brand: String,
    pub owner: String,
    pub is_luxury: bool
}

#[derive(Deserialize)]
pub struct BrandFilter {
    pub brand: Option<String>
}