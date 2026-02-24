use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Post {
    id: u32,
    title: String,
    summary: String,
    content: String,
    date: String,
}

fn sample_posts() -> Vec<Post> {
    vec![
        Post {
            id: 1,
            title: "Hello, Rust!".to_string(),
            summary: "Why Rust is worth learning in 2025.".to_string(),
            content: "Rust combines low-level control with high-level ergonomics. \
                      Memory safety without a garbage collector makes it uniquely \
                      suited for systems programming, web backends, and everything in between."
                .to_string(),
            date: "2025-01-01".to_string(),
        },
        Post {
            id: 2,
            title: "Building a Blog with Axum".to_string(),
            summary: "A quick tour of Axum, Rust's ergonomic async web framework.".to_string(),
            content: "Axum is built on top of Tokio and Tower. It offers type-safe routing, \
                      extractors, and middleware — all with zero-cost abstractions."
                .to_string(),
            date: "2025-02-01".to_string(),
        },
        Post {
            id: 3,
            title: "Ownership in Plain English".to_string(),
            summary: "Demystifying Rust's ownership model.".to_string(),
            content: "Every value in Rust has a single owner. When the owner goes out of scope, \
                      the value is dropped. Borrowing lets you reference a value without taking \
                      ownership — it's that simple."
                .to_string(),
            date: "2025-03-01".to_string(),
        },
    ]
}

async fn index() -> Html<String> {
    let posts = sample_posts();

    let post_items: String = posts
        .iter()
        .map(|p| {
            format!(
                r#"<li class="post-item">
                     <a href="/posts/{id}">{title}</a>
                     <span class="date">{date}</span>
                     <p>{summary}</p>
                   </li>"#,
                id = p.id,
                title = p.title,
                date = p.date,
                summary = p.summary
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    Html(format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Just a Blog</title>
  <style>
    body {{ font-family: system-ui, sans-serif; max-width: 720px; margin: 4rem auto; padding: 0 1rem; color: #222; }}
    h1 {{ font-size: 2rem; margin-bottom: 0.25rem; }}
    .tagline {{ color: #666; margin-bottom: 2rem; }}
    ul {{ list-style: none; padding: 0; }}
    .post-item {{ margin-bottom: 1.5rem; border-bottom: 1px solid #eee; padding-bottom: 1rem; }}
    .post-item a {{ font-size: 1.2rem; font-weight: bold; text-decoration: none; color: #0070f3; }}
    .post-item a:hover {{ text-decoration: underline; }}
    .date {{ display: block; color: #999; font-size: 0.85rem; margin: 0.2rem 0; }}
  </style>
</head>
<body>
  <h1>Just a Blog</h1>
  <p class="tagline">Built entirely in Rust 🦀</p>
  <ul>{post_items}</ul>
</body>
</html>"#,
        post_items = post_items
    ))
}

async fn list_posts() -> Json<Vec<Post>> {
    Json(sample_posts())
}

async fn get_post(Path(id): Path<u32>) -> impl IntoResponse {
    match sample_posts().into_iter().find(|p| p.id == id) {
        Some(post) => Html(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>{title} — Just a Blog</title>
  <style>
    body {{ font-family: system-ui, sans-serif; max-width: 720px; margin: 4rem auto; padding: 0 1rem; color: #222; }}
    a {{ color: #0070f3; }}
    .date {{ color: #999; font-size: 0.9rem; }}
  </style>
</head>
<body>
  <p><a href="/">← Back</a></p>
  <h1>{title}</h1>
  <span class="date">{date}</span>
  <p>{content}</p>
</body>
</html>"#,
            title = post.title,
            date = post.date,
            content = post.content
        ))
        .into_response(),
        None => (StatusCode::NOT_FOUND, Html("<h1>404 — Post not found</h1>".to_string()))
            .into_response(),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/posts", get(list_posts))
        .route("/posts/:id", get(get_post));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🦀 Blog running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
