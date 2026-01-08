use actix_web::web;

use crate::handlers::user_handler::{add_user, create_user, get_user, health_check};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/create", web::post().to(create_user))
            .route("/db/add", web::post().to(add_user))
            .route("/{id}", web::get().to(get_user)),
    );
}
