use std::vec;

use anyhow::Result;
use arxiv_tools::{ArXiv, Paper as ArxivPaper, QueryParams as ArXivQueryParams};
use chrono::{DateTime, Local, TimeZone};
use derive_new::new;
use kernel::models::academic_paper::{AcademicPaper, Author, Journal, Status};
use rsrpp::{config::ParserConfig, models::Section as RsrppSection, parser::parse};
use serde::{Deserialize, Serialize};
use shared::{
    id::{AcademicPaperId, AuthorId, JournalId},
    utils::levenshtein_dist,
};
use ss_tools::{
    structs::{AuthorField as SsAuthorField, Paper as SsPaper, PaperField as SsPaperField},
    QueryParams as SsQueryParams, SemanticScholar,
};

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct Section {
    pub title: String,
    pub content: String,
}

fn datetime_from_str(date_str: &str) -> DateTime<Local> {
    if date_str.is_empty() {
        return Local.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap(); // Fallback to a default date
    }
    if let Ok(parsed) = DateTime::parse_from_rfc2822(date_str) {
        return parsed.with_timezone(&Local);
    } else if let Ok(parsed) = DateTime::parse_from_rfc3339(date_str) {
        return parsed.with_timezone(&Local);
    }

    let mut date_str = date_str.to_string();
    if regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap().is_match(&date_str) {
        date_str.push_str(" 00:00:00+0000");
    } else if regex::Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$")
        .unwrap()
        .is_match(&date_str)
    {
        date_str.push_str("+0000");
    } else if !date_str.ends_with('+') && !date_str.ends_with('-') {
        date_str.push_str("+0000");
    } else if regex::Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\+\d{4}$")
        .unwrap()
        .is_match(&date_str)
    {
        // Already in the correct format
    } else {
        eprintln!("WARNING: Date string does not match expected formats: {}", date_str);
        return Local.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap(); // Fallback to a default date
    }
    match DateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S%z") {
        Ok(date) => date.with_timezone(&Local),
        Err(e) => {
            eprintln!("WARNING: Failed to parse date string: {}. Error: {}", date_str, e);
            Local.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap() // Fallback to a default date
        }
    }
}

