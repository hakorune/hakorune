# Phase 33-16: Visual Flow Diagram & Code Map

## Architecture Diagram: Loop Header PHI as SSOT

```
┌────────────────────────────────────────────────────────────────────────┐
│                   merge_joinir_mir_blocks() Pipeline                    │
│                     (7 phases after Phase 33-16)                        │
└────────────────────────────────────────────────────────────────────────┘

Phase 1: allocate_blocks()
├── Input: JoinIR mir_module.functions
└── Output: remapper with block ID mappings

Phase 2: collect_values()
├── Input: JoinIR blocks
└── Output: used_values set

Phase 3: remap_values()
├── Input: used_values set, remapper
├── Action: Allocate new ValueIds for all used values
└── Output: remapper with BOTH block AND value mappings

    ⚡ VALUE ID BOUNDARY: JoinIR (local 0,1,2...) → Host (100,101,102...)

┌──────────────────────────────────────────────────────────────────────────┐
│ Phase 3.5: LoopHeaderPhiBuilder::build() ⭐ NEW in Phase 33-16          │
│                                                                            │
│ Input:                                                                    │
│   - boundary.loop_var_name: "i"                                          │
│   - remapper.get_value(ValueId(0)): ValueId(100)  ← Init value (host)   │
│   - remapper.get_block(entry_func, entry_block): BasicBlockId(10)       │
│                                                                            │
│ Action:                                                                   │
│   - Allocate phi_dst = ValueId(101)                                     │
│   - Create CarrierPhiEntry:                                              │
│     { phi_dst: ValueId(101),                                             │
│       entry_incoming: (BasicBlockId(10), ValueId(100)),                 │
│       latch_incoming: None }  ← Set in Phase 4!                         │
│                                                                            │
│ Output:                                                                   │
│   - loop_header_phi_info with empty latch_incoming                      │
│   - PASS TO: instruction_rewriter as mutable reference                  │
│                                                                            │
│ Key: PHI dst (ValueId(101)) is NOW ALLOCATED and KNOWN before we        │
│      process instructions. instruction_rewriter will use it!            │
└──────────────────────────────────────────────────────────────────────────┘

Phase 4: merge_and_rewrite()
├── Input: loop_header_phi_info (mutable!)
├
│ Subphase 4a: Process instructions in each block
│ ├── Copy instructions: Use remapped ValueIds
│ └── Other instructions: Standard remapping
│
│ Subphase 4b: Process terminators
│ ├── Tail call (Jump to loop header):
│ │   ├── args = [ValueId(102)]  ← i_next (updated loop variable)
│ │   ├── Call: loop_header_phi_info.set_latch_incoming(
│ │   │          "i",
│ │   │          target_block,    ← Loop header
│ │   │          ValueId(102))    ← Updated value
│ │   └── Emit parameter bindings + Jump
│ │
│ └── Return { value }:
│     ├── OLD (Phase 33-15): Skip exit_phi_inputs
│     │
│     └── NEW (Phase 33-16):
│         ├── Get phi_dst = loop_header_phi_info.get_carrier_phi("i")
│         │                = ValueId(101)  ← PHI output, not parameter!
│         └── Collect: exit_phi_inputs.push((exit_block, ValueId(101)))
│
└── Output: MergeResult with exit_phi_inputs using PHI dsts

    ⚡ KEY MOMENT: loop_header_phi_info.latch_incoming NOW SET!

┌──────────────────────────────────────────────────────────────────────────┐
│ Phase 5: exit_phi_builder::build_exit_phi()                              │
│                                                                            │
│ Input:                                                                    │
│   - exit_phi_inputs: [(exit_block, ValueId(101))]                       │
│   - carrier_inputs: {} (empty in Phase 33-16 minimal)                   │
│                                                                            │
│ Action:                                                                   │
│   - Create exit block                                                    │
│   - If exit_phi_inputs not empty:                                       │
│     { Create PHI: exit_phi_dst = PHI [(exit_block, ValueId(101))]      │
│   - For each carrier: Create carrier PHI                                │
│                                                                            │
│ Output:                                                                   │
│   - exit_phi_result_id: Some(ValueId(103))  ← Exit block's PHI dst     │
│   - carrier_phis: { "i" → ValueId(101) }    ← Header PHI dsts!        │
│                                                                            │
│ Key: carrier_phis now contains header PHI dsts, NOT remapped parameters!│
└──────────────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────────────┐
│ Phase 4.5: LoopHeaderPhiBuilder::finalize() ⭐ NEW in Phase 33-16       │
│                                                                            │
│ Input: loop_header_phi_info with latch_incoming NOW SET                 │
│                                                                            │
│ Action:                                                                   │
│   - Validate all latch_incoming are set                                 │
│   - For each carrier:                                                    │
│     { entry_incoming = (entry_block, ValueId(100)),                    │
│       latch_incoming = (header_block, ValueId(102))  ← From Phase 4!   │
│       Emit PHI: ValueId(101) = PHI [(entry_block, ValueId(100)),       │
│                                      (header_block, ValueId(102))]     │
│   - Prepend PHI instructions to header block                           │
│                                                                            │
│ Output:                                                                   │
│   - Header block now contains:                                          │
│     [PHI instructions...], [original instructions...]                   │
│                                                                            │
│ Key: PHI is now EMITTED into the MIR! SSA definition complete!         │
└──────────────────────────────────────────────────────────────────────────┘

Phase 6: ExitLineOrchestrator::execute()
├── Input: carrier_phis = { "i" → ValueId(101) }
├── Action: Call ExitLineReconnector
│   └── For each exit_binding:
│       ├── Look up carrier PHI: carrier_phis["i"] = ValueId(101)
│       └── Update: variable_map["i"] = ValueId(101)  ← PHI dst!
└── Output: Updated variable_map with PHI dsts
```

