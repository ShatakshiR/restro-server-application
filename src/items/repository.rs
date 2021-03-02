#![allow(proc_macro_derive_resolution_fallback)]
use crate::commons;
use crate::db::DB;
use crate::items::{InsertableItem, Item};
use crate::{error::Error::*, Result};
use futures::StreamExt;
use mongodb::bson;
use mongodb::bson::{doc, oid::ObjectId};
use std::vec::Vec;

const COLLECTION: &str = "Item";

pub async fn get_all_items(connection: &DB) -> Result<Vec<Item>> {
    let mut cursor = connection
        .get_collection(COLLECTION)
        .find(None, None)
        .await
        .map_err(MongoQueryError)?;

    let mut results: Vec<Item> = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(result) => {
                let item: Item = bson::from_document(result).unwrap();
                results.push(item);
            }
            Err(_) => {}
        }
    }

    Ok(results)
}

pub async fn get_item_with_id(id: &String, connection: &DB) -> Result<Option<Item>> {
    let loaded_item = connection
        .get_collection(COLLECTION)
        .find_one(
            Some(doc! { "_id":  ObjectId::with_string(&id).unwrap()}),
            None,
        )
        .await
        .map_err(MongoQueryError)?;

    let item: Option<Item> = bson::from_document(loaded_item.unwrap()).unwrap();

    Ok(item)
}

pub async fn insert_item_with_id(item: &Item, connection: &DB) -> Result<()> {
    let insertable = InsertableItem::from_item(item.clone());
    let serialized_item = bson::to_bson(&insertable).unwrap();
    connection
        .get_collection(COLLECTION)
        .insert_one(bson::from_bson(serialized_item).unwrap(), None)
        .await
        .map_err(MongoQueryError)?;

    Ok(())
}

pub async fn update_item_with_id(id: &String, item: &Item, connection: &DB) -> Result<String> {
    println!("repository: update_item_with_id: begin");

    let oid = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
    let new_item = item.clone();
    let filter = doc! {"_id" : oid};
    let update = doc! {"$set": commons::struct_to_document(&new_item).unwrap()};

    connection
        .get_collection(COLLECTION)
        .update_one(filter, update, None)
        .await
        .map_err(MongoQueryError)?;
    println!("repository: update_item_with_id: end");
    let response = String::from("The record updated successfully.");
    Ok(response)
}

pub async fn delete_item_with_id(id: &String, connection: &DB) -> Result<String> {
    let oid = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
    let filter = doc! {
        "_id": oid,
    };
    connection
        .get_collection(COLLECTION)
        .delete_one(filter, None)
        .await
        .map_err(MongoQueryError)?;
    let response = String::from("The item deleted successfully.");
    Ok(response)
}

// pub async fn delete_all(connection: &DB) -> Result<()> {
//     connection.get_collection(COLLECTION).drop()
//     Ok(())
// }
