# runner_front — Script-built Front EXE (Phase 20.13)

Purpose
- Decide runner mode and normalize inputs before core execution.
- Replace Rust-side front logic via a thin, testable EXE (AOT built from Hakorune scripts).

Notes (AOT / Single‑EXE)
- 本モジュールは将来、単一の hakorune.exe に内蔵（AOT）され、同一プロセス内で呼び出されます。
- 開発時は tools/front_exe/runner_front.sh をゲートONで差し替え可能（1行契約の互換を維持）。

Responsibilities
- Parse CLI/ENV relevant to backend/mode/entry.
- Produce a short verdict and exit code:
  - OK: adopt decision (prints normalized JSON or token) and exit 0
  - NOOP: no decision; Rust runner proceeds with legacy path, exit 0
  - FAIL: stable tag, exit non‑zero (no fallback by design)

Inputs/Outputs (contract)
- Input: CLI args + ENV (documented below)
- Output: one short line (OK/NOOP/FAIL + payload or tag) to stdout
- Exit codes: 0=adopt/noop, 1=fail

ENV (planned; default OFF)
- HAKO_RUNNER_USE_SCRIPT_EXE=1 (alias NYASH_*) — enable front EXE adoption
- HAKO_QUIET=1 — silence noisy logs (front adheres to quiet)

Non‑Goals
- No plugin/ABI routing here (handled downstream)
- No semantics; runner_front only decides and normalizes

TTL / Removal Plan
- Runner boundary diagnostics/tags are temporary; migrate to core/front over time.
