use std::path::PathBuf;

use clap::{ArgEnum, Args, Parser, Subcommand};

/// Encrypt, decrypt and solve classical ciphers.
#[derive(Parser, Debug)]
#[clap(name = "cipher", author = "Tom Thorogood <tomthorogood@outlook.com>")]
pub enum Opt {
    /// Generate completion scripts
    Completions {
        /// Output file to write completion to, if unspecified then writes to
        /// stdout
        #[clap(short, long, parse(from_os_str))]
        output: Option<PathBuf>,
        /// Shell to generate completions for
        #[clap(arg_enum)]
        shell: Shell,
    },
    /// Manage language configuration
    Lang {
        #[clap(subcommand)]
        sub: LangCmd,
    },
    /// Perform statistical analysis on a ciphertext
    Stats {
        /// If present, overrides the selected lang and uses the value given
        #[clap(global = true, short, long)]
        lang: Option<String>,
        /// Text to analyse. If not present then read from stdin
        #[clap(global = true, short, long)]
        text: Option<String>,
        #[clap(subcommand)]
        cmd: StatsCmd,
    },
    /// Encrypt a plaintext with a cipher. Ciphers are specified with
    /// the submodules
    Encrypt(CryptCmd),
    /// Decrypt a ciphertext with a cipher. Ciphers are specified with
    /// the submodules
    Decrypt(CryptCmd),
    /// Solve a ciphertext. Use submodules to solve a specific cipher.
    /// If no cipher is specified, the input will be solved analysing the
    /// text and trying likely ciphers
    Solve {
        /// The cipher to solve as. If not specified, the message will be
        /// automatically solved
        #[clap(subcommand)]
        cipher: Option<CipherSolveCmd>,
        /// A crib to aid in solving. This may not always be used
        #[clap(global = true, short, long)]
        crib: Option<String>,
        /// The position of the crib within the ciphertext
        #[clap(global = true, long, short = 'p', requires("crib"))]
        crib_pos: Option<usize>,
        /// Display the key once solved
        #[clap(global = true, short = 'k', long)]
        show_key: bool,
        /// Hide the plaintext once solved
        #[clap(global = true, short = 'T', long)]
        no_plain: bool,
        /// If present, overrides the selected lang and uses the value given
        #[clap(global = true, short, long)]
        lang: Option<String>,
        /// The text to solve, if not specified then read from stdin
        #[clap(global = true, short, long)]
        text: Option<String>,
    },
}

#[derive(ArgEnum, Clone, Copy, Debug)]
pub enum Shell {
    Bash,
    PowerShell,
}

#[derive(Subcommand, Debug)]
pub enum LangCmd {
    /// List all languages
    List,
    /// Select a language
    Set {
        /// Name of the language
        #[clap(short, long)]
        lang: String,
    },
    /// Select an alphabet. You can view the current selection with `lang list`
    SetAlph {
        /// Language to select alphabet for
        #[clap(short, long)]
        lang: Option<String>,
        /// Length of alphabet to select
        #[clap(short, long)]
        length: usize,
    },
    /// Remove a language
    #[clap(name = "rm")]
    Remove {
        /// Name of the language to remove
        #[clap(short, long)]
        name: String,
        /// Remove without asking for confirmation
        #[clap(short, long)]
        force: bool,
    },
    /// Add a new language
    New {
        /// Name of the language to add
        #[clap(short, long)]
        name: String,
        /// Uppercase alphabet
        #[clap(short, long)]
        upper: String,
        /// Lowercase alphabet
        #[clap(short, long)]
        lower: String,
        /// Text corpus, if not present then read from stdin
        #[clap(short, long)]
        corpus: Option<String>,
    },
    /// Change an existing language, adding or overwriting a cipher
    /// alphabet
    Alphabet {
        /// Name of the language to add the alphabet to
        #[clap(short, long)]
        name: String,
        /// Uppercase alphabet
        #[clap(short, long)]
        upper: String,
        /// Lowercase alphabet
        #[clap(short, long)]
        lower: String,
        /// Uppercase letters which should be removed from the alphabet
        /// when scoring
        #[clap(long)]
        discard_upper: String,
        /// Lowercase letters which should be removed from the alphabet
        /// when scoring
        #[clap(long)]
        discard_lower: String,
        /// Text corpus, if not present then read from stdin
        #[clap(short, long)]
        corpus: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum StatsCmd {
    /// Display a graph showing periodic index of coincedence
    Periodic {
        /// If present, sets the width of the graph
        #[clap(short, long, default_value = "60")]
        width: usize,
        /// If present, consider the the characters given rather than the
        /// language's alphabet
        #[clap(short, long)]
        alphabet: Option<String>,
    },
    /// Display a chart showing letter frequency
    Freq {
        /// If present, also show frequencies for whitespace characters
        #[clap(short, long)]
        whitespace: bool,
        /// If present, also show frequencies for all other (non-whitespace)
        /// characters
        #[clap(short, long)]
        punct: bool,
        /// If present, consider the the characters given rather than the
        /// language's alphabet
        #[clap(short, long)]
        alphabet: Option<String>,
    },
    /// Display the index of coincedence of the text
    Ioc {
        /// If present, consider the the characters given rather than the
        /// language's alphabet
        #[clap(short, long)]
        alphabet: Option<String>,
    },
    /// Display the text length and its factors
    Length {
        /// If present, consider the the characters given rather than the
        /// language's alphabet
        #[clap(short, long)]
        alphabet: Option<String>,
    },
    /// Display the chi squared value for the text
    ChiSquared,
    /// Display the Unigram score for the text
    Unigram,
    /// Display the Bigram score for the text
    Bigram,
    /// Display the Trigram score for the text
    Trigram,
    /// Display the Quadgram score for the text
    Quadgram,
}

#[derive(Args, Debug)]
pub struct CryptCmd {
    /// The algorithm to use
    #[clap(subcommand)]
    pub cipher: CipherCmd,
    /// If present, overrides the selected lang and uses the value given
    #[clap(global = true, short, long)]
    pub lang: Option<String>,
    /// The text to encrypt/decrypt, if not specified then read from stdin
    #[clap(global = true, short, long)]
    pub text: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum CipherCmd {
    /// The Affine cipher
    Affine {
        /// Affine coefficient, a
        #[clap(short, long)]
        a: i32,
        /// Affine constant, b
        #[clap(short, long)]
        b: i32,
    },
    /// The Atbash cipher
    Atbash,
    /// The Caesar cipher
    Caesar {
        /// Caesar shift
        #[clap(short, long)]
        shift: i32,
    },
    /// The Railfence cipher
    Railfence {
        /// Number of rails
        #[clap(short, long)]
        rails: i32,
    },
    /// The Rot13 cipher
    Rot13,
    /// The Scytale cipher
    Scytale {
        /// Number of faces
        #[clap(short, long)]
        faces: i32,
    },
    /// The Substitution cipher
    Substitution {
        /// Keyword or alphabet
        #[clap(short, long)]
        keyword: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum CipherSolveCmd {
    Affine,
}