---

## Code Change Map

### Files to Modify (6 locations)

```
src/mir/builder/control_flow/joinir/merge/
├── mod.rs (MODIFY)
│   └── Between line 107 (after remap_values)
│       and line 110 (before instruction_rewriter)
│       ✅ ADD: Phase 3.5 - Build loop header PHIs
│       ✅ ADD: Phase 4.5 - Finalize loop header PHIs (after exit_phi_builder)
│
├── instruction_rewriter.rs (MODIFY 3 places)
│   ├── Line 29-37: Update fn signature
│   │   ✅ ADD: loop_header_phi_info parameter
│   │
│   ├── ~Line 300 (in tail call section): Track latch incoming
│   │   ✅ ADD: loop_header_phi_info.set_latch_incoming(...)
│   │
│   └── Lines 354-431 (Return processing): Use PHI dsts
│       ✅ MODIFY: Replace skip logic with PHI dst usage
│       ✅ MODIFY: Replace carrier skip with PHI dst usage
│
└── loop_header_phi_builder.rs (ALREADY EXISTS)
    ├── ::build() ✅ Ready to use
    └── ::finalize() ✅ Ready to use
```

### Optional: Pattern Lowerer Update

```
src/mir/builder/control_flow/joinir/patterns/
└── pattern2_with_break.rs
    ├── Line 200: ✅ Already sets loop_var_name
    └── Line 200+: OPTIONAL - Extract other carriers from exit_bindings
        (For Phase 33-16+, not required for minimal)
```

---

## Concrete Code Changes (Copy-Paste Ready)

### Change 1: Add mod.rs imports

**Location**: Top of `src/mir/builder/control_flow/joinir/merge/mod.rs`

```rust
// Already present:
mod instruction_rewriter;
mod exit_phi_builder;
pub mod exit_line;
pub mod loop_header_phi_builder;  // ✅ Already declared!

// Import the types
use loop_header_phi_builder::{LoopHeaderPhiBuilder, LoopHeaderPhiInfo};
```

### Change 2: Add Phase 3.5 after remap_values

**Location**: `mod.rs`, line 107+ (after `remap_values(...)`)

