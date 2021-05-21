extern crate classic_crypto;

mod cli;

fn main() -> classic_crypto::error::Result<()> {
    cli::run()
}
