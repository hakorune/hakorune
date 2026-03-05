---
Status: SSOT
Scope: MirInstruction の kept / lowered-away / removed 台帳（C7b）と lowered-away 実体0化フロー
Decision: accepted (ledger + zero-state)
Updated: 2026-02-12
Related:
- src/mir/instruction.rs
- src/mir/contracts/backend_core_ops.rs
- src/mir/verification/legacy.rs
- docs/development/current/main/design/mir-vm-llvm-instruction-contract-fix-ssot.md
---

# MIR Instruction Diet Ledger (SSOT)

## Goal

`MirInstruction` の語彙を台帳化し、`kept / lowered-away / removed` を一意に固定する。

- 台帳は wish list ではなく、現行実装の事実を記述する。
- `lowered-away` は最終的に実体0（語彙0件）を目標とし、到達後は `removed` へ移送する。

## Data Sources (fact)

分類根拠は以下3点のみを使う。

1. Enum実体
   `src/mir/instruction.rs`（28 variants）
2. Backend contract allowlist
   `src/mir/contracts/backend_core_ops.rs`
3. Legacy rewrite / reject policy
   `src/mir/verification/legacy.rs` (`check_no_legacy_ops`)

## Current Contract Snapshot (2026-02-12)

`backend_core_ops` から機械抽出した現状の受理集合。

| Cohort | Count | Members |
|---|---:|---|
| JSON ∩ VM（両方で受理） | 14 | `BinOp, Branch, Call, Compare, Const, Copy, Jump, KeepAlive, NewBox, ReleaseStrong, Return, TypeOp, UnaryOp, WeakRef` |
| JSON only | 1 | `Phi` |
| VM only | 9 | `Await, Barrier, Debug, FutureNew, FutureSet, Load, Safepoint, Select, Store` |
| JSON/VMとも未受理 | 4 | `Catch, NewClosure, RefNew, Throw` |

運用注記（2026-03）:
- `Catch/Throw` は語彙としては kept だが、selfhost/mainline の日常 lane では `NYASH_TRY_RESULT_MODE=1` に pin して legacy MIR `Catch/Throw` 実行を使わない。
- Rust VM の `Catch/Throw` 実行実装は post-selfhost deferred。

## Ledger Decision (accepted)

### kept (28)

`Await, Barrier, BinOp, Branch, Call, Catch, Compare, Const, Copy, Debug, FutureNew, FutureSet, Jump, KeepAlive, Load, NewBox, NewClosure, Phi, RefNew, ReleaseStrong, Return, Safepoint, Select, Store, Throw, TypeOp, UnaryOp, WeakRef`

### lowered-away (0)

`(empty)`

- `MIR_INSTRUCTION_LOWERED_AWAY_TAGS == []` を SSOT とする。
- `check_no_legacy_ops` は `backend_core_ops::lowered_away_tag` のみを参照し、独自判定を持たない。

### removed (16)

`ArrayGet, ArraySet, BarrierRead, BarrierWrite, BoxCall, Cast, DebugLog, ExternCall, Nop, PluginInvoke, Print, RefGet, RefSet, TypeCheck, WeakLoad, WeakNew`

- 上記16語彙は enum から除去済み。
- 台帳上は `removed` cohort として保持する。

## C7 Status

- C7b（台帳固定）: Done
- C7c（cohort drift check / SSOT参照化）: Done
- C7d1..C7d12（1語彙ずつ移送）: Done
- C7z（enum remove + 参照除去）: Done

## Lowered-away 実体0化フロー（SSOT）

### 現在地（2026-02-12）

- L0: 変換先定義の固定 — Done
- L1: normalize後残存の fail-fast 検知 — Done
- L2: backend受理縮小（strict/dev→既定） — Done
- L3: 発生源停止（builder/emit/optimizer） — Done
- L4: enum remove + 台帳移送 — Done

### L0/L1（Done）

- legacy 語彙の canonical 変換先を定義し、`check_no_legacy_ops` を SSOT 判定へ統一。
- 現在は `lowered-away` 実体が 0 のため、判定結果は常に空集合となる。

### L2（Done）

- VM preflight の fail-fast 契約は維持（unsupported instruction / terminator を freeze で拒否）。
- `lowered-away` 専用 reject は語彙0件のためヒットしないが、判定導線は残す。
- 契約ピン: `tools/smokes/v2/profiles/integration/joinir/phase29bq_mir_preflight_unsupported_reject_vm.sh`

### L3（Done）

- source guard: `tools/checks/mir_no_lowered_away_emitters.sh`
- production path で `ArrayGet/ArraySet/RefGet/RefSet` 直接生成は 0。

### L4（Done）

- `ArrayGet/ArraySet/RefGet/RefSet` に続き、`BoxCall/ExternCall/DebugLog/Nop` も enum から除去し、`removed` へ移送。
- `backend_core_ops` 台帳とテスト（count/cohort/matrix）を同期。
- optimizer / verifier / tests の旧語彙参照を除去。

## Zero-state 完了条件

次を全て満たした時点を「`lowered-away` 実体0」とする。

1. `MIR_INSTRUCTION_LOWERED_AWAY_TAGS.len() == 0`
2. `check_no_legacy_ops` が `backend_core_ops::lowered_away_tag` のみ参照
3. enum に `ArrayGet/ArraySet/RefGet/RefSet` が存在しない
4. production path に `MirInstruction::ArrayGet|ArraySet|RefGet|RefSet` 直接生成が無い
5. 台帳（本書）と `backend_core_ops` テストが一致

## 最小検証コマンド

```bash
rg -n "MIR_INSTRUCTION_(KEPT|LOWERED_AWAY|REMOVED)_TAGS|instruction_diet_cohort|lowered_away_tag" src/mir/contracts/backend_core_ops.rs src/mir/verification/legacy.rs
rg -n "MirInstruction::(ArrayGet|ArraySet|RefGet|RefSet)" src tests
bash tools/checks/mir_no_lowered_away_emitters.sh
LD_PRELOAD=$PWD/tools/tmp/exdev/librename_copy_fallback.so cargo test -q backend_core_ops::tests::
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_mir_preflight_unsupported_reject_vm.sh
LD_PRELOAD=$PWD/tools/tmp/exdev/librename_copy_fallback.so bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq
git diff --check
```

## Acceptance Criteria

1. `MirInstruction` 28語彙 + `removed` 16語彙が `kept/lowered-away/removed` のいずれか1つに属する。
2. `MIR_INSTRUCTION_LOWERED_AWAY_TAGS` は空配列である。
3. `instruction_diet_ledger_counts_match_ssot` が `kept=28/lowered-away=0/removed=16/vocabulary=44` を固定する。
4. `check_no_legacy_ops` が独自matchを持たず `lowered_away_tag` を参照する。
5. `src`/`tests` に `MirInstruction::ArrayGet|ArraySet|RefGet|RefSet` 参照が存在しない。
6. `mir_no_lowered_away_emitters.sh` が PASS する。
7. `phase29bq_mir_preflight_unsupported_reject_vm.sh` が PASS する。
8. `phase29bq_fast_gate_vm.sh --only bq` が PASS する。
