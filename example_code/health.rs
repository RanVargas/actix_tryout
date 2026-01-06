use actix_web::{HttpResponse, web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(health_check)); // Maps to /api/health
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}
