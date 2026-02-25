# Phase 48: Normalized P4 (Continue) Design

**Status**: Phase 48-A/B/C COMPLETE (minimal + JsonParser skip_ws continue、Normalized→MIR 直経路＋canonical 昇格まで完了)
**Date**: 2025-12-12 / 2026-01-XX

## Goal

Design Pattern4 (continue) Normalized architecture, extending the unified Normalized infrastructure that successfully handles P1/P2/P3.

**Key insight**: P4 is the **reverse control flow** of P2 (break). Where P2 exits early, P4 skips to next iteration early. Same infrastructure, different routing.

## Background: Unified Normalized Success

Phase 43-47 established unified Normalized for P1/P2/P3:
- ✅ Pattern1: Simple while loops
- ✅ Pattern2: Break loops (skip_whitespace, _atoi, _parse_number)
- ✅ Pattern3: If-sum loops (conditional carrier updates)

**Infrastructure proven**:
- Structured→Normalized→MIR(direct) pipeline
- EnvLayout, JpInst/JpOp, StepScheduleBox
- ConditionEnv, CarrierInfo, ExitLine
- All patterns use same `loop_step(env, k_exit)` skeleton

## Why P4 Uses Same Normalized

### Control Flow Comparison

| Aspect | P2 (Break) | P4 (Continue) | Difference |
|--------|-----------|---------------|------------|
| Normal flow | Execute body, update carriers, loop | Same | ✅ Identical |
| Early exit | `if (cond) break` → exit loop | `if (cond) continue` → next iteration | Flow direction |
| Carrier updates | Before break check | After continue check | Order |
| Infrastructure | ConditionEnv, ExitLine, PHI | **Same** | ✅ Reusable |

**Key difference**: `continue` = `TailCallFn(loop_step, env', k_exit)` (immediate recursion) vs `break` = `TailCallKont(k_exit, result)` (exit to continuation).

### P4 in Normalized JoinIR

```rust
// P2 (break) structure:
loop_step(env, k_exit) {
    if (header_cond) {
        // body
        if (break_cond) {
            TailCallKont(k_exit, result)  // Exit early
        }
        // update carriers
        TailCallFn(loop_step, env', k_exit)  // Loop back
    } else {
        TailCallKont(k_exit, result)  // Normal exit
    }
}

// P4 (continue) structure:
loop_step(env, k_exit) {
    if (header_cond) {
        // body
        if (continue_cond) {
            TailCallFn(loop_step, env', k_exit)  // Skip to next iteration ← continue!
        }
        // update carriers (only if NOT continued)
        TailCallFn(loop_step, env'', k_exit)  // Loop back
    } else {
        TailCallKont(k_exit, result)  // Normal exit
    }
}
```

**Observation**: `continue` is just an early `TailCallFn` call. No new JpInst needed!

## Target P4 Loops (JsonParser)

### Priority Assessment

| Loop | Pattern | Complexity | Priority | Rationale |
|------|---------|------------|----------|-----------|
| _parse_array (skip whitespace) | P4 minimal | Low | ◎ PRIMARY | Simple continue, single carrier (i) |
| _parse_object (skip whitespace) | P4 minimal | Low | ○ Extended | Same as _parse_array |
| _unescape_string (skip special chars) | P4 mid | Medium | △ Later | String operations, body-local |
| _parse_string (escape handling) | P4 mid | Medium | △ Later | Complex escape sequences |

### Phase 48-A Target: _parse_array (skip whitespace)

**Example** (simplified):
```nyash
local i = 0
local s = "[1, 2]"
local len = s.length()

loop(i < len) {
    local ch = s.substring(i, i+1)

    if (ch == " " || ch == "\t") {
        i = i + 1
        continue  // Skip whitespace
    }

    // Process non-whitespace character
    // ...
    i = i + 1
}
```

**Characteristics**:
- Simple condition: `ch == " " || ch == "\t"` (OR pattern)
- Single carrier: `i` (position counter)
- Body-local: `ch` (character)
- continue before carrier update

**Normalized shape**:
- EnvLayout: `{ i: int }`
- StepSchedule: `[HeaderCond, BodyInit(ch), ContinueCheck, Updates(process), Tail(i++)]`

## Normalized Components for P4

### StepScheduleBox Extension

**P2/P3 steps** (existing):
```rust
enum StepKind {
    HeaderCond,   // loop(cond)
    BodyInit,     // local ch = ...
    BreakCheck,   // if (cond) break  (P2)
    IfCond,       // if (cond) in body  (P3)
    ThenUpdates,  // carrier updates (P3)
    Updates,      // carrier updates
    Tail,         // i = i + 1
}
```

**P4 addition**:
```rust
enum StepKind {
    // ... existing ...

    ContinueCheck,  // if (cond) continue  (P4)
}
```

**P4 schedule**:
```rust
// _parse_array skip whitespace pattern
[HeaderCond, BodyInit, ContinueCheck, Updates, Tail]

// vs P2 pattern
[HeaderCond, BodyInit, BreakCheck, Updates, Tail]

// Observation: Same structure, different check semantics!
```

