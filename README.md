# **classic_crypto_cli**

### A command line tool for encrypting, decrypting, solving and analysing classical ciphers.

## Install
```
cargo install classic_crypto_cli --git https://github.com/upsidedown8/classic_crypto_cli
```
## Usage
```
$ classic_crypto

classic_crypto 0.1.0
Encrypt, decrypt, analyse and solve classical ciphers

USAGE:
    classic_crypto_cli <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    analyse       Analyse a ciphertext
    auto_solve    Solve a message encrypted with an unknown cipher
    decrypt       Decrypt a message using a cipher
    encrypt       Encrypt a message using a cipher
    help          Prints this message or the help of the given subcommand(s)
    lang_gen      Generate a Langauge file from a configuration file and text corpus
    solve         Solve a message encrypted with a known cipher
```
