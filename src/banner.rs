use colored::Colorize;

pub fn header() {
    println!("ritual {} {} (commit {}) build {} [{}].",
              "👻".bold(),
              env!("RITUAL_VERSION").underline(),
              env!("RITUAL_GIT_COMMIT").green(),
              env!("RITUAL_BUILD_NO"),
              env!("RITUAL_BUILD_DATE"));
    println!("D. Antony J.R <antonyjr@protonmail.com>.");
    println!("{}\n", "Make Spirits for the Twenty-First Century Window Sitter.".cyan().bold());
}
