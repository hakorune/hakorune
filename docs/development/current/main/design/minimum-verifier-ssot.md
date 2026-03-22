---
Status: SSOT
Decision: provisional
Date: 2026-03-23
Scope: `phase-29ct` の C1.5 として、capability substrate 導入時に必須の minimum verifier を docs-first で固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29ct/README.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/minimal-capability-modules-ssot.md
  - docs/development/current/main/design/raw-array-substrate-ssot.md
  - lang/src/runtime/substrate/README.md
  - lang/src/runtime/substrate/verifier/README.md
---

# Minimum Verifier (SSOT)

## Goal

- `hako.mem` / `hako.buf` / `hako.ptr` のあとに必要な最小 verifier を docs-first で固定する。
- full sanitizer に行く前に、low-level algorithm owner を支える最小 fail-fast を定義する。
- unrestricted unsafe を避けつつ、bounds/init/ownership の契約違反を静かに通さない。

## Fixed Order

minimum verifier の順番は次で固定する。

1. `bounds`
2. `initialized-range`
3. `ownership`

この順番を current substrate lane の正本とする。

## Verifier Roles

### `bounds`

- catches:
  - `idx < 0`
  - `idx >= len`
  - `cap` 超過
  - inbounds contract を外れた read/write
- reading:
  - out-of-bounds は fail-fast
  - silent clamp / silent no-op はしない

### `initialized-range`

- catches:
  - `set_len` 後の未初期化領域 read
  - reserve/grow 後に未初期化スロットを値ありとして扱うこと
  - initialized range を超えた dereference
- reading:
  - readable range と allocated range を混同しない

### `ownership`

- catches:
  - owned / borrowed の混同
  - borrowed alias の期限切れ再利用
  - move/reuse の契約違反
- reading:
  - ownership mismatch は fail-fast
  - conservative fallback が許されるのは、既存 docs で明示した borrowed alias expiry のような限定ケースだけ

## Current Reading

- `bounds` is the first live verifier box.
- `initialized-range` / `ownership` remain docs-first follow-ups.
- physical staging root is reserved at [`lang/src/runtime/substrate/verifier/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/verifier/README.md)
- first live box lives at [`lang/src/runtime/substrate/verifier/bounds/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/verifier/bounds/README.md)
- RawArray slot load/store now route through the bounds verifier gate before raw pointer access

## Non-Goals

- `initialized-range` / `ownership`
- `RawMap`
- `hako.mem` / `hako.buf` / `hako.ptr` の実装本体
- allocator state machine
- TLS / atomic / GC の実装
- OS VM / final allocator / final ABI stub
- unrestricted raw pointer
- `runtime/collections/` owner migration
- perf lane reopen
- full sanitizer (`double free` / `use-after-free` / exhaustive alias analysis)

## Follow-Up

`bounds` live slice の次は `initialized-range` へ進む。

- first consumer:
  - `RawArray`
  - docs-side truth:
    - [`raw-array-substrate-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/raw-array-substrate-ssot.md)
  - `RawMap`
  - docs-side truth:
    - [`raw-map-substrate-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/raw-map-substrate-ssot.md)
- later:
  - `GC/TLS/atomic widening`
