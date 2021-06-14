use clap::{App, AppSettings, Arg, SubCommand};
use colored::Colorize;

use ritual::make;

fn main() {
    let matches = App::new(format!("{}", "ritual 👻".bold()))
        .version(
            &format!(
                "{} (commit {}) build {} [{}].",
                env!("RITUAL_VERSION").underline(),
                env!("RITUAL_GIT_COMMIT").green(),
                env!("RITUAL_BUILD_NO"),
                env!("RITUAL_BUILD_DATE")
            )[..],
        )
        .author("D. Antony J.R <antonyjr@protonmail.com>.")
        .about(
            &format!(
                "{}",
                "Make Spirits for the Twenty-First Century Window Sitter."
                    .cyan()
                    .bold()
            )[..],
        )
        .subcommand(
            SubCommand::with_name("make")
                .arg(
                    Arg::with_name("source directory")
                        .required(true)
                        .long_help(
                            "The location to where the spirit data is placed according to spec.",
                        )
                        .index(1),
                )
                .version(
                    &format!(
                        "{} (commit {} {}).",
                        env!("RITUAL_VERSION").underline(),
                        env!("RITUAL_GIT_COMMIT").green(),
                        env!("RITUAL_BUILD_NO")
                    )[..],
                )
                .author("D. Antony J.R <antonyjr@protonmail.com>")
                .about(
                    &format!(
                        "{}",
                        "Make Spirit from the given source directory."
                            .magenta()
                            .bold()
                    )[..],
                ),
        )
        .setting(AppSettings::SubcommandRequired)
        .get_matches();

    match matches.subcommand() {
        ("make", Some(make_cmd)) => {
            match make_cmd.value_of("source directory") {
                Some(value) => {
                    make::run(value);
                }
                _ => {}
            };
        }
        _ => {}
    };
}
