use actix_web::{get, post, web, App, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

mod postgres;

#[derive(Deserialize, Serialize)]
struct Item {
    id: i32,
    text: String,
    done: bool,
}

#[post("/item")]
async fn post_item(item: web::Json<Item>) -> Result<impl Responder> {
    Ok(web::Json(item))
}

#[get("/item/{id}")]
async fn get_item(id: web::Path<i32>) -> Result<impl Responder> {
    let item = Item {
        id: *id,
        text: "Finish this todo api".to_string(),
        done: false,
    };
    Ok(web::Json(item))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = postgres::get_pool();
    HttpServer::new(move || App::new().app_data(web::Data::new(pool.clone())).service(post_item).service(get_item))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, http, test, App};

    use super::*;

    #[actix_web::test]
    async fn test_post_item() {
        let item = Item {
            id: 1,
            text: "Finish this todo api".to_string(),
            done: false,
        };

        let app = test::init_service(App::new().service(post_item)).await;

        let req = test::TestRequest::post()
            .uri("/item")
            .set_json(&item)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body_bytes = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(
            body_bytes,
            actix_web::web::Bytes::from(serde_json::to_string(&item).unwrap())
        );
    }

    #[actix_web::test]
    async fn test_get_item() {
        let item = Item {
            id: 1,
            text: "Finish this todo api".to_string(),
            done: false,
        };

        let app = test::init_service(App::new().service(get_item)).await;

        let req = test::TestRequest::get().uri("/item/1").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body_bytes = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(
            body_bytes,
            actix_web::web::Bytes::from(serde_json::to_string(&item).unwrap())
        );
    }

    #[actix_web::test]
    async fn test_get_item_id_must_be_int() {
        let app = test::init_service(App::new().service(get_item)).await;

        let req = test::TestRequest::get().uri("/item/qwe").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);

        let body_bytes = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body_bytes, "can not parse \"qwe\" to a i32");
    }
}
