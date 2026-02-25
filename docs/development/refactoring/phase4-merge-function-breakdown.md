# Phase 4: merge_joinir_mir_blocks Breakdown Guide

**CRITICAL PHASE** - This is the most complex modularization task.

**Context**: `merge_joinir_mir_blocks()` is a 714-line monster function that performs 6 distinct operations. This guide provides step-by-step instructions for breaking it down safely.

---

## Overview

### Current State
- **File**: `src/mir/builder/control_flow.rs`
- **Function**: `merge_joinir_mir_blocks()` (lines 864-1578)
- **Size**: 714 lines (44% of the entire file!)
- **Complexity**: 6 distinct phases, multiple data structures, critical to JoinIR integration

### Target State
- **Directory**: `src/mir/builder/control_flow/joinir/merge/`
- **Files**: 7 files (1 coordinator + 6 sub-modules)
- **Average size**: ~100-150 lines per file
- **Maintainability**: Each phase in its own file

---

## Function Analysis

### Current Structure (Lines 864-1578)

```rust
fn merge_joinir_mir_blocks(
    &mut self,
    mir_module: &MirModule,
    boundary: Option<&JoinInlineBoundary>,
    debug: bool,
) -> Result<Option<ValueId>, String> {
    // Phase 1: Block ID Allocation (lines 864-923)
    // - Create JoinIrIdRemapper
    // - Allocate new block IDs for all functions
    // - Map function entry blocks

    // Phase 2: Value Collection (lines 931-971)
    // - Collect all ValueIds used across functions
    // - Track Const String → function name mapping
    // - Collect function parameters for tail call conversion

    // Phase 3: ValueId Remapping (lines 973-1100)
    // - Allocate new ValueIds for all collected values
    // - Create remapping table
    // - Handle determinism (BTreeSet/BTreeMap usage)

    // Phase 4: Block Merging & Instruction Rewriting (lines 1102-1400)
    // - For each function in sorted order:
    //   - For each block in sorted order:
    //     - Rewrite instructions with remapped IDs
    //     - Convert Call → Jump for intra-module calls
    //     - Convert Return → Jump to exit block
    //     - Handle tail call optimization
    // - Insert merged blocks into current_function

    // Phase 5: Exit PHI Construction (lines 1402-1500)
    // - Collect return values from all functions
    // - Create exit block with PHI node
    // - Return exit PHI value

    // Phase 6: Boundary Reconnection (lines 1502-1578)
    // - If boundary with host_outputs specified:
    //   - Map exit PHI back to host variable_map
    //   - Update SSA slots in variable_map
    //   - Reconnect inlined code to host context
}
```

---

## Proposed Module Breakdown

### 1. `merge/mod.rs` - Coordinator (~100 lines)

**Purpose**: Orchestrate the 6 phases without implementing logic.

```rust
//! JoinIR MIR Block Merging Coordinator
//!
//! This module coordinates the merging of JoinIR-generated MIR functions
//! into the host MIR builder. The process is broken into 6 phases:
//!
//! 1. Block ID allocation
//! 2. Value collection
//! 3. ValueId remapping
//! 4. Instruction rewriting
//! 5. Exit PHI construction
//! 6. Boundary reconnection

use crate::mir::{MirModule, ValueId};
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;

mod id_remapper;
mod block_allocator;
mod value_collector;
mod instruction_rewriter;
mod exit_phi_builder;

use id_remapper::JoinIrIdRemapper;

pub(in crate::mir::builder) fn merge_joinir_mir_blocks(
    builder: &mut crate::mir::builder::MirBuilder,
    mir_module: &MirModule,
    boundary: Option<&JoinInlineBoundary>,
    debug: bool,
) -> Result<Option<ValueId>, String> {
    // Phase 1: Allocate block IDs
    let mut remapper = block_allocator::allocate_blocks(builder, mir_module, debug)?;

    // Phase 2: Collect values
    let (used_values, value_to_func_name, function_params) =
        value_collector::collect_values(mir_module, &mut remapper, debug)?;

    // Phase 3: Remap ValueIds
    id_remapper::remap_values(builder, &used_values, &mut remapper, debug)?;

    // Phase 4: Merge blocks and rewrite instructions
    let exit_block_id = instruction_rewriter::merge_and_rewrite(
        builder,
        mir_module,
        &mut remapper,
        &value_to_func_name,
        &function_params,
        debug,
    )?;

    // Phase 5: Build exit PHI
    let exit_phi_value = exit_phi_builder::build_exit_phi(
        builder,
        mir_module,
        &remapper,
        exit_block_id,
        debug,
    )?;

    // Phase 6: Reconnect boundary (if specified)
    if let Some(boundary) = boundary {
        reconnect_boundary(builder, boundary, exit_phi_value, debug)?;
    }

    Ok(exit_phi_value)
}

fn reconnect_boundary(
    builder: &mut crate::mir::builder::MirBuilder,
    boundary: &JoinInlineBoundary,
    exit_phi_value: Option<ValueId>,
    debug: bool,
) -> Result<(), String> {
    // Phase 6 implementation (simple, can stay in mod.rs)
    // ... (lines 1502-1578 logic)
    Ok(())
}
```

