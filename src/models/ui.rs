use askama::Template;

use super::posts::Posts;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate{
    pub(crate) error: Option<String>,
    pub(crate) message: Option<String>
}

// #[derive(Template)]
// #[template(path = "register.html")]
// pub struct RegisterTemplate{
//     pub(crate) error: Option<String>
// }


#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate{
    pub(crate) error: Option<String>
}


#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub(crate) email: Option<String>,
    pub(crate) posts: Option<Vec<Posts>>
}

#[derive(Template)]
#[template(path = "posts.html")]
pub struct PostTemplate {
    pub(crate) error: Option<String>,
}