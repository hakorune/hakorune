---
Status: Active
Decision: provisional
Date: 2026-02-13
Scope: Phase 29x X7 の early-exit cleanup（return/break/continue）順序契約。
Related:
  - docs/reference/language/scope-exit-semantics.md
  - docs/reference/language/lifecycle.md
  - docs/development/current/main/phases/phase-29y/20-RC-INSERTION-SSOT.md
  - src/mir/passes/rc_insertion.rs
  - src/bin/rc_insertion_selfcheck.rs
---

# Phase 29x X7: Early-Exit Cleanup Order SSOT

## 0. Goal

`return` / `break` / `continue` で scope を抜けるときの cleanup 順序を 1 箇所で固定し、
X8-X10 の実装で同じ契約を使い回せる状態にする。

## 1. Ordering Contract (normative for RC lane)

Early-exit edge では次の順序を守る。

1. exit kind を確定する（`return` / `break` / `continue`）
2. 退出する lexical scope の cleanup 対象 binding を確定する
3. 対象 binding の current value を `release_strong` として挿入する
4. その後に exit control transfer を実行する

禁止:
- cleanup のあとに binding を再評価して release 対象を変える
- Jump/Branch block に「理由不明の release」を置く

## 2. Current implementation boundary (2026-02-13, X10 reflected)

Implemented:
- `return` path の cleanup（Return terminator 前）
  - single block
  - jump-chain（single predecessor）
  - multi-predecessor Return（intersection）
- `break` path の cleanup（X9-min）
  - empty Jump block -> immediate multi-pred Return edge
  - pred-local delta（Return join intersection に入らない値）を Jump 前で `release_strong`
- `continue` path の cleanup（X10-min）
  - empty Jump block -> multi-pred Branch header edge
  - pred-local delta（header join intersection に入らない値）を Jump 前で `release_strong`

Not yet implemented in `rc-insertion-minimal`:
- 一般形の break cleanup（non-empty Jump / non-immediate Return / edge-args を伴う形）
- 一般形の continue cleanup（non-empty Jump / non-Branch header / edge-args を伴う形）

運用:
- 未実装 path は “silent success” で隠さず、X11 以降で明示拡張する。

## 3. Scope-end relation

- X4/X5 の scope-end cleanup は、この X7 順序契約の `return` 特化版として扱う。
- nested lexical block 単位の cleanup timing は X7 では拡張しない（loop/early-exit lane で扱う）。

## 4. Evidence for X7

Docs:
- このファイル（順序契約）
- `docs/development/current/main/phases/phase-29x/README.md`（Week2 pin）

Current executable evidence:
- `bash tools/smokes/v2/profiles/integration/apps/phase29x_rc_scope_end_release_vm.sh` PASS
- `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_rc_break_cleanup_vm.sh` PASS
- `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_rc_continue_cleanup_vm.sh` PASS
- `cargo run -q --bin rc_insertion_selfcheck --features rc-insertion-minimal` PASS

## 5. Next tasks

- X13: observability 拡張設計へ進み、RC lane（X2-X12）の固定を前提に root surface 5カテゴリ契約へ接続する
