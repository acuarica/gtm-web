use std::process::Command;
use std::env;

fn main() {
    let sha = run_command(Command::new("git").args(&["rev-parse", "--short", "HEAD"]));
    println!("cargo:rustc-env=GTM_SHA_SHORT={}", sha);

    let semver = env::var("CARGO_PKG_VERSION").unwrap();

    println!("cargo:rustc-env=GTM_VERSION={}-dev-{}", semver, sha);
}

fn run_command(command: &mut Command) -> String {
    if let Ok(o) = command.output() {
        if o.status.success() {
            return String::from_utf8_lossy(&o.stdout).trim().to_owned();
        }
    }
    return "UNKNOWN".to_owned();
}
