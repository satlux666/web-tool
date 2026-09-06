mod banner;
mod utils;
mod scanner;
mod recon;
mod extract;
mod report;
mod wordlists;

use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser)]
#[command(name = "luxweb")]
#[command(about = "luxweb by lucifer")]
struct Args {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Port {
        #[arg(short, long)]
        target: String,
        #[arg(short, long, default_value = "top")]
        ports: String,
        #[arg(short = 't', long, default_value_t = 350)]
        threads: usize,
    },
    Subs {
        #[arg(short, long)]
        target: String,
        #[arg(short, long, default_value_t = 150)]
        limit: usize,
    },
    Full {
        #[arg(short, long)]
        target: String,
        #[arg(long)]
        proxy: Option<String>,
        #[arg(short = 't', long, default_value_t = 350)]
        threads: usize,
    },
}

fn main() {
    banner::print_banner();
    let args = Args::parse();

    match args.cmd {
        Cmd::Port { target, ports, threads } => {
            scanner::port_scan(&target, &ports, threads);
        }
        Cmd::Subs { target, limit } => {
            recon::subdomains(&target, limit);
        }
        Cmd::Full { target, proxy, threads } => {
            println!("{} full scan started", "[*]".magenta());
            let client = utils::make_client(proxy);
            recon::basic_osint(&target, &client);
            let techs = recon::tech_detect(&target, &client);
            let subs = recon::subdomains(&target, 180);
            let ports = scanner::port_scan(&target, "top", threads);
            let (emails, phones) = extract::emails_and_phones(&target, &client);
            let paths = recon::interesting_paths(&target, &client);

            let mut notes = Vec::new();
            if paths.iter().any(|x| x.contains(".env") || x.contains(".git")) {
                notes.push("sensitive path found".into());
            }
            if techs.iter().any(|x| x.contains("WordPress")) {
                notes.push("wordpress detected".into());
            }

            report::write_json(&target, &ports, &subs, &emails, &phones, &paths, &techs, &notes);
            report::write_txt(&target, &ports, &subs, &emails, &phones, &paths, &techs, &notes);

            println!("{} done", "[+]".green());
        }
    }
}
