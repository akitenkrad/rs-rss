use crate::sites::WebArticle;
use chrono::Local;
use dotenv::dotenv;
use indicatif::ProgressBar;
use keywords::rsc::{extract_keywords, load_keywords, Language};
use serde_json::{json, Value};
use std::env;

#[cfg(test)]
mod tests;

pub async fn notify_slack(
    articles: Vec<WebArticle>,
    skip_outdated_articles: bool,
) -> Result<(), String> {
    dotenv().ok();

    let now = Local::now();
    let client = reqwest::Client::new();
    let slack_url = env::var("SLACK_WEBHOOK_URL").expect("SLACK_WEBHOOK_URL is not set");
    let kws = load_keywords();
    let mut target_articles: Vec<(WebArticle, usize, Value)> = Vec::new();

    let mut index = 1;
    let bar = ProgressBar::new(articles.len() as u64);
    for article in articles.iter() {
        if skip_outdated_articles && article.timestamp < now - chrono::Duration::days(1) {
            continue;
        }

        let mut extracted_keywords =
            extract_keywords(article.title.as_str(), kws.clone(), Language::Japanese);
        extracted_keywords.extend(extract_keywords(
            article.description.as_str(),
            kws.clone(),
            Language::Japanese,
        ));
        extracted_keywords.sort_by(|a, b| a.alias.cmp(&b.alias));
        extracted_keywords.dedup_by(|a, b| a.alias == b.alias);

        let payload = json!({
            "attachments": [
                {
                    "color": "#36a64f",
                    "pretext": format!("No.{} - {} @{}", index, article.site, article.timestamp.format("%Y.%m.%d")),
                    "title": format!("{TITLE}",
                        TITLE=article.title,
                    ),
                    "title_link": article.url,
                    "text": format!("{DIVIDER}\nKEYWORDS: {SCORE}\n{KEYWORDS}\n{DIVIDER}\n{TEXT}",
                        DIVIDER="-".repeat(75),
                        SCORE=extracted_keywords.iter().map(|kwd| kwd.score).sum::<u8>(),
                        KEYWORDS=extracted_keywords.iter().map(|kwd| kwd.alias.clone()).collect::<Vec<String>>().join(" / "),
                        TEXT=article.description
                    ),
                }
            ]
        });
        index += 1;
        target_articles.push((
            article.clone(),
            extracted_keywords.iter().map(|kwd| kwd.score).sum::<u8>() as usize,
            payload,
        ));
        bar.inc(1);
    }
    bar.finish();

    target_articles.sort_by(|a, b| b.1.cmp(&a.1));
    let bar = ProgressBar::new(target_articles.len() as u64);
    for (_, _, payload) in target_articles.iter() {
        let res = client
            .post(&slack_url)
            .header("Content-Type", "application/json")
            .body(payload.to_string())
            .send()
            .await;

        match res {
            Ok(_) => {
                bar.inc(1);
            }
            Err(e) => eprintln!("Failed to send a message to Slack: {}", e),
        }
        index += 1;
    }
    bar.finish();
    return Ok(());
}
