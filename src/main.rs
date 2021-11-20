//! A command line program for encrypting, decrypting, analysing and
//! breaking classical ciphers.

#![warn(missing_docs)]

use classic_crypto::lang::Lang;
use structopt::StructOpt;

mod cli;
mod config;
mod error;
mod util;

use cli::*;
use config::CipherConfig;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut cfg = CipherConfig::load();

    match Opt::from_args() {
        Opt::Lang(lang_opt) => lang::lang(&mut cfg, lang_opt),
        Opt::Stats { .. } => Ok(()),
        Opt::Encrypt(encrypt_opt) => encrypt_and_decrypt::encrypt(&cfg, encrypt_opt),
        Opt::Decrypt(decrypt_opt) => encrypt_and_decrypt::decrypt(&cfg, decrypt_opt),
        Opt::Solve(solve_opt) => solve::solve(&cfg, solve_opt),
    }?;

    cfg.save()?;

    Ok(())
}
