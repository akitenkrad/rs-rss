use clap::{Parser, Subcommand};
use commands::{
    add_academic_paper::{add_academic_paper, AddAcademicPaperArgs},
    collece_articles::{collect_articles, CollectArticlesArgs},
    notify_web_articles_to_slack::{notify_to_slack, NotifyWebArticlesToSlackArgs},
    start_dashboard::{start_dashboard, StartDashboardArgs},
};
use shared::logger::init_logger;

#[derive(Debug, Parser, Clone)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand, Debug, Clone)]
enum SubCommands {
    /// Collect articles from websites
    CollectArticles(CollectArticlesArgs),
    /// Start the dashboard
    StartDashboard(StartDashboardArgs),
    /// Add an academic paper to the database
    AddAcademicPaper(AddAcademicPaperArgs),
    /// Notify web articles to Slack
    NotifyWebArticlesToSlack(NotifyWebArticlesToSlackArgs),
}

#[tokio::main]
async fn main() {
    init_logger().expect("Failed to initialize logger");
    let cli = Cli::parse();

    match &cli.subcommand {
        SubCommands::CollectArticles(args) => collect_articles(args).await,
        SubCommands::StartDashboard(args) => start_dashboard(args).await,
        SubCommands::AddAcademicPaper(args) => add_academic_paper(args).await,
        SubCommands::NotifyWebArticlesToSlack(args) => notify_to_slack(args).await,
    }
}
