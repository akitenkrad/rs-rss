use chrono::NaiveDate;
use derive_new::new;
use dotenvy::dotenv;
use openai_tools::{json_schema::JsonSchema, Message, OpenAI, ResponseFormat};
use serde::{Deserialize, Serialize};
use shared::{
    errors::{AppError, AppResult},
    id::{WebArticleId, WebSiteId},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebArticleProperty {
    pub summary: Option<String>,
    pub is_new_technology_related: Option<bool>,
    pub is_new_product_related: Option<bool>,
    pub is_new_academic_paper_related: Option<bool>,
    pub is_ai_related: Option<bool>,
    pub is_security_related: Option<bool>,
    pub is_it_related: Option<bool>,
}

impl Default for WebArticleProperty {
    fn default() -> Self {
        Self {
            summary: Some("".to_string()),
            is_new_technology_related: Some(false),
            is_new_product_related: Some(false),
            is_new_academic_paper_related: Some(false),
            is_ai_related: Some(false),
            is_security_related: Some(false),
            is_it_related: Some(false),
        }
    }
}

#[derive(Debug, Clone, new, Default)]
pub struct WebSite {
    pub site_id: WebSiteId,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, new, Default)]
pub struct WebArticle {
    pub site: WebSite,
    pub article_id: WebArticleId,
    pub title: String,
    pub description: String,
    pub url: String,
    pub text: String,
    pub html: String,
    pub timestamp: NaiveDate,
    pub summary: String,
    pub is_new_technology_related: bool,
    pub is_new_product_related: bool,
    pub is_new_academic_paper_related: bool,
    pub is_ai_related: bool,
    pub is_security_related: bool,
    pub is_it_related: bool,
}

impl WebArticle {
    pub async fn fill_attributes(&mut self) -> AppResult<()> {
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
- この記事の要約: summary (string)
- この記事は新しい技術に関するものかどうか: is_new_technology_related (true or false)
- この記事が商品の紹介かどうか: is_new_product_related (true or false)
- この記事は新しい論文の紹介に関わるものかどうか: is_new_academic_paper_related (true or false)
- この記事はデータ分析やLLMなどAIに関わるものかどうか: is_ai_related (true or false)
- この記事はセキュリティに関わるものかどうか: is_security_related (true or false)
- この記事はITに関わるものかどうか: is_it_related (true or false)

[記事のURL]

[タイトル]
{title}

[本文のHTML]
{html}"#,
                    title = self.title,
                    html = self.html,
                ),
            ),
        ];

        let mut json_schema = JsonSchema::new(String::from("web_article"));
        json_schema.add_property(
            String::from("summary"),
            String::from("string"),
            Option::from(r#"記事の要約を日本語で記述してください．"#.to_string()),
        );
        json_schema.add_property(
            String::from("is_new_technology_related"),
            String::from("boolean"),
            Option::from(
                r#"この記事が新しい技術に関するものであるかどうか．
新しい技術とは，データサイエンスやAIに関する技術を指し，例えば新しいモデルやライブラリ，AI技術を用いた新しいサービスなどが挙げられる．"#
                    .to_string(),
            ),
        );
        json_schema.add_property(
            "is_new_product_related".to_string(),
            "boolean".to_string(),
            Option::from(
                r#"この記事が商品の紹介に関するものであるかどうか．
商品とは，新しい製品やサービスを指し，例えば新しいスマートフォン，新しいソフトウェア，新しいサービスなどが挙げられる．
また，商品のレビューも含む．"#
                    .to_string(),
            ),
        );
        json_schema.add_property(
            "is_new_academic_paper_related".to_string(),
            "boolean".to_string(),
            Option::from(
                r#"この記事が新しい論文に関するものであるかどうか．
新しい論文とは，新しい研究成果を指し，例えば新しいアルゴリズム，新しいモデル，新しいデータセットなどが挙げられる．"#
                    .to_string(),
            ),
        );
        json_schema.add_property(
            "is_ai_related".to_string(),
            "boolean".to_string(),
            Option::from(
                r#"この記事がAIに関わるものであるかどうか．
AIに関わるものとは，人工知能や機械学習，LLMや自然言語処理などの技術を指し，
例えばAI技術を用いた新しいサービスの紹介やAIに関わる技術の論文紹介などが挙げられる．"#
                    .to_string(),
            ),
        );
        json_schema.add_property(
            "is_security_related".to_string(),
            "boolean".to_string(),
            Option::from(
                r#"この記事がセキュリティに関わるものであるかどうか．
セキュリティに関わるものとは，情報セキュリティやサイバーセキュリティなどの技術を指し，
例えば新しいセキュリティ技術の紹介や情報漏えいなどのセキュリティ事故，サイバー攻撃の報告，脆弱性のレポートなどが挙げられる．"#
                    .to_string(),
            ),
        );
        json_schema.add_property(
            "is_it_related".to_string(),
            "boolean".to_string(),
            Option::from(
                r#"この記事がITに関わるものであるかどうか．
ITに関わるものとは，情報技術や情報通信技術などの技術を指し，
例えば新しいIT技術の紹介やITに関わる論文紹介，IT技術を用いた企業の取組み事例紹介・プレスリリースなどが挙げられる．"#
                    .to_string(),
            ),
        );

        let response_format = ResponseFormat::new("json_schema".to_string(), json_schema);
        openai
            .model_id(model_id)
            .messages(messages)
            .temperature(1.0)
            .response_format(response_format);

        let response = openai.chat().await.unwrap();
        match serde_json::from_str::<WebArticleProperty>(&response.choices[0].message.content) {
            Ok(properties) => {
                self.summary = properties.summary.unwrap_or("NO SUMMARY".to_string());
                self.is_new_technology_related = properties.is_new_technology_related.unwrap_or(false);
                self.is_new_product_related = properties.is_new_product_related.unwrap_or(false);
                self.is_new_academic_paper_related = properties.is_new_academic_paper_related.unwrap_or(false);
                self.is_ai_related = properties.is_ai_related.unwrap_or(false);
                Ok(())
            }
            Err(e) => {
                tracing::warn!("Failed to parse WebArticleProperty: {}", e);
                Err(AppError::JsonParseError(e))
            }
        }
    }
}

#[derive(Debug, Clone, new)]
pub struct WebSiteListOptions {
    pub limit: i64,
    pub offset: i64,
}
