mod config;
mod my_cookie_store;
mod requests;

use crate::config::{get_password, get_username, set_username};
use crate::requests::Requests;
use clap::Parser;
use console::style;
use dialoguer::theme::ColorfulTheme;
use std::fs;
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
    SignOut,
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
    // #[clap(short, long, parse(try_from_str))]
    // open: bool,
}

fn prompt_password(username: &str) -> String {
    dialoguer::Password::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Password for {}", username))
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
        prompt_password(&username)
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
            let requests = Requests::new().unwrap();
            requests.sign_out().unwrap();
            println!(
                "{}{}",
                style("Stored username: ").green(),
                &set_opts.username
            );
        }
        Subcommand::SignOut => {
            let requests = Requests::new().unwrap();
            requests.sign_out().unwrap();
        }
        Subcommand::Submit(submit) => {
            let requests = ensure_signed_in(submit.ci);
            let url_regex = regex::Regex::new(
                r"\b(?:https?://)?satori\.tcs\.uj\.edu\.pl/contest/(\d+)/problems/(\d+)(?:\s|$|/)",
            )
            .unwrap();
            let content = fs::read_to_string(&submit.filename).unwrap();
            let captures = url_regex.captures(&content);
            let (contest_id, problem_id) = match captures {
                None => {
                    println!(
                        "{}",
                        style("Problem URL was not found in code").red().bold()
                    );
                    println!("{}", style("A valid URL looks like this:").red().bold());
                    println!(
                        "{}{}{}{}",
                        style("https://satori.tcs.uj.edu.pl/contest/").cyan(),
                        style("0123456").white(),
                        style("/problems/").cyan(),
                        style("0123456").white(),
                    );
                    exit(1);
                }
                Some(captures) => (captures[1].to_string(), captures[2].to_string()),
            };
            let results_url = requests
                .submit(&contest_id, &problem_id, &submit.filename, &content)
                .unwrap();
            println!("{}", style("Submitted solution").green());
            println!(
                "{} {}",
                style("Results page:").green(),
                style(results_url).cyan().underlined()
            );
        }
    }
}
