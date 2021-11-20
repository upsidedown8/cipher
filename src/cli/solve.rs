use crate::{util, CipherConfig, Opt};

pub fn solve(cfg: &CipherConfig, solve_opt: Opt) -> anyhow::Result<()> {
    match solve_opt {
        #[allow(unused)]
        Opt::Solve {
            cipher,
            crib,
            crib_pos,
            show_key,
            no_plain,
            lang,
            text,
        } => {
            let text = util::unwrap_or_stdin(text)?;

            Ok(())
        }
        _ => unreachable!(),
    }
}
