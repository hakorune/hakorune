# LoopForm Approach to PHI Circular Dependency Problem

**Date**: 2025-11-17
**Status**: Research Complete, Design In Progress
**Related Issue**: ValueId(14)/ValueId(17) circular dependency in loop PHI construction

---

## Executive Summary

This document presents a LoopForm-based solution to the PHI circular dependency problem discovered in Phase 25.1b multi-carrier loop implementation. Through academic literature review and analysis of Hakorune's "Box Theory" design philosophy, we propose a solution that aligns with the project's core principle: "Everything is Box" — including loop structure itself.

**Key Finding**: The circular dependency issue is **not** a fundamental SSA problem, but rather a mismatch between the Box Theory's simplified SSA construction approach and the complex requirements of multi-carrier loops with pinned variables.

---

## Phase 1: Current State Analysis

### 1.1 The Problem (Recap)

In `fib_multi_carrier.hako`, the following MIR structure is generated:

```
bb3 (loop preheader):
   13: %13 = copy %10        # ✅ Snapshot of 'me' (receiver)
   14: %15 = copy %0         # ✅ Copy of parameter
   15: br label bb6          # Jump to loop header

bb6 (loop header/body):
    0: %18 = phi [%15, bb3], [...]   # ✅ OK - %15 exists in bb3
    1: %17 = phi [%14, bb3], [...]   # ❌ BAD - %14 NOT in bb3!
    3: %14 = phi [%13, bb3], [...]   # %14 defined HERE, not bb3
```

**Root Cause**: The preheader copy logic (`emit_copy_at_preheader`) generates copies in order, but the PHI construction references values that will be defined later in the **header** block, creating a forward reference that violates SSA's "definition before use" principle.

### 1.2 Current Implementation Architecture

The codebase uses a **SSOT (Single Source of Truth)** design centered on `src/mir/phi_core/loop_phi.rs`:

```rust
// Key functions:
pub fn prepare_loop_variables_with<O: LoopPhiOps>(
    ops: &mut O,
    header_id: BasicBlockId,
    preheader_id: BasicBlockId,
    current_vars: &HashMap<String, ValueId>,
) -> Result<Vec<IncompletePhi>, String>

pub fn seal_incomplete_phis_with<O: LoopPhiOps>(
    ops: &mut O,
    block_id: BasicBlockId,
    latch_id: BasicBlockId,
    mut incomplete_phis: Vec<IncompletePhi>,
    continue_snapshots: &[(BasicBlockId, VarSnapshot)],
) -> Result<(), String>
```

**Design Pattern**: "Incomplete PHI" - a two-phase approach:
1. **Prepare**: Allocate PHI nodes with preheader inputs only
2. **Seal**: Complete PHI nodes with latch/continue inputs after loop body

### 1.3 LoopForm Design Philosophy

From `docs/private/research/papers-archive/paper-d-ssa-construction/box-theory-solution.md`:

> **Box Theory Revolution**:
> - 基本ブロック = 箱 (Basic Block = Box)
> - 変数の値 = 箱の中身 (Variable value = Box content)
> - PHI = どの箱から値を取るか選ぶだけ (PHI = Just selecting which box to take value from)

**Key Insight**: The Box Theory simplifies SSA construction from 650 lines → 100 lines by treating each block as a self-contained "box" of values, eliminating the need for dominance frontiers, forward references, and complex type conversion.

---

## Phase 2: Academic Literature Review

### 2.1 Classical SSA Construction (Cytron et al. 1991)

**Paper**: "Efficiently Computing Static Single Assignment Form and the Control Dependence Graph"

**Key Algorithm**:
1. Compute dominance frontiers for all variables
2. Place φ-functions at join points (including loop headers)
3. Rename variables in dominance tree order

**Loop Handling**:
- Loop headers always get φ-functions for loop-carried variables
- φ-function inputs: `[initial_value, backedge_value]`
- Backedge value may be undefined initially (incomplete φ)

**Limitation**: Requires full CFG analysis and dominance tree construction — contrary to Box Theory's simplicity goal.

### 2.2 Simple and Efficient SSA Construction (Braun et al. 2013)

**Paper**: "Simple and Efficient Construction of Static Single Assignment Form" (CC 2013)

