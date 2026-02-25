# Phase 47: Normalized P3 (If-Sum) Design

**Status**: Design Complete, Minimal → Extended Dev → Canonical (P3 Core) 実装中
**Date**: 2025-12-21

## Goal

Extend Normalized JoinIR to support Pattern3 (if-sum) loops using the same infrastructure that successfully handles P1/P2.

**Key insight**: P3 already shares P2's Structured JoinIR foundation (Phase 220), so Normalized extension reuses existing components.

## Background: P2 Normalized Success

Phase 43/245B/46 established canonical Normalized for all P2 patterns:
- ✅ Pattern2Mini (simple break)
- ✅ JsonParser skip_whitespace (Trim pattern)
- ✅ JsonParser _atoi (DigitPos + NumberAccumulation)
- ✅ JsonParser _parse_number (multi-carrier)

**Infrastructure complete**:
- Structured→Normalized→MIR(direct) pipeline
- EnvLayout, JpInst/JpOp, StepScheduleBox
- ConditionEnv, CarrierInfo, ExitLine
- Mode system (Phase 45), Capability system (Phase 44)

## Why P3 Uses Same Normalized

### 1. Shared Structured JoinIR Foundation (Phase 220)

From `joinir-architecture-overview.md` (lines 73-84):

```
Phase 220: P3 if-sum の ConditionEnv 統合完了
- P3 if-sum には ConditionPatternBox + ConditionEnv が必須
- Phase 220-D で loop 条件の変数サポート完了
- Phase 220 で if-sum の expr-result exit contract が P2 と揃った
```

**P3 already uses P2 infrastructure**:
- ✅ ConditionEnv (condition analysis)
- ✅ CarrierInfo (state tracking)
- ✅ ExitLine/Boundary (exit handling)
- ✅ LoopHeaderPHI (SSA construction)

### 2. Pattern Similarity

| Aspect | P2 (Break) | P3 (If-Sum) | Shared? |
|--------|-----------|-------------|---------|
| Loop control | `loop(cond)` + `break` | `loop(cond)` + conditional update | ✅ Yes |
| Carriers | sum, count, result | sum, count (if-conditional) | ✅ Yes |
| Exit condition | Break early | Continue all iterations | Different |
| ConditionEnv | ✅ Used | ✅ Used (Phase 220) | ✅ Yes |
| ExitLine | ✅ Used | ✅ Used | ✅ Yes |

**Key difference**: P3 has **conditional carrier updates** inside loop body, vs P2's unconditional updates before break.

### 3. Normalized Extension Points

P3 needs minimal additions to existing Normalized:

**Already working**:
- ConditionEnv (loop + if conditions)
- CarrierInfo (state tracking)
- EnvLayout (carrier fields)
- LoopHeaderPHI (entry/latch values)

**Need to add**:
- **ConditionalUpdate** pattern in StepScheduleBox
  - P2: `[HeaderCond, BodyInit, BreakCheck, Updates, Tail]`
  - P3: `[HeaderCond, IfCond, ThenUpdates, ElseUpdates, Tail]`
- **If branching** in Normalized JpInst
  - Already exists: `If { cond, then_target, else_target, env }`
  - Just need to emit for P3 body structure

## Architecture: Unified Normalized

```
┌──────────────────────────────────────────┐
│   Structured JoinIR (Pattern1-4 共通)    │
│  - ConditionEnv (P2/P3/P4 統一 Phase 220) │
│  - CarrierInfo                           │
│  - ExitLine/Boundary                     │
└──────────────┬───────────────────────────┘
               │
               ▼
┌──────────────────────────────────────────┐
│   Normalized JoinIR (Pattern1-4 共通)    │  ← P3 もここに載せる！
│  - EnvLayout (P2 完成 → P3 拡張)         │
│  - JpInst/JpOp (If 分岐追加)            │
│  - StepScheduleBox (ConditionalUpdate)   │
└──────────────┬───────────────────────────┘
               │
               ▼
┌──────────────────────────────────────────┐
│   MIR (Pattern1-4 共通)                  │
└──────────────────────────────────────────┘
```

