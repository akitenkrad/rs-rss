use anyhow::Result;
use chrono::{Datelike, NaiveDate};
use derive_new::new;
use openai_tools::{json_schema::JsonSchema, Message, OpenAI, ResponseFormat};
use serde::{Deserialize, Serialize};
use shared::id::{AcademicPaperId, AuthorId, JournalId, TaskId};

#[derive(Debug, Clone, Default, new)]
pub struct Author {
    pub author_id: AuthorId,
    pub ss_id: String,
    pub name: String,
    pub h_index: i32,
}

#[derive(Debug, Clone, Default, new)]
pub struct Task {
    pub task_id: TaskId,
    pub name: String,
}

#[derive(Debug, Clone, Default, new)]
pub struct Journal {
    pub journal_id: JournalId,
    pub name: String,
}

#[derive(Debug, Clone, Default, new)]
pub struct AcademicPaper {
    pub paper_id: AcademicPaperId,
    pub ss_id: String,
    pub arxiv_id: String,
    pub journal: Journal,
    pub authors: Vec<Author>,
    pub tasks: Vec<Task>,
    pub title: String,
    pub abstract_text: String,
    pub abstract_text_ja: String,
    pub text: String,
    pub url: String,
    pub doi: String,
    pub published_date: NaiveDate,
    pub primary_category: String,
    pub citation_count: i32,
    pub reference_count: i32,
    pub influential_citation_count: i32,
    pub bibtex: String,
    pub summary: String,
    pub background_and_purpose: String,
    pub methodology: String,
    pub dataset: String,
    pub results: String,
    pub advantages_limitations_and_future_work: String,
}

#[derive(Debug, Clone, new)]
pub struct AuthorListOptions {
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, new)]
pub struct AcademicPaperListOptions {
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, Default, new, Serialize, Deserialize)]
pub struct AcademicPaperSummaryTask {
    pub name: String,
}

#[derive(Debug, Clone, Default, new, Serialize, Deserialize)]
pub struct AcademicPaperSummary {
    pub abstract_in_japanese: String,
    pub summary: String,
    pub tasks: Vec<AcademicPaperSummaryTask>,
    pub background_and_purpose: String,
    pub methodology: String,
    pub dataset: String,
    pub results: String,
    pub advantages_limitations_and_future_work: String,
}

