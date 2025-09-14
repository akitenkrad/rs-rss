use std::sync::Arc;

use adapter::database::connect_database_with;
use clap::Parser;
use kernel::models::web_article::WebArticle;
use registry::AppRegistryImpl;
use shared::{config::AppConfig, utils::create_progress_bar};
use web_article_crawler::models::{get_all_sites, web_article::WebSiteResource};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct CollectArticlesArgs {}

pub async fn collect_articles(_args: &CollectArticlesArgs) {
    let config = AppConfig::new().expect("Failed to load config");
    let db = connect_database_with(&config.database);
    let registry = Arc::new(AppRegistryImpl::new(db));

    tracing::info!("Starting to collect articles...");
    let mut sites: Vec<Box<dyn WebSiteResource>> = get_all_sites(&registry).await.unwrap();
    let mut articles = Vec::<WebArticle>::new();
    let today = chrono::Local::now();
    let pb = create_progress_bar(sites.len() as usize, Some("Collecting articles".into()));
    for site in sites.iter_mut() {
        match site.get_articles().await {
            Ok(mut site_articles) => {
                for article in site_articles.iter_mut() {
                    // Check if the article is from today
                    if article.timestamp.date_naive() != today.date_naive() {
                        continue;
                    }

                    // Parse the article to get HTML and text
                    let (html, text) = match site.parse_article(&article.article_url).await {
                        Ok((html, text)) => (html, text),
                        Err(e) => {
                            tracing::error!("Failed to parse article {}: {}", article.title, e);
                            ("NO HTML".into(), "NO TEXT".into())
                        }
                    };
                    article.html = html;
                    article.text = text;

                    // Check if the article has HTML and text
                    if article.html == String::from("NO HTML") || article.text == String::from("NO TEXT") {
                        tracing::error!("No HTML/TEXT found for article {}", article.article_url);
                        continue;
                    }

                    //Fill the article attributes
                    let mut web_article = WebArticle::from(article.clone());
                    if let Err(e) = web_article.fill_attributes().await {
                        tracing::error!(
                            "Failed to fill attributes for article {}: {}",
                            web_article.article_id,
                            e
                        );
                    }

                    // Check if the article is related to AI etc
                    if !web_article.is_ai_related
                        && !web_article.is_new_technology_related
                        && !web_article.is_new_product_related
                        && !web_article.is_new_academic_paper_related
                        && !web_article.is_security_related
                        && !web_article.is_it_related
                    {
                        tracing::info!("Skipped an irrelevant article: {}", web_article.title);
                        continue;
                    }

                    articles.push(web_article.clone());
                }
            }
            Err(e) => {
                tracing::error!("Failed to get articles from {}: {}", site.domain(), e);
            }
        }
        pb.inc(1);
    }
    pb.finish_and_clear();
    tracing::info!("Collected {} articles", articles.len());

    // save to DB
    let pb = create_progress_bar(articles.len() as usize, Some("Saving articles to DB".into()));
    let web_article_repository = registry.web_article_repository();
    for article in articles.iter() {
        let _ = match web_article_repository
            .select_or_create_web_article(article.clone())
            .await
        {
            Ok(web_article) => {
                pb.inc(1);
                web_article
            }
            Err(e) => {
                tracing::error!("Failed to save web article {} ({})", article.title, e);
                pb.inc(1);
                continue;
            }
        };
    }
    pb.finish_and_clear();
    tracing::info!("Saved {} articles to DB", articles.len());
}
