use actix_web::{get, web, HttpResponse, Responder};

use crate::Executor;

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().content_type("text/plain").body("Ok")
}

#[get("/api/query/{query:.*}")]
pub async fn query(query: web::Path<String>, executor: web::Data<Executor>) -> impl Responder {
    let Ok(result) = executor.query_data(&query.to_string()).await else {
        println!("Failure");
        return HttpResponse::InternalServerError().body("failed");
    };

    HttpResponse::Ok().json(result)
}
