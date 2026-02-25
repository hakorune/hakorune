---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X37 LLVM+C ABI link gate の最小契約を固定する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-56-thin-rust-core-cabi-min-surface-ssot.md
  - tools/checks/nyrt_core_cabi_surface_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_llvm_cabi_link_min.sh
  - tools/build_llvm.sh
---

# Phase 29x X37: LLVM+C ABI Link Gate SSOT

## 0. Goal

Lane F の入口として、最小 `.hako` fixture で
`.hako -> LLVM -> native link` を固定し、
Core C ABI surface と同時に gate 化する。

## 1. Contract

`phase29x_llvm_cabi_link_min.sh` で次を同時に満たす。

1. `tools/checks/nyrt_core_cabi_surface_guard.sh` が PASS
2. `tools/build_llvm.sh apps/tests/hello_simple_llvm.hako -o <exe>` が成功
3. 生成された `<exe>` が exit 0 で実行され、出力 `42` を観測

補足:
- LLVM 前提（`llvm-config-18` / `python3 llvmlite`）がない環境では SKIP とする。
- 既定 route の影響を受けないよう、X37 は LLVM link line 単体を検証対象にする。

## 2. Acceptance

1. `phase29x_llvm_cabi_link_min.sh` が PASS
2. `README / 29x-90 / 29x-91 / CURRENT_TASK` が X37 完了状態に同期
3. 次タスク `X38`（daily/milestone の LLVM line 既定化）へ進める

## 3. Evidence (X37)

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_cabi_link_min.sh`

## 4. Next Step

X38 で daily/milestone 入口から
Rust runtime build 必須を外し、LLVM line gate を既定化する。
