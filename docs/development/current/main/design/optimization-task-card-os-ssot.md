---
Status: SSOT
Decision: current
Date: 2026-04-16
Scope: Hakorune optimization work を research ではなく task-card-driven operation として回し、thin cut ではなく delete-oriented corridor redesign まで扱うための運用正本
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
   - proof region
   - publication boundary
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

## Core Terms

- `.hako scope`
  - lexical/user-visible meaning scope
  - values/control-flow/contracts live here
- `proof_region`
  - MIR-side region where one already-legal optimization fact is proven to hold
  - this is not new language meaning
- `publication_boundary`
  - MIR-side non-widening contract that says where a specialized executor may be published
  - this is not lexical scope and not a runtime re-recognition hook

Reading lock:

- use `publication_boundary` or `non_widening_contract` in docs
- do not use `scope_lock` as the architecture term; it is too easy to confuse with `.hako` scope

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

proof_region:
  established_facts: []
  region_limits: []

publication_boundary:
  applies_only_to: []
  publish_as: <runtime-private executor or none>
  must_not_touch: []
  must_not_become: []

rewrite_target:
  from: <old corridor>
  to: <new corridor>
  shape_owner: MIR

executor_delta:
  add: <new executor or none>
  demote: <old helper/corridor or none>
  forbid: []

delete_target:
  - <hot corridor / helper / call edge that should disappear from the front>

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

### 2.5. Proof Region And Publication Boundary Stay Separate

- `proof_region` is where the MIR fact holds
- `publication_boundary` is where the specialized executor may be published
- do not merge them into one vague scope term
- do not let runtime rediscover either of them

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

## Delete-Oriented Rule

exact front が次の条件を同時に満たしたら、次の card は thin cut ではなく
delete-oriented card として扱う。

- leaf sibling gate はほぼ C に近い
- exact front だけが桁違いに重い
- route counters が 1 本の hot corridor に張り付いている
- sink/cache/store などの前提実装は既に存在する

delete-oriented card の原則:

- `proof_delta` は generic substrate truth に置く
- `proof_region` でその truth が成立する corridor を限定する
- `publication_boundary` で specialized executor の publish 範囲を限定する
- `rewrite_target` は hot helper call を cold adapter へ退かせる形で書く
- `delete_target` は必須
- runtime は executor だけを持ち、eligibility を再判定しない
- public ABI / keep lane / language semantics は広げない

典型の順番:

1. `measurement`
   - hot corridor を same-artifact で固定する
2. `mir-rewrite`
   - hot helper call を plan-native corridor へ置き換える
3. `runtime-executor`
   - thin executor を足し、旧 helper を cold adapter に降格する
4. `llvm-export`
   - corridor が固まったあとで truthful attrs / export verifier を足す

## Reading Rule

optimization work に入る時は、source より先にこの順番で読む。

1. `CURRENT_TASK.md`
2. [perf-optimization-method-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/perf-optimization-method-ssot.md)
3. [llvm-line-ownership-and-boundary-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/llvm-line-ownership-and-boundary-ssot.md)
4. active phase README
5. active card の owner files だけ

## Current Active Card Shape

current post-selfhost optimization reopen now uses this sequence:

- lane: `phase-137x`
- front: `kilo_micro_substring_concat`
- accept gate: `kilo_micro_substring_only`
- whole-kilo guard: `kilo_kernel_small_hk`
- current keeper:
  - `746,997,552 instr / 66 ms`
- frozen evidence:
  - `view_arc_cache_miss=600000`
  - `slow_plan=600000`
  - `slow_plan_view_span=600000`
  - top symbols still include `substring_hii`, `LocalKey::with`, `borrowed_substring_plan_from_handle`
- next primary owner: `mir-rewrite`
- proof delta:
  - `borrow_view_continuity_to_final_concat`
- proof region:
  - established facts:
    - borrowed lane may stay unmaterialized until the final consumer
    - the active corridor does not escape
    - the active corridor does not cross a public boundary
  - region limits:
    - active `substring + const + substring -> final substring` corridor only
- publication boundary:
  - applies only to:
    - the active `kilo_micro_substring_concat` corridor selected by MIR rewrite
  - publish as:
    - runtime-private executor only
  - must not touch:
    - generic helper body semantics
    - public ABI
    - broad callers outside the active corridor
  - must not become:
    - a generic helper rewrite
- rewrite target:
  - from: `substring_concat3_hhhii` handle corridor
  - to: `plan-native final-consumer corridor`
- executor delta:
  - add: `none in the rewrite card`
  - demote: `substring_hii` / `borrowed_substring_plan_from_handle` hot path to cold adapter candidates
- delete target:
  - hot call to `substring_hii` from the active `substring -> concat` front
  - hot re-entry into `borrowed_substring_plan_from_handle`
  - hot TLS access through `LocalKey::with` on that same front
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
