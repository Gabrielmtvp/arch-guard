use super::Violation;
use anyhow::Result;
use regex::Regex;
use std::fs;
use std::path::Path;

pub fn check(file: &Path) -> Result<Vec<Violation>> {
    let content = fs::read_to_string(file)?;
    let mut violations = Vec::new();

    // Regex to find debug statements
    let debug_re = Regex::new(r"\b(dd|dump|var_dump|print_r|ray)\s*\(")?;

    for (line_num, line) in content.lines().enumerate() {
        if let Some(captures) = debug_re.captures(line) {
            let debug_fn = captures.get(1).unwrap().as_str();

            violations.push(Violation {
                file: file.to_path_buf(),
                line: line_num + 1,
                rule_code: "DBG001".to_string(),
                rule_name: "No Debug Statements".to_string(),
                message: format!(
                    "Debug statement '{}()' found - remove before committing",
                    debug_fn
                ),
                code_snippet: line.trim().to_string(),
                suggestion: Some("Remove this debug statement before committing".to_string()),
            });
        }
    }

    Ok(violations)
}