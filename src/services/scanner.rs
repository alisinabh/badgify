use actix_web::{get, http::header::LOCATION, web, HttpResponse, Responder};

use crate::Executor;

#[get("/scanner/{query:.*}")]
pub async fn scanner(query: web::Path<String>, executor: web::Data<Executor>) -> impl Responder {
    let Ok(scanner_link) = executor.get_scanner_link(&query.to_string()).await else {
        return HttpResponse::InternalServerError()
            .content_type("text/plain")
            .body("Redirect Failure");
    };

    HttpResponse::TemporaryRedirect()
        .insert_header((LOCATION, scanner_link))
        .finish()
}
