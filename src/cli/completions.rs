use super::opt;
use clap::IntoApp;
use clap_complete::Shell;
use std::path::PathBuf;

const BIN_NAME: &str = "crypto";

pub fn completions(output: Option<PathBuf>, shell: opt::Shell) {
    let mut app = opt::Opt::into_app();

    let shell = match shell {
        opt::Shell::Bash => Shell::Bash,
        opt::Shell::PowerShell => Shell::PowerShell,
    };

    match output.and_then(|path| {
        std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)
            .ok()
    }) {
        Some(mut file) => clap_complete::generate(shell, &mut app, BIN_NAME, &mut file),
        None => clap_complete::generate(shell, &mut app, BIN_NAME, &mut std::io::stdout()),
    };
}
