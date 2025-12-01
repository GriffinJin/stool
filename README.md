# Synapse CLI

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.2.0-green.svg)](Cargo.toml)

A powerful command-line toolkit for Sunline development, designed to boost developer productivity. Provides batch Git repository management, workspace management, version replacement, and database file processing capabilities.

## ✨ Features

### 🔧 Repository Management
- **Batch Operations** - Parallel operations on multiple Git repositories
- **Repository Listing** - Display repository status, branch info, and commit differences
- **Branch Switching** - Switch all repositories to a specified branch with force create option
- **Repository Cleaning** - Batch reset and clean repository working directories
- **Clone Script Generation** - Generate batch clone commands (auto-detects HTTP/SSH)

### 📁 Workspace Management
- **Create Workspaces** - Create and manage development workspaces
- **Initialize Workspaces** - Initialize current directory as a workspace
- **List Workspaces** - View all available workspaces
- **Switch Workspaces** - Quickly switch between different workspaces

### 📝 Version Management
- **Version Replacement** - Intelligently find and replace version numbers across projects

### 🗄️ Database Tools
- **Clean ID Fields** - Remove id fields and corresponding values from SQL INSERT statements

## 🚀 Installation

### Prerequisites

Make sure you have the following tools installed:

- **Rust** 1.70 or higher
- **Git** - Version control system
- **ripgrep (rg)** - Fast text search (required for version replacement)
- **sd** - Text substitution tool (required for version replacement)

```bash
# macOS
brew install ripgrep sd

# Ubuntu/Debian
sudo apt install ripgrep
cargo install sd
```

### Build from Source

```bash
git clone <repository-url>
cd synapse-cli
cargo build --release
```

The binary will be located at `target/release/synapse`.

### Installation

```bash
# Install to system path
cargo install --path .

# Or manually copy to PATH
sudo cp target/release/synapse /usr/local/bin/
```

## 📖 Usage Guide

### Repository Commands

#### List Repositories

Display all Git repositories in the current directory:

```bash
# Show repository status
synapse repo ls

# Fetch remote information before listing
synapse repo ls --fetch

# Pull all repositories
synapse repo ls --pull

# Clean all repositories
synapse repo ls --clean

# Combine operations
synapse repo ls --fetch --pull
```

#### Switch Branches

Switch all repositories to a specified branch:

```bash
# Switch to branch
synapse repo switch main

# Force switch (create branch if it doesn't exist)
synapse repo switch --force feature-branch
```

#### Generate Clone Commands

Generate batch clone commands (auto-detects HTTP/SSH protocol):

```bash
# Print clone commands to console
synapse repo genclone

# Save to clone.sh in current directory
synapse repo genclone --save
```

#### Update Version Numbers

Replace version numbers across the project:

```bash
# Replace version in all files
synapse repo updateversion 1.0.0 1.1.0
```

This command uses `ripgrep` to find files containing the old version and `sd` to perform the replacement.

### Workspace Commands

#### Create New Workspace

```bash
synapse workspace new
```

#### Initialize Current Directory

Initialize the current directory as a workspace:

```bash
synapse workspace init
```

#### List All Workspaces

```bash
synapse workspace ls
```

#### Switch to Workspace

Switch to a specified workspace:

```bash
synapse workspace cd <workspace-name>
```

### Database Commands

#### Remove ID Fields from SQL

Clean ID fields from INSERT statements:

```bash
# Process SQL file (creates .bak backup)
synapse db rmid ./data.sql
```

This command:
- Removes the `id` field from INSERT statements
- Removes the corresponding first value
- Creates a `.bak` backup file automatically
- Modifies the original file in place

## 🏗️ Project Structure

```
src/
├── lib.rs              # Library entry point, module exports
├── main.rs             # Binary entry point
├── cli/                # Command-line interface
│   ├── mod.rs          # CLI module entry
│   └── commands.rs     # Command definitions (using clap)
├── repo/               # Repository operations
│   ├── mod.rs          # Repo module entry
│   ├── repo.rs         # Repository discovery and info gathering
│   ├── operations.rs   # Git operations (pull, fetch, switch, clean)
│   └── clone.rs        # Clone command generation
├── workspace/          # Workspace management
│   ├── mod.rs          # Workspace module entry
│   └── operations.rs   # Workspace operations (new, init, ls, cd)
├── version/            # Version management
│   ├── mod.rs          # Version module entry
│   └── replace.rs      # Version replacement functionality
├── db/                 # Database utilities
│   ├── mod.rs          # DB module entry
│   └── rmid.rs         # SQL ID field removal
└── utils/              # Utility modules
    ├── mod.rs          # Utils module entry
    ├── command.rs      # Command execution helpers
    └── parallel.rs     # Parallel execution framework
```

## 🔧 Dependencies

### Runtime Dependencies

- **Rust** 1.70+ (Rust 2024 edition)
- **Git** - Required for repository operations
- **ripgrep (rg)** - Required for version replacement functionality
- **sd** - Required for version replacement functionality

### Rust Crates

- `clap` 4.5.4 - Command-line argument parsing with derive macros
- `regex` 1.10 - Regular expression support
- `directories` 5.0 - Platform-specific directory paths

## ⚙️ Configuration

The tool uses platform-appropriate directories for cache and configuration:

- **macOS**: `~/Library/Application Support/synapse-cli`
- **Linux**: `~/.config/synapse-cli`
- **Windows**: `%APPDATA%\synapse-cli`

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

```bash
# Clone the repository
git clone <repository-url>
cd synapse-cli

# Run tests
cargo test

# Check code formatting
cargo fmt --check

# Run linter
cargo clippy

# Build in debug mode
cargo build

# Build in release mode
cargo build --release
```

### Code Standards

- Follow Rust 2024 edition standards
- Ensure all tests pass before submitting
- Add documentation for new features
- Use clear and descriptive commit messages

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing
- [regex](https://github.com/rust-lang/regex) - Regular expression support
- [ripgrep](https://github.com/BurntSushi/ripgrep) - Fast text search
- [sd](https://github.com/chmln/sd) - Intuitive find & replace tool
- [directories](https://github.com/dirs-dev/directories-rs) - Platform-specific directory paths

## 📞 Support

If you encounter issues or have suggestions:

1. Check the [Issues](https://github.com/your-username/synapse-cli/issues) page
2. Create a new Issue describing the problem
3. Or submit a Pull Request directly

## 🚀 Performance

- **Parallel Processing**: All batch operations use concurrent execution for maximum performance
- **Efficient Search**: Leverages ripgrep for high-speed file searching
- **Smart Caching**: Uses platform-specific cache directories for optimal performance

---

**Synapse CLI** - Making development work simple and efficient! 🚀