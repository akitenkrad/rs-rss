use super::*;

#[test]
fn test_dena_engineering_blog() {
    let site = DeNAEngineeringBlog {};
    let articles = tokio_test::block_on(site.get_articles());
    if let Ok(articles) = articles {
        assert!(articles.len() > 0);

        for article in articles.iter() {
            assert!(article.url.starts_with("https://engineering.dena.com/blog"));
        }

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
