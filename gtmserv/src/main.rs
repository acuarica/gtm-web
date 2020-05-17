#![feature(proc_macro_hygiene, decl_macro)]

extern crate serde_derive;
extern crate serde_json;

use git2::*;
use gtmserv::fetch_projects;
use gtmserv::get_notes;
use gtmserv::get_status;
use gtmserv::WorkdirStatus;
use gtmserv::{to_unixtime, FileEvent};
use std::{collections::HashMap, fs, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
#[cfg_attr(debug_assertions, structopt(version = env!("GTM_VERSION")))]
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
            let projects = fetch_projects();
            let mut wd = HashMap::new();
            for project in projects.unwrap() {
                let mut path = PathBuf::new();
                path.push(project.to_owned());
                path.push(".gtm");
                let entries = fs::read_dir(path).unwrap();
                let mut events = Vec::new();
                for entry in entries {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if !path.is_dir() && path.extension().unwrap() == "event" {
                        let ts: &str = path.file_stem().unwrap().to_str().unwrap();
                        let ts = ts.parse().unwrap();
                        let filepath = fs::read_to_string(path).unwrap();
                        let fe = FileEvent::new(ts, filepath.as_ref());
                        // println!("{:?}", fe);
                        events.push(fe);
                    }
                }
                events.sort_by_key(|k| k.timestamp);
                let cn = get_status(events).commit_note();
                let ws = WorkdirStatus {
                    total: cn.total,
                    label: "TBD".to_string(),
                    commit_note: cn,
                };
                wd.insert(project, ws);
            }
            let json = serde_json::to_string(&wd).unwrap();
            println!("{}", json);
        }
    };

    Ok(())
}
