use std::io::{self, Read};

pub fn stdin_to_string() -> anyhow::Result<String> {
    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf)?;
    Ok(buf)
}
