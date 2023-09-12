use actix_web::{get, HttpResponse};


#[get("/api")]
pub async fn api() -> HttpResponse {
    HttpResponse::Accepted().body("api")
}