---

### 2. `merge/id_remapper.rs` - ID Remapping (~150 lines)

**Purpose**: Manage ValueId and BlockId remapping.

```rust
//! JoinIR ID Remapper
//!
//! Handles the mapping of ValueIds and BlockIds from JoinIR functions
//! to the host MIR builder's ID space.

use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeMap;

/// Remapper for JoinIR → Host MIR ID translation
pub(super) struct JoinIrIdRemapper {
    /// (function_name, old_block_id) → new_block_id
    block_map: BTreeMap<(String, BasicBlockId), BasicBlockId>,

    /// old_value_id → new_value_id
    value_map: BTreeMap<ValueId, ValueId>,

    /// function_name → entry_block_id
    function_entry_map: BTreeMap<String, BasicBlockId>,
}

impl JoinIrIdRemapper {
    pub(super) fn new() -> Self {
        Self {
            block_map: BTreeMap::new(),
            value_map: BTreeMap::new(),
            function_entry_map: BTreeMap::new(),
        }
    }

    pub(super) fn set_block(&mut self, func_name: String, old_id: BasicBlockId, new_id: BasicBlockId) {
        self.block_map.insert((func_name, old_id), new_id);
    }

    pub(super) fn get_block(&self, func_name: &str, old_id: BasicBlockId) -> Option<BasicBlockId> {
        self.block_map.get(&(func_name.to_string(), old_id)).copied()
    }

    pub(super) fn set_value(&mut self, old_id: ValueId, new_id: ValueId) {
        self.value_map.insert(old_id, new_id);
    }

    pub(super) fn get_value(&self, old_id: ValueId) -> Option<ValueId> {
        self.value_map.get(&old_id).copied()
    }

    pub(super) fn set_function_entry(&mut self, func_name: String, entry_block: BasicBlockId) {
        self.function_entry_map.insert(func_name, entry_block);
    }

    pub(super) fn get_function_entry(&self, func_name: &str) -> Option<BasicBlockId> {
        self.function_entry_map.get(func_name).copied()
    }

    // ... (other helper methods from lines 873-959)
}

pub(super) fn remap_values(
    builder: &mut crate::mir::builder::MirBuilder,
    used_values: &std::collections::BTreeSet<ValueId>,
    remapper: &mut JoinIrIdRemapper,
    debug: bool,
) -> Result<(), String> {
    // Phase 3 logic (lines 973-1100)
    // - Allocate new ValueIds
    // - Store in remapper
    Ok(())
}
```

---

### 3. `merge/block_allocator.rs` - Block Allocation (~100 lines)

**Purpose**: Allocate block IDs for all JoinIR functions.

