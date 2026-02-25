# Phase 80: BindingId P3/P4 Expansion Plan

**Status**: Completed (commit `84129a7e`)

**Created**: 2025-12-13

**Progress**:
- ✅ Task 80-0: Current Status Verification (complete)
- ✅ Task 80-A: SSOT Design Doc (complete)
- ✅ Task 80-B: Pattern3 BindingId Wiring (dev-only)
- ✅ Task 80-C: Pattern4 BindingId Wiring (dev-only)
- ✅ Task 80-D: E2E Tests (P3/P4, dev-only)

---

## Task 80-0: Current Status Verification

### Test Results

#### 1. Full lib test suite
```bash
cargo test --release --lib
```

**Result**: ✅ **PASS** - 970 passed; 0 failed; 56 ignored

**Interpretation**: All production code is stable, Phase 79 changes are production-safe.

---

#### 2. normalized_dev tests
```bash
NYASH_JOINIR_NORMALIZED_DEV_RUN=1 RUST_TEST_THREADS=1 cargo test --features normalized_dev --test normalized_joinir_min -- --nocapture
```

**Result**: ✅ **PASS**（Phase 79 の “AST直組み E2E” は Normalized SSOT から外し、別途 Phase 80-D で整理予定）

---

#### 3. Quick smoke tests
```bash
tools/smokes/v2/run.sh --profile quick --filter "json_pp_vm"
```

**Result**: ✅ **PASS** - Representative smoke test passes

**Details**:
- `json_pp_vm`: ✅ PASS (.005s)
- `json_lint_vm`: ❌ FAIL (pre-existing, unrelated to Phase 79/80)
  - Error: "normalized Pattern2 verifier: function 'main' does not end with tail call/if"
  - Also fails on clean Phase 79 commit `b7f78825`
  - Classification: **Out of scope** for Phase 80

**Interpretation**: Core VM functionality stable, Phase 79/80 changes don't break smoke tests

---

## Decision: Defer Pattern2 E2E（DigitPos/Trim）until ExitLine contract is stabilized

Phase 80 の主目的は Pattern3/4 の BindingId 配線なので、Pattern2 の E2E は Phase 81+ に回し、
さらに “promoted carriers（DigitPos/Trim）を含む Pattern2 の ExitLine 接続” が安定してから固定する。

## Task 80-0: Summary

**Status**: ✅ **COMPLETE**

**Key Findings**:
1. ✅ **Production code stable** - 970/970 lib tests PASS
2. ✅ **Phase 79 changes production-safe** - No regressions in lib tests
3. ✅ **Core VM functionality stable** - Smoke tests pass
4. ⚠️ **Pattern2（DigitPos/Trim）E2E は保留** - promoted carriers を含む ExitLine 契約の安定化後に固定する
5. ❌ **Pre-existing smoke test failure** - `json_lint_vm` fails (unrelated)

**Classification**:
- **Phase 80 blockers**: NONE
- **Phase 80 out-of-scope**:
  - Pattern2（DigitPos/Trim）promoted carriers の ExitLine 契約安定化（Phase 81+）
  - json_lint_vm smoke test failure (pre-existing Pattern2 verifier issue)

**Decision**: ✅ **Proceed to Task 80-A**

**Justification**:
- Production code is stable and safe
- Dev-only test failures are **isolated** and **documented**
- Phase 80-B/C を先に進め、E2E（80-D）は Pattern2 契約の整備後に固定する

---

## Next Steps

✅ Task 80-0 complete → Proceed to Task 80-A (Phase 80 SSOT design doc)

---

## Task 80-A: SSOT Design Doc

**Goal**: Document Pattern3/4 BindingId wiring strategy BEFORE implementation

**Status**: ✅ **COMPLETE**

### Phase 80 Scope

**In Scope**:
1. Pattern3 (if-sum) BindingId registration at ValueId determination point
2. Pattern4 (continue/Trim) BindingId registration at ValueId determination point
3. E2E verification tests (2 tests: P3 + P4)
4. Fallback detection capability (existing `[binding_pilot/fallback]` tags)

