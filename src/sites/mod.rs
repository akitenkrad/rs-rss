use chrono::{DateTime, Local};
use futures::Future;
use regex::Regex;

pub trait InfoItem {
    fn title(&self) -> String;
    fn url(&self) -> String;
    fn description(&self) -> String;
    fn timestamp(&self) -> DateTime<Local>;
}

#[derive(Debug)]
pub struct WebArticle {
    pub title: String,
    pub url: String,
    pub text: String,
    pub timestamp: DateTime<Local>,
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

pub trait Site {
    fn name(&self) -> String;
    fn get_articles(&self) -> impl Future<Output = Vec<WebArticle>> + Send;
    fn get_article_text(&self, url: &String) -> impl Future<Output = String> + Send;
    fn user_agent(&self) -> String {
        return format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    }
    fn to_slack_message(&self, article: &WebArticle) -> String {
        return format!(
            "{}\n{}\n{}",
            article.title(),
            article.url(),
            article.description()
        );
    }
    fn trim_text(&self, text: &String) -> String {
        // let ptn = r#"\.?[0-9a-zA-Z\-]+\s*[0-9a-zA-Z:;="'\s\(\)\{\}!\?/,]+"#;
        // let re = Regex::new(ptn).unwrap();
        // let trimmed_text = re.replace_all(text, "").to_string();

        let re = Regex::new(r"\s\s+").unwrap();
        let trimmed_text = re.replace_all(text, "\n").to_string();
        return trimmed_text;
    }
}

mod ai_it_now;
mod codezine;
mod cookpad_techblog;
mod cyberagent_techblog;
mod cybozu_blog;
mod dena_engineering_blog;
mod gigazine;
mod github_developers_blog;
mod gizmodo;
mod google_developers_blog;
mod gree_techblog;
mod gunosy_techblog;
mod hatena_developer_blog;
mod ipa_security_center;
mod itmedia_at_it;
mod itmedia_enterprise;
mod itmedia_general;
mod itmedia_marketing;
mod line_techblog;
mod mercari_engineering_blog;
mod moneyforward_developers_blog;
mod nikkei_xtech;
mod qiita_blog;
mod retrieva_techblog;
mod sakura_internet_techblog;
mod sansan;
mod security_next;
mod stockmark_news;
mod stockmark_techblog;
mod supership;
mod yahoo_japan_techblog;
mod yahoo_news_it;
mod yahoo_news_science;
mod zenn_topic_nlp;
mod zenn_trend;
