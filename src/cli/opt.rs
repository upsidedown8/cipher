use structopt::StructOpt;

/// Encrypt, decrypt and solve classical ciphers.
#[derive(StructOpt)]
#[structopt(name = "cipher")]
#[structopt(rename_all = "snake")]
pub enum Opt {
    /// Manage language configuration
    Lang(LangOpt),
    /// Perform statistical analysis on a ciphertext
    Stats {
        /// Text to analyse. If not present then read from stdin
        text: Option<String>,
        #[structopt(flatten)]
        cmd: StatsOpt,
    },
    /// Encrypt a plaintext with a cipher. Ciphers are specified with
    /// the submodules
    Encrypt(EncryptOpt),
    /// Decrypt a ciphertext with a cipher. Ciphers are specified with
    /// the submodules
    Decrypt(DecryptOpt),
    /// Solve a ciphertext. Use submodules to solve a specific cipher.
    /// If no cipher is specified, the input will be solved analysing the
    /// text and trying likely ciphers
    Solve(SolveOpt),
}

#[derive(StructOpt)]
#[structopt(rename_all = "kebab")]
pub enum LangOpt {
    /// List all languages
    List,
    /// Remove a language
    #[structopt(name = "rm")]
    Remove {
        /// Name of the language to remove
        #[structopt(short, long)]
        name: String,
        /// Remove without asking for confirmation
        #[structopt(short, long)]
        force: bool,
    },
    /// Add a new language
    Add {
        /// Name of the language to add
        #[structopt(short, long)]
        name: String,
        /// Uppercase alphabet
        #[structopt(short, long)]
        upper: String,
        /// Lowercase alphabet
        #[structopt(short, long)]
        lower: String,
        /// Text corpus, if not present then read from stdin
        #[structopt(short, long)]
        corpus: Option<String>,
    },
    /// Change an existing language, adding or overwriting a cipher
    /// alphabet
    Alphabet {
        /// Name of the language to add the alphabet to
        #[structopt(short, long)]
        name: String,
        /// Uppercase alphabet
        #[structopt(short, long)]
        upper: String,
        /// Lowercase alphabet
        #[structopt(short, long)]
        lower: String,
        /// Uppercase letters which should be removed from the alphabet
        /// when scoring
        #[structopt(long)]
        discard_upper: String,
        /// Lowercase letters which should be removed from the alphabet
        /// when scoring
        #[structopt(long)]
        discard_lower: String,
        /// Text corpus, if not present then read from stdin
        #[structopt(short, long)]
        corpus: Option<String>,
    },
}

#[derive(StructOpt)]
#[structopt(rename_all = "kebab")]
pub enum StatsOpt {
    /// Display a graph showing periodic index of coincedence
    Periodic,
    /// Display a chart showing letter frequency
    Freq,
}

#[derive(StructOpt)]
pub struct EncryptOpt {
    /// The cipher to encrypt with
    #[structopt(subcommand)]
    cipher: CipherOpt,
    /// If present, encrypts with a random cipher state (overrides all key fields
    /// and the identity option)
    #[structopt(global = true, short, long)]
    random: bool,
    /// If present, decrypts with an identity version of the cipher (overrides key
    /// fields)
    #[structopt(global = true, short, long)]
    identity: bool,
    /// The text to solve, if not specified then read from stdin
    #[structopt(global = true)]
    text: Option<String>,
}

#[derive(StructOpt)]
pub struct DecryptOpt {
    /// The cipher to decrypt with
    #[structopt(flatten)]
    cipher: CipherOpt,
    /// If present, encrypts with a random cipher state (overrides all key fields
    /// and the identity option)
    #[structopt(global = true, short, long)]
    random: bool,
    /// If present, decrypts with an identity version of the cipher (overrides key
    /// fields)
    #[structopt(global = true, short, long)]
    identity: bool,
    /// The text to solve, if not specified then read from stdin
    #[structopt(global = true)]
    text: Option<String>,
}

#[derive(StructOpt)]
pub struct SolveOpt {
    /// The cipher to solve as. If not specified, the message will be
    /// automatically solved
    #[structopt(subcommand)]
    cipher: Option<CipherSolveOpt>,
    /// A crib to aid in solving. This may not always be used
    #[structopt(global = true, short, long)]
    crib: Option<String>,
    /// The position of the crib within the ciphertext
    #[structopt(global = true, long, requires("crib"))]
    crib_pos: Option<usize>,
    /// Display the key once solved
    #[structopt(global = true, short = "k", long)]
    show_key: bool,
    /// Hide the plaintext once solved
    #[structopt(global = true, short = "T", long)]
    no_plain: bool,
    /// The text to solve, if not specified then read from stdin
    #[structopt(global = true)]
    text: Option<String>,
}

#[derive(StructOpt)]
pub enum CipherOpt {
    /// Encrypts each letter with ax+b
    #[structopt(name = "affine")]
    Affine {
        /// Affine coefficient, a
        #[structopt(short, long)]
        a: usize,
        /// Affine constant, b
        #[structopt(short, long)]
        b: usize,
    },
}

#[derive(StructOpt)]
#[structopt(rename_all = "snake")]
pub enum CipherSolveOpt {
    Affine,
}
