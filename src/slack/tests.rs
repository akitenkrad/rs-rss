use crate::sites::{self, Site};

use super::*;

#[test]
fn test_notify_slack() {
    let site = sites::ai_it_now::AIItNow {};
    let articles = tokio_test::block_on(site.get_articles());
    if let Ok(articles) = articles {
        assert!(articles.len() > 0);
        let result = tokio_test::block_on(notify_slack(articles, false));
        assert_eq!(result, Ok(()));
    } else {
        assert!(false);
    }
}
