use actix_web::web::Json;
use actix_web::{get, web, App, HttpRequest, HttpServer};
use actix_web_lab::web::spa;
use serde::Serialize;
use tokio;

#[derive(Serialize)]
struct Response {
    value: String,
}

#[get("/basic")]
async fn basic_get(req: HttpRequest) -> actix_web::Result<Json<Response>> {
    println!("REQ: {:?}", req);
    Ok(Json(Response {
        value: String::from("a basic test"),
    }))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                spa()
                    .index_file("../frontend/dist/index.html")
                    .static_resources_mount("/")
                    .static_resources_location("./dist")
                    .finish(),
            )
            .service(web::scope("/api").service(basic_get))
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
