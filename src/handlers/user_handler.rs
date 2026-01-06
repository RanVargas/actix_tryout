use actix_web::{HttpResponse, Responder, web, error};
use uuid::Uuid;

use crate::{
    db::{DbError, DbPool},
    models::user::{self, CreateUser, User}
};
use crate::services::user_services;


pub async fn create_user(user: web::Json<CreateUser>) -> HttpResponse {
    let new_user = User {
        id: Uuid::new_v4().to_string(),
        username: user.username.clone(),
        email: user.email.clone(),
    };

    HttpResponse::Created().json(new_user)
}

pub async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<CreateUser>,
) -> actix_web::Result<impl Responder> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let user = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        user_services::insert_new_user(&mut conn, &form.username)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    // user was added successfully; return 201 response with new user info
    Ok(HttpResponse::Created().json(user))
}

pub async fn get_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    // For now, we'll just mock a user response.
    let mock_user = User {
        id: (*user_id).to_string(),
        username: String::from("mock_user"),
        email: String::from("mock_user@example.com"),
    };

    let user_uid = user_id.into_inner();

    // use web::block to offload blocking Diesel queries without blocking server thread
    let user:Option<User> = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        user_services::find_user_by_uid(&mut conn, user_uid)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    /*let user = web::block(move || {
        let mut conn = pool.get()?;
    });*/
    Ok(match user {
        // user was found; return 200 response with JSON formatted user object
        Some(user) => HttpResponse::Ok().json(user),

        // user was not found; return 404 response with error message
        None => HttpResponse::NotFound().body(format!("No user found with UID: {user_uid}")),
    })
    
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json("API is up and running!")
}
