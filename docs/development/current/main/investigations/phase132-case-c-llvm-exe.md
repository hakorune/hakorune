# Phase 132-P0: Case C (Infinite Loop with Early Exit) - LLVM EXE Investigation

## Date
2025-12-15

## Status
✅ **FIXED** - VM / LLVM EXE parity achieved

## Summary
Testing `apps/tests/llvm_stage3_loop_only.hako` (Pattern 5: InfiniteEarlyExit) in LLVM EXE mode reveals a critical exit PHI usage bug.

## Test File
`apps/tests/llvm_stage3_loop_only.hako`
```nyash
static box Main {
  main() {
    local counter = 0
    loop (true) {
      counter = counter + 1
      if counter == 3 { break }
      continue
    }
    print("Result: " + counter)
    return 0
  }
}
```

## Expected Behavior
- **VM execution**: `Result: 3` ✅
- **LLVM EXE**: `Result: 3` (should match VM)

## Actual Behavior
- **VM execution**: `Result: 3` ✅
- **LLVM EXE**: `Result: 0` ❌（修正前）

## Final Root Cause (Confirmed)

**JoinIR merge 側の Exit PHI ValueId 衝突**。

`src/mir/builder/control_flow/joinir/merge/exit_phi_builder.rs` が PHI dst の割り当てに
`builder.value_gen.next()`（モジュールレベルのカウンタ）を使っていたため、
**同一関数内で `ValueId` が衝突**し、結果として exit 側で参照すべき PHI が別の値に潰れていた。

例（概念）:

- `bb0: %1 = const 0`（counter 初期値）
- `bb3: %1 = phi ...`（exit PHI）

**同じ `%1`** になり得るため、exit 側で `counter` を読んでも 0 になってしまう。

### Fix

PHI dst の割り当てを **関数ローカルの allocator** に統一する。

- Before: `builder.value_gen.next()`（module-level）
- After: `func.next_value_id()`（function-level）

コミット:
- `bd07b7f4 fix(joinir): Phase 132-P2 - Exit PHI ValueId collision fix`

### Verification

| テスト | VM | LLVM |
|---|---:|---:|
| Pattern 1 (`/tmp/p1_return_i.hako`) | 3 ✅ | 3 ✅ |
| Case C (`apps/tests/llvm_stage3_loop_only.hako`) | Result: 3 ✅ | Result: 3 ✅ |

## Root Cause Analysis

### MIR Structure (Correct)
```mir
bb3:  ; Exit block
    1: %1: Integer = phi [%8, bb6]  ; ✅ PHI correctly receives counter
    1: %16: String = const "Result: "
    1: %17: String = copy %16
    1: %18: Integer = copy %1       ; ✅ MIR uses %1 (PHI result)
    1: %19: Box("StringBox") = %17 Add %18
    1: %20: Box("StringBox") = copy %19
    1: call_global print(%20)
    1: %21: Integer = const 0
    1: ret %21
```

**MIR is correct**: The exit block (bb3) has a PHI node that receives the counter value from bb6, and subsequent instructions correctly use `%1`.

補足:
- 上記の “MIR が正しい” は「構造として PHI がある」意味で、**ValueId 衝突があると意味論が壊れる**。
- 本件は “JoinIR merge の ValueId allocator の選択” が原因だったため、LLVM 側で 0 に見える形で顕在化した。

### LLVM IR (BUG)
```llvm
bb3:
  %"phi_1" = phi  i64 [%"add_8", %"bb6"]           ; ✅ PHI created correctly
  %".2" = getelementptr inbounds [9 x i8], [9 x i8]* @".str.main.16", i32 0, i32 0
  %"const_str_h_16" = call i64 @"nyash.box.from_i8_string"(i8* %".2")
  %"bin_h2p_r_19" = call i8* @"nyash.string.to_i8p_h"(i64 0)  ; ❌ Uses 0 instead of %"phi_1"
  %"concat_is_19" = call i8* @"nyash.string.concat_is"(i64 0, i8* %"bin_h2p_r_19")  ; ❌ Uses 0
  %"concat_box_19" = call i64 @"nyash.box.from_i8_string"(i8* %"concat_is_19")
  call void @"ny_check_safepoint"()
  %"unified_global_print" = call i64 @"print"(i64 0)  ; ❌ Uses 0
  ret i64 0
```

**Bug identified**: The PHI node `%"phi_1"` is created correctly and receives the counter value from `%"add_8"`. However, **all subsequent uses of ValueId(1) are hardcoded to `i64 0` instead of using `%"phi_1"`**.

### Discarded Hypotheses (Superseded)

当初は「Python LLVM builder が ValueId(1) を解決できず 0 にフォールバックしている」と推測し、
PHI registration / vmap filtering 周りを疑った。

ただし最終的に、本件の主因は **Rust 側（JoinIR merge）の ValueId 衝突**であることが確定したため、
ここでの Python 側の推測は “症状の説明” に留まる（根因ではない）。