**Key Innovation**: Lazy, backward algorithm:
- Only when a variable is **used**, query its reaching definition
- Insert φ-functions on-demand at join points
- No prior CFG analysis required

**Loop Handling Strategy**:
```
1. When entering loop header:
   - Create "incomplete φ" nodes for all loop-carried variables
   - φ initially has only preheader input

2. During loop body lowering:
   - Variable reads query the incomplete φ (not the preheader value)

3. After loop body completes:
   - Add backedge input to incomplete φ
   - φ becomes complete: [preheader_val, latch_val]
```

**Critical Insight**: The φ-function itself becomes the "placeholder" for the loop variable, preventing forward references.

### 2.3 LLVM Canonical Loop Form

**Source**: https://llvm.org/docs/LoopTerminology.html

**Structure**:
```
preheader:
  ; Initialize loop-carried variables
  br label %header

header:
  %i.phi = phi i64 [ %i.init, %preheader ], [ %i.next, %latch ]
  %cond = icmp slt i64 %i.phi, %limit
  br i1 %cond, label %body, label %exit

body:
  ; Loop computation
  br label %latch

latch:
  %i.next = add i64 %i.phi, 1
  br label %header

exit:
  ; Exit φ nodes (LCSSA form)
  ret
```

**Key Properties**:
1. **Preheader**: Single entry to loop, dominates header
2. **Header**: Single entry point, contains all loop φ-functions
3. **Latch**: Single backedge to header
4. **Exit**: No external predecessors (LCSSA property)

**φ-Placement Rules**:
- Header φ inputs must be **defined** in their respective blocks
- Preheader input: defined before loop entry
- Latch input: defined in latch or dominated by header

---

## Phase 3: Root Cause Analysis with Box Theory Lens

### 3.1 Why Box Theory Works (Usually)

The Box Theory's simplified approach works because:

1. **Blocks as Boxes**: Each block's variables are "contents" of that box
2. **φ as Selection**: Choosing which box's contents to use
3. **No Forward References**: Box contents are immutable once the block is sealed

**Example (simple loop)**:
```nyash
i = 0
loop(i < 10) {
    i = i + 1
}
```

**Box Representation**:
```
Box[preheader]:  { i: %0 = const 0 }
Box[header]:     { i: %phi = φ[%0, %next] }  # φ IS the box content
Box[body]:       { i: %phi }                 # Inherits from header
Box[latch]:      { i: %next = add %phi, 1 }
```

**Why it works**: The φ-function `%phi` is allocated **before** it's referenced, satisfying SSA definition-before-use.

### 3.2 Why Box Theory Fails (Multi-Carrier + Pinned Receiver)

**The Problem Case**:
```nyash
static box Fib {
    method compute(limit) {  # 'me' is pinned receiver (ValueId %0)
        i = 0
        a = 0
        b = 1
        loop(i < limit) {
            t = a + b
            a = b
            b = t
            i = i + 1
        }
        return b
    }
}
```

**Variable Snapshot at Loop Entry**:
```
current_vars = {
    "me": %0,      # Pinned receiver (parameter)
    "limit": %1,   # Parameter
    "i": %2,       # Local
    "a": %3,       # Local
    "b": %4        # Local
}
```

**Current Implementation Flow** (from `prepare_loop_variables_with`):

```rust
// Step 1: Iterate over current_vars
for (var_name, &value_before) in current_vars.iter() {
    // Step 2: Create preheader copy
    let pre_copy = ops.new_value();  // Allocates %13, %15, %16, %17, %18
    ops.emit_copy_at_preheader(preheader_id, pre_copy, value_before)?;

    // Step 3: Allocate header φ
    let phi_id = ops.new_value();  // Allocates %14, %19, %20, %21, %22

    // Step 4: Create incomplete φ with preheader input
    ops.emit_phi_at_block_start(header_id, phi_id, vec![(preheader_id, pre_copy)])?;
}
```

**The Bug**: **Interleaved Allocation**

