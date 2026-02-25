---
Status: SSOT
Decision: accepted
Date: 2026-02-13
Scope: GC 方針（意味論上の位置づけ）と runtime 実装順序を 1 枚で固定し、selfhost/compiler lane と runtime lane の混線を防ぐ。
Related:
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/development/current/main/phases/phase-29y/README.md
  - docs/development/current/main/phases/phase-29y/10-ABI-SSOT.md
  - docs/development/current/main/phases/phase-29y/20-RC-INSERTION-SSOT.md
  - docs/development/current/main/phases/phase-29y/30-OBSERVABILITY-SSOT.md
  - docs/reference/language/lifecycle.md
---

# Runtime GC Policy and Order (SSOT)

## 0. Summary

- GC 本体（cycle collector / tracing）は **最後** に回す。
- 先に固定するのは `MIR-first` 境界、ABI、RC insertion、観測契約。
- lifecycle の意味論は **GC必須ではない**。

## 1. Fixed Execution Order

1. `.hako` mirbuilder は failure-driven 維持（先回り拡張をしない）。
2. runtime route は MIR-first を維持（Program JSON は strict/dev で fail-fast）。
3. Phase 29y.1 を順序固定で実施（ABI shim -> RC insertion minimal -> observability）。
4. `.hako VM` dual-run parity を段階拡張（subset + fail-fast 契約）。
5. VM/LLVM 最適化（verify 可能な局所最適化）。
6. 最後に optional で GC/cycle collector（意味論非変更の範囲）。

## 2. Lifecycle Semantics Position

- `fini` は資源解放の決定的契約（論理終端）。
- 物理解放タイミングは runtime 実装責務であり、言語意味論の外側。
- GC ON/OFF で意味論を変えない（変わってよいのは回収タイミングのみ）。

## 3. ABI Contract

- 関数 ABI は `args borrowed / return owned` を固定。
- borrowed を escape（保存/返却/格納）する場合のみ retain 挿入対象とする。

## 4. RC Insertion Contract

- `retain/release/weak_drop` の発火点は 1 箇所（RC insertion pass）に固定。
- lowering 各所への分散実装を禁止し、hidden root の温床を作らない。

## 5. Observability Contract

- root surface はカテゴリ契約で固定:
  - `locals`
  - `temps`
  - `heap_fields`
  - `handles`
  - `singletons`
- 診断は観測のみ。ON/OFF で意味論を変えない。

## 6. Cycle Policy

- 逆参照は weak を推奨（設計で cycle を避ける）。
- 強循環がある場合、no-cycle-collector モードではリークし得る（仕様として許容）。

## 7. Non-goals (Current Phase)

- Phase 29y の範囲で GC/finalizer の新規実装を行わない。
- GCアルゴリズム（cycle 回収方式）の規定は現フェーズの対象外。

## 8. Operational Interpretation (non-normative)

- Beginner mode: cycle collector あり（診断/安全寄り）。
- Expert mode: cycle collector なし（weak 運用を設計で徹底）。
- どちらのモードでも意味論は同じ。違いは回収タイミングとリーク耐性のみ。
