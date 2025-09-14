use crate::models::academic_paper::{
    AcademicPaperCreateRequest, AcademicPaperIdQuery, AcademicPaperListQuery, AcademicPaperListResponse,
    AcademicPaperResponse,
};
use academic_paper_crawler::repository::academic_papers::get_academic_paper;
use axum::{
    extract::{Json, Query, State},
    response::sse::{Event, KeepAlive, Sse},
};
use garde::Validate;
use kernel::models::academic_paper::AcademicPaper;
use registry::AppRegistry;
use shared::errors::{AppError, AppResult};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};

pub async fn select_paginated_academic_papers(
    State(registry): State<AppRegistry>,
    Query(query): Query<AcademicPaperListQuery>,
) -> AppResult<Json<AcademicPaperListResponse>> {
    query.validate()?;

    let mut tx = registry.db().inner_ref().begin().await?;
    let result = registry
        .academic_paper_repository()
        .select_paginated_academic_papers(&mut tx, query.into())
        .await
        .map(AcademicPaperListResponse::from)
        .map(Json);
    tx.commit().await?;
    result
}

pub async fn select_academic_papers_by_id(
    State(registry): State<AppRegistry>,
    Query(query): Query<AcademicPaperIdQuery>,
) -> AppResult<Json<AcademicPaperResponse>> {
    let mut tx = registry.db().inner_ref().begin().await?;
    let result = registry
        .academic_paper_repository()
        .select_academic_paper_by_id(&mut tx, query.paper_id.as_str())
        .await
        .map(AcademicPaperResponse::from)
        .map(Json);
    tx.commit().await?;
    result
}

pub async fn add_academic_paper(
    State(registry): State<AppRegistry>,
    Json(query): Json<AcademicPaperCreateRequest>,
) -> AppResult<Json<AcademicPaperResponse>> {
    query.validate()?;

    // This function is a placeholder for adding an academic paper
    tracing::info!("Adding academic paper...");

    // crawler
    let paper_rsc = get_academic_paper(&query.title, &query.pdf_url).await.map_err(|e| {
        tracing::error!("Failed to get academic paper by URL: {}", e);
        AppError::from(e)
    })?;

    // kernel
    let mut paper = AcademicPaper::from(paper_rsc);
    paper.fill_fields_with_ai().await.map_err(|e| {
        tracing::error!("Failed to fill fields with AI: {}", e);
        AppError::from(e)
    })?;
    paper.fill_bibtex().map_err(|e| {
        tracing::error!("Failed to fill BibTeX: {}", e);
        AppError::from(e)
    })?;

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
    tracing::info!("Reference Count: {}", paper.references_count);
    tracing::info!("Citation Count: {}", paper.citations_count);
    tracing::info!("Influential Citation Count: {}", paper.influential_citation_count);
    tracing::info!("Primary Category: {}", paper.primary_category);

    let mut tx = registry.db().inner_ref().begin().await?;
    let _ = registry
        .academic_paper_repository()
        .create_academic_paper(&mut tx, paper.clone())
        .await?;
    tx.commit().await?;

    Ok(Json(AcademicPaperResponse::from(paper)))
}

pub async fn add_academic_paper_with_sse(
    State(registry): State<AppRegistry>,
    Query(query): Query<AcademicPaperCreateRequest>,
) -> Sse<impl futures_core::Stream<Item = AppResult<Event>>> {
    let (stx, srx) = mpsc::channel::<(usize, String, Option<AcademicPaper>)>(1);

    tokio::spawn(async move {
        // This function is a placeholder for adding an academic paper
        tracing::info!("Adding academic paper...");

        // Send initial status
        if stx
            .send((10, format!("Starting academic paper addition: {}", query.title), None))
            .await
            .is_err()
        {
            tracing::warn!("Client disconnected, stopping status updates");
            return Ok::<(), AppError>(());
        }

        // crawler
        let paper_rsc = match get_academic_paper(&query.title, &query.pdf_url).await {
            Ok(paper) => {
                tracing::info!("Successfully retrieved academic paper");
                if stx
                    .send((
                        20,
                        "Successfully retrieved academic paper from arXiv and Semantic Scholar".into(),
                        Some(paper.clone().into()),
                    ))
                    .await
                    .is_err()
                {
                    tracing::warn!("Client disconnected, stopping status updates");
                    return Ok::<(), AppError>(());
                }
                paper
            }
            Err(err) => {
                tracing::error!("Failed to get academic paper: {}", err);
                return Err(AppError::from(err));
            }
        };

        // kernel
        let mut paper = AcademicPaper::from(paper_rsc);
        match paper.fill_fields_with_ai().await {
            Ok(_) => {
                tracing::info!("Successfully filled fields with AI");
                if stx
                    .send((80, "Successfully filled fields with AI".into(), Some(paper.clone())))
                    .await
                    .is_err()
                {
                    tracing::warn!("Client disconnected, stopping status updates");
                    return Ok::<(), AppError>(());
                }
            }
            Err(err) => {
                tracing::error!("Failed to fill fields with AI: {}", err);
                return Err(AppError::from(err));
            }
        }
        match paper.fill_bibtex() {
            Ok(_) => {
                tracing::info!("Successfully filled BibTeX");
                if stx
                    .send((90, "Successfully filled BibTeX".into(), Some(paper.clone())))
                    .await
                    .is_err()
                {
                    tracing::warn!("Client disconnected, stopping status updates");
                    return Ok::<(), AppError>(());
                }
            }
            Err(err) => {
                tracing::error!("Failed to fill BibTeX: {}", err);
                return Err(AppError::from(err));
            }
        }

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
        tracing::info!("Reference Count: {}", paper.references_count);
        tracing::info!("Citation Count: {}", paper.citations_count);
        tracing::info!("Influential Citation Count: {}", paper.influential_citation_count);
        tracing::info!("Primary Category: {}", paper.primary_category);
        tracing::info!("Authors: {:?}", paper.authors);
        tracing::info!("Journal: {:?}", paper.journal);
        tracing::info!("Tasks: {:?}", paper.tasks);

        let mut tx = registry.db().inner_ref().begin().await?;
        let _ = registry
            .academic_paper_repository()
            .create_academic_paper(&mut tx, paper.clone())
            .await?;
        tx.commit().await?;

        // Final status update - if this fails, it means client disconnected, but task is complete
        if stx
            .send((
                100,
                "Successfully added academic paper to the database".into(),
                Some(paper.clone()),
            ))
            .await
            .is_err()
        {
            tracing::info!("Client disconnected after paper was successfully added to database");
        }

        Ok::<(), AppError>(())
    });

    let stream = ReceiverStream::new(srx).map(|(progress, message, paper)| {
        let paper = match paper {
            Some(p) => AcademicPaperResponse::from(p),
            None => AcademicPaperResponse::from(AcademicPaper::default()),
        };
        let json = serde_json::json!({ "progress": progress, "message": message, "paper": paper });
        Ok::<Event, AppError>(
            Event::default()
                .event("message")
                .id(paper.paper_id.to_string())
                .data(json.to_string()),
        )
    });

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(std::time::Duration::from_secs(15))
            .text("keep-alive"),
    )
}