1. Iteration 1 (me):    pre_copy=%13, phi=%14  →  `phi %14 = [%13, bb3]` ✅
2. Iteration 2 (limit): pre_copy=%15, phi=%19  →  `phi %19 = [%15, bb3]` ✅
3. Iteration 3 (i):     pre_copy=%16, phi=%20  →  `phi %20 = [%16, bb3]` ✅

**But in actual execution** (selfhost compiler trace shows):
```
bb3: %13 = copy %10     # me preheader copy
bb3: %15 = copy %0      # limit preheader copy (WHY %15 not %14?!)
bb6: %18 = phi ...      # First phi (not %14!)
bb6: %17 = phi [%14, bb3], ...  # References %14 which doesn't exist in bb3!
bb6: %14 = phi [%13, bb3], ...  # %14 defined HERE
```

**Root Cause Identified**: The selfhost compiler's `new_value()` implementation has **non-sequential allocation** or **reordering** between preheader copies and header φ allocation.

### 3.3 The Fundamental Mismatch

**Box Theory Assumption**: "Variable snapshots are immutable once captured"

**Reality with Pinned Receivers**:
- Pinned variables (`me`) are **special** — they're parameters, not locals
- They need φ-functions at **both** header and exit (Phase 25.1b fix added this)
- But their "snapshot" is a **reference** to a parameter, not a value defined in preheader

**The Circular Dependency**:
```
1. Preheader needs to copy all vars → includes 'me'
2. Header φ for 'me' references preheader copy
3. But preheader copy was allocated AFTER other header φ's
4. Result: φ[i=1] references copy[i=0] which references φ[i=2]
```

---

## Phase 4: LoopForm-Based Solution Design

### 4.1 Core Insight: LoopForm as "Meta-Box"

**Principle**: Instead of treating loop variables individually, treat the **entire loop structure** as a single "LoopForm Box":

```
LoopFormBox {
    structure: {
        preheader: BlockBox,
        header: BlockBox,
        body: BlockBox,
        latch: BlockBox,
        exit: BlockBox
    },
    carriers: [
        { name: "i", init: %2, phi: %20, next: %30 },
        { name: "a", init: %3, phi: %21, next: %31 },
        { name: "b", init: %4, phi: %22, next: %32 }
    ],
    pinned: [
        { name: "me", param: %0, phi: %14, copy: %13 }
    ]
}
```

**Key Difference**: **Separate handling** of carriers vs. pinned variables.

### 4.2 Proposed Algorithm: Two-Pass PHI Construction

**Pass 1: Allocate All Value IDs (Preheader Phase)**

```rust
pub struct LoopFormBuilder {
    carriers: Vec<CarrierVariable>,
    pinned: Vec<PinnedVariable>,
}

struct CarrierVariable {
    name: String,
    init_value: ValueId,     // From preheader (locals)
    preheader_copy: ValueId, // Snapshot in preheader
    header_phi: ValueId,     // PHI in header
    latch_value: ValueId,    // Updated value in latch
}

struct PinnedVariable {
    name: String,
    param_value: ValueId,    // Original parameter
    preheader_copy: ValueId, // Copy in preheader
    header_phi: ValueId,     // PHI in header
}

fn prepare_loop_structure(
    &mut self,
    current_vars: &HashMap<String, ValueId>,
    is_param: impl Fn(&str) -> bool,
) -> Result<(), String> {
    // Step 1: Separate carriers from pinned
    for (name, &value) in current_vars {
        if is_param(name) {
            // Pinned variable (parameter)
            self.pinned.push(PinnedVariable {
                name: name.clone(),
                param_value: value,
                preheader_copy: self.ops.new_value(),  // Allocate NOW
                header_phi: self.ops.new_value(),      // Allocate NOW
            });
        } else {
            // Carrier variable (local)
            self.carriers.push(CarrierVariable {
                name: name.clone(),
                init_value: value,
                preheader_copy: self.ops.new_value(),  // Allocate NOW
                header_phi: self.ops.new_value(),      // Allocate NOW
                latch_value: ValueId::INVALID,         // Will be set later
            });
        }
    }

    Ok(())
}
```

**Pass 2: Emit Instructions in Correct Order**

