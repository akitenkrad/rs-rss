use super::*;

#[test]
fn test_codezin() {
    let site = CodeZin {};
    let articles = tokio_test::block_on(site.get_articles());
    assert!(articles.len() > 0);
}
