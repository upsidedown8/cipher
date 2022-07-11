use crate::{config::CipherConfig, util::stdin_to_string};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Stores the value log(f / N) for each word.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Words {
    words: HashMap<String, f32>,
}

impl Words {
    pub fn new(corpus: &str) -> Self {
        let words = crate::cli::corpus::words(corpus);
        let counts = crate::cli::corpus::frequencies(words);
        let sum = counts.values().sum::<usize>() as f32;

        Self {
            words: counts
                .into_iter()
                .map(|(k, v)| (k, (v as f32) / sum))
                .collect(),
        }
    }
    pub fn max_len(&self) -> usize {
        self.words.keys().map(|k| k.len()).max().unwrap_or(1)
    }
    pub fn score(&self, word: &str) -> f32 {
        self.words[word]
    }
}

pub fn segment(
    cfg: &CipherConfig,
    text: Option<String>,
    lang: Option<String>,
) -> anyhow::Result<()> {
    let words = cfg.load_words_or_selected(lang)?;

    println!(
        "{}",
        segment_str(
            &match text {
                Some(text) => text,
                None => stdin_to_string()?,
            },
            &words
        )
    );

    Ok(())
}

/// Returns a probability value for a word which should be maximised.
fn score_word(words: &Words, word: &[u8]) -> f32 {
    let word = std::str::from_utf8(word).unwrap();
    words.score(word)
}

/// Finds the max of a and b.
fn max(a: (f32, usize), b: (f32, usize)) -> (f32, usize) {
    match a >= b {
        true => a,
        false => b,
    }
}

pub fn segment_str(s: &str, words: &Words) -> String {
    let max_len = words.max_len();
    let text = s
        .chars()
        .filter(|&c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase() as u8)
        .collect::<Vec<_>>();

    // best[n] = (word probability, length of best word ending at n+1).
    let mut best: Vec<(f32, usize)> = Vec::with_capacity(text.len() + 1);

    for i in 0..text.len() {
        best.push((0.0, 0));

        for word_len in 1..=max_len {
            let score = best[i - word_len].0 * score_word(words, &text[i..][..word_len]);
            best[i] = max(best[i], (score, word_len));
        }
    }

    // read from the best scores to find the correct segmentation.
    let mut segmented = Vec::with_capacity(text.len());
    let mut length = text.len();

    while length > 0 {
        let (_, word_len) = best[length - 1];
        length -= word_len;
        segmented.extend_from_slice(&text[length..][..word_len]);
        segmented.push(b' ');
    }

    // guarunteed to be an ASCII string
    String::from_utf8(segmented).unwrap()
}
