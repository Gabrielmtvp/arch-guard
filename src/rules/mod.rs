pub mod business_logic;
pub mod debug_statements;

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Violation {
    pub file: PathBuf,
    pub line: usize,
    pub rule_code: String,
    pub rule_name: String,
    pub message: String,
    pub code_snippet: String,
    pub suggestion: Option<String>,
}