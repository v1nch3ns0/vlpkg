use std::fs::{OpenOptions, read_to_string, write};
use std::io::Write;
use std::path::Path;

const LOG_FILE: &str = "/var/lib/vlpkg/installed.log";

pub fn log_install(name: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)?;
    
    writeln!(file, "{}", name)?;
    Ok(())
}

pub fn remove_log(name: &str) -> std::io::Result<()> {
    if !Path::new(LOG_FILE).exists() {
        return Ok(());
    }

    let contents = read_to_string(LOG_FILE)?;
    let filtered: String = contents
        .lines()
        .filter(|line| line != &name)
        .map(|line| format!("{}\n", line))
        .collect();

    write(LOG_FILE, filtered)?;
    Ok(())
}