```rust
//! JoinIR Block ID Allocator
//!
//! Allocates new BasicBlockIds for all blocks in JoinIR functions
//! to avoid ID conflicts with the host MIR builder.

use crate::mir::{BasicBlockId, MirModule};
use super::id_remapper::JoinIrIdRemapper;

pub(super) fn allocate_blocks(
    builder: &mut crate::mir::builder::MirBuilder,
    mir_module: &MirModule,
    debug: bool,
) -> Result<JoinIrIdRemapper, String> {
    let mut remapper = JoinIrIdRemapper::new();

    if debug {
        eprintln!("[merge/block_allocator] Allocating block IDs for {} functions",
                  mir_module.functions.len());
    }

    // DETERMINISM FIX: Sort functions by name
    let mut functions: Vec<_> = mir_module.functions.iter().collect();
    functions.sort_by_key(|(name, _)| name.as_str());

    for (func_name, func) in functions {
        if debug {
            eprintln!("[merge/block_allocator]   Function: {}", func_name);
        }

        // DETERMINISM FIX: Sort blocks by ID
        let mut blocks: Vec<_> = func.blocks.iter().collect();
        blocks.sort_by_key(|(id, _)| id.0);

        for (old_block_id, _) in blocks {
            let new_block_id = builder.block_gen.next();
            remapper.set_block(func_name.clone(), *old_block_id, new_block_id);

            if debug {
                eprintln!("[merge/block_allocator]     Block: {:?} → {:?}",
                          old_block_id, new_block_id);
            }
        }

        // Store function entry block
        let entry_block_new = remapper.get_block(func_name, func.entry_block)
            .ok_or_else(|| format!("Entry block not found for {}", func_name))?;
        remapper.set_function_entry(func_name.clone(), entry_block_new);
    }

    if debug {
        eprintln!("[merge/block_allocator] Allocation complete");
    }

    Ok(remapper)
}
```

---

### 4. `merge/value_collector.rs` - Value Collection (~100 lines)

**Purpose**: Collect all ValueIds used across JoinIR functions.

```rust
//! JoinIR Value Collector
//!
//! Collects all ValueIds used in JoinIR functions for remapping.

use crate::mir::{ValueId, MirModule};
use super::id_remapper::JoinIrIdRemapper;
use std::collections::{BTreeSet, HashMap};

pub(super) fn collect_values(
    mir_module: &MirModule,
    remapper: &mut JoinIrIdRemapper,
    debug: bool,
) -> Result<(BTreeSet<ValueId>, HashMap<ValueId, String>, HashMap<String, Vec<ValueId>>), String> {
    let mut used_values = BTreeSet::new();
    let mut value_to_func_name = HashMap::new();
    let mut function_params = HashMap::new();

    if debug {
        eprintln!("[merge/value_collector] Collecting values from {} functions",
                  mir_module.functions.len());
    }

    for (func_name, func) in &mir_module.functions {
        // Collect function parameters
        function_params.insert(func_name.clone(), func.params.clone());

        for block in func.blocks.values() {
            // Collect values from instructions
            let block_values = collect_values_in_block(block);
            used_values.extend(block_values);

            // Track Const String → function name mapping
            for inst in &block.instructions {
                if let crate::mir::MirInstruction::Const { dst, value } = inst {
                    if let crate::mir::types::ConstValue::String(s) = value {
                        // Check if this is a function name
                        if mir_module.functions.contains_key(s) {
                            value_to_func_name.insert(*dst, s.clone());
                            used_values.insert(*dst);
                        }
                    }
                }
            }
        }
    }

    if debug {
        eprintln!("[merge/value_collector] Collected {} values", used_values.len());
    }

    Ok((used_values, value_to_func_name, function_params))
}

fn collect_values_in_block(block: &crate::mir::BasicBlock) -> Vec<ValueId> {
    // Helper function from lines 948-970
    // ... (extract logic from remapper.collect_values_in_block)
    vec![]
}
```

---

### 5. `merge/instruction_rewriter.rs` - Instruction Rewriting (~150 lines)

**Purpose**: Rewrite instructions and merge blocks.

