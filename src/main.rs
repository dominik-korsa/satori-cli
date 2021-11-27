mod config;
mod my_cookie_store;
mod requests;

use crate::config::{get_password, get_username, set_username};
use crate::requests::Requests;
use clap::Parser;
use console::style;
use dialoguer::theme::ColorfulTheme;
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser)]
#[clap()]
struct Opts {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Parser)]
enum Subcommand {
    SetUsername(SetUsername),
    Submit(Submit),
}

#[derive(Parser)]
struct SetUsername {
    username: String,
}

#[derive(Parser)]
struct Submit {
    filename: PathBuf,
    #[clap(long, parse(try_from_str))]
    ci: bool,
}

fn prompt_password() -> String {
    dialoguer::Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your SATORI password")
        .interact()
        .unwrap()
}

fn ensure_signed_in(ci: bool) -> Requests {
    let requests = Requests::new().unwrap();
    if requests.is_signed_in() {
        return requests;
    }
    let username = get_username();
    let username = match username {
        None => {
            println!("{}", style("Username not set").red().bold());
            println!(
                "{} {} {}",
                style("Use the").red(),
                style("satori set-username"),
                style("command").red()
            );
            println!(
                "{} {} {}",
                style("Or set the").red(),
                style("SATORI_USERNAME"),
                style("environment variable").red()
            );
            exit(1);
        }
        Some(username) => username,
    };
    let password = get_password();
    let password = password.unwrap_or_else(|| {
        if ci {
            println!("{}", style("Password not set").red().bold());
            println!(
                "{} {} {}",
                style("Set the").red(),
                style("SATORI_PASSWORD"),
                style("environment variable").red()
            );
            exit(1);
        }
        prompt_password()
    });
    if !requests.sign_in(&username, &password).unwrap() {
        println!("{}", style("Wrong password").red().bold());
        exit(1);
    }
    requests
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcommand {
        Subcommand::SetUsername(set_opts) => {
            set_username(&set_opts.username);
            println!(
                "{}{}",
                style("Stored username: ").green(),
                &set_opts.username
            );
        }
        Subcommand::Submit(submit) => {
            let requests = ensure_signed_in(submit.ci);
        }
    }
}
