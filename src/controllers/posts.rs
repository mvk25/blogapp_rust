use actix_web::{HttpResponse, Responder, web};
use crate::models::ui::DashboardTemplate;
use crate::models::{posts::*, ui::PostTemplate};
use crate::models::app_state::AppState;
use diesel::{RunQueryDsl};
use askama::Template;
use crate::db_operations::posts::get_all_posts;

pub async fn post_page(error: Option<String>) -> HttpResponse {
    let template = PostTemplate { error };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

pub async fn add_post(post: web::Form<NewPostForm>, state: web::Data<AppState>) -> HttpResponse {
    println!("Here we are");
    if post.title.is_empty() || post.body.is_empty() || post.img.is_empty() || post.published_by.is_empty() {
        println!("Empty Fields detected");
        return HttpResponse::BadRequest().body("All fields are required");
    }
    println!("All fields have content");

    let new_post = NewPost {
        title: post.title.clone(),
        body: post.body.clone(),
        img: post.img.clone(),
        published_by: post.published_by.clone()
    };

    let mut connection = state.db_connection.lock().unwrap();
    let result = diesel::insert_into(crate::schema::posts::table).values(&new_post).get_result::<Posts>(&mut *connection);
    match result {
        Ok(_) => {
            let posts = get_all_posts(&mut *connection);
            println!("{:?}", posts);
            let template = DashboardTemplate { email: None, posts: Some(posts) };
            HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
        }
        Err(e) => {
            println!("Error adding Post to Database");
            let error_msg = e.to_string();
            let template = PostTemplate { error: Some(error_msg) };
            HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
        }
    }
}