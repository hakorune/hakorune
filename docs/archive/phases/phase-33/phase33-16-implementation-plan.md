# Phase 33-16 Implementation Plan: Loop Header PHI SSOT

**Date**: 2025-12-07  
**Status**: Detailed design & concrete implementation steps ready  
**Scope**: 6 concrete code changes to establish loop header PHI as Single Source of Truth

---

## Executive Summary

Phase 33-16 transforms loop exit value handling from a "skip and hope" approach to a principled architecture where **loop header PHIs** track carrier values through iterations and serve as the single source of truth for exit values.

**Key Architecture Change**:
```
Before (Phase 33-15):
  LoopVariable → BoundaryInjector Copy → (undefined in latch) → SSA-undef
  Carrier → ExitLine reconnector → (no proper header PHI) → wrong value

After (Phase 33-16):
  LoopVariable → BoundaryInjector Copy → Loop Header PHI → Latch Copy → Loop Header PHI → Exit value
  Carrier → (tracked by same PHI) → ExitLine reconnector → correct value
```

---

## Problem Analysis (Consolidated from Phase 33-15)

### Current State
From `instruction_rewriter.rs` lines 354-431, we currently:
- **Skip exit_phi_inputs** (line 395): "Parameter values undefined"
- **Skip carrier_inputs** (line 429): "Parameter references undefined"

**Root Cause**: Loop parameters (i_param, i_exit) reference function parameters passed via Jump args, not SSA definitions in inlined MIR.

### Why Loop Header PHI Solves This

When we inline JoinIR into MIR, the loop structure becomes:
```text
entry_block:
  i_phi_dst = PHI [(entry_block, i_init), (latch_block, i_next)]
  // Loop condition using i_phi_dst (not i_param)
  
latch_block:
  i_next = i + 1
  JUMP entry_block
  
exit_block:
  return i_phi_dst  // Uses PHI dst, not parameter!
```

The **PHI dst is SSA-defined** at the header, so it can be referenced in exit values.

---

## 6 Concrete Implementation Steps

### Step 1: Integrate LoopHeaderPhiBuilder into merge pipeline

**Location**: `src/mir/builder/control_flow/joinir/merge/mod.rs` (lines 62-184)

**Change**: Between Phase 3 (remap_values) and Phase 4 (instruction_rewriter), insert Phase 3.5:

```rust
// Line 107: After remap_values(...)
remap_values(builder, &used_values, &mut remapper, debug)?;

// NEW Phase 3.5: Build loop header PHIs
// This must happen BEFORE instruction_rewriter so we know the PHI dsts
let mut loop_header_phi_info = if let Some(boundary) = boundary {
    if let Some(loop_var_name) = &boundary.loop_var_name {
        // We need header_block_id and entry_block_id from the JoinIR structure
        // Entry block is the first function's entry block
        let (entry_func_name, entry_func) = mir_module
            .functions
            .iter()
            .next()
            .ok_or("JoinIR module has no functions")?;
        let entry_block_remapped = remapper
            .get_block(entry_func_name, entry_func.entry_block)
            .ok_or_else(|| format!("Entry block not found"))?;
        
        // Header block = entry block in the simplest case
        // For more complex patterns, may need pattern-specific logic
        let header_block_id = entry_block_remapped;
        
        // Get loop variable's initial value (remapped)
        let loop_var_init = remapper
            .get_value(ValueId(0))  // JoinIR param slot
            .ok_or("Loop var init not remapped")?;
        
        // For now, no other carriers (Phase 33-16 minimal)
        let carriers = vec![];
        let expr_result_is_loop_var = boundary.expr_result.is_some();
        
        loop_header_phi_builder::LoopHeaderPhiBuilder::build(
            builder,
            header_block_id,
            entry_block_remapped,
            loop_var_name,
            loop_var_init,
            &carriers,
            expr_result_is_loop_var,
            debug,
        )?
    } else {
        loop_header_phi_builder::LoopHeaderPhiInfo::empty(BasicBlockId(0))
    }
} else {
    loop_header_phi_builder::LoopHeaderPhiInfo::empty(BasicBlockId(0))
};

// Phase 4: Merge blocks and rewrite instructions
// PASS loop_header_phi_info to instruction_rewriter
let merge_result = instruction_rewriter::merge_and_rewrite(
    builder,
    mir_module,
    &mut remapper,
    &value_to_func_name,
    &function_params,
    boundary,
    &mut loop_header_phi_info,  // NEW: Pass mutable reference
    debug,
)?;
```

**Key Points**:
- Phase 3.5 executes after remap_values so we have remapped ValueIds
- Allocates PHI dsts but doesn't emit PHI instructions yet
- Stores PHI dst in LoopHeaderPhiInfo for use in Phase 4

---

