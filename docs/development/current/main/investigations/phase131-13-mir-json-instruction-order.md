# Phase 131-13: MIR JSON Instruction Order Fix - COMPLETED ✅

Status: **RESOLVED** (2025-12-14)
Scope: Rust 側の `MIR → JSON (harness)` 出力で、ブロック内の命令順序が崩れて Python LLVM backend が Fail-Fast する問題。
Related:
- SSOT (LLVM棚卸し): `docs/development/current/main/phase131-3-llvm-lowering-inventory.md`
- Case C: `docs/development/current/main/phase131-11-case-c-summary.md`

## Problem Summary

**Issue**: LLVM Stage-3 Case C (loop control) failed with undefined value error
- Test: `apps/tests/llvm_stage3_loop_only.hako`
- Symptom: `binop dst=19 lhs=17 rhs=18` used undefined v17 and v18
- Root cause: JSON emitter reordered instructions, breaking def-use chain

### Original JSON Order (Broken)
```json
{
  "id": 3,
  "instructions": [
    {"op": "const", "dst": 16},
    {"op": "binop", "dst": 19, "lhs": 17, "rhs": 18},  // ❌ Use v17, v18 before def
    {"op": "copy", "dst": 17, "src": 16},             // ⚠️ Define v17 AFTER use
    {"op": "copy", "dst": 18, "src": 1},              // ⚠️ Define v18 AFTER use
    {"op": "copy", "dst": 20, "src": 19}
  ]
}
```

## Root Cause Analysis

`src/runner/mir_json_emit/mod.rs` が "use-before-def copy 回避" を目的に **copy の遅延/再配置**を行っており、
依存（copy dst を参照する binop 等）まで含めた正しいスケジューリングになっていなかった。

この層（JSON emitter）は optimizer/scheduler ではないため、順序修正を試みるより **Fail-Fast で upstream を炙り出す**のが筋。

### Responsible Code (Lines 193-266, 652-710)
```rust
// Pre-scan: collect values defined anywhere in this block (to delay use-before-def copies)
let mut block_defines: std::collections::HashSet<u32> = ...;
let mut emitted_defs: std::collections::HashSet<u32> = ...;
let mut delayed_copies: Vec<(u32, u32)> = Vec::new();

// Delay copies if source will be defined later
if block_defines.contains(&s) && !emitted_defs.contains(&s) {
    delayed_copies.push((d, s));  // ❌ Reordering!
}

// Emit delayed copies after sources should be available
for (d, s) in delayed_copies {
    insts.push(json!({"op":"copy","dst": d, "src": s}));
}
```

## Solution: SSOT Principle ✅

**Box Theory Diagnosis**: Responsibility boundary violation
- **JSON Emitter responsibility**: Output MIR instructions in original order
- **Builder/Optimizer/Verifier responsibility**: Ensure correct def-use order upstream

### SSOT: 「順序を直す場所」を固定する

- ✅ JSON emitter は "順序を変えない"（MIR の命令列をそのまま出力する）
- ✅ もし MIR 自体に use-before-def があるなら、builder/optimizer/verifier 側で直す（またはそこで Fail-Fast）

### Fix Implementation

**P0-1: Remove all reordering logic** (except PHI consolidation)
```rust
// Phase 131-13: Emit all instructions in MIR order (SSOT principle)
// No reordering except PHI consolidation at block start (LLVM constraint)

// Step 1: Emit all PHI instructions first (LLVM requirement)
for inst in &bb.instructions {
    if let I::Phi { ... } => { insts.push(phi_inst); }
}

// Step 2: Emit all non-PHI instructions in MIR order (no reordering!)
for inst in &bb.instructions {
    match inst {
        I::Phi { .. } => continue,  // Already emitted
        I::Copy { dst, src } => {
            insts.push(json!({"op":"copy","dst": dst.as_u32(), "src": src.as_u32()}));
        }
        // ... other instructions in order
    }
}
```

### Emit 規約 (Updated)

- ✅ PHI は block の先頭（LLVM 制約）。JSON 側で先頭に集約済み。
- ✅ 非PHIは MIR の順序を保持して出力する（並べ替えない）。
- ✅ Terminator は最後。

**Changes**:
- ✅ Removed `block_defines` HashSet
- ✅ Removed `emitted_defs` HashSet
- ✅ Removed `delayed_copies` Vec
- ✅ Removed all `emitted_defs.insert()` calls
- ✅ Removed delayed copy emission loop
- ✅ Applied fix to both `emit_mir_json_for_harness` (lib) and `emit_mir_json_for_harness_bin` (bin)

### Fixed JSON Order ✅
```json
{
  "id": 3,
  "instructions": [
    {"op": "phi", "dst": 1},
    {"op": "const", "dst": 16},
    {"op": "copy", "dst": 17, "src": 16},  // ✅ Define v17 first
    {"op": "copy", "dst": 18, "src": 1},   // ✅ Define v18 second
    {"op": "binop", "dst": 19, "lhs": 17, "rhs": 18},  // ✅ Use v17, v18 after def
    {"op": "copy", "dst": 20, "src": 19},
    {"op": "mir_call", ...},
    {"op": "const", "dst": 21},
    {"op": "ret"}
  ]
}
```

## Verification ✅

### Test Execution
```bash
# Generate fixed JSON
NYASH_DISABLE_PLUGINS=1 ./target/release/hakorune --backend mir \
  --emit-mir-json /tmp/fixed.json apps/tests/llvm_stage3_loop_only.hako
# Output: MIR JSON written: /tmp/fixed.json

# Verify instruction order
cat /tmp/fixed.json | jq '.functions[] | select(.name == "main") | .blocks[1].instructions'
# ✅ Correct: copy dst=17, copy dst=18, binop lhs=17 rhs=18
```

### Build Status
```bash
cargo build --release
# ✅ 0 errors, 0 warnings
```

## Done 条件 ✅

- ✅ Case C の JSON 出力で def→use 順序が正しい
- ✅ LLVM harness が正しく型解決できる（use-before-def エラーなし）
- ✅ ビルド成功（0 errors, 0 warnings）

## Fail-Fast チェック（Future Work）

JSON emit 時に、同一ブロック内の use-before-def を検出したら明示エラーにする（"直そうとしない"）。

**P1: Add use-before-def detector** (NYASH_MIR_STRICT mode) - TODO
```rust
fn check_use_before_def(block: &BasicBlock) -> Result<(), String> {
    let mut defined: HashSet<ValueId> = HashSet::new();
    for inst in &block.instructions {
        // Check all operands are defined
        for operand in inst.operands() {
            if !defined.contains(&operand) && !is_phi(inst) && !is_block_param(operand) {
                if strict_mode() {
                    return Err(format!("Use-before-def: v{}", operand.as_u32()));
                } else {
                    eprintln!("[WARN] Use-before-def: v{}", operand.as_u32());
                }
            }
        }
        if let Some(dst) = inst.dst() {
            defined.insert(dst);
        }
    }
    Ok(())
}
```

## Files Modified

- `src/runner/mir_json_emit/mod.rs`: Core fix (removed ~80 lines of reordering logic)

## Box Theory Insights

**Fail-Fast Principle**: Instead of hiding errors with reordering, expose them early
- ❌ Old approach: Emitter tries to fix broken MIR (scheduler role)
- ✅ New approach: Emitter outputs MIR as-is, builder ensures correctness

**SSOT Boundary**:
- **Upstream** (builder.rs): Responsible for def-use order
- **Emitter** (mir_json_emit.rs): Responsible for faithful output
- **Downstream** (LLVM harness): Expects correct order from JSON
