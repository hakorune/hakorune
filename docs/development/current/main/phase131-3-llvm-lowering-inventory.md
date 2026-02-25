# Phase 131-3: MIR→LLVM Lowering Inventory

**Date**: 2025-12-14
**Purpose**: Identify what is broken in the LLVM (Python llvmlite) lowering pipeline using a few representative cases, and record evidence + next actions.

## Test Cases & Results

| Case | File | Emit | Link | Run | Notes |
|------|------|------|------|-----|-------|
| A | `apps/tests/phase87_llvm_exe_min.hako` | ✅ | ✅ | ✅ | **PASS** - Simple return 42, no BoxCall, exit code verified |
| B | `apps/tests/loop_min_while.hako` | ✅ | ✅ | ✅ | **PASS** - Loop/PHI path runs end-to-end (Phase 131-10): prints `0,1,2` and exits |
| B2 | `/tmp/case_b_simple.hako` | ✅ | ✅ | ✅ | **PASS** - Simple print(42) without loop works |
| C | `apps/tests/llvm_stage3_loop_only.hako` | ✅ | ✅ | ✅ | **PASS** - `loop(true)` + break/continue + print/concat works end-to-end (Phase 132-P2) |

## Phase 132 Update (2025-12-15)

✅ **MAJOR FIX**: Exit PHI wrong result bug fixed!
- **Issue**: Pattern 1 の `return i` が LLVM だけ 0 を返す（VM は 3）
- **Test**: `/tmp/p1_return_i.hako` が VM/LLVM ともに 3 を返すようになった
- **Root Cause (two-layer)**:
  - JoinIR/Boundary: exit 値が境界を通っていなかった（exit_bindings / Jump args）
  - LLVM Python: builder.vmap の PHI placeholder が上書きされ、`ret` が 0 を参照
- **Fix**:
  - JoinIR/Boundary: Pattern 1 で exit 値を明示的に渡す + LoopExitBinding を境界へ設定
  - LLVM Python: `predeclared_ret_phis` で PHI 所有を追跡し、PHI placeholder を上書きしない
- **Details**:
  - Investigation: [phase132-llvm-exit-phi-wrong-result.md](investigations/phase132-llvm-exit-phi-wrong-result.md)
  - Phase summary: [phases/phase-132/README.md](phases/phase-132/README.md)

## Phase 132-P1 Update (2025-12-15)

✅ **STRUCTURAL FIX**: Function-local state isolation in LLVM Python backend

- **Issue**: block id / value id が “関数をまたいで” 衝突し得る（snapshot / cache / PHI state の漏洩）
- **Fix**: `FunctionLowerContext` Box で function-scoped state を隔離し、tuple-key workaround を不要化
- **Code**:
  - `src/llvm_py/context/function_lower_context.py`
  - `src/llvm_py/builders/function_lower.py`

## Root Causes Identified

### 1. TAG-EMIT: Loop PHI → Invalid LLVM IR (Case B) ✅ FIXED (Phase 131-10)

**File**: `apps/tests/loop_min_while.hako`

**Code**:
```nyash
static box Main {
  main() {
    local i = 0
    loop(i < 3) {
      print(i)
      i = i + 1
    }
    return 0
  }
}
```

**MIR Compilation**: SUCCESS (Pattern 1 JoinIR lowering works)
```
[joinir/pattern1] Generated JoinIR for Simple While Pattern
[joinir/pattern1] Functions: main, loop_step, k_exit
📊 MIR Module compiled successfully!
📊 Functions: 4
```

**LLVM Harness Failure**:
```
RuntimeError: LLVM IR parsing error
<string>:35:1: error: expected instruction opcode
bb4:
^
```

**Observed invalid IR snippet**:
```llvm
bb3:
  ret i64 %"ret_phi_17"          ← Terminator FIRST (INVALID!)
  %"ret_phi_17" = phi  i64 [0, %"bb6"]  ← PHI AFTER terminator
```

**What we know**:
- LLVM IR requires: **PHI nodes first**, then non-PHI instructions, then terminator last.
- The harness lowers blocks (including terminators), then wires PHIs, then runs a safety pass:
  - `src/llvm_py/builders/function_lower.py` calls `_lower_blocks(...)` → `_finalize_phis(builder)` → `_enforce_terminators(...)`.
- Per-block lowering explicitly lowers terminators after body ops:
  - `src/llvm_py/builders/block_lower.py` splits `body_ops` and `term_ops`, then lowers `term_ops` after `body_ops`.
