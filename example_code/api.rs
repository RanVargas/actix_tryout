use actix_web::web;

mod health;
mod users; // Import your modules

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/users").configure(users::config))
            .service(web::scope("/health").configure(health::config)),
        // Add more scopes as needed, e.g., .service(web::scope("/auth").configure(auth::config))
    );
}
