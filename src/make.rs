use std::thread;
use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use rand::{thread_rng, Rng};

use crate::banner;

pub fn run(source_dir: &str) {
    banner::header();
    
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
                .template("{spinner: green/bold} {msg:.green}▕{bar:100}|")
                .progress_chars("█▓▒░  "),
            );
    
    pb.set_message("Validating Directory Structure... ");

    for i in 0..50 {
        pb.inc(1);
        if i == 10 {
            pb.set_message("Checking meta data...");
        }
        thread::sleep(Duration::from_millis(5));
    }
    pb.finish_and_clear();

    println!("Validated Directory.");

    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
                .template("{spinner: green/bold} {msg:.green}▕{bar:100}|")
                .progress_chars("█▓▒░  "),
            );
    
    pb.set_message("Validating Directory Structure... ");
    pb.inc(50);

    for i in 50..100 {
        pb.inc(1);
        if i == 10 {
            pb.set_message("Checking meta data...");
        }
        thread::sleep(Duration::from_millis(5));
    }
    pb.finish_and_clear();


}