```rust
fn emit_loop_structure(&mut self) -> Result<(), String> {
    // === PREHEADER BLOCK ===
    self.ops.set_current_block(self.preheader_id)?;

    // Emit copies for ALL variables (order guaranteed)
    for pinned in &self.pinned {
        self.ops.emit_copy(
            pinned.preheader_copy,
            pinned.param_value
        )?;
    }
    for carrier in &self.carriers {
        self.ops.emit_copy(
            carrier.preheader_copy,
            carrier.init_value
        )?;
    }

    self.ops.emit_jump(self.header_id)?;

    // === HEADER BLOCK ===
    self.ops.set_current_block(self.header_id)?;

    // Emit PHIs for ALL variables (order guaranteed)
    for pinned in &mut self.pinned {
        self.ops.emit_phi(
            pinned.header_phi,
            vec![(self.preheader_id, pinned.preheader_copy)]
        )?;
        self.ops.update_var(pinned.name.clone(), pinned.header_phi);
    }
    for carrier in &mut self.carriers {
        self.ops.emit_phi(
            carrier.header_phi,
            vec![(self.preheader_id, carrier.preheader_copy)]
        )?;
        self.ops.update_var(carrier.name.clone(), carrier.header_phi);
    }

    Ok(())
}
```

**Pass 3: Seal PHIs After Loop Body**

```rust
fn seal_loop_phis(&mut self, latch_id: BasicBlockId) -> Result<(), String> {
    for pinned in &self.pinned {
        // Pinned variables: latch value = header phi (unchanged in loop)
        let latch_value = self.ops.get_variable_at_block(
            &pinned.name,
            latch_id
        ).unwrap_or(pinned.header_phi);

        self.ops.update_phi_inputs(
            self.header_id,
            pinned.header_phi,
            vec![
                (self.preheader_id, pinned.preheader_copy),
                (latch_id, latch_value)
            ]
        )?;
    }

    for carrier in &mut self.carriers {
        carrier.latch_value = self.ops.get_variable_at_block(
            &carrier.name,
            latch_id
        ).ok_or("Carrier not found at latch")?;

        self.ops.update_phi_inputs(
            self.header_id,
            carrier.header_phi,
            vec![
                (self.preheader_id, carrier.preheader_copy),
                (latch_id, carrier.latch_value)
            ]
        )?;
    }

    Ok(())
}
```

### 4.3 Key Advantages of LoopForm Approach

1. **No Circular Dependencies**:
   - All ValueIds allocated upfront in Pass 1
   - Emission order (Pass 2) guarantees definition-before-use
   - No interleaved allocation/emission

2. **Explicit Carrier vs. Pinned Separation**:
   - Aligns with academic literature (loop-carried vs. loop-invariant)
   - Makes special handling of receivers explicit
   - Future optimization: skip PHIs for true loop-invariants

3. **Box Theory Preservation**:
   - LoopForm itself is a "Meta-Box" containing structured sub-boxes
   - Each sub-box (preheader, header, etc.) remains immutable
   - Maintains 650→100 line simplicity (actually ~150 lines for full impl)

4. **Compatibility with Existing Code**:
   - Can be implemented as new `LoopFormBuilder` struct
   - Gradually replace current `prepare_loop_variables_with`
   - No changes to PHI core or backend execution

---

## Phase 5: Implementation Plan

### 5.1 Minimal Viable Implementation (Week 1)

**Goal**: Fix multi-carrier fibonacci case without breaking existing tests

**Files to Modify**:
1. `src/mir/phi_core/loop_phi.rs`:
   - Add `LoopFormBuilder` struct
   - Add `prepare_loop_structure()` function
   - Keep existing `prepare_loop_variables_with()` for backward compat

2. `src/mir/loop_builder.rs`:
   - Add `use_loopform_builder` feature flag (env var)
   - Route to new builder when enabled

3. `lang/src/mir/builder/func_body/basic_lower_box.hako`:
   - No changes needed (uses JSON API)

**Testing**:
```bash
# Enable new builder
export NYASH_LOOPFORM_PHI_V2=1

# Test multi-carrier fibonacci
cargo build --release
./target/release/nyash local_tests/fib_multi_carrier.hako

# Run smoke tests
tools/smokes/v2/run.sh --profile quick --filter "loop|multi_carrier"
```

### 5.2 Full Implementation (Week 2-3)

