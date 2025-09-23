use serde::{Deserialize, Serialize};

// api in  types
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrder {
    pub price: f64,
    pub quantity: f64,
    pub user_id: String,
    pub side: Side,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteOrder {
    pub order_id: String,
    pub user_id: String,
}

// outputs types
#[derive(Serialize, Deserialize)]
pub struct CreateOrderResponse {
    pub order_id: String,
    pub filled_quantity: f64,
    pub remianing_quantity: f64,
    pub average_price: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteOrderResponse {
    pub success: bool,
    pub remianing_quantity: f64,
    pub filled_quantity: f64,
    pub average_price: f64,
}
