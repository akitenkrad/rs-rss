use super::*;

#[test]
fn test_trend_micro_security_news() {
    let site = TrendMicroSecurityNews {};
    let articles = tokio_test::block_on(site.get_articles());
    assert!(articles.len() > 0);

    let article = articles.get(0).unwrap();
    println!("Article: {:?}", article);
    let article = tokio_test::block_on(site.get_article_text(&article.url));
    println!("Article text: {}", article);
    assert!(article.is_empty() == false && article != "NO CONTENT");
}