### JpInst Reuse

**No new JpInst needed!** P4 uses existing instructions:

```rust
// P2 break:
If { cond, then_target: k_exit, else_target: continue_body }

// P4 continue:
If { cond, then_target: loop_step_with_tail, else_target: process_body }
```

**Key**: `continue` = immediate `TailCallFn(loop_step, ...)`, not a new instruction.

### EnvLayout (Same as P2)

**P2 example**:
```rust
struct Pattern2Env {
    i: int,      // loop param
    sum: int,    // carrier
}
```

**P4 example** (identical structure):
```rust
struct Pattern4Env {
    i: int,      // loop param (position counter)
    // No additional carriers for skip whitespace
}
```

**No new fields needed** - P4 carriers work same as P2/P3.

## Architecture: Unified Normalized

```
┌──────────────────────────────────────────┐
│   Structured JoinIR (Pattern1-4 共通)    │
│  - ConditionEnv (P2/P3/P4 統一)          │
│  - CarrierInfo                           │
│  - ExitLine/Boundary                     │
└──────────────┬───────────────────────────┘
               │
               ▼
┌──────────────────────────────────────────┐
│   Normalized JoinIR (Pattern1-4 共通)    │  ← P4 もここに載る！
│  - EnvLayout (P2 完成 → P3/P4 拡張)      │
│  - JpInst/JpOp (既存で対応済み)          │
│  - StepScheduleBox (ContinueCheck 追加)   │
└──────────────┬───────────────────────────┘
               │
               ▼
┌──────────────────────────────────────────┐
│   MIR (Pattern1-4 共通)                  │
└──────────────────────────────────────────┘
```

## Implementation Strategy

### Phase 48-A: Minimal _parse_array skip whitespace (dev-only)

**Goal**: Prove P4 can use Normalized infrastructure with minimal additions.

**実装ステータス（48-A 完了サマリ）**:

- ✅ Fixture 追加: `pattern4_continue_min.program.json`
  - 「`i == 2` を `continue` でスキップする最小 P4 ループ」を Program(JSON) として用意。
- ✅ ShapeGuard 拡張:
  - `NormalizedDevShape::Pattern4ContinueMinimal` を追加し、構造ベースで P4 minimal 形状を検出。
- ✅ StepScheduleBox 拡張:
  - `StepKind::ContinueCheck` を追加し、評価順序を  
    `HeaderCond → ContinueCheck → Updates → Tail` に固定。
- ✅ Normalized lowering:
  - `normalize_pattern4_continue_minimal()` を実装し、P2 正規化ロジックを 95% 再利用した continue 対応を追加。
- ✅ テスト:
  - Normalized dev スイートに P4 minimal 用の比較テストを 4 本追加  
    （Structured→Normalized→MIR(direct) vs Structured→MIR / runner / VM bridge）。
  - `cargo test --release` ベースで **939/939 tests PASS**（Phase 48-A 実装時点）。

**Steps**:
1. **ShapeGuard**: Add `Pattern4ContinueMinimal` shape
2. **StepScheduleBox**: Add `ContinueCheck` step kind
3. **Normalized lowering**:
   - Generate `If` JpInst for continue check
   - `then_target` → immediate `TailCallFn(loop_step, ...)` (continue)
   - `else_target` → process body, then tail
4. **Test**: Verify Structured→Normalized→MIR(direct) matches Structured→MIR

**Expected additions**:
- `shape_guard.rs`: +1 shape variant
- `step_schedule.rs`: +1 step kind (`ContinueCheck`)
- `normalized.rs`: +40 lines (normalize_pattern4_continue_minimal)
- `tests/normalized_joinir_min.rs`: +1 P4 test

**Dev fixture**: Create `pattern4_continue_minimal` from _parse_array skip whitespace

### Phase 48-B: _parse_object, _unescape_string (dev-only)

**Status (dev-only)**: `_parse_array` / `_parse_object` の whitespace continue ループを Normalized→MIR(direct) で比較済み。  
Fixture を `jsonparser_parse_{array,object}_continue_skip_ws.program.json` として追加し、shape_guard / normalize_for_shape / direct bridge で dev 専用ルートを通す。  
_unescape_string は未着手（Phase 48-C 以降）。

**Goal**: Extend to multiple carriers, string operations (unescape) after skip_ws 系が固まったら続行。

**Additions**:
- Multi-carrier EnvLayout (if needed)
- String body-local handling (already exists from P2 DigitPos)

### Phase 48-C: Canonical promotion

**Goal**: Move P4 minimal from dev-only to canonical (like P2/P3).

## Key Design Decisions

### 1. Continue = TailCallFn, not new instruction

**Rationale**: `continue` is semantically "skip to next iteration", which is exactly what `TailCallFn(loop_step, env', k_exit)` does in CPS.

**Benefit**: No new JpInst, reuses existing MIR generation.

### 2. ContinueCheck step before Updates

**Rationale**: continue must happen BEFORE carrier updates (skip processing).

