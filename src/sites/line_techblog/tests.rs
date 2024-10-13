use super::*;

#[test]
fn test_line_techblog() {
    let site = LineTechBlog {};
    let articles = tokio_test::block_on(site.get_articles());
    if let Ok(articles) = articles {
        assert!(articles.len() > 0);

        let article = articles.get(0).unwrap();
        println!("Article: {:?}", article);
        let article = tokio_test::block_on(site.get_article_text(&article.url));
        if let Ok(article) = article {
            println!("Article text: {}", article);
            assert!(article.is_empty() == false);
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }
}
