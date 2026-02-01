use super::Violation;
use anyhow::Result;
use regex::Regex;
use std::fs;
use std::path::Path;

pub fn check(file: &Path) -> Result<Vec<Violation>> {
    let content = fs::read_to_string(file)?;
    let mut violations = Vec::new();
    let file_path = file.to_string_lossy();

    // Rule 1: No DB::transaction in Filament files
    if file_path.contains("/Filament/") {
        let transaction_re = Regex::new(r"DB::transaction")?;

        for (line_num, line) in content.lines().enumerate() {
            if transaction_re.is_match(line) {
                violations.push(Violation {
                    file: file.to_path_buf(),
                    line: line_num + 1,
                    rule_code: "BL001".to_string(),
                    rule_name: "Business Logic in UI Layer".to_string(),
                    message: "Database transactions belong in Service layer, not Filament components".to_string(),
                    code_snippet: line.trim().to_string(),
                    suggestion: Some(
                        "Move this transaction to a Service class and call from Filament".to_string(),
                    ),
                });
            }
        }
    }

    // Rule 2: No direct model queries in Filament
    if file_path.contains("/Filament/") {
        let model_query_re = Regex::new(
            r"(Race|Player|User|Prize|Role|Permission|Market)::(?:create|where|find|update|delete|all|first)"
        )?;

        for (line_num, line) in content.lines().enumerate() {
            if model_query_re.is_match(line) {
                // Skip if it's just a relationship definition or type hint
                if line.contains("function") || line.contains("return") && line.contains("::class") {
                    continue;
                }

                violations.push(Violation {
                    file: file.to_path_buf(),
                    line: line_num + 1,
                    rule_code: "BL002".to_string(),
                    rule_name: "Direct Model Access in UI".to_string(),
                    message: "Filament components should use Services, not direct model queries".to_string(),
                    code_snippet: line.trim().to_string(),
                    suggestion: Some(
                        "Create a Service method and call it from this component".to_string(),
                    ),
                });
            }
        }
    }

    // Rule 3: No direct model queries in Contrllers
    if file_path.contains("/Controllers/") {
        let model_query_re = Regex::new(
            r"(Race|Player|User|Prize|Role|Permission|Market)::(?:create|where|find|update|delete|all|first)"
        )?;
        for (line_num, line) in content.lines().enumerate() {
            if model_query_re.is_match(line) {
                violations.push(Violation {
                    file: file.to_path_buf(),
                    line: line_num + 1,
                    rule_code: "BL002".to_string(),
                    rule_name: "Direct Model Access in UI".to_string(),
                    message: "Filament components should use Services, not direct model queries".to_string(),
                    code_snippet: line.trim().to_string(),
                    suggestion: Some(
                        "Create a Service method and call it from this component".to_string(),
                    ),
                });
            }
        }
    }

    // Rule 4: Services should use repositories (not direct models)
    if file_path.contains("/Services/") && !file_path.contains("Interface.php") {
        let model_re = Regex::new(
            r"(Race|Player|User|Prize)::(?:create|where|find|update|delete)"
        )?;

        for (line_num, line) in content.lines().enumerate() {
            if model_re.is_match(line) {
                violations.push(Violation {
                    file: file.to_path_buf(),
                    line: line_num + 1,
                    rule_code: "BL003".to_string(),
                    rule_name: "Service Bypassing Repository".to_string(),
                    message: "Services should use Repositories for data access, not direct model queries".to_string(),
                    code_snippet: line.trim().to_string(),
                    suggestion: Some(
                        "Inject the Repository and use its methods instead of direct model access".to_string(),
                    ),
                });
            }
        }
    }

    Ok(violations)
}