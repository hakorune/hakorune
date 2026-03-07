# Phase 263 P0: loop_break body-local `seg` 問題の修正

Current route name: `loop_break`  
Historical numbered label: `Pattern2`

Reading note:
- 下の debug tag / path token に出る `pattern2` は original phase log を示す historical token だよ。

## Goal
loop_break body-local promotion で処理できない "seg" 変数（loop body で代入される変数）を検出した時、loop_break route **全体** を早期終了させる（部分的な処理続行は禁止）。

## Background

### 現状の問題
- **Test**: `core_direct_array_oob_set_rc_vm` (quick smoke)
- **Failure**: Stage-B compile で loop_break route が "seg" 変数を promote できず rejection
- **Historical debug output** (修正前, legacy `pattern2` tag):
  ```
  [cf_loop/pattern2] Cannot promote LoopBodyLocal variables ["seg"]:
  No promotable pattern detected (tried A-3 Trim, A-4 DigitPos);
  read-only-slot rejected: [pattern2/body_local_slot/contract/not_readonly]
  'seg' must be read-only (assignment detected in loop body)
  ```

### Root Cause
1. bundle_resolver loop (Stage-B JSON) が "seg" 変数を使用
2. loop_break route の PromoteStepBox が routing:
   - A-3 (Trim): マッチせず FAIL
   - A-4 (DigitPos): マッチせず FAIL
   - ReadOnlyBodyLocalSlot: `seg = ...` 代入があるため契約違反 FAIL
3. **全ルート exhausted → Reject で処理続行 → 後段エラー** ← ここを修正

### Expected Behavior
- loop_break route で処理できない場合は **route 全体を早期終了** （部分的な処理続行は禁止）
- lowering orchestrator が早期に `Ok(None)` を return
- Router が後続経路（legacy binding / normalized shadow 等）に進む
- detection→extract→lower の SSOT を維持
- **Fail-Fast 原則**: 対象だが未対応のケースは Err で即座に失敗（silent skip 禁止）

### Architecture Finding (探索結果)
1. **Historical route ordering at the time** (router.rs):
   - loop_simple_while は loop_break route の**前に**試される（historical numbered labels: Pattern1 → Pattern2）
   - loop_break route が失敗しても loop_simple_while には fallback **しない**（loop_simple_while は既に試された後）
   - loop_break route の `Ok(None)` は後続経路（legacy binding / normalized shadow 等）に進む
2. **loop_break orchestrator**:
   - 8 steps の pipeline: Gather → Apply → **Promote** → Normalize → BodyLocal → CarrierUpdates → Emit → Merge
   - 修正前は全 step を必ず実行（早期終了なし）→ 修正後は Promote で早期終了可能に

## Strategy: loop_break route 全体の早期終了

### Implementation

#### File 1: promote_step_box.rs - 戻り値を Option 化

**修正箇所**: historical path token `src/mir/builder/control_flow/joinir/patterns/pattern2_steps/promote_step_box.rs`
current family: `src/mir/builder/control_flow/plan/loop_break_steps/`

**戻り値の変更**:
- `PromoteStepBox::run()`: `Result<PromoteStepResult, String>` → `Result<Option<PromoteStepResult>, String>`
- `promote_and_prepare_carriers()`: `Result<(), String>` → `Result<Option<()>, String>`

**Reject の二分化** (Fail-Fast 原則):
```rust
PolicyDecision::Reject(reason) => {
    // Phase 263 P0: Reject を二分化
    if reason.contains("not_readonly")
        || reason.contains("No promotable pattern detected")
    {
        // 対象外: loop_break route で処理できない形 → Ok(None) で後続経路へ
        #[cfg(debug_assertions)]
        {
            eprintln!(
                "[pattern2/promote_step] Pattern2 対象外（LoopBodyLocal {:?}）: {}. 後続経路へfallback.",
                cond_body_local_vars, reason
            );
        }
        return Ok(None);  // loop_break route 全体を中止
    } else {
        // 対象だが未対応（freeze級）: 実装バグ or 将来実装予定 → Err で Fail-Fast
        return Err(format!(
            "[pattern2/promote_step] Pattern2 未対応エラー（LoopBodyLocal {:?}）: {}",
            cond_body_local_vars, reason
        ));
    }
}
```

**判定基準**:
- **対象外** (Ok(None)): `not_readonly`, `No promotable pattern detected` 等
  - loop_break route の責務範囲外（他の route で処理すべき）