- PHIs are created/wired during finalize via `ensure_phi(...)`:
  - `src/llvm_py/phi_wiring/wiring.py` (positions PHI “at block head”, and logs when a terminator already exists).

This strongly suggests an **emission ordering / insertion-position** problem in the harness, not a MIR generation bug. The exact failure mode still needs to be confirmed by tracing where the PHI is inserted relative to the terminator in the failing block.

**Where to inspect next (code pointers)**:
- Harness pipeline ordering: `src/llvm_py/builders/function_lower.py`
- Terminator emission: `src/llvm_py/builders/block_lower.py`
- PHI insertion rules + debug: `src/llvm_py/phi_wiring/wiring.py` (`NYASH_PHI_ORDERING_DEBUG=1`)
- “Empty block” safety pass (separate concern): `src/llvm_py/builders/function_lower.py:_enforce_terminators`

**✅ FIXED (Phase 131-4)**: Multi-pass block lowering architecture

**Solution implemented**:
- **Pass A**: Lower non-terminator instructions (body ops only)
- **Pass B**: Finalize PHIs (wire incoming edges) - happens in `function_lower.py`
- **Pass C**: Lower deferred terminators (after PHIs are placed)

**Key changes**:
1. `src/llvm_py/builders/block_lower.py`:
   - Split `lower_blocks()` to defer terminators
   - Added `lower_terminators()` function for Pass C
   - Deferred terminators stored in `builder._deferred_terminators`

2. `src/llvm_py/builders/function_lower.py`:
   - Updated pass ordering: Pass A → Pass B → Pass C
   - Added call to `_lower_terminators()` after `_finalize_phis()`

3. `src/llvm_py/instructions/ret.py`:
   - Added `_disable_phi_synthesis` flag check
   - Prevents PHI creation during Pass C (terminators should only use existing values)

**Result**:
- Case B EMIT now succeeds ✅
- Generated LLVM IR is valid (PHIs before terminators)
- No regression in Cases A and B2

---

### 2. TAG-LINK: Symbol Name Mismatch (Case B) - ✅ FIXED (Phase 131-5)

**File**: `apps/tests/loop_min_while.hako`

**Link Error**:
```
/usr/bin/ld: /home/tomoaki/git/hakorune-selfhost/target/aot_objects/loop_min_while.o: in function `condition_fn':
<string>:(.text+0x99): undefined reference to `nyash_console_log'
```

**Root Cause**: Python harness was converting dots to underscores in symbol names.
- Generated symbol: `nyash_console_log` (underscores)
- NyKernel exports: `nyash.console.log` (dots)
- ELF symbol tables support dots in symbol names - no conversion needed!

**Fix Applied** (Phase 131-5):
- File: `src/llvm_py/instructions/externcall.py`
- Removed dot-to-underscore conversion (lines 54-58)
- Now uses symbol names directly as exported by NyKernel
- Result: Case B LINK ✅ (no more undefined reference errors)

**Verification**:
```bash
# NyKernel symbols (dots)
$ objdump -t target/release/libnyash_kernel.a | grep console
nyash.console.log
nyash.console.log_handle
print (alias to nyash.console.log)

# LLVM IR now emits (dots - matching!)
declare i64 @nyash.console.log(i8*)
```

**Status**: TAG-LINK completely resolved. Case B now passes EMIT ✅ LINK ✅

---

### 3. TAG-RUN: Loop Runtime Incorrect (Case B) - ✅ FIXED (Phase 131-10)

**File**: `apps/tests/loop_min_while.hako`

**Expected Behavior**:
```bash
$ ./target/release/hakorune apps/tests/loop_min_while.hako
0
1
2
RC: 0
```

**Outcome** (Phase 131-10):
- LLVM AOT now matches VM behavior (prints `0,1,2`, terminates)
- No regression in Case A / B2

**What happened (timeline)**:
- **Phase 131-6**: Confirmed MIR/VM correctness; investigated as “PHI incoming wiring bug”.
  - Historical diagnosis: `docs/development/current/main/phase131-6-ssa-dominance-diagnosis.md`
  - Historical plan: `docs/development/current/main/phase131-6-next-steps.md`
