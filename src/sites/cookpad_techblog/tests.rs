use super::*;

#[tokio::test]
async fn test_cookpad_techblog() {
    let site = CookpadTechBlog {};
    let articles = site.get_articles().await;
    if let Ok(articles) = articles {
        if articles.len() == 0 {
            println!("No articles found");
            assert!(true);
            return;
        }

        let article = articles.get(0).unwrap();
        println!("Article: {:?}", article);
        let html_and_text = site.get_article_text(&article.url).await;
        match html_and_text {
            Ok(html_and_text) => {
                let (html, text) = html_and_text;
                println!("HTML: {}", html);
                println!("Text: {}", text);
                assert!(html.len() > 0);
                assert!(text.len() > 0);
            }
            Err(e) => {
                println!("Error: {}", e);
                assert!(false);
            }
        }
    } else {
        assert!(false);
    }
}
