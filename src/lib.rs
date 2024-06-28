use anyhow::{Context, Result};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
pub struct Cli {
    /// the pattern to search for
    pub query: String,
    /// the path to the file to read
    pub file_path: std::path::PathBuf,
}



pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).context("could not write to writer")?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::find_matches;

    #[test]
    fn find_a_match() {
        let mut result = Vec::new();
        find_matches("It is a beautiful day!\nLet us rejoice!", "beautiful", &mut result).unwrap();
        assert_eq!(result, b"It is a beautiful day!\n");
    }
}