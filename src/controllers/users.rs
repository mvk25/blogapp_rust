use crate::db_operations::posts::get_all_posts;
use crate::db_operations::users::{get_user_by_email, get_user_by_id};
use diesel::{RunQueryDsl};
use crate::models::users::LoginForm;
use crate::models::users::{Users, NewUser, NewUserForm};
use crate::models::ui::{RegisterTemplate, LoginTemplate, DashboardTemplate};
use bcrypt::{hash, DEFAULT_COST, verify};
use actix_session::Session;
use actix_web::{web, Responder, HttpResponse, HttpRequest};
use askama::Template;
use crate::models::app_state::AppState;

async fn handle_register_error(error: &str) -> HttpResponse {
    let template = RegisterTemplate { error: Some(error.to_string()) };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

async fn handle_login_information(error: &str) -> HttpResponse {
    let template = LoginTemplate { error: None, message: Some(error.to_string()) };
    HttpResponse::Ok().content_type("text/html").append_header((actix_web::http::header::LOCATION, "/dashboard")).body(template.render().unwrap())
}

pub async fn register_page(error: Option<String>) -> HttpResponse {
    let template = RegisterTemplate{ error };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

pub async fn register_user(item: web::Form<NewUserForm>, state: web::Data<AppState>) -> impl Responder {
    if item.name.is_empty() || item.email.is_empty() || item.bio.is_empty() || item.password.is_empty() {
        println!("Empty fields detected");
        return HttpResponse::BadRequest().body("All fields are required");
    }
    println!("All fields have content");

    let hashed_password = match hash(&item.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(e) => {
            println!("Error");
            return handle_register_error("error hashing password").await;
        }
    };

    let new_user = NewUser {
        name: item.name.clone(),
        email: item.email.clone(),
        bio: item.bio.clone(),
        password: hashed_password,
        avatar_url: "".to_string(),
        blocked_reason: "".to_string(),

    };

    let mut connection = state.db_connection.lock().unwrap();
    let res= diesel::insert_into(crate::schema::users::table).values(&new_user).get_result::<Users>(&mut *connection);
    match res {
        Ok(_) => {
            return handle_login_information("Account created, please login to continue").await;
        }
        Err(err) => {
            println!("db error {:#?}", err);
            return handle_register_error("error creating account").await;
        }
    }

}

pub async fn login_page(error: Option<String>, message: Option<String>) -> impl Responder {
    let template = LoginTemplate { error, message };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

pub async fn dashboard_page(state: web::Data<AppState>, session: Session, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let result = match session.get::<String>("user_email") {
        Ok(Some(user_email)) => {
            let mut connection_guard = state.db_connection.lock().map_err(|_| {
                actix_web::error::ErrorInternalServerError("Database error")
            })?;
            match get_user_by_email(&mut *connection_guard, user_email.clone()) {
                Some(user) => {
                    let posts = get_all_posts(&mut connection_guard);
                    let dashboard_template = DashboardTemplate {
                        email: Some(user.email.clone()),
                        posts: Some(posts)
                    };
                    println!("User found");
                    Ok(HttpResponse::Ok().content_type("text/html").body(dashboard_template.render().map_err(|_| {
                        actix_web::error::ErrorInternalServerError("Template error")
                    })?))
                }
                None => {
                    println!("User not found");
                    Ok(HttpResponse::Found()
                        .append_header((actix_web::http::header::LOCATION, "/login"))
                        .finish())
                }
            }
        },
        Ok(None) => {
            println!("No user email in session");
            Ok(HttpResponse::Found()
                .append_header((actix_web::http::header::LOCATION, "/login"))
                .finish())
        },
        Err(_) => {
            println!("Session error");
            Err(actix_web::error::ErrorInternalServerError("Session error"))
        },
    };

    result
}

pub async fn login_user(form: web::Form<LoginForm>, state: web::Data<AppState>, session: Session) -> Result<HttpResponse, actix_web::Error> {
    let mut connection_guard = state.db_connection.lock().unwrap();

    let user_exist = get_user_by_email(&mut *connection_guard, form.email.clone());
    match user_exist {
        Some(user) => {
            if verify(&form.password, &user.password).unwrap_or(false) {
                session.insert("user_email", form.email.clone())?;
                // Redirect to the dashboard route
                println!("Password confirmed");
                Ok(HttpResponse::Found()
                    .append_header((actix_web::http::header::LOCATION, "/dashboard"))
                    .finish())
            } else {
                let error_message = "Wrong password.".to_string();
                let template = LoginTemplate { error: Some(error_message), message: None };
                Ok(HttpResponse::Ok()
                    .content_type("text/html")
                    .body(template.render().unwrap()))
            }
        }
        None => {
            let error_message = "Email not found".to_string();
            let template = LoginTemplate { error: Some(error_message), message: None };
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(template.render().unwrap()))
        }
    }
}