## Investigation Steps

### Step 1: Verify PHI Creation
✅ **Confirmed**: PHI is created in LLVM IR at bb3

### Step 2: Check MIR Exit PHI Generation
✅ **Confirmed**: MIR has correct exit PHI with debug logs:
```
[DEBUG-177] Phase 246-EX: Block BasicBlockId(1) has jump_args metadata: [ValueId(1004)]
[DEBUG-177] Phase 246-EX: Remapped jump_args: [ValueId(8)]
[DEBUG-177] Phase 246-EX: exit_phi_inputs from jump_args[0]: (BasicBlockId(6), ValueId(8))
[DEBUG-177] Phase 246-EX-P5: Added loop_var 'counter' to carrier_inputs: (BasicBlockId(6), ValueId(8))
[DEBUG-177] Exit block PHI (carrier 'counter'): ValueId(1) = phi [(BasicBlockId(6), ValueId(8))]
```

### Step 3: Trace vmap Resolution
🔄 **In progress**: Running with `NYASH_LLVM_VMAP_TRACE=1` to see if PHI is in vmap_cur

## Code Locations

### Python LLVM Builder
- PHI placeholder creation: `/home/tomoaki/git/hakorune-selfhost/src/llvm_py/llvm_builder.py:276-368`
  - Line 343: `self.vmap[dst0] = ph0` - PHI stored in global vmap
- Block lowering: `/home/tomoaki/git/hakorune-selfhost/src/llvm_py/builders/block_lower.py`
  - Line 335: `vmap_cur = dict(builder.vmap)` - Copy global vmap to block-local
- Instruction lowering: `/home/tomoaki/git/hakorune-selfhost/src/llvm_py/builders/instruction_lower.py`
  - Line 8: `vmap_ctx = getattr(owner, '_current_vmap', owner.vmap)` - Use block-local vmap
- Value resolution: `/home/tomoaki/git/hakorune-selfhost/src/llvm_py/utils/values.py:11-56`
  - `resolve_i64_strict` - Checks vmap, global_vmap, then resolver
  - Falls back to `ir.Constant(i64, 0)` if all fail (line 53)

### Rust MIR Generation
- Exit PHI generation: `src/mir/join_ir/lowering/simple_while_minimal.rs`
  - Pattern 5 (InfiniteEarlyExit) lowering

## Interim Hypothesis (Historical)

### Bug Location
`/home/tomoaki/git/hakorune-selfhost/src/llvm_py/llvm_builder.py:342-343`

```python
ph0 = b0.phi(self.i64, name=f"phi_{dst0}")
self.vmap[dst0] = ph0
# ❌ MISSING: self.phi_manager.register_phi(bid0, dst0, ph0)
```

### Failure Chain

1. **PHI Creation** (line 342-343): PHI is created and stored in `self.vmap[1]` ✅
2. **PHI Registration** (MISSING): PHI is **never registered** via `phi_manager.register_phi()` ❌
3. **Block Lowering** (block_lower.py:325): `filter_vmap_preserve_phis` is called
4. **PHI Filtering** (phi_manager.py:filter_vmap_preserve_phis): Checks `is_phi_owned(3, 1)`
5. **Ownership Check** (phi_manager.py:is_phi_owned): Looks for `(3, 1)` in `predeclared` dict
6. **Not Found** ❌: PHI was never registered, so `(3, 1)` is not in `predeclared`
7. **PHI Filtered Out**: PHI is removed from `vmap_cur`
8. **Value Resolution Fails**: Instructions can't find ValueId(1), fall back to `ir.Constant(i64, 0)`

### Fix Strategy

**Option A: Add PHI Registration** (Recommended)

Add `self.phi_manager.register_phi(bid0, dst0, ph0)` after line 343 in `llvm_builder.py:setup_phi_placeholders`:

```python
if not is_phi:
    ph0 = b0.phi(self.i64, name=f"phi_{dst0}")
    self.vmap[dst0] = ph0
    # ✅ FIX: Register PHI for filter_vmap_preserve_phis
    self.phi_manager.register_phi(int(bid0), int(dst0), ph0)
```

This ensures PHIs are included in `vmap_cur` when lowering their defining block.

### Verification Plan

1. Add `register_phi` call in `setup_phi_placeholders`
2. Rebuild and test: `NYASH_LLVM_STRICT=1 tools/build_llvm.sh apps/tests/llvm_stage3_loop_only.hako -o /tmp/case_c`
3. Execute: `/tmp/case_c` should output `Result: 3`
4. Check LLVM IR: Should use `%"phi_1"` instead of `0`

## Acceptance Criteria

- ✅ LLVM EXE output: `Result: 3`
- ✅ LLVM IR uses `%"phi_1"` instead of `0`
- ✅ STRICT mode passes without fallback warnings

## Related Documents
- [Phase 131-3 LLVM Lowering Inventory](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phase131-3-llvm-lowering-inventory.md)
- [Phase 132 Summary](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-132/README.md)
