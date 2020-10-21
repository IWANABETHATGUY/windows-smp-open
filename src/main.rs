use clap::{App, Arg};
use std::{env::current_dir, process::Command};
fn main() -> Result<(), std::io::Error> {
    let matches = App::new("windows-smb-open")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("")
        .arg(
            Arg::with_name("link")
                .value_name("link")
                .help("a smb protocol link")
                .required(true),
        )
        .get_matches();
    if let Some(value) = matches.value_of("link") {
        let value = value.trim();
        let mut res = value.replace("/", "\\");
        if res.starts_with("smb:") {
            res = res[4..].to_string();
        }
        Command::new("explorer")
            .current_dir(&current_dir()?)
            .args(vec![res])
            .spawn()?;
    } else {
        eprintln!("smb link is required");
    }
    Ok(())
}
