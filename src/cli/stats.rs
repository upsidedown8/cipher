use classic_crypto::{
    analysis,
    lang::{with_alphabet::StatsSize, AlphabetLen},
};
use colorful::{Colorful, HSL};

use crate::{util, CipherConfig, StatsCmd};

pub fn stats(
    cfg: &CipherConfig,
    lang: Option<String>,
    text: Option<String>,
    stats_opt: StatsCmd,
) -> anyhow::Result<()> {
    let lang = cfg.load_lang_or_selected(lang)?;
    let text = util::unwrap_or_stdin(text)?;
    let alph = lang.with_alphabet(AlphabetLen::Any);
    let cp = alph.code_points(&text).collect::<Vec<_>>();
    let len = cp.len();

    match stats_opt {
        StatsCmd::Periodic { width, alphabet: _ } => {
            let expected = alph.expected_ioc();
            let ioc: Vec<_> = (1..len.min(100))
                .map(|p| analysis::periodic_ioc(cp.iter().map(|&x| x as usize), p))
                .take_while(|&x| x.is_normal())
                .collect();
            let max = ioc.iter().copied().fold(0.0, f32::max);

            let total_width = width.clamp(20, 100);
            let bar_width = (total_width - 11) as f32;

            //        00 0.00000 ---------------
            if !ioc.is_empty() {
                println!(" p ioc     bar");
            }

            for (period, &ioc) in ioc.iter().enumerate() {
                let width = ((ioc / max) * bar_width).floor() as usize;
                let diff_proportion = (expected - ioc).abs() / expected;

                let bar = format!(
                    "{:>2} {:<6.05} {}",
                    period + 1,
                    ioc,
                    "━"
                        .repeat(width)
                        .gradient(HSL::new(1.0 - diff_proportion, 1.0, 0.5))
                );

                if diff_proportion < 0.2 {
                    println!("{}", bar.bold());
                } else {
                    println!("{}", bar);
                }
            }
        }
        StatsCmd::Freq {
            alphabet: _,
            punct: _,
            whitespace: _,
        } => {
            todo!()
        }
        StatsCmd::Length { alphabet: _ } => todo!(),
        StatsCmd::Ioc { alphabet: _ } => {
            println!("{}", analysis::ioc(cp.iter().map(|&x| x as usize)));
        }
        StatsCmd::ChiSquared => {
            println!("{}", alph.chi_squared(cp));
        }
        StatsCmd::Unigram => {
            println!("{}", alph.score(cp, StatsSize::Unigrams));
        }
        StatsCmd::Bigram => {
            println!("{}", alph.score(cp, StatsSize::Bigrams));
        }
        StatsCmd::Trigram => {
            println!("{}", alph.score(cp, StatsSize::Trigrams));
        }
        StatsCmd::Quadgram => {
            println!("{}", alph.score(cp, StatsSize::Quadgrams));
        }
    }

    Ok(())
}
