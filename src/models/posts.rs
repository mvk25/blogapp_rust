use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable,  Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Posts {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub sub_title: String,
    pub slug: String,
    pub is_published: bool,
    pub img: String,
    pub published_by: String,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub img: String,
    pub slug: String,
    pub sub_title: String,
    pub published_by: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewPostForm {
    pub title: String,
    pub body: String,
    pub img: String,
    pub slug: String,
    pub sub_title: String,
    pub published_by: String,
}

#[derive(Deserialize)]
pub struct PostSlug {
    pub slug: String
}