use mongodb::bson;
use serde::{Deserialize, Serialize};

pub mod repository;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OrderItemRequest {
    pub item_id: String,
    pub quantity: i32,
    pub price: i32,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderItem {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: Option<bson::oid::ObjectId>,
    pub order_id: String,
    pub item_id: String,
    pub quantity: i32,
    pub price: i32,
}

// impl InsertableOrderItem {
//     fn from_order_item(item: OrderItem) -> InsertableOrderItem {
//         InsertableOrderItem {
//             item_id: item.item_id,
//             quantity: item.quantity,
//             price: item.price,
//         }
//     }
// }

