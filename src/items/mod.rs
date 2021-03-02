pub mod handler;
pub mod repository;

use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Item {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: Option<bson::oid::ObjectId>,
    pub title: String,
    pub itemType: Option<String>,
    pub timeToPrepare: Option<i32>,
    pub price: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableItem {
    pub title: String,
    pub itemType: Option<String>,
    pub timeToPrepare: Option<i32>,
    pub price: i32,
    pub name: String,
    pub description: Option<String>,
}

impl InsertableItem {
    fn from_item(item: Item) -> InsertableItem {
        InsertableItem {
            title: item.title,
            itemType: item.itemType,
            timeToPrepare: item.timeToPrepare,
            price: item.price,
            name: item.name,
            description: item.description,
        }
    }
}