```rust
    // Phase 3: Remap ValueIds
    remap_values(builder, &used_values, &mut remapper, debug)?;

    // ===== Phase 3.5: Build loop header PHIs =====
    let mut loop_header_phi_info = if let Some(boundary) = boundary {
        if let Some(loop_var_name) = &boundary.loop_var_name {
            // Get entry function and entry block
            let (entry_func_name, entry_func) = mir_module
                .functions
                .iter()
                .next()
                .ok_or("JoinIR module has no functions")?;
            
            let entry_block_id = remapper
                .get_block(entry_func_name, entry_func.entry_block)
                .ok_or_else(|| format!("Entry block not found"))?;
            
            // Get loop variable's initial value (remapped)
            let loop_var_init = remapper
                .get_value(ValueId(0))
                .ok_or("Loop var init not remapped")?;
            
            if debug {
                eprintln!(
                    "[cf_loop/joinir] Phase 3.5: Building header PHIs for loop_var '{}'",
                    loop_var_name
                );
            }
            
            // Build header PHIs (allocates PHI dsts, doesn't emit yet)
            LoopHeaderPhiBuilder::build(
                builder,
                entry_block_id,     // header_block_id
                entry_block_id,     // entry_block_id
                loop_var_name,
                loop_var_init,
                &[],                // No other carriers yet
                boundary.expr_result.is_some(),
                debug,
            )?
        } else {
            LoopHeaderPhiInfo::empty(BasicBlockId(0))
        }
    } else {
        LoopHeaderPhiInfo::empty(BasicBlockId(0))
    };

    // Phase 4: Merge blocks and rewrite instructions
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

### Change 3: Update instruction_rewriter::merge_and_rewrite signature

**Location**: `instruction_rewriter.rs`, line 29-37

```rust
pub(super) fn merge_and_rewrite(
    builder: &mut crate::mir::builder::MirBuilder,
    mir_module: &MirModule,
    remapper: &mut JoinIrIdRemapper,
    value_to_func_name: &HashMap<ValueId, String>,
    function_params: &HashMap<String, Vec<ValueId>>,
    boundary: Option<&JoinInlineBoundary>,
    loop_header_phi_info: &mut super::loop_header_phi_builder::LoopHeaderPhiInfo,  // NEW
    debug: bool,
) -> Result<MergeResult, String> {
    // ... rest of function unchanged ...
}
```

### Change 4: Add latch tracking in tail call section

**Location**: `instruction_rewriter.rs`, after line ~319 (after param bindings)

```rust
            // Second pass: Insert parameter bindings for tail calls
            // Phase 188-Impl-3: Use actual parameter ValueIds from target function
            if let Some((target_block, args)) = tail_call_target {
                if debug {
                    eprintln!(
                        "[cf_loop/joinir]   Inserting param bindings for tail call to {:?}",
                        target_block
                    );
                }

                // ... existing param binding code (unchanged) ...

                // ===== NEW Phase 33-16: Track latch incoming =====
                if let Some(loop_var_name) = &boundary.and_then(|b| b.loop_var_name.as_ref()) {
                    if !args.is_empty() {
                        let latch_value = args[0];  // Updated loop variable
                        loop_header_phi_info.set_latch_incoming(
                            loop_var_name,
                            target_block,   // Loop header block
                            latch_value,    // i_next value
                        );
                        
                        if debug {
                            eprintln!(
                                "[cf_loop/joinir] Phase 33-16: Set latch incoming for '{}': {:?}",
                                loop_var_name, latch_value
                            );
                        }
                    }
                }

                // Set terminator to Jump
                new_block.terminator = Some(MirInstruction::Jump {
                    target: target_block,
                });
```

### Change 5: Replace exit_phi_inputs skip logic

**Location**: `instruction_rewriter.rs`, lines 354-398 (replace entire block)

```rust
                        MirInstruction::Return { value } => {
                            // Phase 33-16: Use header PHI dst instead of undefined parameters
                            if let Some(ret_val) = value {
                                let remapped_val = remapper.get_value(*ret_val).unwrap_or(*ret_val);
                                
                                // Try to use header PHI dst (SSA-correct)
                                if let Some(loop_var_name) = &boundary.and_then(|b| b.loop_var_name.as_ref()) {
                                    if let Some(phi_dst) = loop_header_phi_info.get_carrier_phi(loop_var_name) {
                                        if debug {
                                            eprintln!(
                                                "[cf_loop/joinir] Phase 33-16: Using loop header PHI {:?} for exit value",
                                                phi_dst
                                            );
                                        }
                                        exit_phi_inputs.push((exit_block_id, phi_dst));
                                    } else {
                                        // Fallback: use parameter (backward compat)
                                        if debug {
                                            eprintln!(
                                                "[cf_loop/joinir] Phase 33-16: No header PHI, fallback to parameter {:?}",
                                                remapped_val
                                            );
                                        }
                                        exit_phi_inputs.push((exit_block_id, remapped_val));
                                    }
                                } else {
                                    // No loop_var_name: use parameter
                                    if debug {
                                        eprintln!(
                                            "[cf_loop/joinir] Phase 33-16: No loop_var_name, using parameter {:?}",
                                            remapped_val
                                        );
                                    }
                                    exit_phi_inputs.push((exit_block_id, remapped_val));
                                }
                            }
                            
                            MirInstruction::Jump {
                                target: exit_block_id,
                            }
                        }
```

### Change 6: Replace carrier_inputs skip logic

**Location**: `instruction_rewriter.rs`, lines 400-431 (replace entire block)

```rust
                            // Phase 33-13/16: Collect carrier exit values using header PHI dsts
                            if let Some(boundary) = boundary {
                                for binding in &boundary.exit_bindings {
                                    // Try to use header PHI dst
                                    if let Some(phi_dst) = loop_header_phi_info.get_carrier_phi(&binding.carrier_name) {
                                        if debug {
                                            eprintln!(
                                                "[cf_loop/joinir] Phase 33-16: Carrier '{}' using header PHI {:?}",
                                                binding.carrier_name, phi_dst
                                            );
                                        }
                                        carrier_inputs.entry(binding.carrier_name.clone())
                                            .or_insert_with(Vec::new)
                                            .push((exit_block_id, phi_dst));
                                    } else if debug {
                                        eprintln!(
                                            "[cf_loop/joinir] Phase 33-16: No header PHI for carrier '{}', skipping",
                                            binding.carrier_name
                                        );
                                    }
                                }
                            }
```

### Change 7: Add Phase 4.5 finalize call

**Location**: `mod.rs`, after Phase 5 (exit_phi_builder call)

```rust
    // Phase 5: Build exit PHI (expr result and carrier PHIs)
    let (exit_phi_result_id, carrier_phis) = exit_phi_builder::build_exit_phi(
        builder,
        merge_result.exit_block_id,
        &merge_result.exit_phi_inputs,
        &merge_result.carrier_inputs,
        debug,
    )?;

    // ===== Phase 4.5: Finalize loop header PHIs =====
    LoopHeaderPhiBuilder::finalize(builder, &loop_header_phi_info, debug)?;

    // Phase 6: Reconnect boundary (if specified)
    // Phase 197-B: Pass remapper to enable per-carrier exit value lookup
    // Phase 33-10-Refactor-P3: Delegate to ExitLineOrchestrator
    // Phase 33-13: Pass carrier_phis for proper variable_map update
    if let Some(boundary) = boundary {
        exit_line::ExitLineOrchestrator::execute(builder, boundary, &carrier_phis, debug)?;
    }
```

---

## Complete Flow Checklist

1. ✅ **Phase 3** (remap_values): ValueIds remapped
2. ✅ **Phase 3.5** (build): 
   - Header PHI dsts allocated
   - Entry incoming set
   - Passed to Phase 4
3. ✅ **Phase 4** (instruction_rewriter):
   - Latch incoming set when processing tail calls
   - Exit values use header PHI dsts (not parameters)
   - Carrier exit values use header PHI dsts
4. ✅ **Phase 4.5** (finalize):
   - PHI instructions emitted into header block
   - Validation that all latch incoming are set
5. ✅ **Phase 5** (exit_phi_builder):
   - Exit PHI created from exit_phi_inputs
   - carrier_phis returned with PHI dsts
6. ✅ **Phase 6** (ExitLineOrchestrator):
   - variable_map updated with header PHI dsts

---

## Testing Commands

```bash
# Build
cargo build --release 2>&1 | head -50

# Test with debug output
NYASH_JOINIR_DEBUG=1 ./target/release/nyash --dump-mir \
  apps/tests/joinir_min_loop.hako 2>&1 | grep "Phase 33-16"

# Check MIR structure
./target/release/nyash --emit-mir-json mir.json apps/tests/joinir_min_loop.hako
jq '.functions[0].blocks[0].instructions[0:3]' mir.json  # First 3 instructions (should be PHIs)
```

---

## Summary Table

| Phase | Component | Input | Output | Size |
|-------|-----------|-------|--------|------|
| 3 | remap_values | ValueId(0-10) | ValueId(100-110) | existing |
| 3.5 | build | ValueId(100) | phi_dst=101 | 50 lines new |
| 4 | merge_and_rewrite | loop_header_phi_info | latch_incoming set | +20 lines |
| 4.5 | finalize | latch_incoming set | PHI emitted | 30 lines new |
| 5 | build_exit_phi | exit_phi_inputs | carrier_phis | existing |
| 6 | ExitLineOrchestrator | carrier_phis | var_map updated | existing |

**Total changes**: ~6 locations, ~100 lines added/modified, 0 lines removed (backward compat)
Status: Historical
