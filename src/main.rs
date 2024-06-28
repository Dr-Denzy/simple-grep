use anyhow::{Context, Result};
use clap::Parser;

use simplegrep::{Cli, find_matches};

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.file_path)
        .with_context(|| format!("could not read file `{:?}`", args.file_path.display()))?;

    find_matches(&content, &args.query, &mut std::io::stdout().lock())?;

    Ok(())
}



