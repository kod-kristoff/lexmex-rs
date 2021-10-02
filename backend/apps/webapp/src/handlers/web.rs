//! Web handlers module

// use crate::{middlewares::request_id::RequestId};
// use crate::errors::ApiError;
use actix_web::{HttpResponse, Responder, web};

// Route: GET "/health-check"
async fn health_check() -> impl Responder {
    // debug!("Request ID: {}", request_id.get());
    HttpResponse::Ok().finish()
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/healthz", web::get().to(health_check));
}
