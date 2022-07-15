pub mod sale;

pub mod buy;

pub struct Item {
    pub name: String,
    pub price: usize,
    production_cost: usize
}