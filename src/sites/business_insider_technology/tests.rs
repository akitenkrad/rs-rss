use super::*;

#[test]
fn test_business_insider_technology() {
    let site = BusinessInsiderTechnology {};
    let articles = tokio_test::block_on(site.get_articles());
    assert!(articles.len() > 0);

    let article = articles.get(0).unwrap();
    println!("Article: {:?}", article);
    let article = tokio_test::block_on(site.get_article_text(&article.url));
    println!("Article text: {}", article);
    assert!(article.is_empty() == false);
}
