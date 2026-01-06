// In main.rs
fn main() {
    HttpServer::new(|| {
        App::new()
            .configure(users::config) // But this would need users::config to handle the full /api/users scope
            .configure(health::config)
    })
}
