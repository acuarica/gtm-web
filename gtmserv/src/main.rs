#![feature(proc_macro_hygiene, decl_macro)]
#![feature(termination_trait_lib)]
#![feature(process_exitcode_placeholder)]
#![feature(try_trait)]

extern crate serde_derive;
extern crate serde_json;

use ansi_term::ANSIString;
use ansi_term::Colour::Red;
use chrono::{Duration, NaiveDate, Utc};
use git2::*;
use gtmserv::InitProjects;
use gtmserv::{get_notes, FileEvent, Timeline, WorkdirStatus};
use std::fmt::Display;
use std::{
    collections::HashMap,
    fs,
    ops::Try,
    path::PathBuf,
    process::{ExitCode, Termination},
};
use structopt::StructOpt;

#[derive(StructOpt)]
#[cfg_attr(debug_assertions, structopt(version = env!("GTM_VERSION")))]
/// The gtm Dashboard services
///
/// Returns gtm time data for the specified services.
/// All data returned is in JSON format.
enum GtmCommand {
    /// Returns commits with gtm time data
    Commits {
        #[structopt(short, long)]
        from_date: Option<String>,
        #[structopt(short, long)]
        to_date: Option<String>,
        #[structopt(short, long)]
        message: Option<String>,
    },

    /// Returns the init(ialized) projects by gtm
    Projects,

    /// Returns the uncommited gtm data
    Status,
}

struct Tty<'a>(ANSIString<'a>);

impl Display for Tty<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if atty::is(atty::Stream::Stdout) {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}", &*self.0)
        }
    }
}

struct GtmResult<E>(Result<(), E>);

impl<E> Try for GtmResult<E> {
    type Ok = ();
    type Error = E;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        self.0
    }

    fn from_error(v: Self::Error) -> Self {
        GtmResult(Err(v))
    }

    fn from_ok(v: Self::Ok) -> Self {
        GtmResult(Ok(v))
    }
}

#[derive(Debug)]
enum GtmError {
    Git(git2::Error),
    Parse(chrono::ParseError, String),
}

impl Termination for GtmResult<GtmError> {
    fn report(self) -> i32 {
        match self.0 {
            Ok(()) => ().report(),
            Err(err) => {
                eprintln!("gtmserv error: {}", Tty(Red.paint(format!("{}", err))));
                ExitCode::FAILURE.report()
            }
        }
    }
}

// impl Error for GtmError {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         match self {
//             GtmError::Git(err) => Some(err),
//             GtmError::Parse(err) => Some(err),
//         }
//     }
//     // fn cause(&self) -> Option<&dyn std::error::Error> {
//     //     std::error.source()
//     // }
// }

impl Display for GtmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GtmError::Git(err) => write!(f, "Git2 error: {}", err),
            GtmError::Parse(err, field) => write!(f, "Could not parse {} argument: {}", field, err),
        }
    }
}

// impl From<chrono::ParseError> for GtmError {
//     fn from(err: chrono::ParseError) -> Self {
//         GtmError::Parse(err)
//     }
// }

impl From<git2::Error> for GtmError {
    fn from(err: git2::Error) -> Self {
        GtmError::Git(err)
    }
}

fn config_path() -> Option<PathBuf> {
    let mut path = dirs::home_dir()?;
    path.push(".git-time-metric");
    path.push("project.json");
    Some(path)
}

fn from_config() -> InitProjects {
    InitProjects::from_file(config_path().unwrap()).unwrap()
}

fn parse_arg_date(date: &Option<String>, field: &str, days: i64) -> Result<i64, GtmError> {
    Ok(match date {
        None => Utc::now().timestamp(),
        Some(date) => NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .map_err(|e| GtmError::Parse(e, field.to_owned()))?
            .checked_add_signed(Duration::days(days))
            .unwrap()
            .and_hms(0, 0, 0)
            .timestamp(),
    })
}

fn main() -> GtmResult<GtmError> {
    let command = GtmCommand::from_args();

    match command {
        GtmCommand::Commits {
            from_date,
            to_date,
            message,
        } => {
            let from_date = parse_arg_date(&from_date, "from", 0)?;
            let to_date = parse_arg_date(&to_date, "to", 1)?;

            let mut notes = Vec::new();
            for project in from_config().get_project_list() {
                let path = PathBuf::from(project.as_str());
                let pkey = path.file_name().unwrap().to_str().unwrap().to_owned();
                let repo = Repository::open(project.to_owned())?;
                get_notes(&mut notes, &repo, pkey, from_date, to_date, &message).unwrap();
            }

            let json = serde_json::to_string(&notes).unwrap();
            println!("{}", json);
        }
        GtmCommand::Projects => {
            let projects = from_config();
            let projects: Vec<&String> = projects.get_project_list().collect();
            let json = serde_json::to_string(&projects).unwrap();
            println!("{}", json);
        }
        GtmCommand::Status => {
            let mut wd = HashMap::new();
            for project in from_config().get_project_list() {
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
                let cn = Timeline::from_events(events).commit_note();
                let ws = WorkdirStatus {
                    total: cn.total,
                    label: "TBD".to_string(),
                    commit_note: cn,
                };

                let path = PathBuf::from(project.as_str());
                let pkey = path.file_name().unwrap().to_str().unwrap().to_owned();
                wd.insert(pkey, ws);
            }
            let json = serde_json::to_string(&wd).unwrap();
            println!("{}", json);
        }
    };

    GtmResult(Ok(()))
}