## Representative P3 Loops

### Phase 47-A: Minimal (sum_count)

**Example**: `phase212_if_sum_min.hako`

```nyash
local sum = 0
local count = 0
local i = 0
local n = 5

loop(i < n) {
    if (i % 2 == 1) {
        sum = sum + i
        count = count + 1
    }
    i = i + 1
}
```

**Characteristics**:
- Simple condition: `i % 2 == 1`
- Two carriers: `sum`, `count` (conditionally updated)
- One loop param: `i` (always updated)
- No break, runs all iterations

**Normalized shape**:
- EnvLayout: `{ i: int, sum: int, count: int }`
- StepSchedule: `[HeaderCond(i < n), IfCond(i % 2 == 1), ThenUpdates(sum, count), Updates(i), Tail]`

### Phase 47-B: Extended dev targets（今回の範囲）

| Fixture | Carriers / Params | 条件式 | EnvLayout 期待 |
|---------|------------------|--------|----------------|
| `pattern3_if_sum_multi_min` | `i`, `sum`, `count` | `i > 0` | `i, sum, count, len` |
| `jsonparser_if_sum_min` | `i`, `sum` | `i > 0`（JsonParser 由来の簡約形） | `i, sum, len` |

特徴:
- どちらも if/else を持ち、Else は no-op（Select/If PHI を確実に通すため）
- MethodCall は含めず、まずは条件＋複数キャリア更新に絞った dev 拡張
- P3 if-sum capability (= P3IfSum) に乗せ、Structured→Normalized→MIR(direct) で A/B 比較する

**Complexity**: Higher than sum_count (method calls, body-local)

### Phase 47-C: Selfhost loops (future)

Complex P3 patterns from selfhost compiler (deferred to later phase).

## Implementation Status

**Phase 47-A-PREP** (✅ Complete, commit 42ecd7a7):
- Fixture added: `pattern3_if_sum_minimal` in `normalized/fixtures.rs`
- Test stub added: `test_normalized_pattern3_if_sum_minimal_runner_dev_switch_matches_structured`
- Basic infrastructure for P3 development mode testing

**Phase 47-A-IMPL** (✅ Complete, 2025-12-12):
- ✅ StepSchedule renamed and extended: `pattern2_step_schedule.rs` → `step_schedule.rs`
- ✅ P3 StepKind added: `IfCond`, `ThenUpdates`, `ElseUpdates`
- ✅ Pattern2 lowering separation: P3 steps panic in P2 lowering（責務境界を明確化）
- ✅ ShapeGuard: `Pattern3IfSumMinimal` detection added（構造ベース検出）
- ✅ Normalized bridge: `normalize_pattern3_if_sum_minimal` を通じて P3 最小ケースを Normalized→MIR(direct) パイプラインに統合
- ✅ Runner / VM tests: P3 minimal について Structured 経路と Normalized→MIR(direct) 経路が一致することを dev-only スイートで確認
- ✅ 938/938 tests PASS（退行なし）

**Phase 47-B (✅ Extended dev)**:
- Fixture 拡張: `pattern3_if_sum_multi_min.program.json`（sum+count） / `jsonparser_if_sum_min.program.json` を normalized_dev フィクスチャに追加
- ShapeGuard: `Pattern3IfSumMulti` / `Pattern3IfSumJson` 追加、capability=P3IfSum
- Normalizer/Bridge: P3 if-sum multi/json を Structured→Normalized→MIR(direct) で dev 実行、Structured と一致
- Tests: `normalized_pattern3_if_sum_multi_vm_bridge_direct_matches_structured` / `normalized_pattern3_json_if_sum_min_vm_bridge_direct_matches_structured`

