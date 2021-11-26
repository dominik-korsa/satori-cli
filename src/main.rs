mod config;
mod requests;

use crate::config::set_username;
use clap::Parser;
use console::style;
use dialoguer::theme::ColorfulTheme;

#[derive(Parser)]
#[clap()]
struct Opts {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Parser)]
enum Subcommand {
    SetUsername(SetUsername),
}

#[derive(Parser)]
struct SetUsername {
    username: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let theme = ColorfulTheme::default();

    match opts.subcommand {
        Subcommand::SetUsername(set_opts) => {
            set_username(&set_opts.username);
            println!(
                "{}{}",
                style("Stored username: ").green(),
                &set_opts.username
            );
        }
    }
}
