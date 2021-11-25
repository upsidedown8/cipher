//! Handles both encrypt and decrypt submodules

use classic_crypto::cipher::{
    Affine, Atbash, Caesar, Cipher, Railfence, Rot13, Scytale, Substitution,
};

use crate::{cli::opt::CipherCmd, util, CipherConfig, CryptCmd};

pub trait CliCipher
where
    Self: std::fmt::Display,
{
    fn encrypt(&self, msg: &str) -> String;
    fn decrypt(&self, msg: &str) -> String;
}

impl<'l, C> CliCipher for C
where
    C: Cipher<'l>,
{
    fn encrypt(&self, msg: &str) -> String {
        self.encrypt(msg)
    }

    fn decrypt(&self, msg: &str) -> String {
        self.decrypt(msg)
    }
}

enum CipherMode {
    Encrypt,
    Decrypt,
}

fn crypt(cfg: &CipherConfig, opt: CryptCmd, mode: CipherMode) -> anyhow::Result<()> {
    let CryptCmd { cipher, lang, text } = opt;

    let lang = &match lang {
        Some(lang) => match cfg.load_lang(&lang) {
            Ok(lang) => Ok(lang),
            Err(_) => cfg.load_selected(),
        },
        None => cfg.load_selected(),
    }?;
    let text = util::unwrap_or_stdin(text)?;
    let cipher: Box<dyn CliCipher> = match cipher {
        CipherCmd::Affine { a, b } => Box::new(Affine::new(lang, a, b)?),
        CipherCmd::Atbash => Box::new(Atbash::identity(lang)),
        CipherCmd::Caesar { shift } => Box::new(Caesar::new(lang, shift)?),
        CipherCmd::Railfence { rails } => Box::new(Railfence::new(lang, rails)?),
        CipherCmd::Rot13 => Box::new(Rot13::identity(lang)),
        CipherCmd::Scytale { faces } => Box::new(Scytale::new(lang, faces)?),
        CipherCmd::Substitution { keyword } => Box::new(Substitution::new(lang, keyword.as_str())?),
    };
    let msg = match mode {
        CipherMode::Encrypt => cipher.encrypt(&text),
        CipherMode::Decrypt => cipher.decrypt(&text),
    };

    println!("{}", msg.trim_end());

    Ok(())
}

/// Handles the encrypt submodule
pub fn encrypt(cfg: &CipherConfig, encrypt_opt: CryptCmd) -> anyhow::Result<()> {
    crypt(cfg, encrypt_opt, CipherMode::Encrypt)
}

/// Handles the decrypt submodule
pub fn decrypt(cfg: &CipherConfig, decrypt_opt: CryptCmd) -> anyhow::Result<()> {
    crypt(cfg, decrypt_opt, CipherMode::Decrypt)
}