```rust
//! JoinIR Instruction Rewriter
//!
//! Rewrites JoinIR instructions with remapped IDs and merges blocks
//! into the host MIR builder.

use crate::mir::{BasicBlockId, ValueId, MirModule, MirInstruction, BasicBlock};
use super::id_remapper::JoinIrIdRemapper;
use std::collections::HashMap;

pub(super) fn merge_and_rewrite(
    builder: &mut crate::mir::builder::MirBuilder,
    mir_module: &MirModule,
    remapper: &mut JoinIrIdRemapper,
    value_to_func_name: &HashMap<ValueId, String>,
    function_params: &HashMap<String, Vec<ValueId>>,
    debug: bool,
) -> Result<BasicBlockId, String> {
    // Create exit block
    let exit_block_id = builder.block_gen.next();

    if debug {
        eprintln!("[merge/instruction_rewriter] Merging blocks from {} functions",
                  mir_module.functions.len());
        eprintln!("[merge/instruction_rewriter] Exit block: {:?}", exit_block_id);
    }

    // DETERMINISM FIX: Sort functions by name
    let mut functions: Vec<_> = mir_module.functions.iter().collect();
    functions.sort_by_key(|(name, _)| name.as_str());

    for (func_name, func) in functions {
        merge_function_blocks(
            builder,
            func_name,
            func,
            remapper,
            value_to_func_name,
            function_params,
            exit_block_id,
            debug,
        )?;
    }

    Ok(exit_block_id)
}

fn merge_function_blocks(
    builder: &mut crate::mir::builder::MirBuilder,
    func_name: &str,
    func: &crate::mir::MirFunction,
    remapper: &mut JoinIrIdRemapper,
    value_to_func_name: &HashMap<ValueId, String>,
    function_params: &HashMap<String, Vec<ValueId>>,
    exit_block_id: BasicBlockId,
    debug: bool,
) -> Result<(), String> {
    // DETERMINISM FIX: Sort blocks by ID
    let mut blocks: Vec<_> = func.blocks.iter().collect();
    blocks.sort_by_key(|(id, _)| id.0);

    for (old_block_id, old_block) in blocks {
        let new_block_id = remapper.get_block(func_name, *old_block_id)
            .ok_or_else(|| format!("Block ID not found: {:?}", old_block_id))?;

        let new_block = rewrite_block(
            old_block,
            remapper,
            value_to_func_name,
            function_params,
            exit_block_id,
            debug,
        )?;

        builder.current_function.as_mut()
            .ok_or("No current function")?
            .blocks.insert(new_block_id, new_block);
    }

    Ok(())
}

fn rewrite_block(
    old_block: &BasicBlock,
    remapper: &JoinIrIdRemapper,
    value_to_func_name: &HashMap<ValueId, String>,
    function_params: &HashMap<String, Vec<ValueId>>,
    exit_block_id: BasicBlockId,
    debug: bool,
) -> Result<BasicBlock, String> {
    let mut new_instructions = Vec::new();

    // Rewrite each instruction (lines 1102-1300)
    for inst in &old_block.instructions {
        let new_inst = rewrite_instruction(
            inst,
            remapper,
            value_to_func_name,
            function_params,
            exit_block_id,
            debug,
        )?;
        new_instructions.push(new_inst);
    }

    // Rewrite terminator (lines 1300-1400)
    let new_terminator = rewrite_terminator(
        &old_block.terminator,
        remapper,
        exit_block_id,
        debug,
    )?;

    Ok(BasicBlock {
        instructions: new_instructions,
        terminator: new_terminator,
    })
}

fn rewrite_instruction(
    inst: &MirInstruction,
    remapper: &JoinIrIdRemapper,
    value_to_func_name: &HashMap<ValueId, String>,
    function_params: &HashMap<String, Vec<ValueId>>,
    exit_block_id: BasicBlockId,
    debug: bool,
) -> Result<MirInstruction, String> {
    // Instruction rewriting logic (lines 1102-1300)
    // - Remap ValueIds
    // - Convert Call → Jump for intra-module calls
    // - Handle tail call optimization
    Ok(inst.clone()) // Placeholder
}

fn rewrite_terminator(
    terminator: &Option<MirInstruction>,
    remapper: &JoinIrIdRemapper,
    exit_block_id: BasicBlockId,
    debug: bool,
) -> Result<Option<MirInstruction>, String> {
    // Terminator rewriting logic (lines 1300-1400)
    // - Remap block IDs
    // - Convert Return → Jump to exit block
    Ok(terminator.clone()) // Placeholder
}
```

---

### 6. `merge/exit_phi_builder.rs` - Exit PHI Construction (~100 lines)

**Purpose**: Construct exit PHI node from return values.

