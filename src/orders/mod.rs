use crate::order_items::{OrderItemRequest, OrderItem};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub mod order_handler;
pub mod repository;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OrderRequest {
    pub ordered_items: Vec<OrderItemRequest>,
    pub table_no: i32,
    pub order_status: String
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: Option<ObjectId>,
    pub order_id: isize,
    pub ordered_items: Vec<OrderItem>,
    pub table_no: i32,
    pub order_status: String,
    pub total_amount: f64,
    pub waiting_time: i32,
}



#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum OrderStatus{
   OrderPlaced,
   OrderUpdated,
   OrderReceived,
   OrderPending,
   OrderDelivered,
}

impl OrderStatus{
    fn get_name(&self)-> &str{
        match self {
            Self::OrderPlaced => "Order Placed",
            Self::OrderUpdated => "Order Updated",
            Self::OrderReceived => "Order Received",
            Self::OrderPending => "Order Pending",
            Self::OrderDelivered => "Order Delivered",
        }
    }
}

impl Display for OrderStatus {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.get_name())
    }
}

impl Debug for OrderStatus {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.get_name())
    }
}