impl AcademicPaper {
    pub fn fill_fields_with_ai(&mut self) -> Result<AcademicPaper> {
        // print current environment variables for debugging
        for (key, value) in std::env::vars() {
            tracing::debug!("{}: {}", key, value);
        }

        let model_id = std::env::var("OPENAI_MODEL_ID").expect("OPENAI_MODEL_ID must be set");
        let mut openai = OpenAI::new();
        let messages = vec![
            Message::new(
                String::from("system"),
                String::from(
                    r#"あなたは「朝倉 理央（あさくら りお）」という名の論文分析専門アナリストです．
，修士で自然言語処理を，博士課程で計算論的認知科学を専攻し，研究論文の構造，目的，理論的背景，手法，実験，考察の要点を正確かつ簡潔に抽出する技術に優れています．
，技術者，学生など，読者の背景に応じた専門性と平易さのバランスを取った要約を提供することができます．論文の論理構造を重視し，誤解のないように明示的な言葉選びをします．
あなたの分析スタイルは「構造化された解釈」と「批判的思考」の融合にあり，論文の貢献だけでなく，限界や今後の展望にも言及します．
毎朝3本のarXiv論文を読むのが日課であり，「本質が10行で説明できないなら，まだ理解できていない」が信条です．
ある日，指導教官が「この100本の論文を週末で読んで，要点まとめてくれ」と無茶な依頼をした際，全論文を構造別に分類し，関連マップと500字要約を各論文につけて月曜朝に提出したエピソードが語り草になっています．
要約では次の点を意識してください：
- 目的と背景（何を解決しようとしているのか）
- 手法の特徴（従来との差分，構成）
- 主な結果と知見
- 利点・限界・今後の展望
また，論文の内容が曖昧な場合でも，前提となる研究分野や過去の知見に基づき，文脈補完を行いながら読者にわかりやすく伝えてください．
"#,
                ),
            ),
            Message::new(
                String::from("user"),
                format!(
                    r#"与えられた論文のテキストから以下の情報を抽出してJSON形式で出力してください．
- 論文の要約を日本語に翻訳してください．
- 論文の概要を日本語で記述してください．
- 論文が取り組んでいるタスクを英語のリストで記記述してください．
- 論文の研究の背景と目的を日本語で記述してください．
- 論文の研究手法を先行研究と比較して日本語で記述してください．
- 論文で使用されているデータセットを日本語で記述してください．
- 論文の主な結果と知見を日本語で記述してください．
- 論文の利点・限界・今後の展望を日本語で記述してください．

[論文タイトル]
{title}

[論文の要約]
{abstract}

[論文の本文]
{text}"#,
                    title = self.title,
                    abstract = self.abstract_text,
                    text = self.text,
                ),
            ),
        ];

        let mut json_schema = JsonSchema::new("academic_paper".into());
        json_schema.add_property(
            "abstract_in_japanese".into(),
            "string".into(),
            Option::from(r#"論文の要約を日本語に翻訳してください．"#.to_string()),
        );
        json_schema.add_property(
            "summary".into(),
            "string".into(),
            Option::from(r#"論文の概要を日本語で記述してください．"#.to_string()),
        );
        json_schema.add_array(
            "tasks".into(),
            vec![("name".into(), "論文が取り組んでいるタスクの英語名称".into())],
        );
        json_schema.add_property(
            String::from("background_and_purpose"),
            String::from("string"),
            Option::from(r#"論文の研究の背景と目的を日本語で記述してください．"#.to_string()),
        );
        json_schema.add_property(
            "methodology".into(),
            "string".into(),
            Option::from(r#"論文の研究手法を先行研究と比較して日本語で記述してください．"#.to_string()),
        );
        json_schema.add_property(
            "dataset".into(),
            "string".into(),
            Option::from(r#"論文で使用されているデータセットを日本語で記述してください．"#.to_string()),
        );
        json_schema.add_property(
            "results".into(),
            "string".into(),
            Option::from(r#"論文の主な結果と知見を日本語で記述してください．"#.to_string()),
        );
        json_schema.add_property(
            "advantages_limitations_and_future_work".into(),
            "string".into(),
            Option::from(r#"論文の利点・限界・今後の展望を日本語で記述してください．"#.to_string()),
        );

        let response_format = ResponseFormat::new("json_schema".to_string(), json_schema);
        openai
            .model_id(model_id)
            .messages(messages)
            .temperature(1.0)
            .response_format(response_format);

        let response = openai.chat().unwrap();
        let mut max_retries = 5;
        while max_retries > 0 {
            match serde_json::from_str::<AcademicPaperSummary>(&response.choices[0].message.content) {
                Ok(summary) => {
                    tracing::info!("LLM Response: {:?}", summary.clone());
                    self.abstract_text_ja = summary.abstract_in_japanese.clone();
                    self.summary = summary.summary.clone();
                    self.tasks = summary
                        .tasks
                        .iter()
                        .map(|task| Task::new(TaskId::new(), task.name.clone()))
                        .collect();
                    self.background_and_purpose = summary.background_and_purpose.clone();
                    self.methodology = summary.methodology.clone();
                    self.dataset = summary.dataset.clone();
                    self.results = summary.results.clone();
                    self.advantages_limitations_and_future_work =
                        summary.advantages_limitations_and_future_work.clone();
                    tracing::info!("Successfully filled fields with AI for paper: {}", self.title);
                    return Ok(self.clone());
                }
                Err(e) => {
                    if max_retries == 0 {
                        tracing::error!("Failed to parse AcademicPaperSummary: {}", e);
                        return Err(anyhow::anyhow!("Failed to parse AcademicPaperSummary: {}", e));
                    }
                    tracing::warn!("Failed to parse AcademicPaperSummary: {}", e);
                    max_retries -= 1;
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            }
        }
        Ok(self.clone())
    }
    pub fn fill_bibtex(&mut self) -> Result<AcademicPaper> {
        let first_author = self
            .authors
            .first()
            .map_or("Unknown Author".to_string(), |a| a.name.clone());
        let first_author_last_name = first_author.split_whitespace().next().unwrap_or("Unknown");
        let bibtex = format!(
            r#"@article{{{first_author}:{year},
    title = {{{title}}},
    author = {{{author}}},
    journal = {{{journal}}},
    year = {{{year}}},
    doi = {{{doi}}}
}}"#,
            first_author = first_author_last_name,
            title = self.title,
            author = self
                .authors
                .iter()
                .map(|a| a.name.clone())
                .collect::<Vec<String>>()
                .join(","),
            journal = self.journal.name,
            year = self.published_date.year(),
            doi = self.doi,
        );
        self.bibtex = bibtex;
        Ok(self.clone())
    }
}