**P4 step order**:
```
HeaderCond → BodyInit → ContinueCheck → Updates (processing) → Tail (increment)
                             ↓ (if true)
                        TailCallFn (skip Updates)
```

### 3. Same EnvLayout as P2

**Rationale**: P4 carriers (position, accumulators) are same types as P2.

**Benefit**: No new EnvLayout design, reuses P2 infrastructure 100%.

## Comparison with P2/P3

| Component | P2 (Break) | P3 (If-Sum) | P4 (Continue) | Shared? |
|-----------|-----------|-------------|---------------|---------|
| EnvLayout | ✅ | ✅ | ✅ | ✅ Yes |
| ConditionEnv | ✅ | ✅ | ✅ | ✅ Yes |
| CarrierInfo | ✅ | ✅ | ✅ | ✅ Yes |
| ExitLine | ✅ | ✅ | ✅ | ✅ Yes |
| StepKind | BreakCheck | IfCond, ThenUpdates | ContinueCheck | Additive |
| JpInst | If, TailCallFn, TailCallKont | ✅ Same | ✅ Same | ✅ Yes |
| Control flow | Exit early | Conditional update | Skip early | Different |

**Infrastructure reuse**: 95%+ (only StepKind and control flow routing differ)

## Testing Strategy

### Phase 48-A: Minimal

**Test**: `test_normalized_pattern4_continue_minimal`

```rust
#[cfg(feature = "normalized_dev")]
#[test]
fn test_normalized_pattern4_continue_minimal() {
    let source = r#"
        local i = 0
        local n = 5
        local count = 0
        loop(i < n) {
            if (i == 2) {
                i = i + 1
                continue
            }
            count = count + 1
            i = i + 1
        }
        print("count = " + count.to_string())
    "#;

    // Compare Structured→MIR vs Normalized→MIR(direct)
    assert_vm_output_matches(source);
}
```

**Expected output**:
```
count = 4  (skipped i==2, so counted 0,1,3,4)
```

## Success Criteria

**Phase 48-A complete when**:
1. `test_normalized_pattern4_continue_minimal` passes (dev-only)
2. Structured→Normalized→MIR(direct) output matches Structured→MIR
3. All 938+ tests still pass (no regressions)
4. ShapeGuard can detect Pattern4ContinueMinimal
5. Documentation updated (architecture overview, CURRENT_TASK)

→ 上記 1–5 はコミット `7200309c` 時点ですべて満たされており、Phase 48-A は完了ステータスだよ。

**Phase 48-B complete when**:
1. ✅ _parse_object, _unescape_string tests pass (dev-only)
2. ✅ Multi-carrier + string operations work in P4 Normalized

**Phase 48-C complete when**:
1. ✅ P4 minimal promoted to canonical (always Normalized)
2. ✅ Performance validated

## Scope Management

**In Scope (Phase 48-A)**:
- ✅ Minimal P4 (simple continue pattern)
- ✅ Dev-only Normalized support
- ✅ Reuse P2/P3 infrastructure (ConditionEnv, CarrierInfo, ExitLine)

**Out of Scope (deferred)**:
- ❌ Complex P4 patterns (nested if, multiple continue points)
- ❌ Canonical promotion (Phase 48-C)
- ❌ Selfhost loops (later phase)

## File Impact Estimate

**Expected modifications** (Phase 48-A):
1. `shape_guard.rs`: +20 lines (Pattern4ContinueMinimal shape)
2. `step_schedule.rs`: +10 lines (ContinueCheck step kind)
3. `normalized.rs`: +40 lines (normalize_pattern4_continue_minimal)
4. `tests/normalized_joinir_min.rs`: +30 lines (P4 test)
5. `phase48-norm-p4-design.md`: +250 lines (this doc)
6. `joinir-architecture-overview.md`: +10 lines (Phase 48 section)
7. `CURRENT_TASK.md`: +5 lines (Phase 48 entry)

**Total**: ~365 lines (+), pure additive (no P1/P2/P3 code changes)

## Benefits

1. **Infrastructure reuse**: 95% of P2/P3 Normalized code works for P4
2. **Unified pipeline**: All patterns (P1/P2/P3/P4) use same Normalized
3. **Incremental rollout**: Dev-only → canonical, proven approach from P2/P3
4. **Semantic clarity**: `continue` = immediate `TailCallFn` (no new concepts)

## Next Steps After Phase 48

1. **Phase 48-A implementation**: Minimal P4 (continue) dev-only
2. **Phase 48-B**: Extended P4 (multi-carrier, string ops)
3. **Phase 48-C**: Canonical promotion
4. **Selfhost loops**: Complex patterns from selfhost compiler

## References

- **P2 Completion**: [PHASE_43_245B_NORMALIZED_COMPLETION.md](./PHASE_43_245B_NORMALIZED_COMPLETION.md)
- **P3 Design**: [phase47-norm-p3-design.md](./phase47-norm-p3-design.md)
- **P3 Implementation**: Phase 47-A-LOWERING (commit 99bdf93d)
- **Architecture**: [joinir-architecture-overview.md](./joinir-architecture-overview.md)
