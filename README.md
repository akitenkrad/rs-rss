# rsrss

## How to add new site

### 1. Create site directory

```bash
mkdir src/sites/<<TARGET SITE>>
touch src/sites/<<TARGET SITE>>/mod.rs
touch src/sites/<<TARGET SITE>>/tests.rs
```

#### mod.rs template
```rust
use crate::sites::{Site, WebArticle};
use chrono::DateTime;
use feed_parser::parsers;
pub struct Gigazin {}

#[cfg(test)]
mod tests;

impl Site for Gigazin {
    async fn get_articles(&self) -> Vec<WebArticle> {
        let body = reqwest::get("https://gigazine.net/news/rss_2.0/")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let feeds = parsers::rss2::parse(&body).unwrap();
        let mut articles = Vec::new();
        for feed in feeds {
            articles.push(WebArticle {
                title: feed.title,
                url: feed.link,
                text: feed.description.unwrap_or("".to_string()),
                timestamp: DateTime::parse_from_rfc2822(&feed.publish_date.unwrap())
                    .unwrap()
                    .into(),
            });
        }
        return articles;
    }
}
```

#### tests.rs template

```rust
use super::*;

#[test]
fn test_codezin() {
    let site = Gigazin {};
    let articles = tokio_test::block_on(site.get_articles());
    assert!(articles.len() > 0);
}
```

### 2. Edit src/sites/mod.rs

```rust
use chrono::{DateTime, Local};
use futures::Future;

mod codezin;
mod gigazin;
mod stockmarknews;
mod stockmarktechblog;
mod <<TARGET SITE>>
```

### 3. Implement get_articles()
### 4. Run test
```bash
cargo test
```