- **Phase 131-7**: Multi-pass lowering follow-up: ensure values defined in Pass A are visible to Pass C (global `vmap` sync).
- **Phase 131-8**: ExternCall argument resolution fixed: use PHI-safe resolution (`resolve_i64_strict`) so PHI values are not treated as null pointers.
- **Phase 131-9**: MIR global PHI type inference: fix loop-carrier PHI mistakenly inferred as `String` (prevent `"0" + "1"` concatenation behavior).
  - Implemented in `src/mir/builder/lifecycle.rs` (Phase 131-9 section).
- **Phase 131-10**: Console ABI routing: avoid calling `nyash.console.log(i8*)` with non-string values.
  - Route string literals to `nyash.console.log(i8*)`
  - Route handles/integers to `nyash.console.log_handle(i64)`
  - Unboxed integer fallback handled in `crates/nyash_kernel/src/plugin/console.rs`

**Key lesson**:
- “PHI is correct but loop prints 0 forever” can be a combination of:
  - multi-pass value propagation bugs,
  - ExternCall resolution/ABI mismatch,
  - and MIR type inference drift.

---

### 4. Case C: loop(true) + break/continue - from TAG-EMIT to TAG-RUN

**File**: `apps/tests/llvm_stage3_loop_only.hako`

**Code**:
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

**MIR Compilation**: SUCCESS（Phase 131-11）

**What changed**:
- Pattern gap was resolved by introducing a dedicated infinite-loop early-exit pattern (Phase 131-11 A–C).
- A loop-carrier PHI type-cycle bug was fixed by seeding the PHI type from the entry(init) value (Phase 131-11 H).
  - Root cause report: `docs/development/current/main/phase-131-11-g-phi-type-bug-report.md`

**Current issue**: **TAG-RUN (segfault in print/concat)**  
Loop/carrier propagation is now consistent enough to run iterations, but the post-loop `print("Result: " + counter)` path segfaults.

**Next actions**:
- Use strict mode + traces to catch the first wrong value/ABI boundary:
  - `NYASH_LLVM_STRICT=1`, `NYASH_LLVM_TRACE_VALUES=1`, `NYASH_LLVM_TRACE_PHI=1`
- Reduce to isolate the failing segment:
  - `return counter` (no print)
  - `print(counter)` (no concat)
  - `print("Result: " + counter)` (concat + print)
- Dump IR around concat/externcall:
  - `NYASH_LLVM_DUMP_IR=/tmp/case_c.ll`

**Update (Phase 131-13)**:
- snapshot-only + strict resolver により、Case C の不一致が “LLVM の値解決バグ” ではなく  
  “Rust の MIR→JSON emit が block 内命令順序を崩している” 問題として顕在化した。
  - Investigation note: `docs/development/current/main/investigations/phase131-13-mir-json-instruction-order.md`
- Add `is_infinite_loop: bool` feature to `LoopFeatures` (detect `loop(true)`).
- Fix classification so `has_break && has_continue` does not route to Pattern 4.
- Introduce a dedicated pattern kind + lowerer for **infinite loop + early-exit (+ optional continue)**:
  - Example name: `InfiniteEarlyExit` (avoid “Pattern5” naming collision with existing Trim/P5).
  - Scope (minimum): `loop(true)` with exactly one `break` site and one `continue` site (Fail-Fast outside this).

**Files to Modify**:
1. `src/mir/loop_pattern_detection/mod.rs` - Add `is_infinite_loop` field, update classify()
2. `src/mir/builder/control_flow/joinir/patterns/ast_feature_extractor.rs` - Detect `loop(true)` in condition
3. `src/mir/builder/control_flow/joinir/patterns/router.rs` - Pass condition to extract_features()
4. `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs` - Add infinite loop lowering path

**Docs (Phase 131-11)**:
- Detailed analysis: `docs/development/current/main/case-c-infinite-loop-analysis.md`
- Implementation summary: `docs/development/current/main/phase131-11-case-c-summary.md`

**Location**: `src/mir/builder/control_flow/joinir/router.rs` - pattern matching logic

---

## Success Cases

### Case A: Minimal (No BoxCall, No Loop)
- **EMIT**: ✅ Object generated successfully
- **LINK**: ✅ Linked with NyKernel runtime
- **RUN**: ✅ Exit code 42 verified
- **Validation**: Full LLVM exe line SSOT confirmed working

### Case B2: Simple BoxCall (No Loop)
- **EMIT**: ✅ Object generated successfully
- **LINK**: ✅ Linked with NyKernel runtime
- **RUN**: ✅ `print(42)` executes (loop-free path)
- **Validation**: BoxCall → ExternCall lowering works correctly

