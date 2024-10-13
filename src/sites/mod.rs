use chrono::{DateTime, Local};
use regex::Regex;

pub enum Category {
    Blog,
    Organization,
    Security,
    News,
}

pub trait InfoItem {
    fn title(&self) -> String;
    fn url(&self) -> String;
    fn description(&self) -> String;
    fn timestamp(&self) -> DateTime<Local>;
}

#[derive(Debug)]
pub struct WebArticle {
    pub site: String,
    pub title: String,
    pub url: String,
    pub text: String,
    pub timestamp: DateTime<Local>,
}

impl WebArticle {
    pub fn to_slack_message(&self, no: usize) -> String {
        return format!("No.{COUNT}  -  {NAME}\n{TITLE}\n{DEVIDER}\nKEYWORDS: {SCORE}\n{KEYWORDS}\nURL: {LINK}\nPUBLISHED: {AT}",
            COUNT=no, 
            NAME=self.site,
            TITLE=self.title,
            DEVIDER="-".repeat(50),
            SCORE=0,
            KEYWORDS="",
            LINK=self.url,
            AT=self.timestamp().format("%Y-%m-%d").to_string());
    }
}
impl InfoItem for WebArticle {
    fn title(&self) -> String {
        return self.title.clone();
    }

    fn url(&self) -> String {
        return self.url.clone();
    }

    fn description(&self) -> String {
        return self.text.clone();
    }

    fn timestamp(&self) -> DateTime<Local> {
        return self.timestamp.clone();
    }
}

#[async_trait::async_trait]
pub trait Site {
    fn name(&self) -> String;
    fn category(&self) -> Category;
    async fn get_articles(&self) -> Result<Vec<WebArticle>, String>;
    async fn get_article_text(&self, url: &String) -> Result<String, String>;
    fn trim_text(&self, text: &String) -> String {
        // let ptn = r#"\.?[0-9a-zA-Z\-]+\s*[0-9a-zA-Z:;="'\s\(\)\{\}!\?/,]+"#;
        // let re = Regex::new(ptn).unwrap();
        // let trimmed_text = re.replace_all(text, "").to_string();

        let re = Regex::new(r"\s\s+").unwrap();
        let trimmed_text = re.replace_all(text, "\n").to_string();
        return trimmed_text;
    }
    async fn request(&self, url: &String) -> String {
        let client = reqwest::Client::new();
        let body = client
            .get(url)
            .header(
                reqwest::header::USER_AGENT,
                format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            )
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        return body;
    }
}

pub mod ai_it_now;
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
pub mod hatena_bookmark_it;
pub mod hatena_developer_blog;
pub mod ipa_security_center;
pub mod itmedia_at_it;
pub mod itmedia_enterprise;
pub mod itmedia_general;
pub mod itmedia_marketing;
pub mod jpcert;
pub mod line_techblog;
pub mod macafee_security_news;
pub mod mercari_engineering_blog;
pub mod moneyforward_developers_blog;
pub mod motex;
pub mod nikkei_xtech;
pub mod qiita_blog;
pub mod retrieva_techblog;
pub mod sakura_internet_techblog;
pub mod sansan;
pub mod security_next;
pub mod sophos_news;
pub mod stockmark_news;
pub mod stockmark_techblog;
pub mod supership;
pub mod tokyo_univ_engineering;
pub mod trend_micro_security_advisories;
pub mod trend_micro_security_news;
pub mod yahoo_japan_techblog;
pub mod yahoo_news_it;
pub mod yahoo_news_science;
pub mod zen_mu_tech;
pub mod zenn_topic;
pub mod zenn_trend;
