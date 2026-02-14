# AGENTS.md

## Project Overview
- This repository is a Rust workspace.
- Workspace crates live under `crates/`.

## Current Crates
- `crates/server`: binary crate.
- `crates/client`: library crate.
- `crates/cli`: binary crate depending on `client`.

## Development Commands
Use `just` for common workflows:
- `just check`
- `just build`
- `just fmt`
- `just fmt-check`
- `just clippy`
- `just test`
- `just ci`
- `just run-server`
- `just run-cli [name]`

## Conventions
- Keep shared code in `crates/client` where practical.
- Add new crates under `crates/` and register them in the root workspace `Cargo.toml`.
- Keep CI green by ensuring `just ci` passes locally before pushing.
