# Just a Blog 🦀

A minimal blog built entirely in Rust, using [Axum](https://github.com/tokio-rs/axum).

## Routes

| Method | Path         | Description              |
|--------|--------------|--------------------------|
| GET    | `/`          | Blog homepage            |
| GET    | `/posts`     | JSON list of all posts   |
| GET    | `/posts/:id` | Single post page         |

## Running

Make sure you have [Rust installed](https://rustup.rs), then:

```bash
cargo run
```

The server starts at **http://localhost:3000**.

## Stack

- **[Axum](https://github.com/tokio-rs/axum)** — async web framework
- **[Tokio](https://tokio.rs)** — async runtime
- **[Serde](https://serde.rs)** — JSON serialization

## TODO

- [ ] Markdown rendering for post content
- [ ] Load posts from files or a database
- [ ] RSS feed
- [ ] Dark mode
