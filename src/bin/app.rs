use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::net::TcpListener;

use academic_paper_crawler::repository::academic_papers::get_academic_paper;
use adapter::database::connect_database_with;
use anyhow::{Context as _, Result};
use axum::{http::Method, Router};
use clap::{Parser, Subcommand};
use dashboard::route::v1;
use kernel::models::{academic_paper::AcademicPaper, web_article::WebArticle};
use registry::AppRegistryImpl;
use shared::{config::AppConfig, logger::init_logger, utils::create_progress_bar};
use tower_http::{
    cors::{self, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use web_article_crawler::models::{get_all_sites, web_article::WebSiteResource};

#[derive(Debug, Parser, Clone)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand, Debug, Clone)]
enum SubCommands {
    /// Collect articles from websites
    CollectArticles(CollectArticlesArgs),
    /// Start the dashboard
    StartDashboard(StartDashboardArgs),
    /// Add an academic paper to the database
    AddAcademicPaper(AddAcademicPaperArgs),
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct CollectArticlesArgs {}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct StartDashboardArgs {}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct AddAcademicPaperArgs {
    #[arg(long)]
    title: String,
    /// The URL of the academic paper to add
    #[arg(long)]
    pdf_url: String,
}

async fn collect_articles(_args: &CollectArticlesArgs) {
    let config = AppConfig::new().expect("Failed to load config");
    let db = connect_database_with(&config.database);
    let registry = Arc::new(AppRegistryImpl::new(db));

    tracing::info!("Starting to collect articles...");
    let mut sites: Vec<Box<dyn WebSiteResource>> = get_all_sites(&registry).await.unwrap();
    let mut articles = Vec::<WebArticle>::new();
    let today = chrono::Local::now().date_naive();
    let pb = create_progress_bar(sites.len() as usize, Some("Collecting articles".into()));
    for site in sites.iter_mut() {
        match site.get_articles().await {
            Ok(mut site_articles) => {
                for article in site_articles.iter_mut() {
                    // Check if the article is from today
                    if article.timestamp.date_naive() != today {
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

fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_headers(cors::Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(cors::Any)
}

async fn bootstrap() -> Result<()> {
    let app_config = AppConfig::new()?;
    let pool = connect_database_with(&app_config.database);
    let registry = Arc::new(AppRegistryImpl::new(pool));

    let app = Router::new()
        .merge(v1::routes())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(tower_http::LatencyUnit::Millis),
                ),
        )
        .layer(cors())
        .with_state(registry);

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Listening on {}", addr);
    axum::serve(listener, app)
        .await
        .context("Unexpected error happened in server")
        .inspect_err(|e| {
            tracing::error!(
                error.cause_chain = ?e,error.message = %e, "Unexpected error"
            )
        })
}

async fn start_dashboard(_args: &StartDashboardArgs) {
    // Start the dashboard
    tracing::info!("Starting dashboard...");
    bootstrap().await.expect("Failed to start dashboard");
}

async fn add_academic_paper(args: &AddAcademicPaperArgs) {
    // This function is a placeholder for adding an academic paper
    tracing::info!("Adding academic paper...");

    // crawler
    let paper_rsc = get_academic_paper(&args.title, &args.pdf_url)
        .await
        .expect("Failed to get academic paper by URL");

    // kernel
    let mut paper = AcademicPaper::from(paper_rsc);
    paper
        .fill_fields_with_ai()
        .await
        .expect("Failed to fill fields with AI");
    paper.fill_bibtex().expect("Failed to fill BibTeX");

    // Log the paper details
    tracing::info!("Academic Paper Details:");
    tracing::info!("ArXiv ID: {}", paper.arxiv_id);
    tracing::info!("Semantic Scholar ID: {}", paper.ss_id);
    tracing::info!("Title: {}", paper.title);
    tracing::info!("Abstract: {}", paper.abstract_text_ja);
    tracing::info!("Summary: {}", paper.summary);
    tracing::info!("Background and Purpose: {}", paper.background_and_purpose);
    tracing::info!("Methodology: {}", paper.methodology);
    tracing::info!("Dataset: {}", paper.dataset);
    tracing::info!("Results: {}", paper.results);
    tracing::info!(
        "Advantages, Limitations, and Future Work: {}",
        paper.advantages_limitations_and_future_work
    );
    tracing::info!("BibTeX: {}", paper.bibtex);
    tracing::info!("PDF URL: {}", paper.url);
    tracing::info!("Published Date: {}", paper.published_date);
    tracing::info!("DOI: {}", paper.doi);
    tracing::info!("Reference Count: {}", paper.reference_count);
    tracing::info!("Citation Count: {}", paper.citation_count);
    tracing::info!("Influential Citation Count: {}", paper.influential_citation_count);
    tracing::info!("Primary Category: {}", paper.primary_category);
    tracing::info!("Authors: {:?}", paper.authors);
    tracing::info!("Journal: {:?}", paper.journal);
    tracing::info!("Tasks: {:?}", paper.tasks);

    // Save to DB
    let config = AppConfig::new().expect("Failed to load config");
    let db = connect_database_with(&config.database);
    let registry = Arc::new(AppRegistryImpl::new(db));

    let academic_paper_repository = registry.academic_paper_repository();
    match academic_paper_repository.create_academic_paper(paper).await {
        Ok(_) => tracing::info!("Successfully added academic paper"),
        Err(e) => tracing::error!("Failed to add academic paper: {}", e),
    }
}

#[tokio::main]
async fn main() {
    init_logger().expect("Failed to initialize logger");
    let cli = Cli::parse();

    match &cli.subcommand {
        SubCommands::CollectArticles(args) => collect_articles(args).await,
        SubCommands::StartDashboard(args) => start_dashboard(args).await,
        SubCommands::AddAcademicPaper(args) => add_academic_paper(args).await,
    }
}
