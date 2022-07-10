use crate::util::stdin_to_string;
use std::{
    collections::hash_map::{Entry, HashMap},
    fs,
    io::{self, Write},
    path::PathBuf,
};

pub fn corpus(file: Option<PathBuf>, out: PathBuf) -> anyhow::Result<()> {
    let corpus = match file {
        Some(path) => std::fs::read_to_string(path)?,
        None => stdin_to_string()?,
    };

    // find all words
    let r_find = regex::Regex::new(r"\b[a-zA-Z]+('[a-zA-Z'])?\b")?;
    let words: Vec<_> = r_find
        .find_iter(&corpus)
        .map(|m| m.as_str().to_lowercase())
        .collect();

    // write word file
    let f_words_path = out.join("words.txt");
    println!("writing {}", f_words_path.to_string_lossy());
    let mut f_words = io::BufWriter::new(fs::File::create(f_words_path)?);
    for w in &words {
        write!(f_words, "{w} ")?;
    }

    let mut freqs = HashMap::new();
    for w in &words {
        match freqs.entry(w) {
            Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
            }
            Entry::Vacant(e) => {
                e.insert(1);
            }
        }
    }

    let mut freqs = freqs.into_iter().collect::<Vec<_>>();
    freqs.sort_unstable_by_key(|&(_, f)| usize::MAX - f);

    // write frequency file
    let f_frequency_path = out.join("freqs.txt");
    println!("writing {}", f_frequency_path.to_string_lossy());
    let mut f_frequency = io::BufWriter::new(fs::File::create(f_frequency_path)?);
    for (w, freq) in freqs {
        writeln!(f_frequency, "{w}\t{freq}")?;
    }

    Ok(())
}