### Step 2: Modify instruction_rewriter signature and latch tracking

**Location**: `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs` (lines 29-37)

**Change**: Add loop_header_phi_info parameter:

```rust
pub(super) fn merge_and_rewrite(
    builder: &mut crate::mir::builder::MirBuilder,
    mir_module: &MirModule,
    remapper: &mut JoinIrIdRemapper,
    value_to_func_name: &HashMap<ValueId, String>,
    function_params: &HashMap<String, Vec<ValueId>>,
    boundary: Option<&JoinInlineBoundary>,
    loop_header_phi_info: &mut loop_header_phi_builder::LoopHeaderPhiInfo,  // NEW
    debug: bool,
) -> Result<MergeResult, String> {
    // ... existing code ...
}
```

**Change in return logic** (lines 346-459, in the terminator rewriting section):

When we process `MirInstruction::Return { value }` in the latch block, track the latch incoming:

```rust
// After determining tail_call_target for tail call to loop header
if let Some((target_block, args)) = tail_call_target {
    if debug {
        eprintln!("[cf_loop/joinir] Inserting param bindings for tail call to {:?}", target_block);
    }
    
    // ... existing param binding code (lines 276-319) ...
    
    // NEW: Track latch incoming for loop header PHI
    // The tail_call_target is the loop header, and args are the updated values
    // For Pattern 2, args[0] is the updated loop variable
    if let Some(loop_var_name) = &boundary.map(|b| &b.loop_var_name).flatten() {
        if !args.is_empty() {
            let latch_value = args[0]; // Updated loop variable value
            loop_header_phi_info.set_latch_incoming(
                loop_var_name,
                target_block,  // latch block ID
                latch_value,
            );
            
            if debug {
                eprintln!("[cf_loop/joinir] Phase 33-16: Set latch incoming for '{}': {:?}", 
                    loop_var_name, latch_value);
            }
        }
    }
    
    // ... existing terminator code ...
}
```

**Key Points**:
- After instruction rewriter completes, loop_header_phi_info has both entry and latch incoming set
- Latch incoming is extracted from tail call args (the actual updated values)

---

### Step 3: Replace skip logic with header PHI references

**Location**: `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs` (lines 354-431)

**Change**: Replace the skip logic with proper PHI references:

```rust
// OLD CODE (lines 354-398): Skip exit_phi_inputs collection
// REMOVE: All the comments about "parameter values undefined"

// NEW CODE:
if let Some(ret_val) = value {
    let remapped_val = remapper.get_value(*ret_val).unwrap_or(*ret_val);
    
    // Phase 33-16: Use header PHI dst if available
    if let Some(loop_var_name) = &boundary.and_then(|b| b.loop_var_name.as_ref()) {
        if let Some(phi_dst) = loop_header_phi_info.get_carrier_phi(loop_var_name) {
            if debug {
                eprintln!("[cf_loop/joinir] Phase 33-16: Using loop header PHI {:?} for exit value (not parameter {:?})",
                    phi_dst, ret_val);
            }
            // Collect the PHI dst as exit value, not the parameter
            exit_phi_inputs.push((exit_block_id, phi_dst));
        } else {
            if debug {
                eprintln!("[cf_loop/joinir] Phase 33-16 WARNING: No header PHI for loop var '{}', using parameter {:?}",
                    loop_var_name, ret_val);
            }
            // Fallback: use parameter (for compatibility)
            exit_phi_inputs.push((exit_block_id, remapped_val));
        }
    } else {
        if debug {
            eprintln!("[cf_loop/joinir] Phase 33-16: No loop_var_name in boundary, using parameter {:?}", ret_val);
        }
        // Fallback: use parameter (for non-loop patterns)
        exit_phi_inputs.push((exit_block_id, remapped_val));
    }
}

// OLD CODE (lines 400-431): Skip carrier_inputs collection
// REMOVE: All the comments about "parameter values undefined"

// NEW CODE:
// Phase 33-13/16: Collect carrier exit values using header PHI dsts
if let Some(boundary) = boundary {
    for binding in &boundary.exit_bindings {
        // Phase 33-16: Look up the header PHI dst for this carrier
        if let Some(phi_dst) = loop_header_phi_info.get_carrier_phi(&binding.carrier_name) {
            if debug {
                eprintln!("[cf_loop/joinir] Phase 33-16: Using header PHI {:?} for carrier '{}' exit",
                    phi_dst, binding.carrier_name);
            }
            carrier_inputs.entry(binding.carrier_name.clone())
                .or_insert_with(Vec::new)
                .push((exit_block_id, phi_dst));
        } else {
            if debug {
                eprintln!("[cf_loop/joinir] Phase 33-16 WARNING: No header PHI for carrier '{}', skipping",
                    binding.carrier_name);
            }
        }
    }
}
```