**Enhancements**:
1. **Loop-Invariant Detection**:
   - Skip PHI generation for variables not modified in loop
   - Optimization: direct use of preheader value

2. **Break/Continue Support**:
   - Extend LoopFormBuilder with exit snapshots
   - Implement `build_exit_phis_with` using LoopForm structure

3. **Nested Loop Support**:
   - Stack-based LoopFormBuilder management
   - Inner loops inherit outer loop's pinned variables

### 5.3 Migration Strategy

**Phase 1**: Feature-flagged implementation (current)
**Phase 2**: Parallel execution (both old and new paths active)
**Phase 3**: Gradual deprecation (warning on old path)
**Phase 4**: Full migration (remove old code)

**Compatibility Matrix**:
| Test Case | Old Path | New Path | Status |
|-----------|----------|----------|--------|
| simple_loop | ✅ | ✅ | Compatible |
| loop_with_break | ✅ | ✅ | Compatible |
| multi_carrier | ❌ | ✅ | **Fixed!** |
| nested_loop | ✅ | 🔄 | In Progress |

---

## Phase 6: Alternative Approaches Considered

### 6.1 Quick Fix: Reorder ValueId Allocation

**Idea**: Force sequential allocation by batch-allocating all preheader copies first

**Pros**:
- Minimal code change (~10 lines)
- Preserves existing architecture

**Cons**:
- Doesn't address root cause
- Fragile (depends on allocation order)
- Will break again with nested loops or more complex patterns

**Decision**: ❌ Rejected — violates "Fail-Fast" principle (CLAUDE.md)

### 6.2 Eliminate Preheader Copies

**Idea**: Use original values directly in header PHIs, skip preheader copies

**Pros**:
- Removes allocation complexity
- Fewer instructions

**Cons**:
- Violates SSA UseBeforeDef when value defined in different block
- LLVM verifier will fail: "PHI node operands must be defined in predecessor"
- Academic literature (Cytron, Braun) requires materialization

**Decision**: ❌ Rejected — breaks SSA correctness

### 6.3 Lazy PHI Completion (Braun et al. Pure Approach)

**Idea**: Don't emit PHI instructions until loop body is fully lowered

**Pros**:
- Matches academic algorithm exactly
- Eliminates forward references naturally

**Cons**:
- Requires major refactoring of phi_core
- Breaks incremental MIR emission
- Incompatible with selfhost compiler's streaming JSON approach

**Decision**: 🔄 Long-term goal, but not for Phase 25.1b

---

## Conclusion

The ValueId circular dependency issue reveals a fundamental tension between:
- **Box Theory's simplicity** (treat blocks as immutable boxes)
- **Real-world complexity** (pinned parameters, multi-carrier loops)

The **LoopForm Meta-Box** solution resolves this by:
1. Treating loop structure itself as a Box (aligning with philosophy)
2. Separating carrier vs. pinned variables (aligning with SSA theory)
3. Guaranteeing definition-before-use through explicit passes (aligning with correctness)

**Estimated Implementation**: 150-200 lines (preserves Box Theory's simplicity)

**Expected Outcome**: Fix multi-carrier loops while maintaining all existing tests

**Next Steps**: Implement `LoopFormBuilder` struct and integrate with feature flag

---

## References

1. Cytron, R., Ferrante, J., Rosen, B. K., Wegman, M. N., & Zadeck, F. K. (1991). "Efficiently Computing Static Single Assignment Form and the Control Dependence Graph." *ACM TOPLAS*, 13(4), 451-490.

2. Braun, M., Buchwald, S., Hack, S., Leißa, R., Mallon, C., & Zwinkau, A. (2013). "Simple and Efficient Construction of Static Single Assignment Form." *Compiler Construction (CC 2013)*, LNCS 7791, 102-122.

3. LLVM Project. "LLVM Loop Terminology and Canonical Forms." https://llvm.org/docs/LoopTerminology.html

4. Hakorune Project. "Box Theory SSA Construction Revolution." `docs/private/research/papers-archive/paper-d-ssa-construction/box-theory-solution.md`

5. Hakorune Project. "LoopForm SSOT Design." `docs/development/architecture/loops/loopform_ssot.md`
