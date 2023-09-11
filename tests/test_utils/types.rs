use chrono::NaiveDateTime;
use serde::Deserialize;
use uuid::Uuid;
#[derive(Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub price: f64,
}
#[derive(Deserialize, Clone)]
pub struct Customer {
    pub name: String,
    pub emails: Vec<String>,
    pub plan: Plan,
    pub country_code: Option<u8>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum Plan {
    Premium,
    Family { shared_membership: u8 },
    Standard,
}
#[derive(Deserialize, Clone)]
pub struct Order {
    pub id: uuid::Uuid,
    pub customer_id: uuid::Uuid,
    pub item_id: uuid::Uuid,
    pub quantity: i64,
    pub purchased_at: NaiveDateTime,
}
