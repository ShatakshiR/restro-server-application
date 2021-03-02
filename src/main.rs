use db::DB;
extern crate mongodb;
extern crate serde_derive;
extern crate serde_json;
use std::convert::Infallible;
use warp::{Filter, Rejection};

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

mod commons;
mod db;
mod error;
mod items;

use items::handler;

#[tokio::main]
async fn main() -> Result<()> {
    let db = DB::init().await?;

    let item = warp::path("menu");

    let item_routes = item
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::create_item_handler)
        .or(item
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handler::edit_item_handler))
        .or(item
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handler::delete_item_handler))
        .or(item
            .and(warp::get())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handler::item_with_id_handler))
        .or(item
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handler::items_list_handler));

    let routes = item_routes.recover(error::handle_rejection);

    println!("Started on port 8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
