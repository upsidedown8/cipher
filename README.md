# **cipher**

### A command line tool for encrypting, decrypting, solving and analysing classical ciphers.

## Usage
```
$ cargo install cipher --git https://github.com/upsidedown8/cipher

$ cipher

cipher 0.1.0
Encrypt, decrypt, analyse and solve classical ciphers

USAGE:
    cipher <SUBCOMMAND>

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
