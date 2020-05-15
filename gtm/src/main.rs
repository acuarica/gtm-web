#![feature(proc_macro_hygiene, decl_macro)]

extern crate serde_derive;
extern crate serde_json;

use git2::*;
use gtm::fetch_projects;
use structopt::StructOpt;

#[derive(StructOpt)]
/// The gtm Dashboard services
///
/// Returns gtm time data for the specified services.
enum GtmCommand {
    Commits {
        #[structopt(long)]
        from_date: String,
        #[structopt(long)]
        to_date: String,
    },
    Projects,
    Status,
}

fn main() -> Result<(), git2::Error> {
    let command = GtmCommand::from_args();

    match command {
        GtmCommand::Commits { from_date, to_date } => {
            let repo = Repository::open("/Users/luigi/work/home").unwrap();
            let notes = gtm::get_notes(&repo).unwrap();
            let json = serde_json::to_string(&notes).unwrap();
            println!("{}", json);
        }
        GtmCommand::Projects => {
            let projects = fetch_projects();
            let json = serde_json::to_string(&projects).unwrap();
            println!("{}", json);
        }
        GtmCommand::Status => {
            // let projects = fetch_projects();
            // let json = serde_json::to_string(&projects).unwrap();
            println!("{}", "{}");
        }
    };

    Ok(())
}
