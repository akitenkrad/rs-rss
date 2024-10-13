use crate::sites::WebArticle;
use chrono::Local;
use serde_json::json;
use std::env;

pub async fn notify_slack(articles: Vec<WebArticle>) {
    let now = Local::now();
    let client = reqwest::Client::new();
    let slack_url = env::var("SLACK_WEBHOOK_URL").expect("SLACK_WEBHOOK_URL is not set");

    for (i, article) in articles.iter().enumerate() {
        if article.timestamp < now - chrono::Duration::days(1) {
            println!("Skip an old article: {}", article.title);
            continue;
        }

        let payload = json!({
            "attachments": [
                {
                    "color": "#36a64f",
                    "pretext": format!("No.{} - {} @{}", i+1, article.site, article.timestamp.format("%Y.%m.%d")),
                    "title": format!("{TITLE}\n{DIVIDER}\nKEYWORDS:{SCORE}\n{KEYWORDS}\n{DIVIDER}",
                        TITLE=article.title,
                        DIVIDER="-".repeat(75),
                        SCORE=0,
                        KEYWORDS="",
                    ),
                    "title_link": article.url,
                    "text": article.text,
                }
            ]
        });

        let res = client
            .post(&slack_url)
            .header("Content-Type", "application/json")
            .body(payload.to_string())
            .send()
            .await;

        match res {
            Ok(_) => println!("Successfully sent a message to Slack"),
            Err(e) => eprintln!("Failed to send a message to Slack: {}", e),
        }
    }
}
