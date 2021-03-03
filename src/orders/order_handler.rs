use crate::orders::{repository, OrderRequest, Order};
use crate::{db::DB, WebResult};
use warp::{http::StatusCode, reject, reply::json, Reply};
use serde_path_to_error;
use crate::error::Error;
use bytes::buf::{Buf, BufExt};


pub async fn create_handler_path(buf:impl Buf, db: DB) -> WebResult<impl Reply> {
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let body: OrderRequest = serde_path_to_error::deserialize(des)
        .map_err(|e| reject::custom(Error::JSONPathError(e.to_string())))?;
    
        Ok(format!("called with: {:?}", body))
}


pub async fn create_order_handler(body: OrderRequest, db: DB) -> WebResult<impl Reply> {
    let order = repository::create_order(&body, &db)
        .await
        .map_err(|e| reject::custom(e))?;
        Ok(json(&order))
}

pub async fn update_order_handler(id: String, body: Order, db: DB) -> WebResult<impl Reply> {
    let order = repository::update_order_with_id(&id, &body, &db)
        .await
        .map_err(|e| reject::custom(e))?;

    Ok(json(&order))
}

pub async fn update_handler_path(id: String, buf:impl Buf, db: DB) -> WebResult<impl Reply> {
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let body: Order = serde_path_to_error::deserialize(des)
        .map_err(|e| reject::custom(Error::JSONPathError(e.to_string())))?;
    
        Ok(format!("called with: {:?}", body))
}

pub async fn delete_order_handler(id: String, db: DB) -> WebResult<impl Reply> {
    repository::delete_item_with_id(&id, &db)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}
