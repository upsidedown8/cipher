use clap::Shell;
use std::env;
use std::io::Error;
use std::path::Path;
use structopt::StructOpt;

#[path = "cli/opt.rs"]
mod opt;

fn main() -> Result<(), Error> {
    use Shell::*;

    let variants = [Bash, Fish, Zsh, PowerShell, Elvish];

    let completions_path = Path::new("target").join("completions");
    std::fs::create_dir_all(&completions_path)?;

    for sh in variants {
        opt::Opt::clap().gen_completions(env!("CARGO_PKG_NAME"), sh, &completions_path);
    }

    Ok(())
}
