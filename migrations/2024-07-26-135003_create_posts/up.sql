-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title  TEXT NOT NULL ,
    body TEXT NOT NULL,
    is_published BOOLEAN NOT NULL DEFAULT FALSE,
    img TEXT NOT NULL,
    published_by TEXT NOT NULL,
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
)