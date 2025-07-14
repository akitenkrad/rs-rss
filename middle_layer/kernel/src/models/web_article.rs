use chrono::NaiveDate;
use derive_new::new;
use dotenvy::dotenv;
use openai_tools::{
    chat::request::ChatCompletion,
    common::{message::Message, role::Role, structured_output::Schema},
};
use serde::{Deserialize, Serialize};
use shared::{
    common::Status,
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
    pub status: Status,
}

impl WebArticle {
    pub async fn fill_attributes(&mut self) -> AppResult<()> {
        dotenv().ok();
        let model_id = std::env::var("OPENAI_MODEL_ID").expect("OPENAI_MODEL_ID must be set");
        let mut chat = ChatCompletion::new();
        let messages = vec![
            Message::from_string(
                Role::System,
                r#"あなたは「綾瀬 智理（あやせ ちり）」という名のAIです．  
あなたは高度な自然言語処理能力と論理的読解力を備えた，**Webインテリジェンス・アナリストAI**です．  
主にWeb記事の内容を解析・要約し，必要に応じてキーフレーズ抽出，信頼性評価，Q&A形式の情報変換なども行います．

## あなたの背景・技術力
- 科学技術・ビジネス・政策・AI・サイバーセキュリティなど，多様な分野のWeb記事に対応できます．
- 記事の論理構造（主張・根拠・結論）を把握して要約できます．
- 出典や事実ベースの情報を重視し，憶測は避けてください．
- ファクトチェックの補助として，引用元・日付・著者・数値データを正確に抽出できます．

## 出力のスタイル
- 目的に応じて **要点・構造・箇条書き・Q&A** 形式などを柔軟に切り替えてください．
- ユーザが読みやすいように，情報を**段階的・簡潔・網羅的**にまとめてください．
- 内容の要約は常に中立的な立場で行い，主観的な評価は控えてください．
- もし内容の真偽が確認できない場合，「不確実」「出典不明」など明示してください．

## キャラクターとふるまい
- 落ち着いていて誠実，編集者のような口調．
- 事実と論理にこだわり，信頼できる要約と情報抽出を重視．
- 情報の見逃しを防ぐために慎重に解析し，「これは不要では？」という情報も残してくれる．
- 情報が曖昧なときは，自信を持って断定せず，根拠を明示します．

## あなたの目的
ユーザーが読む価値のある情報だけを，短時間で理解できるように要約・抽出し，  
**Web上の情報の本質をすばやく伝えること**があなたの使命です．
"#,
            ),
            Message::from_string(
                Role::User,
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
{url}

[記事のタイトル]
{title}

[本文のHTML]
{html}
"#,
                    url = self.url,
                    title = self.title,
                    html = self.html,
                ),
            ),
        ];

        let mut json_schema = Schema::chat_json_schema("web_article");
        json_schema.add_property("summary", "string", "記事の要約を日本語で記述してください．");
        json_schema.add_property(
            "is_new_technology_related",
            "boolean",
                "この記事が新しい技術に関するものであるかどうか．新しい技術とは，データサイエンスやAIに関する技術を指し，例えば新しいモデルやライブラリ，AI技術を用いた新しいサービスなどが挙げられる．",
        );
        json_schema.add_property(
            "is_new_product_related",
            "boolean",
                "この記事が商品の紹介に関するものであるかどうか．商品とは，新しい製品やサービスを指し，例えば新しいスマートフォン，新しいソフトウェア，新しいサービスなどが挙げられる．また，商品のレビューも含む．"
        );
        json_schema.add_property(
            "is_new_academic_paper_related",
            "boolean",
            "この記事が新しい論文に関するものであるかどうか．新しい論文とは，新しい研究成果を指し，例えば新しいアルゴリズム，新しいモデル，新しいデータセットなどが挙げられる．"
        );
        json_schema.add_property(
            "is_ai_related",
            "boolean",
            "この記事がAIに関わるものであるかどうか．AIに関わるものとは，人工知能や機械学習，LLMや自然言語処理などの技術を指し，例えばAI技術を用いた新しいサービスの紹介やAIに関わる技術の論文紹介などが挙げられる．"
        );
        json_schema.add_property(
            "is_security_related",
            "boolean",
                "この記事がセキュリティに関わるものであるかどうか．セキュリティに関わるものとは，情報セキュリティやサイバーセキュリティなどの技術を指し，例えば新しいセキュリティ技術の紹介や情報漏えいなどのセキュリティ事故，サイバー攻撃の報告，脆弱性のレポートなどが挙げられる．"
        );
        json_schema.add_property(
            "is_it_related",
            "boolean",
            "この記事がITに関わるものであるかどうか．ITに関わるものとは，情報技術や情報通信技術などの技術を指し，例えば新しいIT技術の紹介やITに関わる論文紹介，IT技術を用いた企業の取組み事例紹介・プレスリリースなどが挙げられる．"
        );

        chat.model_id(model_id)
            .messages(messages)
            .temperature(1.0)
            .json_schema(json_schema);

        let response = chat.chat().await.unwrap();
        match serde_json::from_str::<WebArticleProperty>(
            &response.choices[0].message.content.clone().unwrap().text.unwrap(),
        ) {
            Ok(properties) => {
                self.summary = properties.summary.unwrap_or("NO SUMMARY".to_string());
                self.is_new_technology_related = properties.is_new_technology_related.unwrap_or(false);
                self.is_new_product_related = properties.is_new_product_related.unwrap_or(false);
                self.is_new_academic_paper_related = properties.is_new_academic_paper_related.unwrap_or(false);
                self.is_ai_related = properties.is_ai_related.unwrap_or(false);
                self.is_security_related = properties.is_security_related.unwrap_or(false);
                self.is_it_related = properties.is_it_related.unwrap_or(false);
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

#[derive(Debug, Clone, new)]
pub struct WebArticleListOptions {
    pub limit: i64,
    pub offset: i64,
}
