use std::io::Write;

// use scraper::Selector;
// use std::{fs::File, io::Write};
// use tokio::time;
use indicatif::ProgressBar;
use sites::*;
use sites::{Site, WebArticle};

pub mod sites;
pub mod slack;

async fn get_articles_from_eath_site() -> Vec<WebArticle> {
    let mut sites: Vec<Box<dyn Site>> = vec![
        Box::new(ai_db::AIDB::default()),
        Box::new(ai_it_now::AIItNow::default()),
        Box::new(ai_news::AINews::default()),
        Box::new(ai_scholar::AIScholar::default()),
        Box::new(aismiley::AISmiley::default()),
        Box::new(aizine::AIZine::default()),
        Box::new(aws_security_blog::AWSSecurityBlog::default()),
        Box::new(business_insider_science::BusinessInsiderScience::default()),
        Box::new(business_insider_technology::BusinessInsiderTechnology::default()),
        Box::new(canon_malware_center::CanonMalwareCenter::default()),
        Box::new(codezine::CodeZine::default()),
        Box::new(cookpad_techblog::CookpadTechBlog::default()),
        Box::new(crowdstrike_blog::CrowdStrikeBlog::default()),
        Box::new(cyberagent_techblog::CyberAgentTechBlog::default()),
        Box::new(cybozu_blog::CybozuBlog::default()),
        Box::new(dena_engineering_blog::DeNAEngineeringBlog::default()),
        Box::new(gigazine::Gigazine::default()),
        Box::new(github_developers_blog::GitHubDevelopersBlog::default()),
        Box::new(gizmodo::Gizmodo::default()),
        Box::new(google_developers_blog::GoogleDevelopersBlog::default()),
        Box::new(gree_techblog::GreeTechBlog::default()),
        Box::new(gunosy_techblog::GunosyTechBlog::default()),
        Box::new(ipa_security_center::IPASecurityCenter::default()),
        Box::new(itmedia_at_it::ITMediaAtIt::default()),
        Box::new(itmedia_enterprise::ITMediaEnterprise::default()),
        Box::new(itmedia_marketing::ITMediaMarketing::default()),
        Box::new(itmedia_general::ITMediaGeneral::default()),
        Box::new(jpcert::JPCert::default()),
        Box::new(line_techblog::LineTechBlog::default()),
        Box::new(macafee_security_news::MacAfeeSecurityNews::default()),
        Box::new(medium::Medium::new("Artificial Intelligence", "artificial-intelligence")),
        Box::new(medium::Medium::new("AI", "ai")),
        Box::new(medium::Medium::new("Machine Learning", "machine-learning")),
        Box::new(medium::Medium::new("ChatGPT", "chatgpt")),
        Box::new(medium::Medium::new("Data Science", "data-science")),
        Box::new(medium::Medium::new("OpenAI", "openai")),
        Box::new(medium::Medium::new("LLM", "llm")),
        Box::new(mercari_engineering_blog::MercariEngineeringBlog::default()),
        Box::new(mit_ai::MITAI::default()),
        Box::new(mit_research::MITResearch::default()),
        Box::new(moneyforward_developers_blog::MoneyForwardDevelopersBlog::default()),
        Box::new(motex::MoTex::default()),
        Box::new(nikkei_xtech::NikkeiXTech::default()),
        Box::new(qiita_blog::QiitaBlog::default()),
        Box::new(retrieva_techblog::RetrievaTechBlog::default()),
        Box::new(rust_blog::RustBlog::default()),
        Box::new(sakura_internet_techblog::SakuraInternetTechBlog::default()),
        Box::new(sansan::Sansan::default()),
        Box::new(security_next::SecurityNext::default()),
        Box::new(sophos_news::SophosNews::default()),
        Box::new(stockmark_news::StockmarkNews::default()),
        Box::new(stockmark_techblog::StockmarkTechBlog::default()),
        Box::new(supership::Supership::default()),
        Box::new(tech_crunch::TechCrunch::default()),
        Box::new(tokyo_univ_engineering::TokyoUniversityEngineering::default()),
        Box::new(trend_micro_security_news::TrendMicroSecurityNews::default()),
        Box::new(trend_micro_security_advisories::TrendMicroSecurityAdvisories::default()),
        Box::new(yahoo_news_it::YahooNewsIT::default()),
        Box::new(yahoo_japan_techblog::YahooJapanTechBlog::default()),
        Box::new(zen_mu_tech::ZenmuTech::default()),
        Box::new(zenn_topic::ZennTopic::new("自然言語処理")),
        Box::new(zenn_topic::ZennTopic::new("生成ai")),
        Box::new(zenn_topic::ZennTopic::new("rust")),
        Box::new(zenn_trend::ZennTrend::default()),
    ];

    // Collect articles from each site
    let pb = ProgressBar::new(sites.len() as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.green/cyan} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓▒░"),
    );
    let mut articles = Vec::new();
    for site in sites.iter_mut() {
        let msg = format!("Collecting articles from: {}", site.name().clone());
        pb.set_message(msg);
        let result = site.get_articles().await;
        match result {
            Ok(articles_) => {
                articles.extend(articles_);
            }
            Err(e) => {
                eprintln!("Error: {} / {}", e, site.name());
            }
        }
        pb.inc(1);
    }
    pb.finish_and_clear();

    // Fill in article properties
    let pb = ProgressBar::new(articles.len() as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.green/cyan} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓▒░"),
    );
    for article in &mut articles {
        let now = chrono::Local::now();
        if article.timestamp < now - chrono::Duration::hours(24) {
            pb.inc(1);
            continue;
        }
        for site in sites.iter_mut() {
            let target_domain = match site.get_domain(&article.url) {
                Ok(domain) => domain,
                Err(_) => {
                    eprintln!("Error (get_domain): {} / {}", article.url, site.name());
                    continue;
                }
            };
            if site.domain() == target_domain {
                match site.parse_article(&article.url).await {
                    Ok(html_and_text) => {
                        let (html, _) = html_and_text;
                        article.html = html;
                        match site.complete_article_properties(&article.title, &article.html) {
                            Ok(property) => {
                                article.property = property;
                            }
                            Err(e) => {
                                eprintln!("Error (chat): {} / {}", e, article.url);
                            }
                        }
                        break;
                    }
                    Err(_) => {}
                }
            }
        }
        if article.html.is_empty() {
            eprintln!("Error (failed to parse html): [{}] {}", article.site, article.url);
        }
        pb.inc(1);
    }
    pb.finish_and_clear();
    return articles;
}

#[tokio::main]
async fn main() {
    let articles = get_articles_from_eath_site().await;

    // Save articles to a json file
    let file_path = "articles.json";
    let mut file = std::fs::File::create(file_path).unwrap();
    let json = serde_json::to_string_pretty(&articles).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    println!("Articles saved to {}", file_path);

    // let _ = notify_slack(articles).await;
}
