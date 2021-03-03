#[cfg(test)]
mod test {

    use warp::test::request;
    use warp::{http::StatusCode, reply, Rejection, Reply};
    // use rustlang_rocket_mongodb::rocket;
    fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
        warp::any().map(move || db.clone())
    }

    #[test]
    fn get_items() {
        let res = warp::test::request().path("/menu").method("GET").reply();
        assert_eq!(res.status(), Status::Ok);
    }

    #[test]
    fn get_item() {
        // Well get and post tests are identical ...
        let res = warp::test::request()
            .path("/menu/603d2fb200cae7af00d0fb90")
            .method("GET")
            .header(ContentType, JSON)
            .reply();
        assert_eq!(res.status(), Status::Ok);

        let id = res.body_string().unwrap();
        assert!(res.body().is_some());
        assert!(res.body_string().unwrap().contains(&id[603d2fb200cae7af00d0fb90]));
    }
}

//     #[test]
//     fn post_cat() {
//         let client = Client::new(rocket()).expect("valid rocket instance");
//         let mut response = client
//             .post("/cats")
//             .header(ContentType::JSON)
//             .body(r#"{ "name": "chacha" }"#)
//             .dispatch();
//         assert_eq!(response.status(), Status::Ok);

//         let id = response.body_string().unwrap();
//         let id: Vec<&str> = id.split("\"").collect();
//         let mut response = client.get(format!("/cats/{}", id[3])).dispatch();
//         assert!(response.body().is_some());
//         assert!(response.body_string().unwrap().contains(&id[3]));
//         client.delete("/cats").dispatch();
//     }

//     #[test]
//     fn update_cat() {
//         let client = Client::new(rocket()).expect("valid rocket instance");
//         let mut response = client
//             .post("/cats")
//             .header(ContentType::JSON)
//             .body(r#"{ "name": "chacha" }"#)
//             .dispatch();

//         assert_eq!(response.status(), Status::Ok);
//         assert!(response.body().is_some());
//         let id = response.body_string().unwrap();
//         let id: Vec<&str> = id.split("\"").collect();
//         let response = client
//             .put(format!("/cats/{}", id[3]))
//             .header(ContentType::JSON)
//             .body(r#"{ "name": "chichi" }"#)
//             .dispatch();
//         assert_eq!(response.status(), Status::Ok);
//         let mut response = client.get(format!("/cats/{}", id[3])).dispatch();
//         assert_eq!(response.status(), Status::Ok);
//         assert!(response.body().is_some());
//         assert!(response.body_string().unwrap().contains("chichi"));
//         client.delete("/cats").dispatch();
//     }

//     #[test]
//     fn delete_cat() {
//         let client = Client::new(rocket()).expect("valid rocket instance");
//         let mut response = client
//             .post("/cats")
//             .header(ContentType::JSON)
//             .body(r#"{ "name": "chacha" }"#)
//             .dispatch();
//         assert_eq!(response.status(), Status::Ok);

//         let id = response.body_string().unwrap();
//         let id: Vec<&str> = id.split("\"").collect();
//         let mut response = client.delete(format!("/cats/{}", id[3])).dispatch();
//         assert!(response.body().is_some());
//         assert!(response.body_string().unwrap().contains(&id[3]));
//         client.delete("/cats").dispatch();
//     }

//     #[test]
//     fn delete_all() {
//         let client = Client::new(rocket()).expect("valid rocket instance");
//         client.delete("/cats").dispatch();
//         let response = client
//             .post("/cats")
//             .header(ContentType::JSON)
//             .body(r#"{ "name": "chacha" }"#)
//             .dispatch();
//         assert_eq!(response.status(), Status::Ok);
//         let response = client.delete("/cats").dispatch();
//         assert_eq!(response.status(), Status::Ok);
//     }
// }
