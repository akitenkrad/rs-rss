use indicatif::{ProgressBar, ProgressStyle};

pub fn create_progress_bar(total: usize, msg: Option<String>) -> ProgressBar {
    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} [{wide_bar}] {percent}% ({msg})")
            .unwrap()
            .progress_chars("█▓▒░")
            .tick_chars("⠋⠙⠚⠉"),
    );
    if let Some(m) = msg {
        pb.set_message(m);
    } else {
        pb.set_message("Processing".to_string());
    }
    pb
}
