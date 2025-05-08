use chrono::{DateTime, Local};
use dotenvy::dotenv;
use kernel::models::web_article::{WebArticle, WebSite};
use openai_tools::{json_schema::JsonSchema, Message, OpenAI, ResponseFormat};
use regex::Regex;
use registry::Registry;
use request::{Response, Url};
use serde::{Deserialize, Serialize};
use shared::errors::{AppError, AppResult};
use shared::id::{WebArticleId, WebSiteId};
use std::boxed::Box;

pub type Html = String;
pub type Text = String;
pub type Cookie = String;

pub enum Category {
    Blog,
    Organization,
    Security,
    News,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebArticleProperty {
    pub summary: Option<String>,
    pub is_new_technology_related: Option<bool>,
    pub is_new_product_related: Option<bool>,
    pub is_new_paper_related: Option<bool>,
    pub is_ai_related: Option<bool>,
}

impl Default for WebArticleProperty {
    fn default() -> Self {
        Self {
            summary: Some("".to_string()),
            is_new_technology_related: Some(false),
            is_new_product_related: Some(false),
            is_new_paper_related: Some(false),
            is_ai_related: Some(false),
        }
    }
}

impl WebArticleProperty {
    pub fn to_payload(&self) -> String {
        let payload = format!(
            "NEW_TECH: {}, NEW_PROD: {}, NEW_PAPER: {}, AI: {}",
            if self.is_new_technology_related.unwrap() { "○" } else { "×" },
            if self.is_new_product_related.unwrap() { "○" } else { "×" },
            if self.is_new_paper_related.unwrap() { "○" } else { "×" },
            if self.is_ai_related.unwrap() { "○" } else { "×" }
        );
        return payload;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebArticleResource {
    pub site_name: String,
    pub title: String,
    pub url: String,
    pub description: String,
    pub timestamp: DateTime<Local>,
    pub text: String,
    pub html: String,
    pub property: WebArticleProperty,
}

impl WebArticleResource {
    pub fn new(site: String, title: String, url: String, description: String, timestamp: DateTime<Local>) -> Self {
        Self {
            site_name: site,
            title,
            url,
            description,
            timestamp,
            text: "".to_string(),
            html: "".to_string(),
            property: WebArticleProperty::default(),
        }
    }
}

impl From<WebArticleResource> for WebArticle {
    fn from(article: WebArticleResource) -> Self {
        let WebArticleResource {
            site_name,
            title,
            url,
            description,
            timestamp,
            text,
            html,
            property,
        } = article;
        WebArticle {
            site: WebSite::new(WebSiteId::new(), site_name, String::default()),
            article_id: WebArticleId::new(),
            title,
            url,
            description,
            timestamp: timestamp.date_naive(),
            text,
            html,
            summary: property.summary.unwrap_or_default(),
            is_new_technology_related: property.is_new_technology_related.unwrap_or_default(),
            is_new_product_related: property.is_new_product_related.unwrap_or_default(),
            is_new_paper_related: property.is_new_paper_related.unwrap_or_default(),
            is_ai_related: property.is_ai_related.unwrap_or_default(),
        }
    }
}

#[async_trait::async_trait]
pub trait WebSiteResource {
    fn site_id(&self) -> WebSiteId;
    fn site_name(&self) -> String;
    fn category(&self) -> Category;
    async fn get_articles(&mut self) -> AppResult<Vec<WebArticleResource>>;
    async fn parse_article(&mut self, url: &str) -> AppResult<(Html, Text)>;
    async fn login(&mut self) -> AppResult<Cookie>;
    fn domain(&self) -> String;
    fn trim_text(&self, text: &str) -> String {
        let re = Regex::new(r"\s\s+").unwrap();
        let trimmed_text = re.replace_all(text, "\n").to_string();
        return trimmed_text;
    }
    async fn get_site_id(&self, registry: &Registry, name: &str, url: &str) -> AppResult<WebSiteId> {
        let site = registry.web_site_repository().read_or_create_web_site(name, url).await?;
        Ok(site.site_id)
    }
    fn get_domain(&self, url: &str) -> AppResult<String> {
        return Ok(Url::parse(url)?.domain().unwrap_or_default().to_string());
    }
    async fn request(&self, url: &str, cookie_str: &str) -> AppResult<Response> {
        let url = request::Url::parse(url).unwrap();

        // Set Header
        let mut headers = request::header::HeaderMap::new();
        headers.insert(
            request::header::USER_AGENT,
            format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).parse().unwrap(),
        );

        // Set Cookie
        let cookies = std::sync::Arc::new(request::cookie::Jar::default());
        cookies.add_cookie_str(cookie_str, &url);

        let client_builder = request::ClientBuilder::new();
        let client = client_builder
            .default_headers(headers)
            .cookie_provider(cookies)
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .unwrap();

        let response = match client.get(url).send().await {
            Ok(response) => response,
            Err(e) => {
                return Err(AppError::RequestError(e));
            }
        };
        return Ok(response);
    }

    fn complete_article_properties(&self, title: &str, article: &str) -> AppResult<WebArticleProperty> {
        dotenv().ok();
        let model_id = std::env::var("OPENAI_MODEL_ID").expect("OPENAI_MODEL_ID must be set");
        let mut openai = OpenAI::new();
        let messages = vec![
            Message::new(
                String::from("system"),
                String::from("あなたは自然言語に関する世界トップレベルの研究者です．"),
            ),
            Message::new(
                String::from("user"),
                format!(
                    r#"与えられたWeb記事のタイトルと本文のHTMLから次の情報を抽出してください．
- この記事の要約
- この記事は新しい技術に関するものかどうか (true or false)
- この記事が商品の紹介かどうか (true or false)
- この記事は新しい論文の紹介に関わるものかどうか (true or false)
- この記事はデータ分析やLLMなどAIに関わるものかどうか (true or false)

[タイトル]
{title}

[本文のHTML]
{article}"#,
                    title = title,
                    article = article
                ),
            ),
        ];

        let mut json_schema = JsonSchema::new(String::from("web_article"));
        json_schema.add_property(
            String::from("summary_of_article"),
            String::from("string"),
            Option::from(r#"記事の要約を記述してください．"#.to_string()),
        );
        json_schema.add_property(
            String::from("is_new_technology_related"), 
            String::from("boolean"),
            Option::from(r#"この記事が新しい技術に関するものであるかどうか．新しい技術とは，データサイエンスやAIに関する技術を指し，例えば新しいモデル，新しいライブラリ，AI技術を用いた新しいサービスなどが挙げられる．"#.to_string())
        );
        json_schema.add_property(
            "is_new_product_introduction".to_string(),
            "boolean".to_string(),
            Option::from(r#"この記事が商品の紹介に関するものであるかどうか．商品とは，新しい製品やサービスを指し，例えば新しいスマートフォン，新しいソフトウェア，新しいサービスなどが挙げられる．また，商品のレビューも含む．"#.to_string())
        );
        json_schema.add_property(
            "is_new_paper".to_string(),
            "boolean".to_string(),
            Option::from(r#"この記事が新しい論文に関するものであるかどうか．新しい論文とは，新しい研究成果を指し，例えば新しいアルゴリズム，新しいモデル，新しいデータセットなどが挙げられる．"#.to_string())
        );
        json_schema.add_property(
            "is_ai_related".to_string(),
            "boolean".to_string(),
            Option::from(r#"この記事がAIに関わるものであるかどうか．AIに関わるものとは，人工知能や機械学習，LLMや自然言語処理などの技術を指し，例えばAI技術を用いた新しいサービスの紹介やAIに関わる技術の論文紹介などが挙げられる．"#.to_string())
        );

        let response_format = ResponseFormat::new("json_schema".to_string(), json_schema);
        openai
            .model_id(model_id)
            .messages(messages)
            .temperature(1.0)
            .response_format(response_format);

        let response = openai.chat().unwrap();
        match serde_json::from_str::<WebArticleProperty>(&response.choices[0].message.content) {
            Ok(properties) => return Ok(properties),
            Err(e) => {
                tracing::warn!("Failed to parse WebArticleProperty: {}", e);
                return Err(AppError::JsonParseError(e));
            }
        }
    }
}

impl From<Box<dyn WebSiteResource>> for WebSite {
    fn from(site: Box<dyn WebSiteResource>) -> Self {
        Self {
            site_id: site.site_id(),
            name: site.site_name(),
            url: site.domain(),
        }
    }
}
