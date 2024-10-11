use super::*;

#[test]
fn test_stockmarknews() {
    let site = StockmarkNews {};
    let articles = tokio_test::block_on(site.get_articles());
    assert!(articles.len() > 0);
}
