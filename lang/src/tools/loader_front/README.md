# loader_front — Loader Front EXE (Phase 20.13)

Purpose
- Decide high-level loading (policy/allowlist/resolution) ahead of kernel loader.
- Output a short, stable verdict; kernel honors it strictly (no fallback).

Responsibilities
- Evaluate env/policy and candidate modules.
- Emit: OK/NOOP/FAIL (tagged), exit code semantics same as runner_front.

Inputs/Outputs
- Input: JSON/CLI with policy keys and candidates
- Output: one-line verdict (short) + exit code

ENV (planned; default OFF)
- HAKO_LOADER_USE_SCRIPT_EXE=1 (alias NYASH_*) — enable loader front EXE
- HAKO_QUIET=1 — quiet mode adherence

Non‑Goals
- No filesystem/ABI side-effects beyond read-only inspection.

TTL
- Diagnostics/formatting migrates from Rust boundary to this front EXE when stable.

Notes (AOT / Single‑EXE)
- 将来的に単一exeに内蔵（AOT）され、ENV反映→通常ロード継続を同一プロセスで行います。
- 開発時は tools/front_exe/loader_front.sh をゲートONで差し替え可能です。