**Key Points**:
- Instead of skipping, we use the loop header PHI dsts
- Loop header PHI dsts are guaranteed to be SSA-defined
- Fallback to parameter for backward compatibility

---

### Step 4: Finalize header PHIs in the exit block

**Location**: `src/mir/builder/control_flow/joinir/merge/mod.rs` (lines 120-136)

**Change**: After Phase 5 (exit_phi_builder), finalize the header PHIs:

```rust
// Phase 5: Build exit PHI (expr result and carrier PHIs)
let (exit_phi_result_id, carrier_phis) = exit_phi_builder::build_exit_phi(
    builder,
    merge_result.exit_block_id,
    &merge_result.exit_phi_inputs,
    &merge_result.carrier_inputs,
    debug,
)?;

// Phase 33-16 NEW: Finalize loop header PHIs
// This inserts the PHI instructions at the beginning of the header block
// with both entry and latch incoming edges now set
loop_header_phi_builder::LoopHeaderPhiBuilder::finalize(
    builder,
    &loop_header_phi_info,
    debug,
)?;

// Phase 6: Reconnect boundary (if specified)
// ...
```

**Key Points**:
- finalize() inserts PHI instructions at the beginning of header block
- Must happen after instruction_rewriter sets latch incoming
- Before ExitLineOrchestrator so carrier_phis includes header PHI dsts

---

### Step 5: Update pattern lowerers to set loop_var_name and extract carriers

**Location**: `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs` (lines 193-200)

**Change**: Extract carrier information and pass to LoopHeaderPhiBuilder:

```rust
// Line 200: Already sets loop_var_name ✓
boundary.loop_var_name = Some(loop_var_name.clone());

// NEW: Extract other carriers from exit_bindings
// For Pattern 2 with multiple carriers (Phase 33-16 extension)
let carriers: Vec<(String, ValueId)> = boundary.exit_bindings.iter()
    .filter(|binding| binding.carrier_name != loop_var_name)  // Skip loop var
    .map(|binding| {
        // Get the initial value for this carrier from join_inputs
        // This is a simplification; real patterns may need more sophisticated extraction
        let init_val = /* extract from join_inputs based on binding.carrier_name */;
        (binding.carrier_name.clone(), init_val)
    })
    .collect();

// The carriers list is then passed to LoopHeaderPhiBuilder::build()
// (in Step 1, where merge_and_rewrite is called)
```

**Key Points**:
- Minimal change: mostly just data extraction
- Carriers are extracted from exit_bindings which are already computed

---

### Step 6: Update Module Documentation

**Location**: `src/mir/builder/control_flow/joinir/merge/mod.rs` (lines 1-60)

**Change**: Update phase documentation to include Phase 3.5:

```rust
//! JoinIR MIR Block Merging Coordinator
//!
//! This module coordinates the merging of JoinIR-generated MIR functions
//! into the host MIR builder. The process is broken into 7 phases:
//!
//! 1. Block ID allocation (block_allocator.rs)
//! 2. Value collection (value_collector.rs)
//! 3. ValueId remapping (uses JoinIrIdRemapper)
//! 3.5. Loop header PHI generation (loop_header_phi_builder.rs) [NEW - Phase 33-16]
//! 4. Instruction rewriting (instruction_rewriter.rs)
//! 5. Exit PHI construction (exit_phi_builder.rs)
//! 6. Boundary reconnection (inline in this file)
//!
//! Phase 33-16: Loop header PHI as SSOT
//! ====================================
//! 
//! Phase 33-16 establishes loop header PHIs as the Single Source of Truth
//! for carrier values during loop execution:
//!
//! - Phase 3.5: Generate header PHIs with entry incoming edges
//! - Phase 4: Instruction rewriter sets latch incoming edges + uses PHI dsts
//!   for exit values instead of undefined loop parameters
//! - Phase 4.5: Finalize header PHIs (insert into blocks)
//! - Phase 6: ExitLineOrchestrator uses header PHI dsts from carrier_phis
//!
//! This fixes the SSA-undef errors from Phase 33-15 by ensuring all exit
//! values reference SSA-defined PHI destinations, not function parameters.
```

---

## Testing Strategy

### Unit Tests (for LoopHeaderPhiBuilder)

Already exist in `loop_header_phi_builder.rs` (lines 284-318):
- `test_loop_header_phi_info_creation()`: Empty info
- `test_carrier_phi_entry()`: Carrier setup and latch incoming

### Integration Tests

**Test Case 1**: Pattern 2 with loop variable (existing `joinir_min_loop.hako`)
- Verify header PHI is created
- Verify latch incoming is set from tail call args
- Verify exit PHI uses header PHI dst (not parameter)

