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

pub fn footer() {
    println!("{}{}{}",
       "Report issues on this program at https://github.com/antony-jr/ritual\n".yellow(),
       "Thank you for using Spirit 💖, if you find this project cool then please\n".bold(),
	   "consider to star 🌟 this project at https://github.com/antony-jr/spirit".bold());
	
}
