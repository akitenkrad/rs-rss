// use crate::sites::{self, Site};
//
// use super::*;
//
// #[test]
// fn test_notify_slack() {
//     let site = sites::ai_it_now::AIItNow {};
//     let articles = site.get_articles().await;
//     if let Ok(articles) = articles {
//         if articles.len() == 0 {
//             println!("No articles found");
//             assert!(true);
//             return;
//         }
//         let result = tokio_test::block_on(notify_slack(articles, false));
//         assert_eq!(result, Ok(()));
//     } else {
//         assert!(false);
//     }
// }
//
