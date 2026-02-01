use crate::rules::Violation;
use colored::*;

pub fn print_results(violations: &[Violation], total_files: usize, quiet: bool) {
    if violations.is_empty() {
        if !quiet {
            println!("{}", "✓ No violations found!".green().bold());
            println!("Checked {} files", total_files);
        }
        return;
    }

    // Group violations by file
    let mut by_file: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
    for violation in violations {
        by_file
            .entry(violation.file.clone())
            .or_insert_with(Vec::new)
            .push(violation);
    }

    println!(
        "{} {} found in {} files:\n",
        "❌".red(),
        if violations.len() == 1 {
            "violation".to_string()
        } else {
            format!("{} violations", violations.len())
        }
        .red()
        .bold(),
        by_file.len()
    );

    for (file, file_violations) in by_file.iter() {
        println!("{}", format!("📄 {}", file.display()).yellow().bold());

        for violation in file_violations {
            println!("   {} {}", "Line".dimmed(), violation.line.to_string().cyan());
            println!(
                "   {} {}",
                "Rule:".dimmed(),
                format!("[{}] {}", violation.rule_code, violation.rule_name).magenta()
            );
            println!("   {}", violation.message);
            println!("   {}", format!("│ {}", violation.code_snippet).dimmed());

            if let Some(suggestion) = &violation.suggestion {
                println!("   {} {}", "💡".yellow(), suggestion.italic());
            }

            println!();
        }
    }

    println!("{}", "━".repeat(50).dimmed());
    println!(
        "Summary: {} violations in {} files (checked {} total)",
        violations.len().to_string().red().bold(),
        by_file.len(),
        total_files
    );
}