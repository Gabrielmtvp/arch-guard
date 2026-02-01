mod formatters;
mod rules;
mod utils;

use anyhow::Result;
use clap::Parser;
use colored::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "zecure-guard")]
#[command(author = "Zecure Team")]
#[command(version = "0.1.0")]
#[command(about = "Architecture linter for Zecure Admin", long_about = None)]

struct Cli {
    /// Directory to check (defaults to current directory)
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Only show violations, no success messages
    #[arg(short, long)]
    quiet: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Print Header
    if !cli.quiet {
        println!("{}", "🛡️  Zecure Guard - Architecture Linter".bold().cyan());
        println!("{}", "━".repeat(50).dimmed());
        println!();
    }

    let php_files = utils::file_finder::find_php_files(&cli.path)?;

    if cli.verbose {
        println!("Found {} PHP files to check\n", php_files.len());
    }

    // Run all checks
    let mut all_violations = Vec::new();

    for file in &php_files {
        // Skip vendor directory
        if file.to_string_lossy().contains("/vendor/") {
            continue;
        }

        let mut violations = Vec::new();

        // Run each rule
        violations.extend(rules::debug_statements::check(file)?);
        violations.extend(rules::business_logic::check(file)?);

        all_violations.extend(violations);
    }

    // Display results
    formatters::terminal::print_results(&all_violations, php_files.len(), cli.quiet);

    // Exit with error code if violations found
    if !all_violations.is_empty() {
        std::process::exit(1);
    }

    Ok(())
}