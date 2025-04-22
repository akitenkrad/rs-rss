# rsrss

[![CircleCI](https://dl.circleci.com/status-badge/img/circleci/X1fiE4koKU88Z9sKwWoPAH/CwXRRPaUf8UKhZHHUQqHNJ/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/circleci/X1fiE4koKU88Z9sKwWoPAH/CwXRRPaUf8UKhZHHUQqHNJ/tree/main)

## How to add new site

### 1. Create site directory

```bash
mkdir src/sites/<<TARGET SITE>>
touch src/sites/<<TARGET SITE>>/mod.rs
touch src/sites/<<TARGET SITE>>/tests.rs
```

#### mod.rs template

```rust
use crate::sites::{Category, Html, Text, Site, WebArticle};
use chrono::DateTime;
use feed_parser::parsers; use anyhow::{Error, Result};
pub struct Gigazin {}

#[cfg(test)]
mod tests;

impl Site for Gigazin {
    async fn get_articles(&self) -> Result<Vec<WebArticle>>{
        let body = reqwest::get("https://gigazine.net/news/rss_2.0/")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let feeds = if let Ok(r) = parsers::rss2::parse(&body) {
            r
        } else {
            return Err(Error::msg("Failed to parse RSS"));
        };
        let mut articles = Vec::new();
        for feed in feeds {
            articles.push(WebArticle {
                site: self.name(),
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
    let articles = site.get_articles().await;
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
