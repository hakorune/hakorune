Hakorune Script Checker (MVP)

Purpose
- Quickly validate Hakorune source files by parsing → MIR build → MIR verify without executing.
- Useful while Python/llvmlite migration is in-flight to keep scripts healthy.

Usage
- Build once (auto-build if missing):
  - cargo build --release
- Run checker:
  - tools/hako-check/hako-check.sh path/to/file.hako
  - or set alias explicitly: HAKO_BIN=tools/bin/hako tools/hako-check/hako-check.sh file.hako

Behavior
- Runs: nyash --backend mir --verify <file>
- Exit codes:
  - 0: OK
  - 2+: Parse/MIR verify failure (nyash returns non‑zero; checker forwards)

Notes
- Binary alias
  - Preferred alias: tools/bin/hako (or tools/bin/hakorune)
  - Backward‑compat: target/release/nyash も利用可（自動検出）
- This MVP only checks a single file and depends on the Rust parser.
- Extend with flags (parser selection, JSON emit) as migration progresses.
