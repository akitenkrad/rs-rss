use crate::sites::WebArticle;
use chrono::Local;
use dotenv::dotenv;
use indicatif::ProgressBar;
use keywords::rsc::{extract_keywords, load_keywords, Keyword, Language};
use serde_json::json;
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
    let mut target_articles: Vec<(WebArticle, isize, Vec<Keyword>)> = Vec::new();
    let mut excluded_articles: Vec<(WebArticle, isize, Vec<Keyword>)> = Vec::new();

    // collect articles and keywords
    {
        let bar = ProgressBar::new(articles.len() as u64);
        for article in articles.iter() {
            if skip_outdated_articles && article.timestamp < now - chrono::Duration::days(1) {
                println!(
                    "Skipped: {} (outdated: {:?})",
                    article.title, article.timestamp
                );
                continue;
            }

            let mut extracted_keywords =
                extract_keywords(article.title.as_str(), kws.clone(), Language::Japanese);
            extracted_keywords.extend(extract_keywords(
                article.description.as_str(),
                kws.clone(),
                Language::Japanese,
            ));

            let score = extracted_keywords.iter().map(|kwd| kwd.score).sum::<i8>();
            if score < 0 {
                print!("Skipped: {} (score: {})\n", article.title, score);
                excluded_articles.push((article.clone(), score as isize, extracted_keywords));
            } else {
                target_articles.push((article.clone(), score as isize, extracted_keywords));
            }
            bar.inc(1);
        }
        bar.finish();
    }

    // send excluded articles to Slack
    {
        excluded_articles.sort_by(|a, b| b.0.timestamp.cmp(&a.0.timestamp));
        let bar = ProgressBar::new(excluded_articles.len() as u64);
        for (index, (article, score, kws)) in excluded_articles.into_iter().enumerate() {
            let payload = json!({
                "attachments": [
                    {
                        "color": "#36a64f",
                        "pretext": format!("No.{} - {} @{}", index + 1, article.site, article.timestamp.format("%Y.%m.%d")),
                        "title": format!("{TITLE}",
                            TITLE=article.title,
                        ),
                        "title_link": article.url,
                        "text": format!("{DIVIDER}\nKEYWORDS: {SCORE}\n{KEYWORDS}\n{DIVIDER}\n{TEXT}",
                            DIVIDER="-".repeat(75),
                            SCORE=score,
                            KEYWORDS=kws.iter().map(|kwd| kwd.alias.clone()).collect::<Vec<String>>().join(" / "),
                            TEXT=article.description
                        ),
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
                Ok(_) => {
                    bar.inc(1);
                }
                Err(e) => eprintln!("Failed to send a message to Slack: {}", e),
            }
        }
        bar.finish();
    }

    // send target articles to Slack
    {
        target_articles.sort_by(|a, b| b.1.cmp(&a.1));
        let bar = ProgressBar::new(target_articles.len() as u64);
        for (index, (article, score, mut kws)) in target_articles.into_iter().enumerate() {
            kws.sort_by(|a, b| a.alias.cmp(&b.alias));
            kws.dedup_by(|a, b| a.alias == b.alias);
            let payload = json!({
                "attachments": [
                    {
                        "color": "#36a64f",
                        "pretext": format!("No.{} - {} @{}", index + 1, article.site, article.timestamp.format("%Y.%m.%d")),
                        "title": format!("{TITLE}",
                            TITLE=article.title,
                        ),
                        "title_link": article.url,
                        "text": format!("{DIVIDER}\nKEYWORDS: {SCORE}\n{KEYWORDS}\n{DIVIDER}\n{TEXT}",
                            DIVIDER="-".repeat(75),
                            SCORE=score,
                            KEYWORDS=kws.iter().map(|kwd| kwd.alias.clone()).collect::<Vec<String>>().join(" / "),
                            TEXT=article.description
                        ),
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
                Ok(_) => {
                    bar.inc(1);
                }
                Err(e) => eprintln!("Failed to send a message to Slack: {}", e),
            }
        }
        bar.finish();
    }

    return Ok(());
}
