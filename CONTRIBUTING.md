# Contributing to STool

Thanks for your interest in contributing! This project welcomes contributions of all kinds.

## Getting Started
- Ensure you have Rust installed (`rustup` recommended).
- Install dev tools: `rg` (ripgrep) and `sd`.
- Clone repo and build: `cargo build`.

## Development Workflow
- Create a feature branch: `git checkout -b feature/your-feature`.
- Run `cargo fmt` and `cargo clippy` before pushing.
- Add tests if applicable.
- Submit a Pull Request with a clear description.

## Commit Style
- Use concise, descriptive commit messages.
- Prefix scope when helpful: `feat(repo): add clean command`.

## Code Style
- Prefer small, focused changes.
- Keep modules cohesive (`cli`, `git`, `version`, `utils`).
- Avoid unrelated refactors in feature PRs.

## Reporting Issues
- Use the issue templates.
- Provide reproduction steps and environment details.

## License
By contributing, you agree your contributions are licensed under the MIT License.