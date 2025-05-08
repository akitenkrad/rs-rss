use adapter::database::{connect_database_with, models::web_article};
use anyhow::{Context, Error, Result};
use crawler::models::{
    get_all_sites,
    web_article::{WebArticleResource, WebSiteResource},
};
use registry::Registry;
use shared::{
    config::AppConfig,
    env::{which, Environment},
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn init_logger() -> Result<()> {
    let log_level = match which() {
        Environment::Development => "debug",
        Environment::Production => "info",
    };

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| log_level.into());
    let subscriber = tracing_subscriber::fmt::layer().with_file(true).with_line_number(true).with_target(false);

    tracing_subscriber::registry().with(subscriber).with(env_filter).try_init()?;

    Ok(())
}

#[tokio::main]
async fn main() {
    init_logger().expect("Failed to initialize logger");

    let config = AppConfig::new().expect("Failed to load config");
    let db = connect_database_with(&config.database);
    let registry = Registry::new(db);

    tracing::info!("Starting to collect articles...");
    let mut sites: Vec<Box<dyn WebSiteResource>> = get_all_sites();
    let mut articles = Vec::<WebArticleResource>::new();
    for site in sites.iter_mut() {
        match site.get_articles().await {
            Ok(site_articles) => {
                articles.extend(site_articles);
            }
            Err(e) => {
                tracing::error!("Failed to get articles from {}: {}", site.domain(), e);
            }
        }
    }
    tracing::info!("Collected {} articles", articles.len());

    //TODO: Fill the WebArticleResource attributes

    // ↓ save to DB

    // Example usage of the registry
    let web_article_repository = registry.web_article_repository();
    // Use the repository as needed...
}
