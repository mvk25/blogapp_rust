use diesel::PgConnection;
use diesel::prelude::*;
use crate::models::posts::Posts;
use crate::schema::posts::dsl::*;

pub fn get_all_posts(connection: &mut PgConnection) -> Vec<Posts> {
    let mut all_posts: Vec<Posts> = Vec::new();
    let results = posts
        .filter(is_published.eq(false))
        .limit(5)
        .select(Posts::as_select())
        .load(connection);

    match results {
        Ok(data) => {
            for post in data.into_iter() {
                all_posts.push(post);
            }
        }
        Err(e) => println!("Error occurred {:?}", e)
    }

    all_posts
}

pub fn get_post_by_slug(connection: &mut PgConnection, post_slug: String) -> Option<Posts> {
    let post = posts.filter(slug.eq(post_slug)).first::<Posts>(connection).optional().unwrap_or_else(|err| {
        println!("An error occurred while fetching the single post: {err}");
        None
    });

    post
}