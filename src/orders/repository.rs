#![allow(proc_macro_derive_resolution_fallback)]
use crate::commons;
use crate::commons::Counters;
use crate::items::Item;
use crate::db::DB;
use crate::orders::{OrderRequest, Order};
use crate::{error::Error::*, Result};
use futures::StreamExt;
use mongodb::bson;
use mongodb::bson::{doc, oid::ObjectId};
use std::vec::Vec;
use crate::order_items::{repository, OrderItem, OrderItemRequest};
use crate::orders::OrderStatus;

const COLLECTION: &str = "Orders";

// pub async fn get_all_items(connection: &DB) -> Result<Vec<Orders>> {
//     let mut cursor = connection
//         .get_collection(COLLECTION)
//         .find(None, None)
//         .await
//         .map_err(MongoQueryError)?;

//     let mut results: Vec<Item> = Vec::new();
//     while let Some(result) = cursor.next().await {
//         match result {
//             Ok(result) => {
//                 let item: Item = bson::from_document(result).unwrap();
//                 results.push(item);
//             }
//             Err(_) => {}
//         }
//     }

//     Ok(results)
// }

// pub async fn get_item_with_id(id: &String, connection: &DB) -> Result<Option<Item>> {
//     let loaded_item = connection
//         .get_collection(COLLECTION)
//         .find_one(
//             Some(doc! { "_id":  ObjectId::with_string(&id).unwrap()}),
//             None,
//         )
//         .await
//         .map_err(MongoQueryError)?;

//     let item: Option<Item> = bson::from_document(loaded_item.unwrap()).unwrap();

//     Ok(item)
// }

pub async fn create_order(order: &OrderRequest, connection: &DB) -> Result<Order> {
    let mut insertable:OrderRequest = order.clone();
    insertable.order_status = "Order Created".to_string();
    let serialized_item = bson::to_bson(&insertable).unwrap();
    let cursor = connection
        .get_collection(COLLECTION)
        .insert_one(bson::from_bson(serialized_item).unwrap(), None)
        .await
        .map_err(MongoQueryError)?;
    let res = cursor.inserted_id;
    let inserted_id = res.as_object_id().expect("Retrieved _id should have been of type ObjectId");
    print!("inserted_id: {}", &inserted_id);
    // let inserted_id:ObjectId = bson::from_bson(cursor.inserted_id).unwrap();
    
    let items = repository::create_order_items(&order.ordered_items, connection, &inserted_id).await?;    
    let ordered_items: Vec<OrderItem> = items.into_iter().collect();
    let mut items_ids: Vec<ObjectId> = Vec::new();
    for item in ordered_items.clone(){
      items_ids.push(ObjectId::with_string(&item.item_id).unwrap());
    } 
    
    let result = calculate_waiting_time_and_price(connection, &items_ids).await?;
    let order_id_generated =  calculate_the_display_order_id(connection).await?;

    let created_order = Order{
    id: Some(inserted_id.clone()),  
    ordered_items:ordered_items,
    total_amount: result.1,
    waiting_time: result.0,
    order_status: "Order Placed".to_string(),
    order_id: order_id_generated,
    table_no:order.table_no,
   };
    update_order_with_objectid(&inserted_id, &created_order, connection).await?;
    
    Ok(created_order.clone())   
}

pub async fn update_order_with_objectid(id: &ObjectId, order: &Order, connection: &DB) ->Result<String> {
    let new_order = order.clone();
    let filter = doc! {"_id" : id};
    let update = doc! {"$set": commons::struct_to_document(&new_order).unwrap()};

    let cursor = connection
        .get_collection(COLLECTION)
        .update_one(filter, update, None)
        .await
        .map_err(MongoQueryError)?;

    let updated_id = cursor.upserted_id;
    let response = match updated_id{
        Some(updated_id)=> bson::from_bson(updated_id).unwrap(),
        None => "Sorry, unable to update".to_string()
    };
  
    Ok(response)
}

pub async fn update_order_with_id(id: &String, order: &Order, connection: &DB) -> Result<String> {
    let oid = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
    let cursor = update_order_with_objectid(&oid, order, connection).await?;
    Ok(cursor)
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

pub async fn calculate_the_display_order_id(connection: &DB) -> Result<isize>{
    let filter = doc!{"_id" : "orderId"};
    let update = doc!{"$inc": {"sequence_value":1}};
    let cursor = connection
    .get_collection(COLLECTION)
    .find_one(filter.clone(), None)
    .await
    .map_err(MongoQueryError)?;
    
    let mut new_sequence:isize = 0;  
    
    for obj in cursor{
        let  counter: Counters = bson::from_document(obj).unwrap();
        new_sequence = counter.sequence_value+1;
            print!("orderId: {}", counter.sequence_value);
    }

    connection
    .get_collection(COLLECTION)
    .update_one(filter, update, None)
    .await
    .map_err(MongoQueryError)?;

    Ok(new_sequence)
}


//Calculating the average waiting time for each order based on the time to prepare each item in the order
//Also calculating the total amount to be paid against each order, including the 5% taxes. 
async fn calculate_waiting_time_and_price(connection: &DB, items_ids: &Vec<ObjectId>) -> Result<(i32, f64)> {
        
        let mut item_cursor = connection
                                .get_collection("Item")
                                .find(Some(doc! {"_id": items_ids}), None)
                                .await
                                .map_err(MongoQueryError)?;

        let mut items: Vec<Item> = Vec::new();
        let mut total_amount: f64 = 0.0;
        let mut waiting_time: i32 = 0;

        while let Some(result) = item_cursor.next().await {
            match result {
                Ok(result) => {
                    let item: Item = bson::from_document(result).unwrap();
                    total_amount = total_amount + item.price as f64;
                    waiting_time = waiting_time + item.timeToPrepare.unwrap();
                    items.push(item);
                }
                Err(_) => {}
            }
        }
       
        //Calculating total_amount by adding 5% taxes.
        total_amount = total_amount + (total_amount*0.05);

        //Average waiting time for the order
        if items.len()!=0 {
            waiting_time = waiting_time as i32/items.len() as i32;
        }

        Ok((waiting_time, total_amount))
}