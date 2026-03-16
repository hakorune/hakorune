---
Status: SSOT
Decision: accepted
Date: 2026-03-17
Scope: backend-zero / llvmlite demotion / Rust keep lane において、削除より先に preservation と external archive を必須化する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/phases/phase-29ck/README.md
  - crates/nyash-llvm-compiler/README.md
  - src/llvm_py/README.md
---

# Backend Legacy Preservation And Archive SSOT

## Purpose

- backend-zero の目的を「Rust / llvmlite をすぐ消すこと」ではなく、「daily route から外して final shape を固定すること」に揃える。
- Rust backend lane と Python/llvmlite lane は、削除前に preservation-first で保全する。
- 将来この repo から retire する場合も、source・artifact・再現情報を external archive repo に保存してからだけ実施できるようにする。

## 1. Current Policy

1. Rust backend lane と Python/llvmlite lane は、どちらも active keep である。
2. keep lane の役割は次だよ。
   - compat replay
   - canary / probe
   - portability fallback
   - archive candidate の準備
3. current backend-zero phase のゴールは「daily route を `.hako -> LlvmBackendBox -> hako_aot -> backend helper` に固定すること」であって、Rust / llvmlite の delete ではない。

## 2. Deletion Is Not Near-Term

次のものは near-term delete target ではない。

1. `crates/nyash-llvm-compiler/**`
2. `src/runner/modes/llvm/**` の compat keep route
3. `tools/llvmlite_harness.py`
4. `src/llvm_py/**`

理由:

- selfhost / backend-zero / portability の証跡としてまだ価値がある
- regression replay と compare route が残っている
- cross-platform artifact preservation がまだ終わっていない

## 3. Preservation-First Rule

Rust / llvmlite lane をこの repo から retire してよいのは、次を全部満たした時だけだよ。

1. external archive repo が存在する
   - source mirror を持つ
   - tagged release を持つ
   - restore 手順を持つ
2. archive repo に preservation bundle が置かれている
   - Rust backend source
   - Python/llvmlite source
   - relevant docs / smoke entrypoints
3. artifact preservation が完了している
   - Windows bundle
   - Ubuntu/Linux bundle
   - macOS bundle
   - iOS-related deliverable when supported by the lane
4. artifact metadata が保存されている
   - commit hash
   - build date
   - target triple / platform tag
   - checksums
   - acceptance / smoke result summary
5. current repo 側 docs が archive repo 参照先へ更新されている

満たしていない間は、demote はよいが delete はしない。

## 4. Preservation Bundle Requirements

external archive repo へ移す bundle は、少なくとも次を含む。

1. source snapshot
   - exact source tree or mirrored subtree
2. release artifacts
   - platform-specific binaries or packages
3. provenance metadata
   - `SHA256SUMS`
   - build manifest
   - smoke / acceptance note
4. restore instructions
   - how to replay object/exe emission
   - how to run the canary / compat route

原則:

- 「ファイルだけ保存」は不十分
- source + artifact + metadata + restore 手順の 4 点セットが必要

## 5. Release Artifact Rule

archive repo の release artifact は、少なくとも platform と provenance を辿れる命名にする。

推奨内容:

1. platform bundle
   - `hakorune-<platform>-<arch>-<commit>.zip|tar.gz`
2. checksum bundle
   - `SHA256SUMS`
3. build note
   - `build-info.txt`
4. acceptance note
   - `smoke-results.txt`

目的:

- 「将来必要になった時に取り出せる」だけでなく
- 「どの source / contract から作られたか」をすぐ検証できるようにする

## 6. Repository Rule

この repo 側では次を守る。

1. keep lane は explicit keep として縮める
2. daily route へ戻さない
3. docs から current owner / keep owner / archive target を区別して書く
4. archive repo が整う前に source delete をしない

## 7. Non-goals

1. backend-zero 完了と同時に Rust / llvmlite を削除すること
2. artifact をこの repo の release だけで永続保全したと見なすこと
3. source を失ったまま binary-only で retire すること
