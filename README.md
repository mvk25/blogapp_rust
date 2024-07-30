# BlogApp

**BlogApp** is a simple blog application built using Rust. It includes features like user authentication, post creation, and display. The app uses Actix-web as the web framework and Diesel for database interactions.

## Project Layout

The project consists of the following key components:

- **Login Page**: Allows existing users to log in to their accounts. The page requires a username and password, which are validated using bcrypt.

- **Register Page**: New users can register for an account. The registration form collects necessary information, including username, email, and password. Passwords are hashed before storing them in the database.

- **User Dashboard**: Once logged in, users are redirected to the dashboard. The dashboard displays the user's posts and provides options to create new posts, edit existing ones, or delete posts.

## Post Creation and Display

### Post Creation

Users can create new blog posts through the user dashboard. The post creation form includes fields for the title, subtitle, body content, and an optional image URL. The form data is validated and then stored in the database using Diesel.

### Post Display

The posts created by users are displayed on the dashboard. Each post includes the following details:

- **Title**: The main heading of the post.
- **Subtitle**: A brief description or secondary heading for the post.
- **Body**: The main content of the post, supporting rich text and formatting.
- **Image**: An optional image associated with the post.

The frontend uses the Askama templating engine to render the posts dynamically. Askama provides a powerful and flexible way to generate HTML pages, ensuring that the content is presented consistently and beautifully across different pages of the application.

## Database Issues

### PostgreSQL Database

The application uses PostgreSQL as the database backend, managed through the Diesel ORM. However, there are currently some issues with the database schema or data retrieval:

- **Field Mapping Issue**: There is an issue where data meant for one field appears in another. For example, when attempting to access the `body` of a post, the data from the `slug` field is retrieved instead.

These issues are being investigated, and we are working towards identifying the root cause. Future updates to the project will include fixes to these problems, ensuring that the data is stored and retrieved correctly from the database.

## Templating and Data Handling

### Askama Templating Engine

The application uses the Askama templating engine to render HTML pages. Askama integrates seamlessly with Rust, allowing for the embedding of Rust expressions and control structures within the HTML templates.

Data from the database is passed into the templates using Rust structs. For example, the `DashboardTemplate` struct is used to pass user-specific data to the dashboard view:

```rust
#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub(crate) email: Option<String>,
    pub(crate) posts: Option<Vec<Posts>>,
}

### Using Rust in HTML Templates

In the HTML template, Rust code and syntax are embedded to dynamically generate content based on the data passed to the template. For example:

```html
<div class="row mb-4 wow fadeIn">
    {% if let Some(postings) = posts %}
        {% for post in postings %}
            <div class="col-lg-4 col-md-12 mb-4">
                <h2>{{ post.title }}</h2>
                <p>{{ post.body }}</p>
            </div>
        {% endfor %}
    {% endif %}
</div>
