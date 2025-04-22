use crate::sites::WebArticle;
use dotenv::dotenv;
use indicatif::ProgressBar;
use keywords::rsc::{extract_keywords, load_keywords, Keyword, Language};
use serde_json::{json, Value};
use std::env;

#[cfg(test)]
mod tests;

fn is_included(article: WebArticle) -> bool {
    let prop = &article.property;

    let new_tech = prop.is_new_technology_related.unwrap_or_default();
    let new_prod = prop.is_new_product_introduction.unwrap_or_default();
    let new_paper = prop.is_new_paper.unwrap_or_default();
    let ai_related = prop.is_ai_related.unwrap_or_default();

    // | New Product | New Technology | New Paper | AI Related | Result |
    // |-------------|----------------|-----------|------------|--------|
    // |           0 |              0 |         0 |          0 | false  |
    // |           0 |              0 |         0 |          1 | true   |
    // |           0 |              0 |         1 |          0 | true   |
    // |           0 |              0 |         1 |          1 | true   |
    // |           0 |              1 |         0 |          0 | true   |
    // |           0 |              1 |         0 |          1 | true   |
    // |           0 |              1 |         1 |          0 | true   |
    // |           0 |              1 |         1 |          1 | true   |
    // |           1 |              0 |         0 |          0 | false  |
    // |           1 |              0 |         0 |          1 | true   |
    // |           1 |              0 |         1 |          0 | true   |
    // |           1 |              0 |         1 |          1 | true   |
    // |           1 |              1 |         0 |          0 | false  |
    // |           1 |              1 |         0 |          1 | true   |
    // |           1 |              1 |         1 |          0 | true   |
    // |           1 |              1 |         1 |          1 | true   |

    if !new_prod && !new_tech && !new_paper && !ai_related {
        return false;
    }
    if new_prod && !new_tech && !new_paper && !ai_related {
        return false;
    }
    if new_prod && new_tech && !new_paper && !ai_related {
        return false;
    }

    return true;
}

fn to_payload(index: usize, article: WebArticle, score: isize, kws: Vec<Keyword>) -> Value {
    let payload = json!({
        "attachments": [
            {
                "color": "#36a64f",
                "pretext": format!("No.{} - *{}* @{}", index + 1, article.site, article.timestamp.format("%Y.%m.%d %H:%M:%S")),
                "title": format!("{TITLE}",
                    TITLE=article.title,
                ),
                "title_link": article.url,
                "text": format!("{DIVIDER}{PROPS}\nKEYWORDS: {SCORE}\n{KEYWORDS}\n{DIVIDER}\n>{TEXT}",
                    DIVIDER="-".repeat(75),
                    PROPS=article.property.to_payload(),
                    SCORE=score,
                    KEYWORDS=kws.iter().map(|kwd| format!("{}({})", kwd.alias.clone(), kwd.score)).collect::<Vec<String>>().join(" / "),
                    TEXT=article.description.replace("\n", "\n>")
                ),
            }
        ]
    });
    return payload;
}

pub async fn notify_slack(articles: Vec<WebArticle>) -> Result<(), String> {
    dotenv().ok();

    let client = request::Client::new();
    let slack_url = env::var("SLACK_WEBHOOK_URL").expect("SLACK_WEBHOOK_URL is not set");
    let kws = load_keywords();
    let mut target_articles: Vec<(WebArticle, isize, Vec<Keyword>)> = Vec::new();
    let mut excluded_articles: Vec<(WebArticle, isize, Vec<Keyword>)> = Vec::new();

    // collect articles and keywords
    {
        let bar = ProgressBar::new(articles.len() as u64);
        for article in articles.iter() {
            let mut extracted_keywords = extract_keywords(article.title.as_str(), kws.clone(), Language::Japanese);
            extracted_keywords.extend(extract_keywords(article.description.as_str(), kws.clone(), Language::Japanese));

            let score = extracted_keywords.iter().map(|kwd| kwd.score).sum::<isize>();
            if is_included(article.clone()) {
                excluded_articles.push((article.clone(), score as isize, extracted_keywords));
            }
            bar.inc(1);
        }
        bar.finish_and_clear();
    }

    // send excluded articles to Slack
    {
        excluded_articles.sort_by(|a, b| b.0.timestamp.cmp(&a.0.timestamp));
        let bar = ProgressBar::new(excluded_articles.len() as u64);
        for (index, (article, score, kws)) in excluded_articles.into_iter().enumerate() {
            let payload = to_payload(index, article.clone(), score, kws.clone());
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
            let payload = to_payload(index, article.clone(), score, kws.clone());
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
