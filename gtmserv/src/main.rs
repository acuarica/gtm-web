#![feature(proc_macro_hygiene, decl_macro)]

extern crate serde_derive;
extern crate serde_json;

use gtmserv::get_notes;
use git2::*;
use gtmserv::fetch_projects;
use gtmserv::to_unixtime;
use structopt::StructOpt;

#[derive(StructOpt)]
#[cfg_attr(debug_assertions, structopt(version = "holaversion"))]
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
            let from_date = to_unixtime(from_date).unwrap();
            let to_date = to_unixtime(to_date).unwrap();

            let mut notes = Vec::new();
            let projects = fetch_projects();
            for project in projects.unwrap() {
                let repo = Repository::open(project.to_owned()).unwrap();
                get_notes(&mut notes, &repo, project.to_owned(), from_date, to_date).unwrap();
            }

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
