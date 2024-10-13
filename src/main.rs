// use scraper::Selector;
// use std::{fs::File, io::Write};
// use tokio::time;
use sites::*;
use sites::{Site, WebArticle};
use slack::notify_slack;

pub mod sites;
pub mod slack;

fn get_articles_from_eath_site() -> Vec<WebArticle> {
    let sites: Vec<Box<dyn Site>> = vec![
        Box::new(ai_it_now::AIItNow {}),
        Box::new(aws_security_blog::AWSSecurityBlog {}),
        Box::new(business_insider_science::BusinessInsiderScience {}),
        Box::new(business_insider_technology::BusinessInsiderTechnology {}),
        Box::new(canon_malware_center::CanonMalwareCenter {}),
        Box::new(codezine::CodeZine {}),
        Box::new(cookpad_techblog::CookpadTechBlog {}),
        Box::new(crowdstrike_blog::CrowdStrikeBlog {}),
        Box::new(cyberagent_techblog::CyberAgentTechBlog {}),
        Box::new(cybozu_blog::CybozuBlog {}),
        Box::new(dena_engineering_blog::DeNAEngineeringBlog {}),
        Box::new(gigazine::Gigazine {}),
        Box::new(github_developers_blog::GitHubDevelopersBlog {}),
        Box::new(gizmodo::Gizmodo {}),
        Box::new(google_developers_blog::GoogleDevelopersBlog {}),
        Box::new(gree_techblog::GreeTechBlog {}),
        Box::new(gunosy_techblog::GunosyTechBlog {}),
        Box::new(hatena_bookmark_it::HatenaBookmarkIT {}),
        Box::new(hatena_developer_blog::HatenaDeveloperBlog {}),
        Box::new(ipa_security_center::IPASecurityCenter {}),
        Box::new(itmedia_at_it::ITMediaAtIt {}),
        Box::new(itmedia_enterprise::ITMediaEnterprise {}),
        Box::new(itmedia_general::ITMediaGeneral {}),
        Box::new(itmedia_marketing::ITMediaMarketing {}),
        Box::new(jpcert::JPCert {}),
        Box::new(line_techblog::LineTechBlog {}),
        Box::new(macafee_security_news::MacAfeeSecurityNews {}),
        Box::new(mercari_engineering_blog::MercariEngineeringBlog {}),
        Box::new(moneyforward_developers_blog::MoneyForwardDevelopersBlog {}),
        Box::new(motex::MoTex {}),
        Box::new(nikkei_xtech::NikkeiXTech {}),
        Box::new(qiita_blog::QiitaBlog {}),
        Box::new(retrieva_techblog::RetrievaTechBlog {}),
        Box::new(rust_blog::RustBlog {}),
        Box::new(sakura_internet_techblog::SakuraInternetTechBlog {}),
        Box::new(sansan::Sansan {}),
        Box::new(security_next::SecurityNext {}),
        Box::new(sophos_news::SophosNews {}),
        Box::new(stockmark_news::StockmarkNews {}),
        Box::new(stockmark_techblog::StockmarkTechBlog {}),
        Box::new(supership::Supership {}),
        Box::new(tokyo_univ_engineering::TokyoUniversityEngineering {}),
        Box::new(trend_micro_security_news::TrendMicroSecurityNews {}),
        Box::new(trend_micro_security_advisories::TrendMicroSecurityAdvisories {}),
        Box::new(yahoo_news_it::YahooNewsIT {}),
        Box::new(yahoo_japan_techblog::YahooJapanTechBlog {}),
        Box::new(zen_mu_tech::ZenmuTech {}),
        Box::new(zenn_topic::ZennTopic {
            topic: "自然言語処理".to_string(),
        }),
        Box::new(zenn_topic::ZennTopic {
            topic: "生成AI".to_string(),
        }),
        Box::new(zenn_trend::ZennTrend {}),
    ];
    let mut articles = Vec::new();
    for site in sites {
        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(site.get_articles());
        match result {
            Ok(articles_) => {
                articles.extend(articles_);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    return articles;
}

#[tokio::main]
async fn main() {
    let articles = get_articles_from_eath_site();
    notify_slack(articles).await;
}
