use indicatif::{ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref PROGRESS_BAR: Mutex<Option<ProgressBar>> = Mutex::new(None);
}

pub fn init_progress_bar() -> ProgressBar {
    let mut pb = PROGRESS_BAR.lock().unwrap();
    if pb.is_none() {
        let new_pb = ProgressBar::new(100);
        new_pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
        );
        *pb = Some(new_pb);
    }
    pb.as_ref().unwrap().clone()
}

pub fn increment_progress() {
    let pb = init_progress_bar();
    pb.inc(1);
}

pub fn finish_progress() {
    let pb = init_progress_bar();
    pb.finish_with_message("完了しました");
}
