#![allow(proc_macro_derive_resolution_fallback)]
use crate::db::DB;
use crate::order_items::{OrderItemRequest, OrderItem};
use crate::{error::Error::*, Result};
use futures::StreamExt;
use mongodb::bson;
use mongodb::bson::{doc, oid::ObjectId, Document};
use std::vec::Vec;

const COLLECTION: &str = "OrderItem";

pub async fn create_order_items(ordered_items: &Vec<OrderItemRequest>,connection: &DB, order_id: &ObjectId) 
-> Result<Vec<OrderItem>> 
    {
    let mut order_items: Vec<Document> = Vec::new();
    for order_item in ordered_items.clone() {
        // let order_item_request:OrderItemRequest = order_item.clone();
        let insertable_order_item = doc!{
            "order_id": ObjectId:: to_hex(order_id),
            "item_id": order_item.item_id,
            "quantity": order_item.quantity,
            "price": order_item.price
        };
        // let insert = commons::struct_to_document(&insertable_order_item).unwrap();
        order_items.push(insertable_order_item);
    }

    connection
        .get_collection(COLLECTION)
        .insert_many(order_items, None)
        .await
        .map_err(MongoQueryError)?;

    let mut cursor = connection
        .get_collection(COLLECTION)
        .find(Some(doc!{ "order_id": ObjectId:: to_hex(order_id)}), None)
        .await
        .map_err(MongoQueryError)?;

    let mut results: Vec<OrderItem> = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(result) => {
                let item: OrderItem = bson::from_document(result).unwrap();
                results.push(item);
            }
            Err(_) => { print!("Create Order Items: empty find order  response")}
        }
    }
   
    Ok(results)
}

