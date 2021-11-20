//! Handles both encrypt and decrypt submodules

use classic_crypto::cipher::{
    Affine, Atbash, Caesar, Cipher, Railfence, Rot13, Scytale, Substitution,
};

use crate::{cli::opt::CipherOpt, util, CipherConfig, EncryptDecryptOpt};

trait CliCipher {
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

fn encrypt_or_decrypt(
    cfg: &CipherConfig,
    opt: EncryptDecryptOpt,
    mode: CipherMode,
) -> anyhow::Result<()> {
    let EncryptDecryptOpt { cipher, lang, text } = opt;

    let lang = &match lang {
        Some(lang) => match cfg.load_lang(&lang) {
            Ok(lang) => Ok(lang),
            Err(_) => cfg.load_selected(),
        },
        None => cfg.load_selected(),
    }?;
    let text = util::unwrap_or_stdin(text)?;
    let cipher: Box<dyn CliCipher> = match cipher {
        CipherOpt::Affine { a, b } => Box::new(Affine::new(lang, a, b)?),
        CipherOpt::Atbash => Box::new(Atbash::identity(lang)),
        CipherOpt::Caesar { shift } => Box::new(Caesar::new(lang, shift)?),
        CipherOpt::Railfence { rails } => Box::new(Railfence::new(lang, rails)?),
        CipherOpt::Rot13 => Box::new(Rot13::identity(lang)),
        CipherOpt::Scytale { faces } => Box::new(Scytale::new(lang, faces)?),
        CipherOpt::Substitution { keyword } => Box::new(Substitution::new(lang, keyword.as_str())?),
    };
    let msg = match mode {
        CipherMode::Encrypt => cipher.encrypt(&text),
        CipherMode::Decrypt => cipher.decrypt(&text),
    };

    println!("{}", msg.trim_end());

    Ok(())
}

/// Handles the encrypt submodule
pub fn encrypt(cfg: &CipherConfig, encrypt_opt: EncryptDecryptOpt) -> anyhow::Result<()> {
    encrypt_or_decrypt(cfg, encrypt_opt, CipherMode::Encrypt)
}

/// Handles the decrypt submodule
pub fn decrypt(cfg: &CipherConfig, decrypt_opt: EncryptDecryptOpt) -> anyhow::Result<()> {
    encrypt_or_decrypt(cfg, decrypt_opt, CipherMode::Decrypt)
}