**Phase 47-C (⏩ Canonical 化)**:
- P3 if-sum minimal/multi/json を canonical Normalized セットに昇格（mode/env に関わらず direct ルート）
- Bridge/runner は P2 と同様に P3 を常時 Normalized→MIR(direct) へルーティング
- Docs/overview を canonical セット拡張に合わせて更新

## Implementation Strategy

### Phase 47-A: Minimal sum_count (dev-only, 実装済みの範囲)

**Goal**: Prove P3 can use Normalized infrastructure with minimal additions.

**実装済みステップ**:
1. **ShapeGuard**: `Pattern3IfSumMinimal` shape 追加
   - Compare + Select + tail call を持つ 3 関数構成の P3 if-sum 最小パターンを検出。
2. **StepScheduleBox**: P3 用 StepKind 追加
   - `IfCond` / `ThenUpdates` / `ElseUpdates` を `step_schedule.rs` に導入し、P2/P3 共通の箱として再利用。
3. **Normalized lowering（minimal）**:
   - `normalize_pattern3_if_sum_minimal` により、P3 最小ケースを P2 と同じ器の NormalizedModule に変換し、direct ブリッジで扱える形に整備。
4. **Tests**:
   - Runner 経路: `normalized_pattern3_if_sum_minimal_runner_dev_switch_matches_structured`
   - Normalization 経路: `test_phase47a_pattern3_if_sum_minimal_normalization`（shape + normalize 成功）
   - （詳細な VM Bridge 比較は Phase 47-B 以降で拡張していく）

**Dev fixture**: `apps/tests/phase212_if_sum_min.hako`（最小 if-sum パターン）

### Phase 47-B: array_filter (dev-only)

**Goal**: Extend to body-local + method calls.

**Additions**:
- Body-local handling in EnvLayout (already exists for P2 DigitPos)
- Method call in if condition (ExprLowerer already supports)

### Phase 47-C: Canonical promotion

**Goal**: Move P3 minimal from dev-only to canonical (like P2).

**Criteria**:
- All invariants verified (Phase 47-A/B tests passing)
- No regressions in 937+ tests
- Performance acceptable (Normalized vs Structured comparison)

## Normalized Components for P3

### EnvLayout Extension

**P2 example**:
```rust
struct Pattern2Env {
    i: int,      // loop param
    sum: int,    // carrier
}
```

**P3 extension** (same structure):
```rust
struct Pattern3Env {
    i: int,      // loop param
    sum: int,    // carrier (conditionally updated)
    count: int,  // carrier (conditionally updated)
}
```

**No new fields needed** - P3 carriers work same as P2.

### StepScheduleBox Extension

**P2 steps**:
```rust
enum StepKind {
    HeaderCond,   // loop(cond)
    BodyInit,     // local ch = ...
    BreakCheck,   // if (cond) break
    Updates,      // sum = sum + 1
    Tail,         // i = i + 1
}
```

**P3 addition**:
```rust
enum StepKind {
    // ... existing P2 steps
    IfCond,           // if (cond) in body
    ThenUpdates,      // carrier updates in then branch
    ElseUpdates,      // carrier updates in else branch (if any)
}
```

**P3 schedule**:
```rust
// sum_count pattern
[HeaderCond, IfCond, ThenUpdates, Updates, Tail]

// vs P2 pattern
[HeaderCond, BodyInit, BreakCheck, Updates, Tail]
```

### JpInst Reuse

**Already exists** in Normalized (from P2):
```rust
pub enum JpInst {
    Let { dst, op, args },
    If { cond, then_target, else_target, env },  // ← P3 uses this!
    TailCallFn { target, args },
    TailCallKont { target, args },
}
```

**P3 usage**:
- `If` instruction for body if-statement
- `then_target` → block with carrier updates
- `else_target` → block without updates (or with different updates)

**No new JpInst needed!**

## Testing Strategy

### Phase 47-A: Minimal

**Test (runner-based, implemented)**: `normalized_pattern3_if_sum_minimal_runner_dev_switch_matches_structured`

