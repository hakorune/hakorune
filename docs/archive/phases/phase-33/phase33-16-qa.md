# Phase 33-16: Q&A - Implementation Flow Details

## Your Questions Answered

### Q1: Where exactly should LoopHeaderPhiBuilder::build() be called?

**Answer**: Between Phase 3 (remap_values) and Phase 4 (instruction_rewriter) in `merge/mod.rs`

**Location**: Line 107, after `remap_values()`

**Why here**:
- ✅ **After** remap_values: We have remapped ValueIds (needed for phi_dst allocation)
- ✅ **Before** instruction_rewriter: We need to know PHI dsts so instruction_rewriter can use them in exit values
- ✅ Clear phase boundary: Phase 3.5

**Code location in file**:
```rust
// Line 107: After remap_values(...)
remap_values(builder, &used_values, &mut remapper, debug)?;

// INSERT HERE: Phase 3.5 - Build loop header PHIs
let mut loop_header_phi_info = if let Some(boundary) = boundary {
    // ... build logic ...
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

---

### Q2: How do I get the header_block_id (loop_step's entry block after remapping)?

**Answer**: It's the entry function's entry block, obtained via remapper

**Exact code**:
```rust
let (entry_func_name, entry_func) = mir_module
    .functions
    .iter()
    .next()
    .ok_or("JoinIR module has no functions")?;
    
let entry_block_remapped = remapper
    .get_block(entry_func_name, entry_func.entry_block)
    .ok_or_else(|| format!("Entry block not found"))?;

// For Pattern 2, entry block == header block
let header_block_id = entry_block_remapped;
```

**Why this works**:
- JoinIR's first function is always the entry function
- `entry_func.entry_block` is the BasicBlockId in JoinIR space
- `remapper.get_block()` returns the remapped BasicBlockId in the host MIR

**For more complex patterns** (future):
- Pattern 3/4 might have different header block logic
- For now, assume entry_block == header_block (safe for Pattern 2)

---

### Q3: How do I get the loop variable's initial value (host-side)?

**Answer**: Get it from the remapper (it's the remapped join_inputs[0])

**Exact code**:
```rust
// Loop variable's initial value is from join_inputs[0]
// It's been remapped by remap_values() in Phase 3
let loop_var_init = remapper
    .get_value(ValueId(0))  // JoinIR param slot (always 0 for loop var)
    .ok_or("Loop var init not remapped")?;
```

**Why ValueId(0)**:
- In JoinIR, loop parameter is always allocated as ValueId(0)
- Pattern 2 lowerer does this (pattern2_with_break.rs line 84):
  ```rust
  env.insert(loop_var_name.clone(), crate::mir::ValueId(0));
  ```
- After remap_values(), this becomes a new ValueId in host space

**What you DON'T need**:
- ❌ Don't look in boundary.host_inputs[0] directly
- ❌ Don't use boundary.join_inputs[0] (it's the pre-remap value)
- ✅ Use remapper.get_value(ValueId(0)) (it's the post-remap value)

---

### Q4: Where should instruction_rewriter record latch_incoming?

**Answer**: In the tail call handling section, after parameter bindings

**Location**: `instruction_rewriter.rs`, in the tail call branch (lines 276-335)

**Exact code**:
```rust
if let Some((target_block, args)) = tail_call_target {
    // ... existing parameter binding code (lines 276-319) ...
    
    // NEW: Track latch incoming AFTER param bindings
    if let Some(loop_var_name) = &boundary.and_then(|b| b.loop_var_name.as_ref()) {
        if !args.is_empty() {
            let latch_value = args[0]; // Updated loop variable from tail call args
            loop_header_phi_info.set_latch_incoming(
                loop_var_name,
                target_block,  // This is the loop header block (from tail call target)
                latch_value,   // This is i_next (the updated value)
            );
            
            if debug {
                eprintln!("[cf_loop/joinir] Phase 33-16: Set latch incoming for '{}': {:?}", 
                    loop_var_name, latch_value);
            }
        }
    }
    
    // ... then set terminator to Jump (line 321-323) ...
}
```

**Why this location**:
- Tail call args are the ACTUAL updated values (i_next, not i_param)
- args[0] is guaranteed to be the loop variable (Pattern 2 guarantees)
- target_block is the loop header (where we're jumping back to)
- Called for EACH block that has a tail call (ensures all paths tracked)

**Key insight**: The latch block is NOT explicitly identified; we identify it by the Jump target!

---

### Q5: Should the Phase 33-15 skip logic be removed or modified to use header PHI dst?

**Answer**: Modify, NOT remove. Use header PHI dst when available, fallback to parameter.

**What to do**:

1. **Replace skip logic** (lines 354-398 in instruction_rewriter.rs):
```rust
// OLD: Skip exit_phi_inputs collection
// if debug { eprintln!(...skip...); }

// NEW: Use header PHI dst if available
if let Some(ret_val) = value {
    let remapped_val = remapper.get_value(*ret_val).unwrap_or(*ret_val);
    
    // Phase 33-16: Prefer header PHI dst
    if let Some(loop_var_name) = &boundary.and_then(|b| b.loop_var_name.as_ref()) {
        if let Some(phi_dst) = loop_header_phi_info.get_carrier_phi(loop_var_name) {
            // Use PHI dst (SSA-correct!)
            exit_phi_inputs.push((exit_block_id, phi_dst));
        } else {
            // Fallback: Use parameter (for backward compatibility)
            exit_phi_inputs.push((exit_block_id, remapped_val));
        }
    } else {
        // No boundary or loop_var_name: use parameter
        exit_phi_inputs.push((exit_block_id, remapped_val));
    }
}
```

2. **Modify carrier_inputs logic** (lines 400-431):
```rust
// OLD: Skip carrier_inputs collection
// if debug { eprintln!(...skip...); }