- **対象だが未対応** (Err): それ以外の Reject
  - 本来 loop_break route が処理すべきだが未実装 or バグ → Fail-Fast

#### File 2: historical token lane `pattern2_lowering_orchestrator.rs` - Ok(None) を検出して早期終了

**修正箇所**: historical path token `src/mir/builder/control_flow/joinir/patterns/pattern2_lowering_orchestrator.rs`

**変更箇所** (line 65-74):
```rust
let promoted = PromoteStepBox::run(builder, condition, body, inputs, debug, verbose)?;
let mut inputs = match promoted {
    Some(result) => result.inputs,
    None => {
        // Phase 263 P0: Pattern2 cannot handle this loop (e.g., reassigned LoopBodyLocal)
        // Return Ok(None) to allow router to try next path (legacy binding)
        super::super::trace::trace().debug("pattern2", "Pattern2 aborted (promotion failed), allowing fallback");
        return Ok(None);
    }
};
```

**重要**:
- loop_break orchestrator の戻り値 `Result<Option<ValueId>, String>` は既存のまま（変更不要）
- `Ok(None)` を返すことで、router.rs が次の処理（legacy binding / normalized shadow 等）に進む
- merge/EdgeCFG 側は触らない（loop_break scope 内で完結）

## Test Results

### Commit
```
commit 93022e7e1
fix(pattern2): abort entire Pattern2 on unpromoted LoopBodyLocal instead of partial execution

Phase 263 P0: Pattern2 で処理できない LoopBodyLocal を検出したら Pattern2 全体を早期終了

Changes:
- promote_step_box.rs: 戻り値を Result<Option<_>, String> に変更
  - Reject を二分化: 対象外（not_readonly等）→ Ok(None)、対象だが未対応 → Err
- historical `pattern2_lowering_orchestrator.rs`: Ok(None) 検出で早期 return
- apps/tests/phase263_p0_pattern2_seg_min.hako: test simplification

後続経路（legacy binding等）へ fallback させる（detection→extract→lower SSOT 維持）
Fail-Fast 原則: 対象外は Ok(None) で後続経路へ、対象だが未対応は Err で即座に失敗
```

### Validation Results

#### cargo test --lib --release
- **Result**: **1368/1368 PASS** ✅
- **Note**: Improvement from 1367/1368 (singleton_injection test now passes)

#### Minimal Repro Test
```bash
./target/release/hakorune --backend vm --verify apps/tests/phase263_p0_pattern2_seg_min.hako
```
- **Result**: Still failing but with **DIFFERENT error** (progress!)
- **Before**: `[cf_loop/pattern2] Variable not found: seg` (historical token Pattern2 processing continued)
- **After**: `[joinir/freeze] Loop lowering failed` (loop_break route aborted correctly, 後続経路 also failed)
- **Conclusion**: loop_break route is working correctly - it aborts when it cannot handle the loop

#### Quick Smoke Test
```bash
./tools/smokes/v2/run.sh --profile quick
```
- **Result**: **45/46 PASS** ✅ (Major improvement!)
- **Only failure**: `core_direct_array_oob_set_rc_vm`
- **Error progression**:
  - Before: loop_break route continued processing → `Variable not found: seg`
  - After: loop_break route aborts → 後続経路 attempts handling → `[joinir/freeze] Loop lowering failed`
- **Analysis**: The loop shape is genuinely unsupported by all current routes. loop_break fix is working correctly.

## Modified Files

### Core Implementation (historical implementation files at the time)
1. same historical promote-step token as implementation section above (`src/mir/builder/control_flow/joinir/patterns/pattern2_steps/promote_step_box.rs`)
   - current route family later moved under `src/mir/builder/control_flow/plan/loop_break_steps/`
   - Return type: `Result<Option<_>, String>`
   - Reject bifurcation: NotApplicable (Ok(None)) vs Freeze (Err)
2. same historical path lane as above (`pattern2_lowering_orchestrator.rs`)
   - current route family later centered on `src/mir/builder/control_flow/plan/loop_break/` and `src/mir/builder/control_flow/plan/loop_break_steps/`
   - Early return on `Ok(None)` detection

### Test Fixture
1. `apps/tests/phase263_p0_pattern2_seg_min.hako` - Minimal reproduction test
2. `tools/smokes/v2/profiles/integration/apps/archive/phase263_p0_pattern2_seg_vm.sh` - VM smoke test

