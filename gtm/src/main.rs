#![feature(proc_macro_hygiene, decl_macro)]

extern crate serde_derive;
extern crate serde_json;

use git2::*;
use gtm::fetch_projects;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    command: String,
}

fn main() -> Result<(), git2::Error> {
    let args = Args::from_args();

    match args.command.as_str() {
        "commits" => {
            let repo = Repository::open("/Users/luigi/work/home").unwrap();
            let notes = gtm::get_notes(&repo).unwrap();
            let json = serde_json::to_string(&notes).unwrap();
            println!("{}", json);
        }
        "projects" => {
            let projects = fetch_projects();
            let json = serde_json::to_string(&projects).unwrap();
            println!("{}", json);
        }
        _ => {
            eprintln!("No option given");
        }
    };

    Ok(())
}
