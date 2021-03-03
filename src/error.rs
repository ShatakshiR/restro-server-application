use mongodb::bson;
use serde::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use warp::{http::StatusCode, reply, Rejection, Reply};
use std::error::Error as StdError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("mongodb error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    #[error("error during mongodb query: {0}")]
    MongoQueryError(mongodb::error::Error),
    #[error("could not access field in document: {0}")]
    MongoDataError(#[from] bson::document::ValueAccessError),
    #[error("invalid id used: {0}")]
    InvalidIDError(String),
    #[error("JSON path error: {0}")]
    JSONPathError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
    errors: Option<Vec<FieldError>>,
}

#[derive(Serialize)]
struct FieldError {
    field: String,
    field_errors: Vec<String>,
}


impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<Box<dyn Reply>, Infallible> {
    let (code, message, errors) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string(), None)
    } else if let Some(e) = err.find::<Error>() {
        match e{
            Error::JSONPathError(_) => (StatusCode::BAD_REQUEST, e.to_string(), None),
            Error::MongoDataError(_) => (StatusCode::METHOD_NOT_ALLOWED, e.to_string(), None),
            Error::MongoQueryError(_) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string(), None),
            Error::MongoError(_) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string(), None),
            Error::InvalidIDError(_) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string(), None),
        }
    }else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        (
            StatusCode::BAD_REQUEST,
            e.source()
                .map(|cause| cause.to_string())
                .unwrap_or_else(|| "BAD_REQUEST".to_string()),
            None,
        )
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
                (StatusCode::METHOD_NOT_ALLOWED,"Method Not Allowed".to_string(), None)
            
    } else {               
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string(), None)           
    };

    let json = warp::reply::json(&ErrorResponse {
        message: message.into(),
        errors,
    });

    Ok(Box::new(reply::with_status(json, code)))
}


// pub async fn handle_rejection(err: Rejection) -> std::result::Result<Box<dyn Reply>, Infallible> {
//     let code;
//     let message;

//     if err.is_not_found() {
//         code = StatusCode::NOT_FOUND;
//         message = "Not Found";
//     } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
//         (
//             StatusCode::BAD_REQUEST,
//             e.source()
//                 .map(|cause| cause.to_string())
//                 .unwrap_or_else(|| "BAD_REQUEST".to_string()),
//             None,
//         )
//     } else if let Some(e) = err.find::<Error>() {
//         match e {
//             Error::JSONPathError(_) => (StatusCode::BAD_REQUEST, e.to_string(), None),
//         }
//     } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
//         code = StatusCode::METHOD_NOT_ALLOWED;
//         message = "Method Not Allowed";
//     } else {
//         eprintln!("unhandled error: {:?}", err);
//         code = StatusCode::INTERNAL_SERVER_ERROR;
//         message = "Internal Server Error";
//     }

//     let json = warp::reply::json(&ErrorResponse {
//         message: message.into(),
//         errors,
//     });


//     Ok(Box::new(reply::with_status(json, code)))
// }

