use std::io::{self, Read};

pub fn stdin_to_string() -> anyhow::Result<String> {
    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn unwrap_or_stdin(text: Option<String>) -> anyhow::Result<String> {
    match text {
        Some(text) => Ok(text),
        None => stdin_to_string(),
    }
}
