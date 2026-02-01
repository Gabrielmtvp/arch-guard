# arch-guard

⚡ Architecture linter for Laravel/Filament projects. Enforces clean architecture in milliseconds.

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

## Why arch-guard?

Manual code reviews catch the same violations repeatedly:
- ❌ Business logic in Filament components
- ❌ Missing database transactions
- ❌ Services directly accessing models
- ❌ Debug statements left in code

**arch-guard automates these checks** and runs in < 0.5 seconds.

## Features

- 🛡️ **Layered Architecture Enforcement** - Services use Repositories, UI stays presentational
- 🔄 **Transaction Detection** - Warns about multi-record operations without transactions
- ⚡ **N+1 Query Detection** - Identifies potential performance issues
- 🚫 **Debug Statement Blocking** - Catches dd(), dump(), ray() before commit
- 📊 **Clear Output** - Beautiful terminal formatting with file links and suggestions
- 🚀 **Blazing Fast** - Written in Rust, checks 1000+ files in milliseconds

## Installation

### From Binary (Recommended)
```bash
# macOS/Linux
curl -sSL https://github.com/yourusername/arch-guard/releases/latest/download/arch-guard-$(uname -s)-$(uname -m) -o /usr/local/bin/arch-guard
chmod +x /usr/local/bin/arch-guard
```

### From Source
```bash
git clone https://github.com/yourusername/arch-guard.git
cd arch-guard
cargo install --path .
```

## Usage
```bash
# Check current directory
arch-guard check

# Check specific path
arch-guard check app/Filament

# Verbose output
arch-guard check --verbose

# Only show violations (quiet mode)
arch-guard check --quiet
```

## Example Output
```
🛡️  arch-guard - Architecture Linter
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

❌ 2 violations found in 1 file:

📄 app/Filament/Resources/RaceResource/Pages/CreateRace.php
   Line 45
   Rule: [BL001] Business Logic in UI Layer
   Database transactions belong in Service layer, not Filament components
   │ if (Race::where('name', $this->data['name'])->exists()) {
   💡 Move this transaction to a Service class and call from Filament

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Summary: 2 violations in 1 file (checked 234 total)
```

## Rules

| Code | Rule | Description |
|------|------|-------------|
| **DBG001** | No Debug Statements | Blocks dd(), dump(), var_dump(), ray() |
| **BL001** | No Transactions in UI | DB::transaction should be in Services |
| **BL002** | No Direct Model Access in UI | Filament should use Services |
| **BL003** | Services Use Repositories | Services should delegate to Repositories |
| **TX001** | Missing Transactions | Multiple DB operations need transactions |

## Configuration

Create `.arch-guard.toml` in your project root:
```toml
[rules]
debug_statements = true
business_logic_in_ui = true
services_use_repositories = true

[ignore]
paths = ["vendor/", "storage/", "tests/"]

[severity]
DBG001 = "error"
BL001 = "error"
BL002 = "warning"
```

## Git Hook Integration

Add to pre-commit:
```bash
#!/bin/bash
arch-guard check --quiet || exit 1
```

## Roadmap

- [ ] AST-based analysis (tree-sitter)
- [ ] Auto-fix capabilities
- [ ] Custom rule definitions
- [ ] CI/CD integrations
- [ ] IDE extensions

## Contributing

Contributions welcome!

## License

MIT - see [LICENSE](LICENSE)

## 🏷️ **GitHub Topics to Add**
```
rust
laravel
filament
linter
static-analysis
architecture
code-quality
cli
developer-tools
php
clean-architecture
code-review-automation
