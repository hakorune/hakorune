---
Status: SSOT
Decision: current
Date: 2026-04-16
Scope: Hakorune optimization work を research ではなく task-card-driven operation として回すための運用正本
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/current-optimization-mechanisms-ssot.md
  - docs/development/current/main/design/optimization-layer-roadmap-ssot.md
  - docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md
  - docs/development/current/main/design/llvm-line-ownership-and-boundary-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/phases/phase-137x/README.md
---

# Optimization Task-Card OS SSOT

## Goal

- 最適化を「何が効きそうかを読む研究」ではなく「1 cut ごとに judge する運用」に固定する
- `front -> proof -> rewrite target -> runtime executor -> perf gate` を毎回同じ schema で回す
- `MIR owner / runtime executor / LLVM consumer` の責務を card ごとに明示し、owner seam をまたいだ迷走を防ぐ

## Authority Order

Optimization の authority order は次で固定する。

1. `.hako`
   - semantics
   - policy
   - user-visible contract
2. `MIR`
   - proof
   - rewrite eligibility
   - rewrite target
3. `runtime/kernel`
   - already-approved corridor の executor
   - cold adapter
4. `LLVM`
   - truthful facts の consumer
   - profitability / widening / codegen

禁止:

- runtime が rewrite eligibility を再判定しない
- LLVM metadata に language semantics を載せない
- perf lane の current issue を、まず keep lane (`llvm_py` / `native_driver`) へ押し戻さない

## Primary Owner Classes

1. `measurement`
   - perf, asm, counters, route trace を増やす
   - source widening はしない
2. `mir-proof`
   - 1 個の truth を MIR contract に追加する
   - runtime executor は変えない
3. `mir-rewrite`
   - 既存 proof の上で rewrite target を切り替える
   - runtime executor はまだ薄いままでもよい
4. `runtime-executor`
   - already-approved target を実行する薄い executor / cold adapter を追加する
   - runtime は new proof owner にならない
5. `llvm-export`
   - truthful attrs / manifests / export verifier を追加する
   - semantics や rewrite eligibility は持たない

1 card で持ってよい `primary owner` は 1 個だけだよ。

## Required Card Schema

すべての optimization card は最低限この項目を持つ。

```yaml
id: opt-XXXX
status: draft | active | passed | missed | analysis-only | rejected

front: <exact benchmark>
accept_gate: <neighbor benchmark>
whole_kilo_guard: <whole-program guard>

owner:
  primary: measurement | mir-proof | mir-rewrite | runtime-executor | llvm-export
  secondary: none

problem_statement:
  one_sentence: <current front で何が壊れているか>

frozen_evidence:
  counters: []
  top_symbols: []
  rejected_probes: []

proof_delta:
  adds_exactly_one_truth: []
  does_not_add: []

rewrite_target:
  from: <old corridor>
  to: <new corridor>
  shape_owner: MIR

executor_delta:
  add: <new executor or none>
  demote: <old helper/corridor or none>
  forbid: []

preserves: []
invalidates: []

first_commands: []

done_condition: []
reject_condition: []

artifacts:
  - before/after perf note
  - before/after top symbols
  - before/after counters
  - one-paragraph verdict

rollback:
  - immediate revert on reject
```

## Operational Rules

### 1. 1 card = 1 primary owner

- `mir-proof` と `runtime-executor` を同じ card に混ぜない
- card が複数 owner を必要とした時点で split する

### 2. 1 card = 1 proof delta

- 新しく真にしてよい truth は 1 個だけ
- 2 個目が必要なら、その card はまだ research 段階

### 3. Frozen Evidence First

- code edit の前に `frozen_evidence` を固定する
- card 着手後に owner files 以外の source spelunking が必要になったら、その card は reject して `measurement` card を切り直す

### 4. Preserves / Invalidates Are Mandatory

- `preserves` を書けない card は、何を壊さずに前進するかが不明
- `invalidates` を書けない card は、古い前提の破棄範囲が不明

### 5. Verdict Taxonomy Is Fixed

- `passed`
- `missed`
- `analysis-only`
- `rejected`

`close enough` や `maybe win` は verdict として使わない。

### 6. Reject Means Immediate Revert

次のいずれかで card は reject:

- exact front に win が出ない
- accept gate が落ちる
- whole-kilo guard が悪化する
- new top owner が extra cache/TLS/helper traffic に移る
- public ABI / language semantics / keep-lane owner widening が混入する

reject の差分はその場で戻す。履歴は docs にだけ残す。

## Reading Rule

optimization work に入る時は、source より先にこの順番で読む。

1. `CURRENT_TASK.md`
2. [perf-optimization-method-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/perf-optimization-method-ssot.md)
3. [llvm-line-ownership-and-boundary-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/llvm-line-ownership-and-boundary-ssot.md)
4. active phase README
5. active card の owner files だけ

## Current Active Card Shape

current post-selfhost optimization re-entry card is fixed like this:

- lane: `phase-137x`
- front: `kilo_micro_substring_concat`
- accept gate: `kilo_micro_substring_only`
- whole-kilo guard: `kilo_kernel_small_hk`
- primary owner: `runtime-executor`
- proof delta:
  - `borrow_view_continuity_to_concat`
- rewrite target:
  - from: `substring_concat3_hhhii` handle corridor
  - to: `plan-native concat corridor`
- executor delta:
  - add: `concat3_plan_executor`-class narrow executor
  - demote: old handle helper path to cold adapter
- preserves:
  - `.hako` semantics
  - public ABI
  - canonical MIR single-source rule
- reject:
  - cache/helper layer growth without exact-front win
  - new string-only MIR dialect
  - VMValue/public ABI widening
  - keep-lane owner pivot without route break evidence

## Non-goals

- 「効きそうな最適化案を増やす」こと自体を goal にしない
- keep lane (`llvm_py`, `native_driver`) を optimization card の default owner にしない
- metadata-only hint で corridor 本体を作らない
- no-goal のまま broad `noalias` / TBAA / third ABI を開かない
