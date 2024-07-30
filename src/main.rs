mod controllers;
mod models;
mod schema;
mod db_operations;

use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{cookie::{SameSite, Key}, web, App, HttpServer};
use db_operations::db::establish_connection;
use dotenvy::dotenv;
use models::app_state::AppState;
use std::sync::Mutex;
use controllers::users::*;
use controllers::posts::*;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().unwrap();

    let secret_key = Key::generate();
    HttpServer::new(move || {
        let app_state = web::Data::new(AppState { db_connection: Mutex::new(establish_connection())});
        App::new()
            .app_data(app_state)
            .wrap(SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .cookie_secure(false)
                .cookie_http_only(true)
                .cookie_same_site(SameSite::Lax)
                .build()
            )
            // .service(Files::new("/login", "./static/css/").index_file(index))
            .route("/register", web::get().to(register_page))
            .route("/register", web::post().to(register_user))
            .route("/login", web::get().to(login_page))
            .route("/login", web::post().to(login_user))
            .route("/dashboard", web::get().to(dashboard_page))
            .route("/add-post", web::get().to(post_page))
            .route("/add-post", web::post().to(add_post))
            .route("post/{slug}", web::get().to(single_post))

    }).bind("127.0.0.1:8080")?.run().await

}