//! A command line program for encrypting, decrypting, analysing and
//! breaking classical ciphers.

use classic_crypto::lang::Lang;

mod cli;
mod config;
mod error;
mod util;

use clap::Parser;
use cli::*;
use config::CipherConfig;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut cfg = CipherConfig::load();

    match Opt::parse() {
        Opt::Corpus { file, out } => corpus::corpus(file, out),
        Opt::Segment { text, lang } => segment::segment(&cfg, text, lang),
        Opt::Completions { output, shell } => {
            completions::completions(output, shell);
            Ok(())
        }
        Opt::Encrypt(encrypt_opt) => crypt::encrypt(&cfg, encrypt_opt),
        Opt::Decrypt(decrypt_opt) => crypt::decrypt(&cfg, decrypt_opt),
        Opt::Lang { sub } => lang::lang(&mut cfg, sub),
        solve @ Opt::Solve { .. } => solve::solve(&cfg, solve),
        Opt::Stats { lang, text, cmd } => stats::stats(&cfg, lang, text, cmd),
    }?;

    cfg.save()?;

    Ok(())
}