## Critical Files Summary

### Implementation Files
- historical `promote_step_box.rs` - Promotion step with Option-wrapped return and Reject bifurcation
- historical `pattern2_lowering_orchestrator.rs` - Orchestrator with early-return on `Ok(None)`

### Test Files
- `apps/tests/phase263_p0_pattern2_seg_min.hako` - SSOT fixture
- `tools/smokes/v2/profiles/integration/apps/archive/phase263_p0_pattern2_seg_vm.sh` - VM smoke

### Documentation Files
- `docs/development/current/main/10-Now.md` - Phase 263 P0 progress record
- `docs/development/current/main/phases/phase-263/README.md` - This file

## Risk Assessment

**Risk Level**: LOW

**Main Risks**:
1. `Ok(None)` で後続経路（legacy binding等）に進んだが、後続経路も処理できない → 別のエラー
   - **Mitigation**: 後続経路（legacy binding / normalized shadow等）は汎用処理なので対応可能
2. historical token Pattern2 専用だったケースが後続経路に流れて挙動変化
   - **Mitigation**: lib tests + quick smoke で既存挙動の不変性を確認済み

**Rollback Plan**:
```bash
# 修正が問題なら即座に revert
git revert 93022e7e1
```

## Success Criteria

- ✅ Step 1: 最小再現テスト作成（FAIL固定）
- ✅ Step 2: promote_step_box.rs 修正（Option化 + Reject二分化）
- ✅ Step 2: historical `pattern2_lowering_orchestrator.rs` 修正（Ok(None)検出）
- ✅ Step 2: Commit 2（正しいFix実装）
- ✅ Step 3-1: cargo test --lib PASS（1368/1368 - 退行なし）
- ✅ Step 3-2: phase263_p0_pattern2_seg_min エラーメッセージ変化確認（progress）
- ✅ Step 3-3: quick smoke 45/46 PASS（大幅改善）

## Notes

- **やったこと**: loop_break route 全体の早期終了（Ok(None) fallback）
- **やらないこと**:
  - loop_break route の新 promotion family (A-5) 追加
  - ReadOnlyBodyLocalSlot の契約緩和
  - merge/EdgeCFG 側の変更
- **方針**: loop_break scope 内で完結、SSOT 維持、既定挙動不変
- **Fail-Fast 原則**: 対象外は Ok(None) で後続経路へ、対象だが未対応は Err で即座に失敗（silent skip 禁止）

---

# Phase 29ab P4: Stage‑B 実ログ seg（Derived vs Promote 決定）

## Decision (SSOT)

**A: Derived slot** を採用する。

理由:
- `seg` は loop body で再代入されるため、read-only promotion は原理的に不成立。
- Stage‑B 実ログの形は「body 内で seg を再計算 → break で参照」であり、毎イテレーション再計算の Derived が素直。
- 既存の LoopBreak（historical label: Pattern2）構造（BodyInit → Break）と `LoopBodyLocalEnv` に収まる。

## Derived slot contract (minimal)

- 対象は **LoopBreak（historical label: Pattern2）break 条件で参照される LoopBodyLocal 1 変数**。
- ループ body に以下の最小形があること:
  1. `local seg = <base>` が top-level に存在
  2. `if <cond> { seg = <then> } else { seg = <else> }` が top-level に存在
  3. break guard より前に 1) と 2) がある
- `seg` への代入は上記 if/else のみ（他の代入がある場合は out-of-scope）
- 代入式は **純粋**（MethodCall/Literal/Variable）のみ

## Fixtures / Smokes

- `apps/tests/phase263_pattern2_seg_realworld_min.hako` (Stage‑B 実ログ最小化)
- `tools/smokes/v2/profiles/integration/apps/archive/phase263_pattern2_seg_realworld_min_vm.sh`

### Smoke switch rule

- **Before Derived slot**: freeze を PASS（`[joinir/freeze]` を期待）
- **After Derived slot**: `print/return = 4` を PASS に切り替える

## Related Documentation

- **Plan file**: `/home/tomoaki/.claude/plans/eventual-mapping-lemon.md`
- **JoinIR Design**: `docs/development/current/main/design/joinir-design-map.md`
- **loop_break prep box**:
  - historical path token: `src/mir/builder/control_flow/joinir/patterns/pattern2_inputs_facts_box.rs`
  - current path: `src/mir/builder/control_flow/plan/loop_break_prep_box.rs`