// NEW: Use header PHI dsts for carriers
if let Some(boundary) = boundary {
    for binding in &boundary.exit_bindings {
        // Phase 33-16: Look up header PHI dst for this carrier
        if let Some(phi_dst) = loop_header_phi_info.get_carrier_phi(&binding.carrier_name) {
            carrier_inputs.entry(binding.carrier_name.clone())
                .or_insert_with(Vec::new)
                .push((exit_block_id, phi_dst));
        }
        // If no PHI dst, skip this carrier (not yet implemented)
    }
}
```

**Why this approach**:
- ✅ Phase 33-16 adds header PHIs → use them (SSA-correct)
- ✅ If no header PHIs → fallback to old behavior (backward compat)
- ✅ Gradual migration: Patterns enable loop_var_name progressively
- ✅ Easy to debug: Explicit "Using PHI" vs "Fallback" logs

**Don't do**:
- ❌ Don't remove skip logic entirely (patterns without loop_var_name would break)
- ❌ Don't add loop_header_phi_info to merge_and_rewrite() signature if you don't track latch
- ✅ Do add both build() and finalize() to merge/mod.rs

---

### Q6: Flow Summary - How does it all fit together?

**Complete flow**:

```
merge_joinir_mir_blocks() {
    // Phase 1: Allocate block IDs
    allocate_blocks()
    
    // Phase 2: Collect values
    collect_values()
    
    // Phase 3: Remap ValueIds
    remap_values(builder, &used_values, &mut remapper)
    
    // ===== Phase 3.5 (NEW) =====
    // Build loop header PHIs with entry incoming edges
    let mut loop_header_phi_info = if let Some(boundary) = boundary {
        if let Some(loop_var_name) = &boundary.loop_var_name {
            // Get header_block_id (entry block after remap)
            let entry_block = remapper.get_block(entry_func, entry_func.entry_block)?;
            
            // Get loop_var_init (remapped ValueId(0))
            let loop_var_init = remapper.get_value(ValueId(0))?;
            
            // Build header PHIs (allocates PHI dsts, doesn't emit yet)
            LoopHeaderPhiBuilder::build(
                builder,
                entry_block,      // header_block_id
                entry_block,      // entry_block_id
                loop_var_name,
                loop_var_init,
                &[],              // No other carriers yet
                boundary.expr_result.is_some(),
                debug,
            )?
        } else {
            LoopHeaderPhiInfo::empty(...)
        }
    } else {
        LoopHeaderPhiInfo::empty(...)
    };
    
    // ===== Phase 4 (MODIFIED) =====
    // Instruction rewriter sets latch incoming and uses PHI dsts
    let merge_result = instruction_rewriter::merge_and_rewrite(
        builder,
        mir_module,
        &mut remapper,
        ...,
        &mut loop_header_phi_info,  // PASS MUTABLE REFERENCE
        debug,
    )?;
    // Inside merge_and_rewrite:
    //   - When processing tail calls: record latch_incoming
    //   - When processing Return: use header PHI dsts (not parameters)
    
    // ===== Phase 5 =====
    // Build exit PHI from exit_phi_inputs and carrier_inputs
    let (exit_phi_result_id, carrier_phis) = exit_phi_builder::build_exit_phi(...)?;
    
    // ===== Phase 4.5 (NEW) =====
    // Finalize loop header PHIs (insert into blocks)
    LoopHeaderPhiBuilder::finalize(builder, &loop_header_phi_info, debug)?;
    
    // ===== Phase 6 =====
    // Reconnect exit values using carrier_phis from Phase 5
    if let Some(boundary) = boundary {
        ExitLineOrchestrator::execute(builder, boundary, &carrier_phis, debug)?;
    }
    
    // ... continue with boundary jump and exit block switch ...
}
```

**Key transitions**:
1. Phase 3 → Phase 3.5: remap_values() gives us remapped ValueIds
2. Phase 3.5 → Phase 4: loop_header_phi_info allocated (PHI dsts ready)
3. Phase 4 → Phase 4.5: instruction_rewriter sets latch_incoming
4. Phase 4.5 → Phase 5: Finalize emits PHIs into blocks
5. Phase 5 → Phase 6: exit_phi_builder returns carrier_phis (PHI dsts)
6. Phase 6: ExitLineOrchestrator uses carrier_phis to update variable_map

---

## Summary: Exact Answer to Your Core Question

**Where to call build()**: Line 107 in merge/mod.rs, after remap_values()  
**How to get header_block_id**: `remapper.get_block(entry_func_name, entry_func.entry_block)?`  
**How to get loop_var_init**: `remapper.get_value(ValueId(0))?`  
**Where to record latch_incoming**: In tail call handling (line ~300), use args[0] as latch_value  
**Replace skip logic**: Yes, with fallback mechanism for backward compatibility  

**The magic**: Loop header PHI dst (allocated in Phase 3.5, finalized in Phase 4.5) is SSA-defined and can be safely used in exit values instead of parameters!
Status: Historical
