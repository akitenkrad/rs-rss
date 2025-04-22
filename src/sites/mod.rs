use anyhow::{Error, Result};
use chrono::{DateTime, Local};
use dotenvy::dotenv;
use openai_tools::{json_schema::JsonSchema, Message, OpenAI, ResponseFormat};
use regex::Regex;
use request::{Response, Url};
use serde::{Deserialize, Serialize};

type Html = String;
type Text = String;
type Cookie = String;

pub enum Category {
    Blog,
    Organization,
    Security,
    News,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebArticleProperty {
    pub summary_of_article: Option<String>,
    pub is_new_technology_related: Option<bool>,
    pub is_new_product_introduction: Option<bool>,
    pub is_new_paper: Option<bool>,
    pub is_ai_related: Option<bool>,
}

impl Default for WebArticleProperty {
    fn default() -> Self {
        Self {
            summary_of_article: Some("".to_string()),
            is_new_technology_related: Some(false),
            is_new_product_introduction: Some(false),
            is_new_paper: Some(false),
            is_ai_related: Some(false),
        }
    }
}

impl WebArticleProperty {
    pub fn to_payload(&self) -> String {
        let payload = format!(
            "NEW_TECH: {}, NEW_PROD: {}, NEW_PAPER: {}, AI: {}",
            if self.is_new_technology_related.unwrap_or_default() {
                "○"
            } else {
                "×"
            },
            if self.is_new_product_introduction.unwrap_or_default() {
                "○"
            } else {
                "×"
            },
            if self.is_new_paper.unwrap_or_default() { "○" } else { "×" },
            if self.is_ai_related.unwrap_or_default() { "○" } else { "×" }
        );
        return payload;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebArticle {
    pub site: String,
    pub title: String,
    pub url: String,
    pub description: String,
    pub timestamp: DateTime<Local>,
    pub html: Html,
    pub property: WebArticleProperty,
}

impl WebArticle {
    pub fn new(site: String, title: String, url: String, description: String, timestamp: DateTime<Local>) -> Self {
        Self {
            site,
            title,
            url,
            description,
            timestamp,
            html: "".to_string(),
            property: WebArticleProperty::default(),
        }
    }
}

#[async_trait::async_trait]
pub trait Site {
    fn name(&self) -> String;
    fn category(&self) -> Category;
    async fn get_articles(&mut self) -> Result<Vec<WebArticle>>;
    async fn parse_article(&mut self, url: &str) -> Result<(Html, Text)>;
    async fn login(&mut self) -> Result<Cookie>;
    fn domain(&self) -> String;
    fn trim_text(&self, text: &str) -> String {
        let re = Regex::new(r"\s\s+").unwrap();
        let trimmed_text = re.replace_all(text, "\n").to_string();
        return trimmed_text;
    }
    fn get_domain(&self, url: &str) -> Result<String> {
        return Ok(Url::parse(url)?.domain().unwrap_or_default().to_string());
    }
    async fn request(&self, url: &str, cookie_str: &str) -> Result<Response> {
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

        let response = client.get(url).send().await.unwrap();
        if response.status().is_success() {
            return Ok(response);
        } else {
            return Err(Error::msg(format!("Failed to fetch: {}", response.status())));
        }
    }

    fn complete_article_properties(&self, title: &str, article: &str) -> Result<WebArticleProperty> {
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
                println!("Error: {}", e);
                return Err(Error::msg(format!("Failed to get WebArticlePreperty: {}", e)));
            }
        }
    }
}

pub mod ai_db;
pub mod ai_it_now;
pub mod ai_news;
pub mod ai_scholar;
pub mod aismiley;
pub mod aizine;
pub mod aws_security_blog;
pub mod business_insider_science;
pub mod business_insider_technology;
pub mod canon_malware_center;
pub mod codezine;
pub mod cookpad_techblog;
pub mod crowdstrike_blog;
pub mod cyberagent_techblog;
pub mod cybozu_blog;
pub mod dena_engineering_blog;
pub mod gigazine;
pub mod github_developers_blog;
pub mod gizmodo;
pub mod google_developers_blog;
pub mod gree_techblog;
pub mod gunosy_techblog;
pub mod ipa_security_center;
pub mod itmedia_at_it;
pub mod itmedia_enterprise;
pub mod itmedia_executive;
pub mod itmedia_general;
pub mod itmedia_marketing;
pub mod jpcert;
pub mod line_techblog;
pub mod macafee_security_news;
pub mod medium;
pub mod mercari_engineering_blog;
pub mod mit_ai;
pub mod mit_research;
pub mod moneyforward_developers_blog;
pub mod motex;
pub mod nikkei_xtech;
pub mod qiita_blog;
pub mod retrieva_techblog;
pub mod rust_blog;
pub mod sakura_internet_techblog;
pub mod sansan;
pub mod security_next;
pub mod sophos_news;
pub mod stockmark_news;
pub mod stockmark_techblog;
pub mod supership;
pub mod tech_crunch;
pub mod tokyo_univ_engineering;
pub mod trend_micro_security_advisories;
pub mod trend_micro_security_news;
pub mod yahoo_japan_techblog;
pub mod yahoo_news_it;
pub mod yahoo_news_science;
pub mod zen_mu_tech;
pub mod zenn_topic;
pub mod zenn_trend;
