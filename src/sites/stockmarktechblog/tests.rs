use super::*;

#[test]
fn test_stockmarktechblog() {
    let site = StockmarkTechBlog {};
    let articles = tokio_test::block_on(site.get_articles());
    assert!(articles.len() > 0);
}
