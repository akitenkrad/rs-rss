use std::sync::Arc;

use academic_paper_crawler::repository::academic_papers::get_academic_paper;
use adapter::database::connect_database_with;
use clap::Parser;
use kernel::models::academic_paper::AcademicPaper;
use registry::AppRegistryImpl;
use shared::config::AppConfig;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct AddAcademicPaperArgs {
    #[arg(long)]
    title: String,
    /// The URL of the academic paper to add
    #[arg(long)]
    pdf_url: String,
}

pub async fn add_academic_paper(args: &AddAcademicPaperArgs) {
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
    tracing::info!(
        "Connecting to database...: {}:{}/{}",
        config.database.host,
        config.database.port,
        config.database.database
    );
    let db = connect_database_with(&config.database);
    let registry = Arc::new(AppRegistryImpl::new(db));

    let mut tx = registry
        .db
        .inner_ref()
        .begin()
        .await
        .expect("Failed to begin transaction");
    let academic_paper_repository = registry.academic_paper_repository();
    match academic_paper_repository.create_academic_paper(&mut tx, paper).await {
        Ok(_) => {
            tracing::info!("Successfully added academic paper");
            tx.commit().await.expect("Failed to commit transaction");
        }
        Err(e) => {
            tracing::error!("Failed to add academic paper: {}", e);
            tx.rollback().await.expect("Failed to rollback transaction");
        }
    }
}