```rust
#[cfg(feature = "normalized_dev")]
#[test]
fn test_normalized_pattern3_if_sum_minimal() {
    let source = r#"
        local sum = 0
        local count = 0
        local i = 0
        local n = 5
        loop(i < n) {
            if (i % 2 == 1) {
                sum = sum + i
                count = count + 1
            }
            i = i + 1
        }
        print("sum = " + sum.to_string())
        print("count = " + count.to_string())
    "#;

    // 現在は Runner ベースで Structured→Normalized→Structured roundtrip のみ実装済み。
    // VM Bridge での Normalized→MIR(direct) 比較は後続ステップ。
}
```

**Expected output**（phase212_if_sum_min.hako 相当）:
`sum = 2`（3 回中 2 回加算）

### Phase 47-B: array_filter

**Test**: `test_normalized_pattern3_array_filter`

Similar structure, verify method calls + body-local work in Normalized.

## Success Criteria

**Phase 47-A complete when**:
1. ✅ `test_normalized_pattern3_if_sum_minimal` passes (dev-only)
2. ✅ Structured→Normalized→MIR(direct) output matches Structured→MIR
3. ✅ All 937+ tests still pass (no regressions)
4. ✅ ShapeGuard can detect Pattern3IfSumMinimal
5. ✅ Documentation updated (architecture overview, CURRENT_TASK)

**Phase 47-B complete when**:
1. ✅ array_filter test passes (dev-only)
2. ✅ Body-local + method calls work in P3 Normalized

**Phase 47-C complete when**:
1. ✅ P3 minimal promoted to canonical (always Normalized)
2. ✅ Performance validated

## Scope Management

**In Scope (Phase 47-A)**:
- ✅ Minimal P3 (sum_count pattern)
- ✅ Dev-only Normalized support
- ✅ Reuse P2 infrastructure (ConditionEnv, CarrierInfo, ExitLine)

**Out of Scope (deferred)**:
- ❌ Complex P3 patterns (nested if, multiple conditions)
- ❌ Canonical promotion (Phase 47-C)
- ❌ Pattern4 (continue) support (separate NORM-P4 phase)
- ❌ Selfhost loops (later phase)

## File Impact Estimate

**Expected modifications** (Phase 47-A):
1. `shape_guard.rs`: +20 lines (Pattern3IfSumMinimal shape)
2. `step_schedule.rs`: +40 lines (P3 step kinds, rename from pattern2_*)
3. `normalized_bridge/direct.rs`: +60 lines (If instruction handling)
4. `tests/normalized_joinir_min.rs`: +30 lines (P3 test)
5. `phase47-norm-p3-design.md`: +200 lines (this doc)
6. `joinir-architecture-overview.md`: +10 lines (Phase 47 section)
7. `CURRENT_TASK.md`: +5 lines (Phase 47 entry)

**Total**: ~365 lines (+), pure additive (no P2 code changes)

## Benefits

1. **Infrastructure reuse**: 90% of P2 Normalized code works for P3
2. **Unified pipeline**: All patterns (P1/P2/P3) use same Normalized
3. **Incremental rollout**: Dev-only → canonical, same as P2
4. **Clear path to P4**: Pattern4 (continue) follows same approach

## Next Steps After Phase 47

1. **NORM-P4**: Pattern4 (continue) Normalized support
2. **Canonical promotion**: Move P3/P4 from dev-only to canonical
3. **Selfhost loops**: Complex patterns from selfhost compiler
4. **Performance optimization**: Profile Normalized vs Structured

## References

- **P2 Completion**: [PHASE_43_245B_NORMALIZED_COMPLETION.md](./PHASE_43_245B_NORMALIZED_COMPLETION.md)
- **Phase 46 P2-Mid**: [phase46-norm-canon-p2-mid.md](./phase46-norm-canon-p2-mid.md)
- **Architecture**: [joinir-architecture-overview.md](./joinir-architecture-overview.md)
- **Phase 220 P3 Foundation**: joinir-architecture-overview.md lines 73-84