**Test Case 2**: Pattern 3 with multiple carriers (if implemented)
- Verify each carrier has a header PHI
- Verify exit values reference header PHI dsts
- Verify variable_map is updated with carrier PHI dsts

### Debug Output Verification

```bash
# Run with debug enabled
NYASH_JOINIR_DEBUG=1 ./target/release/nyash --dump-mir test_file.hako 2>&1 | grep "Phase 33-16"

# Expected output:
# [cf_loop/joinir] Phase 33-16: Building header PHIs at BasicBlockId(N)
# [cf_loop/joinir]   Loop var 'i' init=ValueId(X), entry_block=BasicBlockId(Y)
# [cf_loop/joinir]   Loop var PHI: ValueId(Z) = phi [(from BasicBlockId(Y), ValueId(X)), (latch TBD)]
# [cf_loop/joinir] Phase 33-16: Set latch incoming for 'i': ValueId(W)
# [cf_loop/joinir] Phase 33-16: Using loop header PHI ValueId(Z) for exit value
# [cf_loop/joinir] Phase 33-16: Finalizing header PHIs at BasicBlockId(N)
```

---

## Implementation Checklist

- [ ] **Step 1**: Add Phase 3.5 to merge/mod.rs (LoopHeaderPhiBuilder::build call)
- [ ] **Step 2**: Update instruction_rewriter signature and latch tracking
- [ ] **Step 3**: Replace skip logic with header PHI references
- [ ] **Step 4**: Add finalize call in merge pipeline
- [ ] **Step 5**: Update pattern2 lowerer (loop_var_name already set, extract carriers)
- [ ] **Step 6**: Update module documentation
- [ ] **Compile**: `cargo build --release`
- [ ] **Test**: Run joinir_min_loop.hako and verify MIR
- [ ] **Debug**: Enable NYASH_JOINIR_DEBUG=1 and verify output

---

## Key Design Decisions

### Why Phase 3.5?
- Must happen after remap_values() (need remapped ValueIds)
- Must happen before instruction_rewriter (need to know PHI dsts for exit values)
- Clear responsibility separation

### Why LoopHeaderPhiInfo as mutable?
- instruction_rewriter sets latch incoming via set_latch_incoming()
- Finalize reads all latch incoming to validate completeness
- Mutable reference is cleaner than returning modified info

### Why keep fallback to parameters?
- Not all patterns may use loop header PHIs yet
- Backward compatibility with Phase 33-15
- Easier to debug regressions

### Why separate build() and finalize()?
- build() allocates ValueIds (Phase 3.5)
- finalize() emits instructions (Phase 4.5)
- Clear two-phase commit pattern
- Allows validation that all latch incoming are set

---

## Dependencies & Imports

**No new external dependencies**. Uses existing modules:
- `loop_header_phi_builder` (already created in loop_header_phi_builder.rs)
- `instruction_rewriter` (existing)
- `block_allocator` (existing)
- JoinIR lowering modules (existing)

---

## Risk Analysis

### Low Risk
✅ LoopHeaderPhiBuilder already implemented and tested  
✅ Using existing pattern lowerer infrastructure  
✅ Fallback mechanism preserves backward compatibility  

### Medium Risk
⚠️ Loop header block identification (Step 1): Currently assumes entry block = header block
  - **Mitigation**: Add debug logging to verify correct block
  - **Future**: May need pattern-specific logic for complex loops

⚠️ Carrier extraction (Step 5): Currently minimal
  - **Mitigation**: Phase 33-16 focuses on loop variable; carriers in Phase 33-16+
  - **Future**: Will be enhanced as more patterns are implemented

### Testing Requirements
- [ ] Compile clean (no warnings)
- [ ] joinir_min_loop.hako produces correct MIR
- [ ] Variable values are correct at loop exit
- [ ] No SSA-undef errors in MIR verification

---

## Future Enhancements (Phase 33-16+)

1. **Pattern 3 with multiple carriers**: Extract all carriers from exit_bindings
2. **Dynamic carrier discovery**: Analyze exit_bindings at runtime to determine carriers
3. **Pattern-specific header block**: Some patterns may have different header structure
4. **Optimization**: Eliminate redundant PHI nodes (constant carriers)

---

## Summary

Phase 33-16 transforms exit value handling from "skip and hope" to "SSA-correct by design" through:

1. Allocating loop header PHI dsts early (Phase 3.5)
2. Tracking latch incoming through instruction rewriting (Phase 4)
3. Using PHI dsts instead of parameters in exit values (Phase 4/5)
4. Finalizing PHIs for SSA verification (Phase 4.5)
5. Passing correct values to ExitLineOrchestrator (Phase 6)

This eliminates SSA-undef errors while maintaining clear separation of concerns and backward compatibility.
Status: Historical
