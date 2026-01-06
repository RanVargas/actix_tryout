use actix_web::{HttpResponse, web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::post().to(create_user)) // Maps to /api/users
        .route("/{id}", web::get().to(get_user));
}

async fn create_user() -> HttpResponse { /* ... */
}
async fn get_user() -> HttpResponse { /* ... */
}
