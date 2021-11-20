// use crate::{util, CipherConfig, StatsOpt};

// pub fn stats(cfg: &CipherConfig, stats_opt: StatsOpt) -> anyhow::Result<()> {
//     let lang = cfg.load_selected()?;

//     let StatsOpt {
//         periodic,
//         freq,
//         text,
//     } = stats_opt;

//     let text = match text {
//         Some(text) => text,
//         None => util::stdin_to_string()?,
//     };

//     Ok(())
// }
