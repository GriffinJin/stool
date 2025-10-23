# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.2.0] - 2025-10-23
### Added
- Refactor to modular project structure (`cli`, `git`, `version`, `utils`).
- Implement `repo clean` with parallel `git reset` and `git clean`.
- Add `genclone` with HTTP/SSH transport and script save.
- Add version replacement using `rg` + `sd`.

### Fixed
- Remove duplicate functions and syntax errors in `main.rs`.
- Resolve parallel execution and compilation issues.

### Docs
- Add comprehensive README and open-source docs (LICENSE, CONTRIBUTING, CODE_OF_CONDUCT).