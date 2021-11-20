use crate::{util, CipherConfig, Lang, LangOpt};
use std::io::{stdin, stdout, Read, Write};

/// Handles the lang submodule
pub fn lang(cfg: &mut CipherConfig, lang_opt: LangOpt) -> anyhow::Result<()> {
    match lang_opt {
        LangOpt::Add {
            name,
            upper,
            lower,
            corpus,
        } => {
            let corpus = match corpus {
                Some(corpus) => corpus,
                None => util::stdin_to_string()?,
            };

            let lang = Lang::new(upper, lower, &corpus)?;

            cfg.add_lang(name, &lang)?;
        }
        LangOpt::List => {
            for name in cfg.lang_names() {
                println!("{}", name);
            }
        }
        LangOpt::Remove { name, force } => {
            let name = name.trim();

            let exists = cfg.lang_names().any(|n| n == name);
            let rm = !exists || force || {
                print!("delete {}? [y/N] ", name);
                stdout().flush()?;

                let mut buf = [0; 1];
                stdin().read_exact(&mut buf)?;

                matches!(buf[0], b'y' | b'Y')
            };

            if rm {
                cfg.rm_lang(name)?;
            }
        }
        LangOpt::Alphabet {
            upper,
            lower,
            discard_lower,
            discard_upper,
            corpus,
            name,
        } => {
            todo!()
        }
    }

    Ok(())
}
