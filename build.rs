use chrono::Utc;
use std::env;
use std::process::Command;

fn main() {
    let now = Utc::now();

    let sha = run_command(Command::new("git").args(&["rev-parse", "--short", "HEAD"]));
    println!("cargo:rustc-env=GTM_SHA_SHORT={}", sha);

    let core = env::var("CARGO_PKG_VERSION").unwrap();
    let profile = if cfg!(debug_assertions) {
        "dev"
    } else {
        "prod"
    };
    let build_timestamp = now.format("%Y-%m-%d").to_string();
    println!(
        "cargo:rustc-env=GTM_VERSION={core}-{profile}+g{sha}.{build_timestamp}",
        core = core,
        profile = profile,
        sha = sha,
        build_timestamp = build_timestamp,
    );
}

fn run_command(command: &mut Command) -> String {
    if let Ok(o) = command.output() {
        if o.status.success() {
            return String::from_utf8_lossy(&o.stdout).trim().to_owned();
        }
    }
    return "UNKNOWN".to_owned();
}
