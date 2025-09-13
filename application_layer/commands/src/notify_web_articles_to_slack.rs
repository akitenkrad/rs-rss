use adapter::database::connect_database_with;
use clap::Parser;
use kernel::models::web_article::WebArticle;
use keywords::rsc::{extract_keywords, load_keywords, Keyword, Language};
use registry::AppRegistryImpl;
use serde_json::{json, Value};
use shared::config::AppConfig;
use shared::utils::create_progress_bar;
use std::sync::Arc;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct NotifyWebArticlesToSlackArgs {}

async fn select_todays_articles() -> Vec<WebArticle> {
    let config = AppConfig::new().expect("Failed to load config");
    let db = connect_database_with(&config.database);
    let registry = Arc::new(AppRegistryImpl::new(db));

    let web_article_registry = registry.web_article_repository();
    let articles = match web_article_registry.select_todays_web_articles().await {
        Ok(articles) => articles,
        Err(err) => {
            tracing::error!("Failed to select today's articles: {}", err);
            return vec![];
        }
    };

    let today = chrono::Local::now();
    let yesterday = today - chrono::Duration::days(1);
    articles
        .iter()
        .filter(|article| yesterday <= article.timestamp && article.timestamp <= today)
        .cloned()
        .collect()
}

fn article2props(article: &WebArticle) -> String {
    let mut props = vec![];
    if article.is_ai_related {
        props.push("AI Related");
    }
    if article.is_it_related {
        props.push("IT Related");
    }
    if article.is_security_related {
        props.push("Security Related");
    }
    if article.is_new_academic_paper_related {
        props.push("New Academic Paper Related");
    }
    if article.is_new_technology_related {
        props.push("New Technology Related");
    }
    if article.is_new_product_related {
        props.push("New Product Related");
    }
    return props.join(" | ");
}

fn to_payload(index: usize, article: WebArticle, score: isize, kws: Vec<Keyword>) -> Value {
    let prop_text = article2props(&article);
    let pretext = format!("No.{} - *{}* @{}", index + 1, article.site.name, article.timestamp);
    let score = if score > 0 {
        format!("*{}*", score)
    } else {
        format!("{}", score)
    };
    let text = format!(
        "{DIVIDER}{PROPS}\nKEYWORDS: {SCORE}\n{KEYWORDS}\n{DIVIDER}\n>{TEXT}",
        DIVIDER = "-".repeat(75),
        PROPS = prop_text,
        SCORE = score,
        KEYWORDS = kws
            .iter()
            .map(|kwd| format!("{}({})", kwd.alias.clone(), kwd.score))
            .collect::<Vec<String>>()
            .join(" / "),
        TEXT = article.summary.replace("\n", "\n>")
    );

    let payload = json!({
        "attachments": [
            {
                "color": "#36a64f",
                "pretext": pretext,
                "title": article.title,
                "title_link": article.url,
                "text": text,
            }
        ]
    });
    payload
}

pub async fn notify_to_slack(_args: &NotifyWebArticlesToSlackArgs) {
    let articles = select_todays_articles().await;

    let client = request::Client::new();
    let slack_url = std::env::var("SLACK_WEBHOOK_URL").expect("SLACK_WEBHOOK_URL is not set");
    let kws = load_keywords();
    let mut target_articles: Vec<(WebArticle, isize, Vec<Keyword>)> = Vec::new();

    // collect articles and keywords
    {
        let bar = create_progress_bar(articles.len() as usize, Some("Collecting articles and keywords".into()));
        for article in articles.iter() {
            let mut extracted_keywords = extract_keywords(article.title.as_str(), kws.clone(), Language::Japanese);
            extracted_keywords.extend(extract_keywords(
                article.description.as_str(),
                kws.clone(),
                Language::Japanese,
            ));

            let score = extracted_keywords.iter().map(|kwd| kwd.score).sum::<isize>();
            if score > 0 {
                target_articles.push((article.clone(), score, extracted_keywords));
            }
            bar.inc(1);
        }
        bar.finish();
    }

    // send target articles to Slack
    {
        target_articles.sort_by(|a, b| b.1.cmp(&a.1));
        let bar = create_progress_bar(
            target_articles.len() as usize,
            Some("Sending target articles to Slack".into()),
        );
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
                Ok(_) => {}
                Err(e) => eprintln!("Failed to send a message to Slack: {}", e),
            }
            bar.inc(1);
        }
        bar.finish();
    }
}
