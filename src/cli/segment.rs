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
                .map(|(k, v)| {
                    let cost = ((v as f32) / sum * (k.len() as f32 + 1.0).ln()).ln();
                    (k, cost)
                })
                .collect(),
        }
    }
    pub fn max_len(&self) -> usize {
        self.words.keys().map(|k| k.len()).max().unwrap_or(1)
    }
    pub fn score(&self, word: &str) -> f32 {
        match self.words.get(word) {
            Some(&score) => score,
            None => -10.0 * word.len() as f32,
        }
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
    match a > b {
        true => a,
        false => b,
    }
}

/// Algorithm inspired by: https://stackoverflow.com/questions/8870261/how-to-split-text-without-spaces-into-list-of-words/11642687#11642687
pub fn segment_str(s: &str, words: &Words) -> String {
    let max_len = words.max_len();
    let text = s
        .chars()
        .filter(|&c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase() as u8)
        .collect::<Vec<_>>();

    // best[n] = (word probability, length of best word ending at n+1).
    let mut best: Vec<(f32, usize)> = Vec::with_capacity(text.len());

    for i in 0..=text.len() {
        best.push((if i == 0 { 0.0 } else { -10_000.0 }, 0));

        for word_len in 1..=max_len.min(best.len()).min(i) {
            let word = &text[i.min(i - word_len)..i];
            let score = match i {
                0 => score_word(words, word),
                i => score_word(words, word) + best[i - word_len].0,
            };
            best[i] = max((score, word_len), best[i]);
        }
    }

    // read from the best scores to find the correct segmentation.
    let mut words = Vec::new();
    let mut length = text.len();

    while length > 0 {
        let (_, word_len) = best[length];
        length -= word_len;
        words.push(&text[length..][..word_len]);
    }

    words.reverse();

    let segmented = words.join(&b' ');

    // guarunteed to be an ASCII string
    String::from_utf8(segmented).unwrap()
}
