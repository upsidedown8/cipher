use crate::{util, CipherConfig, Lang, LangCmd};
use std::io::{stdin, stdout, Read, Write};

use super::segment::Words;

/// Handles the lang submodule
pub fn lang(cfg: &mut CipherConfig, lang_opt: LangCmd) -> anyhow::Result<()> {
    match lang_opt {
        LangCmd::Set { lang } => {
            cfg.set_selected(&lang)?;
        }
        LangCmd::SetAlph { lang, length } => {
            cfg.set_primary_alph(lang, length)?;
        }
        LangCmd::New {
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
            let words = Words::new(&corpus);

            cfg.add_lang(name, &lang, &words)?;
        }
        LangCmd::List => {
            let selected = cfg.selected_lang();

            for name in cfg.lang_names() {
                if selected == Some(name) {
                    print!("* ");
                } else {
                    print!("  ");
                }

                print!("{}", name);
                if let Some(meta) = cfg.lang_meta(name) {
                    print!(
                        " primary:{}, alphabets:[{}]",
                        meta.primary,
                        meta.alphabets.iter().map(|a| a.to_string()).fold(
                            String::new(),
                            |mut acc, x| {
                                acc.push_str(&x);
                                acc.push(',');
                                acc
                            }
                        )
                    );
                }

                println!();
            }
        }
        LangCmd::Remove { name, force } => {
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
        #[allow(unused)]
        LangCmd::Alphabet {
            upper,
            lower,
            discard_lower,
            discard_upper,
            corpus,
            name,
        } => {
            // let corpus = util::unwrap_or_stdin(corpus)?;
            // let mut lang = cfg.load_lang(&name)?;

            // let alphabet = AlphabetBuilder::new(upper, lower, &corpus, &lang)?
            //     .add_sub(cp, primary_cp);

            todo!()
        }
    }

    Ok(())
}
