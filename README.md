# **cipher**

### A command line tool for encrypting, decrypting, solving and analysing classical ciphers.

## Usage
```
$ cargo install cipher --git https://github.com/upsidedown8/cipher
$ cipher --help

cipher 

Tom Thorogood <tomthorogood@outlook.com>

Encrypt, decrypt and solve classical ciphers

USAGE:
    cipher <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    completions    Generate completion scripts
    decrypt        Decrypt a ciphertext with a cipher. Ciphers are specified with the submodules
    encrypt        Encrypt a plaintext with a cipher. Ciphers are specified with the submodules
    help           Print this message or the help of the given subcommand(s)
    lang           Manage language configuration
    solve          Solve a ciphertext. Use submodules to solve a specific cipher. If no cipher is specified, the input
                   will be solved by analysing the text and trying likely ciphers
    stats          Perform statistical analysis on a ciphertext
```
