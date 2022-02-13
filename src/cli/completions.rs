use super::opt;
use clap::IntoApp;
use clap_complete::Shell;
use std::path::PathBuf;

pub fn completions(output: Option<PathBuf>, shell: opt::Shell) {
    let mut app = opt::Opt::into_app();

    let shell = match shell {
        opt::Shell::Bash => Shell::Bash,
        opt::Shell::PowerShell => Shell::PowerShell,
    };

    match output
        .map(|path| {
            std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(path)
                .ok()
        })
        .flatten()
    {
        Some(mut file) => clap_complete::generate(shell, &mut app, "cipher", &mut file),
        None => clap_complete::generate(shell, &mut app, "cipher", &mut std::io::stdout()),
    };
}
