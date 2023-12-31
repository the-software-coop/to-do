use actix_web::{post, web, App, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Item {
    text: String,
    // done: bool,
}

#[post("/item")]
async fn item(item: web::Json<Item>) -> Result<String> {
    Ok(format!("New item: {}!", item.text))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(item))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    use super::item;

    #[actix_web::test]
    async fn test_index_post() {
        let app = test::init_service(App::new().service(item)).await;
        let req = test::TestRequest::post().uri("/item").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }
}