```rust
//! JoinIR Exit PHI Builder
//!
//! Constructs the exit block PHI node that merges return values
//! from all inlined JoinIR functions.

use crate::mir::{BasicBlockId, ValueId, MirModule, MirInstruction, BasicBlock};
use super::id_remapper::JoinIrIdRemapper;

pub(super) fn build_exit_phi(
    builder: &mut crate::mir::builder::MirBuilder,
    mir_module: &MirModule,
    remapper: &JoinIrIdRemapper,
    exit_block_id: BasicBlockId,
    debug: bool,
) -> Result<Option<ValueId>, String> {
    if debug {
        eprintln!("[merge/exit_phi_builder] Building exit PHI");
    }

    // Collect return values from all functions (lines 1402-1450)
    let mut return_values = Vec::new();

    for (func_name, func) in &mir_module.functions {
        for (old_block_id, block) in &func.blocks {
            if let Some(MirInstruction::Return { value: Some(old_value) }) = &block.terminator {
                // Remap the return value
                let new_value = remapper.get_value(*old_value)
                    .ok_or_else(|| format!("Return value not remapped: {:?}", old_value))?;

                // Remap the block ID (predecessor of exit block)
                let new_block_id = remapper.get_block(func_name, *old_block_id)
                    .ok_or_else(|| format!("Block ID not remapped: {:?}", old_block_id))?;

                return_values.push((new_block_id, new_value));
            }
        }
    }

    if return_values.is_empty() {
        if debug {
            eprintln!("[merge/exit_phi_builder] No return values, creating void exit block");
        }

        // Create empty exit block
        builder.current_function.as_mut()
            .ok_or("No current function")?
            .blocks.insert(exit_block_id, BasicBlock {
                instructions: vec![],
                terminator: None,
            });

        return Ok(None);
    }

    // Create PHI node (lines 1450-1500)
    let phi_value = builder.value_gen.next();

    let phi_inst = MirInstruction::Phi {
        dst: phi_value,
        incoming: return_values.iter()
            .map(|(block_id, value_id)| (*block_id, *value_id))
            .collect(),
    };

    // Create exit block with PHI
    builder.current_function.as_mut()
        .ok_or("No current function")?
        .blocks.insert(exit_block_id, BasicBlock {
            instructions: vec![phi_inst],
            terminator: None,
        });

    if debug {
        eprintln!("[merge/exit_phi_builder] Exit PHI created: {:?}", phi_value);
    }

    Ok(Some(phi_value))
}
```

---

## Implementation Steps

### Step 1: Create Directory Structure (5 min)
```bash
mkdir -p src/mir/builder/control_flow/joinir/merge
```

### Step 2: Create Skeleton Files (10 min)
```bash
# Create empty files with module documentation
touch src/mir/builder/control_flow/joinir/merge/mod.rs
touch src/mir/builder/control_flow/joinir/merge/id_remapper.rs
touch src/mir/builder/control_flow/joinir/merge/block_allocator.rs
touch src/mir/builder/control_flow/joinir/merge/value_collector.rs
touch src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs
touch src/mir/builder/control_flow/joinir/merge/exit_phi_builder.rs
```

### Step 3: Extract id_remapper.rs (30 min)
- Copy `JoinIrIdRemapper` struct definition
- Move helper methods
- Add `remap_values()` function
- Test compilation: `cargo build --release`

### Step 4: Extract block_allocator.rs (30 min)
- Copy block allocation logic (lines 888-923)
- Create `allocate_blocks()` function
- Test compilation: `cargo build --release`

### Step 5: Extract value_collector.rs (30 min)
- Copy value collection logic (lines 931-971)
- Create `collect_values()` function
- Test compilation: `cargo build --release`

### Step 6: Extract instruction_rewriter.rs (1.5 hours)
- Copy instruction rewriting logic (lines 1102-1400)
- Create `merge_and_rewrite()` function
- Create helper functions for instruction/terminator rewriting
- Test compilation: `cargo build --release`

### Step 7: Extract exit_phi_builder.rs (30 min)
- Copy exit PHI logic (lines 1402-1500)
- Create `build_exit_phi()` function
- Test compilation: `cargo build --release`

