use crate::models::academic_paper::AcademicPaperResource;
use anyhow::Result;

pub async fn get_academic_paper(title: &str, url: &str) -> Result<AcademicPaperResource> {
    let mut paper = AcademicPaperResource {
        title: title.to_string(),
        url: url.to_string(),
        text: vec![],
        arxiv_paper: None,
        ss_paper: None,
    };

    tracing::info!("Starting to parse academic paper from URL: {}", url);
    match paper.parse_pdf_url().await {
        Ok(_) => tracing::info!("Successfully parsed PDF URL: {}", url),
        Err(e) => {
            tracing::error!("Failed to parse PDF URL {}: {}", url, e);
            return Err(e);
        }
    }

    tracing::info!("Starting to fetch data from ArXiv for URL: {}", url);
    match paper.fetch_arxiv_paper().await {
        Ok(_) => tracing::info!("Successfully fetched ArXiv data for URL: {}", url),
        Err(e) => {
            tracing::error!("Failed to fetch ArXiv data for URL {}: {}", url, e);
            return Err(e);
        }
    }

    tracing::info!("Starting to fetch data from Semantic Scholar for URL: {}", url);
    match paper.fetch_ss_paper().await {
        Ok(_) => tracing::info!("Successfully fetched Semantic Scholar data for URL: {}", url),
        Err(e) => {
            tracing::error!("Failed to fetch Semantic Scholar data for URL {}: {}", url, e);
            return Err(e);
        }
    }

    Ok(paper)
}
