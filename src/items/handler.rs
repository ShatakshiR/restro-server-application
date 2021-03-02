use crate::items::{repository, Item};
use crate::{db::DB, WebResult};
use warp::{http::StatusCode, reject, reply::json, Reply};

pub async fn items_list_handler(db: DB) -> WebResult<impl Reply> {
    let items = repository::get_all_items(&db)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json(&items))
}

pub async fn item_with_id_handler(id: String, db: DB) -> WebResult<impl Reply> {
    let item = repository::get_item_with_id(&id, &db)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json(&item))
}

pub async fn create_item_handler(body: Item, db: DB) -> WebResult<impl Reply> {
    repository::insert_item_with_id(&body, &db)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::CREATED)
}

pub async fn edit_item_handler(id: String, body: Item, db: DB) -> WebResult<impl Reply> {
    repository::update_item_with_id(&id, &body, &db)
        .await
        .map_err(|e| reject::custom(e))?;

    Ok(StatusCode::OK)
}

pub async fn delete_item_handler(id: String, db: DB) -> WebResult<impl Reply> {
    repository::delete_item_with_id(&id, &db)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}
