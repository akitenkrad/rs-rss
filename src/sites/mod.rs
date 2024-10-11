use chrono::{DateTime, Local};
use futures::Future;
mod codezin;
mod gigazin;

pub trait InfoItem {
    fn title(&self) -> String;
    fn url(&self) -> String;
    fn description(&self) -> String;
    fn timestamp(&self) -> DateTime<Local>;
}

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
    fn get_articles(&self) -> impl Future<Output = Vec<WebArticle>> + Send;
    fn to_slack_message(&self, article: WebArticle) -> String {
        return format!(
            "{}\n{}\n{}",
            article.title(),
            article.url(),
            article.description()
        );
    }
}