## Next Steps

### ✅ Priority 1: COMPLETED - Fix TAG-EMIT (PHI After Terminator Bug)
**Target**: Case B (`loop_min_while.hako`)

**Status**: ✅ FIXED in Phase 131-4 (see Root Cause #1 above)

**Result**: Case B EMIT now succeeds. Multi-pass block lowering architecture working.

---

### ✅ Priority 2: COMPLETED - Fix TAG-LINK (Symbol Name Mismatch)
**Target**: Case B (`loop_min_while.hako`)

**Status**: ✅ FIXED in Phase 131-5 (see Root Cause #2 above)

**Approach Taken**:
1. Investigated NyKernel exported symbols → found dots in names (`nyash.console.log`)
2. Identified Python harness converting dots to underscores (WRONG!)
3. Removed conversion - ELF supports dots natively
4. Verified with objdump and test execution

**Files Modified**:
- `src/llvm_py/instructions/externcall.py` - Removed dot-to-underscore conversion

**Result**: Case B now passes EMIT ✅ LINK ✅ (but RUN fails - see Priority 3)

---

### ✅ Priority 3: COMPLETED - Fix TAG-RUN (Loop Runtime Correctness)
**Target**: Case B (`loop_min_while.hako`)

**Status**: ✅ FIXED (Phase 131-10)

**Fixes applied (high level)**:
1. Multi-pass lowering value propagation: Pass A outputs are synced for Pass C usage
2. ExternCall value resolution: unify on PHI-safe resolver (`resolve_i64_strict`)
3. MIR global PHI type inference: correct loop-carrier PHI types from incomings
4. Console ABI routing: `console.log(i8*)` vs `console.log_handle(i64)` dispatch

**Result**:
- Case B now passes EMIT ✅ LINK ✅ RUN ✅
- Output matches VM behavior (`0,1,2`), and the program terminates

---

### Priority 4: Fix TAG-EMIT (JoinIR Pattern Coverage)
**Target**: Case C (`llvm_stage3_loop_only.hako`)

**Approach**:
1. Analyze `loop(true) { ... break ... continue }` control flow
2. Design JoinIR Pattern variant (Pattern 1.5 or Pattern 5?)
3. Implement pattern in `src/mir/builder/control_flow/joinir/patterns/`
4. Update router to match this pattern

**Files**:
- `src/mir/builder/control_flow/joinir/router.rs` - add pattern matching
- `src/mir/builder/control_flow/joinir/patterns/` - new pattern module

**Expected**: Infinite loops with break/continue should lower to JoinIR

---

### Priority 5: Comprehensive Loop Coverage Test
**After** P3+P4 fixed:

**Test Matrix**:
```bash
# Pattern 1: Simple while
apps/tests/loop_min_while.hako

# Pattern 2: Infinite loop + break
apps/tests/llvm_stage3_loop_only.hako

# Pattern 3: Loop with if-phi
apps/tests/loop_if_phi.hako

# Pattern 4: Nested loops
apps/tests/nested_loop_inner_break_isolated.hako
```

All should pass: EMIT ✅ LINK ✅ RUN ✅

---

## Box Theory Modularization Feedback

### LLVM Line SSOT Analysis

#### ✅ Good: Single Entry Point
- `tools/build_llvm.sh` is the SSOT for LLVM exe line
- Clear 4-phase pipeline: Build → Emit → Link → Run
- Env vars control compiler mode (`NYASH_LLVM_COMPILER=harness|crate`)

#### ❌ Bad: Harness Duplication Risk
- Python harness: `src/llvm_py/llvm_builder.py` (~2000 lines)
- Rust crate: `crates/nyash-llvm-compiler/` (separate implementation)
- Both translate MIR14→LLVM, risk of divergence

#### 🔧 Recommendation: Harness as Box
```
Box: LLVMCompilerBox
  - Method: compile_to_object(mir_json: str, output: str)
  - Default impl: Python harness (llvmlite)
  - Alternative impl: Rust crate (inkwell - deprecated)
  - Interface: MIR JSON v1 schema (fixed contract)
```

**Benefits**:
- Single interface definition
- Easy A/B testing (Python vs Rust)
- Plugin architecture: external LLVM backends

---

### Duplication Found: BB Emission Logic

**Location 1**: `src/llvm_py/llvm_builder.py:400-600`
**Location 2**: (likely) `crates/nyash-llvm-compiler/src/codegen/` (if crate path is used)

**Problem**: Empty BB handling differs between harness and crate path

**Solution**: Box-first extraction
```rust
// Extract to: src/mir/llvm_ir_validator.rs
pub fn validate_basic_blocks(blocks: &[BasicBlock]) -> Result<(), String> {
    for bb in blocks {
        if bb.instructions.is_empty() && bb.terminator.is_none() {
            return Err(format!("Empty BB detected: {:?}", bb.id));
        }
    }
    Ok(())
}
```

Call this validator **before** harness invocation (in Rust MIR emission path).

---

### Legacy Deletion Candidates

#### 1. LoopBuilder Remnants (Phase 33 cleanup incomplete?)
**Search**: `grep -r "LoopBuilder" src/mir/builder/control_flow/`
**Action**: Verify no dead imports/comments remain

#### 2. Unreachable BB Emission Code
**Location**: `src/llvm_py/llvm_builder.py`
**Check**: Does harness skip `"reachable": false` blocks from MIR JSON?
**Action**: If not, add filter before BB emission

**Code snippet to check**:
```python
# src/llvm_py/llvm_builder.py (approx line 450)
for block in function["blocks"]:
    if block.get("reachable") == False:  # ← Add this check?
        continue
    self.emit_basic_block(block)
```

---

## Validation: build_llvm.sh SSOT Conformance

### ✅ Confirmed SSOT Behaviors
1. **Feature selection**: `NYASH_LLVM_FEATURE=llvm` (default harness) vs `llvm-inkwell-legacy`
2. **Compiler mode**: `NYASH_LLVM_COMPILER=harness` (default) vs `crate` (ny-llvmc)
3. **Object caching**: `NYASH_LLVM_SKIP_EMIT=1` for pre-generated .o files
4. **Runtime selection**: `NYASH_LLVM_NYRT=crates/nyash_kernel/target/release`

### ❌ Missing SSOT: Error Logs
- Python harness errors go to stderr (lost after build_llvm.sh exits)
- No env var for `NYASH_LLVM_HARNESS_LOG=/tmp/llvm_harness.log`

**Recommendation**:
```bash
# In build_llvm.sh, line ~118:
HARNESS_LOG="${NYASH_LLVM_HARNESS_LOG:-/tmp/nyash_llvm_harness_$$.log}"
NYASH_LLVM_OBJ_OUT="$OBJ" NYASH_LLVM_USE_HARNESS=1 \
  "$BIN" --backend llvm "$INPUT" 2>&1 | tee "$HARNESS_LOG"
```

---

## Timeline Estimate (historical)

- **P1 (Loop PHI → LLVM IR fix)**: 1-2 hours (harness BB emission logic)
- **P2 (JoinIR pattern coverage)**: 3-4 hours (pattern design + implementation)
- **P3 (Comprehensive test)**: 1 hour (run matrix + verify)

**Total**: 5-7 hours to full LLVM loop support

---

## Executive Summary (updated)

### Phase 131-10 Results (Case B End-to-End PASS)

**✅ Case A (Minimal)**: PASS - Simple return works perfectly
- EMIT ✅ LINK ✅ RUN ✅
- Validates: Build pipeline, NyKernel runtime, basic MIR→LLVM lowering

**✅ Case B (Loop+PHI)**: PASS - Loop/PHI works in LLVM AOT
- **Phase 131-4**: Fixed TAG-EMIT (PHI after terminator) ✅
- **Phase 131-5**: Fixed TAG-LINK (symbol name mismatch) ✅
- **Phase 131-10**: Fixed TAG-RUN (value propagation + console ABI routing) ✅

**✅ Case B2 (BoxCall)**: PASS - print() without loops works
- EMIT ✅ LINK ✅ RUN ✅
- Validates: BoxCall→ExternCall lowering, runtime ABI

**❌ Case C (Break/Continue)**: TAG-EMIT failure - **JoinIR pattern gap**
- **Root Cause**: `loop(true) { break }` pattern not recognized by JoinIR router
- **Status**: Unchanged from Phase 131-3

---

### Phase 131-5 Achievements (still valid)

**✅ Fixed TAG-LINK (Symbol Name Mismatch)**:
1. **Investigation**: Used `objdump` to discover NyKernel exports symbols with dots
2. **Root Cause**: Python harness was converting `nyash.console.log` → `nyash_console_log`
3. **Fix**: Removed dot-to-underscore conversion in `externcall.py`
4. **Verification**: Case B now links successfully against NyKernel
5. **No Regression**: Cases A and B2 still pass

**Files Modified**:
- `src/llvm_py/instructions/externcall.py` (4 lines removed)

**Impact**: All ExternCall symbols now match NyKernel exports exactly.

---

### Critical Path Update

1. ✅ **Fix PHI ordering** (P1 - Phase 131-4) - DONE
2. ✅ **Fix symbol mapping** (P2 - Phase 131-5) - DONE
3. ✅ **Fix loop runtime correctness** (P3 - Phase 131-10) - DONE
4. ⏳ **Add JoinIR infinite-loop early-exit pattern** (P4) - PENDING
5. ⏳ **Comprehensive test** (P5) - PENDING

**Total Effort So Far**: ~3 hours (Investigation + 2 fixes)
**Remaining**: ~4-6 hours (Runtime bug + Pattern 5 + Testing)

---

### Box Theory Modularization Insights

#### ✅ Good: LLVM Line SSOT
- `tools/build_llvm.sh` is well-structured (4-phase pipeline)
- Clear separation: Emit → Link → Run
- Environment variables control behavior cleanly

#### ⚠️ Risk: Harness Duplication
- Python harness (`src/llvm_py/`) vs Rust crate (`crates/nyash-llvm-compiler/`)
- Both implement MIR14→LLVM, risk of divergence
- **Recommendation**: Box-ify with interface contract (MIR JSON v1 schema)

#### 🔧 Technical Debt Found
1. **Harness duplication** - Python harness vs Rust crate divergence risk
2. **Unreachable block handling** - MIR JSON marks all blocks `reachable: false` (may be stale metadata)
3. **Error logging** - Python harness errors lost after build_llvm.sh exits

---

## Appendix: Test Commands

### Case A (Minimal - PASS)
```bash
tools/build_llvm.sh apps/tests/phase87_llvm_exe_min.hako -o tmp/case_a
tmp/case_a
echo $?  # Expected: 42
```

### Case B (Loop PHI - PASS)
```bash
tools/build_llvm.sh apps/tests/loop_min_while.hako -o tmp/case_b
tmp/case_b
# Output: 0,1,2 (+ Result line), then exit
```

### Case B2 (Simple BoxCall - PASS)
```bash
cat > /tmp/case_b_simple.hako << 'EOF'
static box Main {
    main() {
        print(42)
        return 0
    }
}
EOF
tools/build_llvm.sh /tmp/case_b_simple.hako -o tmp/case_b2
tmp/case_b2
# Output: prints `42` (+ Result line), then exit
```

### Case C (Complex Loop - FAIL at MIR)
```bash
tools/build_llvm.sh apps/tests/llvm_stage3_loop_only.hako -o tmp/case_c
# Error: JoinIR pattern not supported
```

---

## MIR JSON Inspection (Case B Debug)
```bash
# Generate MIR JSON
./target/release/hakorune --emit-mir-json /tmp/case_b.json --backend mir apps/tests/loop_min_while.hako

# Check for unreachable blocks
jq '.cfg.functions[] | select(.name=="main") | .blocks[] | select(.reachable==false)' /tmp/case_b.json

# Inspect bb4 (the problematic block)
jq '.cfg.functions[] | select(.name=="main") | .blocks[] | select(.id==4)' /tmp/case_b.json
```

---

## Success Criteria

**Phase 131-10 Complete** when:
1. ✅ Case A continues to pass (regression prevention) - **VERIFIED**
2. ✅ Case B (loop_min_while.hako) passes end-to-end - **VERIFIED** (EMIT ✅ LINK ✅ RUN ✅)
3. ✅ Case B2 continues to pass (BoxCall regression prevention) - **VERIFIED**
4. ❌ Case C (llvm_stage3_loop_only.hako) lowers to JoinIR and runs - **NOT YET**
5. ⚠️ All 4 cases produce correct output - **PARTIAL** (3/4 passing)
6. ⚠️ No plugin errors (or plugin errors are benign/documented) - **ACCEPTABLE** (plugin errors don't affect AOT execution)

**Definition of Done**:
- All test cases: EMIT ✅ LINK ✅ RUN ✅
- Exit codes match expected values
- Output matches expected output (where applicable)
