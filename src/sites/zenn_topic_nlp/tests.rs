use super::*;

#[test]
fn test_zenn_topic() {
    let site = ZennTopic {
        topic: "自然言語処理".to_string(),
    };
    let articles = tokio_test::block_on(site.get_articles());
    assert!(articles.len() > 0);

    let article = articles.get(0).unwrap();
    println!("Article: {:?}", article);
    let article = tokio_test::block_on(site.get_article_text(&article.url));
    println!("Article text: {}", article);
    assert!(article.is_empty() == false);
}