fn get_journal_name(arxiv_paper: &Option<ArxivPaper>, ss_paper: &Option<SsPaper>) -> String {
    let arxiv_journal = if arxiv_paper.is_some() {
        "arXiv".to_string()
    } else {
        "Unknown Journal".to_string()
    };

    if ss_paper.is_some() {
        ss_paper
            .as_ref()
            .and_then(|p| p.journal.clone())
            .map_or(arxiv_journal.clone(), |journal| {
                if journal.name.clone().unwrap_or_default().is_empty() {
                    arxiv_journal.clone()
                } else {
                    match journal.name.clone() {
                        Some(name) if !name.is_empty() => name,
                        _ => arxiv_journal.clone(),
                    }
                }
            })
    } else {
        arxiv_journal
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct AcademicPaperResource {
    pub title: String,
    pub url: String,
    pub text: Vec<Section>,
    pub arxiv_paper: Option<ArxivPaper>,
    pub ss_paper: Option<SsPaper>,
}

impl AcademicPaperResource {
    /// Fills the `AcademicPaperResource` with data from a PDF URL.
    /// target field: `text`
    pub async fn parse_pdf_url(&mut self) -> Result<AcademicPaperResource> {
        let mut config = ParserConfig::new();
        match parse(&self.url, &mut config, true).await {
            Ok(pages) => {
                let sections = RsrppSection::from_pages(&pages);
                self.text = sections
                    .into_iter()
                    .map(|section| Section {
                        title: section.title,
                        content: section.contents.join("\n"),
                    })
                    .filter(|section| {
                        !section.title.to_lowercase().contains("reference") && !section.content.is_empty()
                    })
                    .collect();
                tracing::info!("Successfully parsed PDF at URL: {}", self.url);
                for (i, section) in self.text.iter().enumerate() {
                    tracing::info!("Section {}: {}", i + 1, section.title);
                }
                Ok(self.clone())
            }
            Err(e) => {
                self.text = Vec::new();
                tracing::warn!("Failed to parse PDF at URL: {}. Error: {}", self.url, e);
                Ok(self.clone())
            }
        }
    }
    /// Fetches the paper from ArXiv using the title.
    /// target field: `arxiv_paper`, `title`
    pub async fn fetch_arxiv_paper(&mut self) -> Result<AcademicPaperResource> {
        assert!(!self.title.is_empty(), "Title must not be empty");
        let mut arxiv = ArXiv::from_args(ArXivQueryParams::title(&self.title.clone()));
        let found_papers = arxiv.query().await;
        if found_papers.is_empty() {
            tracing::warn!("No papers found for title: {}", self.title);
            return Ok(self.clone());
        }

        let paper = found_papers.into_iter().max_by(|a, b| {
            let dist_a = levenshtein_dist(&self.title, &a.title);
            let dist_b = levenshtein_dist(&self.title, &b.title);
            dist_a.cmp(&dist_b)
        });
        self.arxiv_paper = paper;
        if self.title.is_empty() {
            self.title = self.arxiv_paper.as_ref().map_or(String::new(), |p| p.title.clone());
        }
        tracing::info!("Successfully fetched ArXiv paper for title: {}", self.title);
        Ok(self.clone())
    }
    /// Fetches the paper from Semantic Scholar using the title.
    /// target field: `ss_paper`, `title`
    pub async fn fetch_ss_paper(&mut self) -> Result<AcademicPaperResource> {
        let mut ss = SemanticScholar::new();
        let mut query_params = SsQueryParams::default();
        assert!(!self.title.is_empty(), "Title must not be empty");
        query_params.query_text(&self.title.clone());
        query_params.fields(vec![
            SsPaperField::PaperId,
            SsPaperField::Title,
            SsPaperField::Abstract,
            SsPaperField::Authors(vec![
                SsAuthorField::Name,
                SsAuthorField::AuthorId,
                SsAuthorField::HIndex,
            ]),
            SsPaperField::Url,
            SsPaperField::PublicationDate,
            SsPaperField::Journal,
            SsPaperField::CitationCount,
            SsPaperField::ReferenceCount,
            SsPaperField::InfluentialCitationCount,
        ]);
        match ss.query_a_paper_by_title(query_params, 10, 10).await {
            Ok(paper) => {
                self.ss_paper = Some(paper);
                if self.title.is_empty() {
                    self.title = self.ss_paper.as_ref().and_then(|p| p.title.clone()).unwrap_or_default();
                }
                tracing::info!("Successfully fetched Semantic Scholar paper for title: {}", self.title);
            }
            Err(e) => {
                tracing::warn!(
                    "Failed to fetch Semantic Scholar paper for title: {}. Error: {}",
                    self.title,
                    e
                );
                self.ss_paper = None;
            }
        }
        Ok(self.clone())
    }
}

impl From<AcademicPaperResource> for AcademicPaper {
    fn from(resource: AcademicPaperResource) -> Self {
        let AcademicPaperResource {
            title,
            url,
            text,
            arxiv_paper,
            ss_paper,
        } = resource;
        AcademicPaper {
            paper_id: AcademicPaperId::new(),
            ss_id: ss_paper
                .as_ref()
                .map_or(String::new(), |p| p.paper_id.clone().unwrap_or(String::new())),
            arxiv_id: arxiv_paper.as_ref().map_or(String::new(), |p| p.id.clone()),
            journal: Journal::new(JournalId::new(), get_journal_name(&arxiv_paper, &ss_paper)),
            authors: ss_paper.as_ref().map_or(vec![], |p| match p.authors.as_ref() {
                None => vec![],
                Some(authors) => authors
                    .iter()
                    .map(|a| {
                        Author::new(
                            AuthorId::new(),
                            a.author_id.clone().unwrap_or(String::new()),
                            a.name.clone().unwrap_or(String::new()),
                            a.hindex.unwrap_or(0) as i32,
                        )
                    })
                    .collect(),
            }),
            tasks: vec![],
            title,
            abstract_text: match ss_paper.as_ref() {
                Some(p) => p
                    .abstract_text
                    .clone()
                    .unwrap_or_else(|| arxiv_paper.as_ref().map_or(String::new(), |p| p.abstract_text.clone())),
                None => String::new(),
            },
            abstract_text_ja: String::new(),
            text: text
                .into_iter()
                .map(|s| format!("# {}\n\n{}", s.title, s.content))
                .collect::<Vec<String>>()
                .join("\n\n"),
            url,
            doi: match arxiv_paper.as_ref() {
                Some(p) => p.doi.clone(),
                None => String::new(),
            },
            published_date: datetime_from_str(
                &ss_paper
                    .as_ref()
                    .and_then(|p| p.publication_date.clone())
                    .unwrap_or_default(),
            ),
            created_at: Local::now(),
            updated_at: Local::now(),
            primary_category: match arxiv_paper.as_ref() {
                Some(p) => p.primary_category.clone(),
                None => String::default(),
            },
            citations_count: ss_paper.as_ref().and_then(|p| p.citation_count).unwrap_or(0) as i32,
            references_count: ss_paper.as_ref().and_then(|p| p.reference_count).unwrap_or(0) as i32,
            influential_citation_count: ss_paper
                .as_ref()
                .and_then(|p| p.influential_citation_count)
                .unwrap_or(0) as i32,
            bibtex: String::new(),
            summary: String::new(),
            background_and_purpose: String::new(),
            methodology: String::new(),
            dataset: String::new(),
            results: String::new(),
            advantages_limitations_and_future_work: String::new(),
            status: Status::New,
        }
    }
}
