use crate::{
    cli::{
        crypt::CliCipher,
        opt::{CipherSolveCmd, StatsSizeOpt},
    },
    util, CipherConfig, Opt,
};
use classic_crypto::{cipher::*, lang::StatsSize};

pub fn solve(cfg: &CipherConfig, solve_opt: Opt) -> anyhow::Result<()> {
    #[allow(unused)]
    if let Opt::Solve {
        cipher,
        crib,
        crib_pos,
        show_key,
        no_plain,
        stats_size,
        lang,
        text,
    } = solve_opt
    {
        let lang = &cfg.load_lang_or_selected(lang)?;
        let text = &util::unwrap_or_stdin(text)?;
        let stats_size = match stats_size {
            Some(StatsSizeOpt::Unigrams) => StatsSize::Unigrams,
            Some(StatsSizeOpt::Bigrams) => StatsSize::Unigrams,
            Some(StatsSizeOpt::Trigrams) => StatsSize::Unigrams,
            _ => StatsSize::Quadgrams,
        };
        let solution: Box<dyn CliCipher> = match cipher {
            None => todo!(),
            Some(opt) => match opt {
                CipherSolveCmd::Affine => Box::new(Affine::solve(lang, text, stats_size)),
                CipherSolveCmd::Atbash => Box::new(Atbash::solve(lang, text, ())),
                CipherSolveCmd::Caesar => Box::new(Caesar::solve(lang, text, stats_size)),
                CipherSolveCmd::ClassicVigenere { max_key_length } => {
                    Box::new(ClassicVigenere::solve(
                        lang,
                        text,
                        ClassicVigenereSolve {
                            stats_size,
                            max_key_length,
                        },
                    ))
                }
                CipherSolveCmd::Railfence => Box::new(Railfence::solve(lang, text, stats_size)),
                CipherSolveCmd::Rot13 => Box::new(Rot13::solve(lang, text, ())),
                CipherSolveCmd::Scytale => Box::new(Scytale::solve(lang, text, stats_size)),
                CipherSolveCmd::Substitution {
                    max_iterations,
                    min_repetitions,
                } => Box::new(Substitution::solve(
                    lang,
                    text,
                    SubstitutionSolve {
                        stats_size,
                        max_iterations,
                        min_repetitions,
                    },
                )),
            },
        };

        if show_key {
            println!("{}", solution);
        }

        if !no_plain {
            println!("{}", solution.decrypt(text).trim_end());
        }
    }

    Ok(())
}