### Step 8: Create mod.rs Coordinator (1 hour)
- Create coordinator function
- Wire up all sub-modules
- Move boundary reconnection logic (lines 1502-1578)
- Test compilation: `cargo build --release`

### Step 9: Update control_flow/mod.rs (15 min)
- Update imports to use new `merge` module
- Remove old `merge_joinir_mir_blocks()` implementation
- Test compilation: `cargo build --release`

### Step 10: Comprehensive Testing (1 hour)
```bash
# Build verification
cargo build --release
cargo test --lib

# Smoke tests
tools/smokes/v2/run.sh --profile quick

# Debug trace verification
NYASH_OPTION_C_DEBUG=1 ./target/release/nyash apps/tests/loop_min_while.hako 2>&1 | grep "merge"

# Determinism check (run 3 times)
for i in 1 2 3; do
  echo "=== Run $i ==="
  cargo test --release test_loop_patterns 2>&1 | grep "test result"
done
```

---

## Verification Checklist

- [ ] All 267+ tests pass
- [ ] Build time ≤ current (no regression)
- [ ] Debug traces still appear with `NYASH_OPTION_C_DEBUG=1`
- [ ] No compiler warnings
- [ ] No clippy warnings (`cargo clippy --all-targets`)
- [ ] Determinism test passes (3 consecutive runs produce identical results)
- [ ] Smoke tests pass for all patterns (1/2/3)

---

## Rollback Procedure

If Phase 4 fails:

```bash
# 1. Remove new directory
rm -rf src/mir/builder/control_flow/joinir/merge

# 2. Restore original control_flow.rs
git checkout src/mir/builder/control_flow.rs

# 3. Verify build
cargo build --release
cargo test --lib

# 4. Document failure
echo "$(date): Phase 4 rollback due to: $REASON" >> docs/development/refactoring/rollback_log.txt
```

---

## Common Pitfalls

### Pitfall 1: HashMap Non-Determinism
**Problem**: Using `HashMap` for iteration causes non-deterministic ValueId allocation.
**Solution**: Use `BTreeMap` and `BTreeSet` for all ID mappings.

### Pitfall 2: Missing Value Remapping
**Problem**: Forgetting to remap a ValueId causes "value not found" errors.
**Solution**: Systematically collect ALL values before remapping phase.

### Pitfall 3: Block Order Matters
**Problem**: Processing blocks in random order causes test failures.
**Solution**: Always sort by name/ID before iteration.

### Pitfall 4: Forgetting to Update Imports
**Problem**: Old imports cause compilation failures.
**Solution**: Update all imports in a single commit after extraction.

---

## Success Criteria

- ✅ 714-line function → 6 focused modules (100-150 lines each)
- ✅ All tests pass (no regressions)
- ✅ Debug traces work (`NYASH_OPTION_C_DEBUG=1`)
- ✅ Determinism maintained (BTreeMap/BTreeSet usage)
- ✅ Code is easier to understand (each phase isolated)
- ✅ Future maintenance easier (modify one phase at a time)

---

## Timeline

- **Total Estimated Effort**: 6 hours
- **Buffer**: +2 hours for unexpected issues
- **Recommended Schedule**: 2 days (3 hours/day)

**Day 1**:
- Steps 1-5 (directory setup, id_remapper, block_allocator, value_collector)
- Verification after each step

**Day 2**:
- Steps 6-10 (instruction_rewriter, exit_phi_builder, coordinator, testing)
- Comprehensive verification

---

## Conclusion

Phase 4 is the most critical modularization task. By breaking down the 714-line monster function into 6 focused modules, we:

1. **Improve maintainability** - Each phase in its own file
2. **Reduce complexity** - 714 lines → 100-150 lines per file
3. **Enable future development** - Easy to modify individual phases
4. **Maintain stability** - Zero breaking changes, backward compatible

**Next Steps**:
1. Review this guide
2. Set aside 2 days for implementation
3. Execute steps 1-10 systematically
4. Run comprehensive verification
5. Commit success or rollback if needed

**Questions?** Refer to the full implementation plan or open an issue.

---

**Document Version**: 1.0
**Created**: 2025-12-05
**Author**: Claude Code (AI-assisted planning)
**Status**: Ready for execution