**Out of Scope** (deferred to Phase 81+):
- Name fallback removal (dual-path stays)
- BindingId mandatory enforcement
- Pattern1 Minimal (already has structural detection, doesn't use carriers)
- by-name rule branching (prohibited by invariant)

---

### Design Principles (unchanged from Phase 74-79)

1. **Dev-only**: All code `#[cfg(feature = "normalized_dev")]` or `#[cfg(debug_assertions)]`
2. **Dual-path maintained**: BindingId priority + name fallback (no removal yet)
3. **Structural detection only**: NO by-name rule branching
4. **Fail-Fast**: Explicit errors with tags, no silent fallbacks
5. **Zero production impact**: All changes gated, 970/970 lib tests must PASS

---

### Pattern3 (if-sum) BindingId Wiring Strategy

**Entry point**: `src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs`

**Key function**: `lower_pattern3_if_sum()`

**ConditionEnv creation**: `ConditionEnvBuilder` 生成直後（`lower_if_sum_pattern()` に渡す前）

**BindingId registration points**:

1. **Loop var registration**（dev-only）:
   ```rust
   #[cfg(feature = "normalized_dev")]
   if let Some(loop_var_bid) = builder.binding_map.get(&loop_var_name).copied() {
       cond_env.register_loop_var_binding(loop_var_bid, loop_var_join_id);
       eprintln!("[phase80/p3] Registered loop var '{}' BindingId({}) -> ValueId({})",
                 loop_var_name, loop_var_bid.0, loop_var_join_id.0);
   }
   ```

2. **Condition bindings registration**（dev-only）:
   ```rust
   #[cfg(feature = "normalized_dev")]
   for binding in &inline_boundary.condition_bindings {
       if let Some(bid) = builder.binding_map.get(&binding.name).copied() {
           cond_env.register_condition_binding(bid, binding.join_value);
           eprintln!(
               "[phase80/p3] Registered condition binding '{}' BindingId({}) -> ValueId({})",
               binding.name, bid.0, binding.join_value.0
           );
       }
   }
   ```

**Timing**: BEFORE condition lowering

**Note (重要)**:
- Pattern3/4 は lowering の後段（ExitMeta / merge 側）で carrier join_id が確定するため、Phase 80 では **carriers の BindingId 登録はしない**。
- ここでの目的は「条件 lowering（lookup_with_binding）」の経路を先に BindingId 化して、shadowing の破綻を防ぐこと。

---

### Pattern4 (continue/Trim) BindingId Wiring Strategy

**Entry point**: `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs`

**Key function**: `cf_loop_pattern4_with_continue()`

**ConditionEnv creation**: `lower_loop_with_continue_minimal()` 内

**BindingId registration points**:

1. **Caller side**（dev-only）: `binding_map` を lowerer に渡す

2. **Lowerer side**（dev-only）: join_id が確定したタイミングで
   - loop var / condition bindings / carriers を BindingId 登録する
   - ログは `[phase80/p4]` タグで可視化する

**Special note**: Trim patterns (skip_whitespace) use promoted carriers, BindingId comes from `promoted_bindings` map via CarrierBindingAssigner

**Timing**: ValueId allocation の直後（condition lowering の前）

---

### Fallback Detection Mechanism

**Existing infrastructure** (no changes needed):
- `ConditionEnv::lookup_with_binding()` - tries BindingId first, logs `[binding_pilot/hit]`
- `ConditionEnv::lookup_or_fallback()` - fallback to name, logs `[binding_pilot/fallback]`

**Detection strategy**:
1. Run tests with `HAKO_JOINIR_DEBUG=1` (or legacy `NYASH_JOINIR_DEBUG=1`)
2. Check for `[binding_pilot/hit]` tags (BindingId path success)
3. Check for NO `[binding_pilot/fallback]` tags (name fallback NOT used)
4. If fallback occurs → test fails with diagnostic

**Note (Phase 82)**: Both `HAKO_JOINIR_DEBUG` and `NYASH_JOINIR_DEBUG` are supported.
Recommended: Use `HAKO_JOINIR_DEBUG=1` (NYASH_ variant is deprecated but still works)

**Acceptance criteria** (Task 80-D):
- P3 test: BindingId hit, NO fallback
- P4 test: BindingId hit, NO fallback

**Fallback Behavior Documentation (Phase 83)**:

**Expected**: Promoted carriers should ALWAYS use BindingId path, never fallback
- DigitPos carriers (`is_digit_pos`): ✅ BindingId hit only
- Trim carriers (`is_ch_match`): ✅ BindingId hit only
- Loop variables in P3/P4: ✅ BindingId hit only

**Verification**:
```bash
# Phase 80 tests verify BindingId resolution works (no runtime errors)
cargo test --features normalized_dev --test normalized_joinir_min test_phase80_p3_bindingid_lookup_works
cargo test --features normalized_dev --test normalized_joinir_min test_phase80_p4_bindingid_lookup_works

# Phase 81 tests verify ExitLine contract (promoted carriers handled correctly)
cargo test --features normalized_dev --test normalized_joinir_min test_phase81_digitpos_exitline_contract
cargo test --features normalized_dev --test normalized_joinir_min test_phase81_trim_exitline_contract
```

**Debug Tags** (dev-only, during MIR compilation):
- `[binding_pilot/hit]`: BindingId lookup succeeded ✅ (expected)
- `[binding_pilot/fallback]`: Name-based fallback occurred ❌ (should NOT appear for promoted carriers)
- `[binding_pilot/legacy]`: No BindingId provided, using name (legacy code paths only)

**Status (Phase 83)**: All Phase 80/81 tests PASS, indicating NO fallback to name-based lookup for promoted carriers.

---

### Implementation Order

1. ✅ Task 80-A: Design doc (this section)
2. ✅ Task 80-B: Pattern3 wiring（loop var + condition bindings）
3. ✅ Task 80-C: Pattern4 wiring（lowerer 側で登録）
4. ✅ Task 80-D: E2E tests（P3 1本 / P4 1本, `tests/normalized_joinir_min.rs`）

---

### Success Metrics

- [x] P3: 条件 lowering が BindingId 経路で解決できる（E2Eテストで固定）
- [x] P4: 条件 lowering が BindingId 経路で解決できる（E2Eテストで固定）
- [x] Fallback 検知（`[binding_pilot/fallback]` をテストで検知できる）
- [x] `cargo test --release --lib` が PASS（production 影響なし）
- [x] 追加コードは dev-only（`normalized_dev`）に閉じている

---

## Goal (unchanged from Phase 80 spec)

- Pattern3 (if-sum) の条件 lowering で lookup_with_binding() が効く
- Pattern4 (continue/skip_ws) の条件 lowering で lookup_with_binding() が効く
- Fallback 監視: name fallback に落ちたら分かる仕掛け（既存ログタグ活用）

## Non-Goal

- Name fallback の撤去はまだ（Phase 81+ で対応）
- BindingId 完全義務化はまだ（dual-path 維持）

## Invariant

- by-name ルール分岐禁止（structural detection のみ）
- binding_id_map 登録は「ValueId が確定した時点」のみ
- promoted が絡む場合は CarrierVar.binding_id / promoted_bindings を経由

## Implementation Tasks (pending Task 80-0 completion)

1. Pattern3 (if-sum) BindingId 登録配線 ✅
2. Pattern4 (continue) BindingId 登録配線 ✅
3. E2E tests (P3 1本 / P4 1本) ✅

## Success Metrics

- P3/P4 代表ケースで lookup_with_binding() 経路がヒット（ログ or テスト）
- Fallback 検知可能（既存 `[binding_pilot/fallback]` タグ活用）
- `cargo test --release --lib` PASS（退行なし）
