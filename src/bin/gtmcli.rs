#![feature(proc_macro_hygiene, decl_macro)]
#![feature(termination_trait_lib)]
#![feature(process_exitcode_placeholder)]
#![feature(try_trait)]

extern crate serde_derive;
extern crate serde_json;

use ansi_term::{ANSIString, Colour::Red};
use chrono::{Duration, NaiveDate};
use gtm::{
    epoch,
    projects::Projects,
    services::{write_commits, write_project_list, write_workdir_status},
    NotesFilter,
};
use io::BufWriter;
use std::{
    fmt::Display,
    io,
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
enum Args {
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
    Io(io::Error, PathBuf),
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
            GtmError::Io(err, path) => write!(f, "Io error {:?} : {}", path, err),
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

fn from_config() -> Result<Projects, GtmError> {
    let path = Projects::config_path().unwrap();
    Projects::from_file(&path).map_err(|e| GtmError::Io(e, path))
}

fn parse_arg_date(
    date: &Option<String>,
    field: &str,
    days: i64,
) -> Result<Option<epoch>, GtmError> {
    Ok(match date {
        None => None,
        Some(date) => Some(
            NaiveDate::parse_from_str(date, "%Y-%m-%d")
                .map_err(|e| GtmError::Parse(e, field.to_owned()))?
                .checked_add_signed(Duration::days(days))
                .unwrap()
                .and_hms(0, 0, 0)
                .timestamp(),
        ),
    })
}

fn main() -> GtmResult<GtmError> {
    let command = Args::from_args();

    match command {
        Args::Commits {
            from_date,
            to_date,
            message,
        } => {
            let from = parse_arg_date(&from_date, "from", 0)?;
            let to = parse_arg_date(&to_date, "to", 1)?;
            let out = std::io::stdout();
            let mut writer = BufWriter::with_capacity(1024 * 1024, out);
            write_commits(
                &mut writer,
                from_config()?.keys(),
                &NotesFilter {
                    from,
                    to,
                    needle: message,
                },
            )?;
        }
        Args::Projects => {
            let out = std::io::stdout();
            let writer = BufWriter::with_capacity(1024 * 1024, out);
            write_project_list(writer, &from_config()?);
        }
        Args::Status => {
            let out = std::io::stdout();
            let mut writer = BufWriter::with_capacity(1024 * 1024, out);
            write_workdir_status(&mut writer, from_config()?.keys());
        }
    };

    GtmResult(Ok(()))
}
