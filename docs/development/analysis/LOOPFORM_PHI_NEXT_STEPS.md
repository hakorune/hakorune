# LoopForm PHI Solution - Next Steps for Integration

**Date**: 2025-11-17
**Prerequisites**: Prototype implementation complete (`loopform_builder.rs` created)
**Goal**: Integrate LoopFormBuilder into production codebase with feature flag

---

## Step 1: Implement LoopFormOps for MirBuilder

**File**: `src/mir/loop_builder.rs`

**Code to Add**:

```rust
use crate::mir::phi_core::loopform_builder::{LoopFormBuilder, LoopFormOps};

impl<'a> LoopFormOps for LoopBuilder<'a> {
    fn new_value(&mut self) -> ValueId {
        self.parent_builder.new_value()
    }

    fn is_parameter(&self, name: &str) -> bool {
        // Check if variable is a function parameter
        self.parent_builder.function_params.contains(name)
        // OR: check if variable starts with specific pattern
        // name == "me" || self.parent_builder.is_param(name)
    }

    fn set_current_block(&mut self, block: BasicBlockId) -> Result<(), String> {
        self.parent_builder.set_current_block(block)
    }

    fn emit_copy(&mut self, dst: ValueId, src: ValueId) -> Result<(), String> {
        self.parent_builder.emit_copy(dst, src)
    }

    fn emit_jump(&mut self, target: BasicBlockId) -> Result<(), String> {
        self.parent_builder.emit_jump(target)
    }

    fn emit_phi(
        &mut self,
        dst: ValueId,
        inputs: Vec<(BasicBlockId, ValueId)>,
    ) -> Result<(), String> {
        self.parent_builder.emit_phi_at_block_start(
            self.parent_builder.current_block()?,
            dst,
            inputs
        )
    }

    fn update_phi_inputs(
        &mut self,
        block: BasicBlockId,
        phi_id: ValueId,
        inputs: Vec<(BasicBlockId, ValueId)>,
    ) -> Result<(), String> {
        // Find existing PHI instruction in block and update its inputs
        self.parent_builder.update_phi_instruction(block, phi_id, inputs)
    }

    fn update_var(&mut self, name: String, value: ValueId) {
        self.parent_builder.bind_variable(name, value);
    }

    fn get_variable_at_block(&self, name: &str, block: BasicBlockId) -> Option<ValueId> {
        self.parent_builder.get_variable_in_block(name, block)
    }
}
```

**Note**: You may need to add helper methods to `MirBuilder` if they don't exist:
- `update_phi_instruction(block, phi_id, inputs)` - updates an existing PHI
- `function_params` field or `is_param(name)` method

---

## Step 2: Add Feature Flag to Loop Construction

**File**: `src/mir/loop_builder.rs`

**Modify**: `build_loop()` method or equivalent

```rust
pub fn build_loop(
    &mut self,
    condition: &ASTNode,
    body: &Vec<ASTNode>,
) -> Result<ValueId, String> {
    // 設計当初は feature flag（NYASH_LOOPFORM_PHI_V2）で切り替える案だったが、
    // 現在は LoopForm v2 が既定実装なので常に build_loop_with_loopform を使う。
    self.build_loop_with_loopform(condition, body)
}

fn build_loop_with_loopform(
    &mut self,
    condition: &ASTNode,
    body: &Vec<ASTNode>,
) -> Result<ValueId, String> {
    // Create blocks
    let preheader_id = self.new_block();
    let header_id = self.new_block();
    let body_id = self.new_block();
    let latch_id = self.new_block();
    let exit_id = self.new_block();

    // Initialize LoopFormBuilder
    let mut loopform = LoopFormBuilder::new(preheader_id, header_id);

    // Capture current variables
    let current_vars = self.get_current_variable_map();

    // Pass 1: Prepare structure (allocate all ValueIds)
    loopform.prepare_structure(self, &current_vars)?;

    // Pass 2: Emit preheader
    loopform.emit_preheader(self)?;

    // Pass 3: Emit header PHIs
    loopform.emit_header_phis(self)?;

    // Emit condition check in header
    self.set_current_block(header_id)?;
    let cond_value = self.lower_expression(condition)?;
    self.emit_branch(cond_value, body_id, exit_id)?;

    // Lower loop body
    self.set_current_block(body_id)?;
    for stmt in body {
        self.lower_statement(stmt)?;
    }

    // Jump to latch
    if !is_current_block_terminated(self.parent_builder)? {
        self.emit_jump(latch_id)?;
    }

    // Latch: jump back to header
    self.set_current_block(latch_id)?;
    self.emit_jump(header_id)?;

    // Pass 4: Seal PHIs
    loopform.seal_phis(self, latch_id)?;

    // Exit block
    self.set_current_block(exit_id)?;
    // Exit PHIs handled by loopform.build_exit_phis() if break statements exist

    Ok(ValueId::VOID) // or appropriate return value
}

fn build_loop_legacy(
    &mut self,
    condition: &ASTNode,
    body: &Vec<ASTNode>,
) -> Result<ValueId, String> {
    // Existing implementation using prepare_loop_variables_with()
    // ... (keep current code unchanged)
}
```

---

## Step 3: Test with Fibonacci Example

**Command**:
```bash
# Build with new implementation
cargo build --release

# Test fibonacci（現在はフラグ不要）
./target/release/nyash local_tests/fib_multi_carrier.hako

# Expected output: 8
```

**Debugging**:
```bash
# Enable MIR dump（現在はフラグ不要）
./target/release/nyash --dump-mir local_tests/fib_multi_carrier.hako

# Check for:
# 1. Preheader copies in correct order
# 2. Header PHIs referencing preheader copies (no forward refs)
# 3. Latch block jumps back to header
```

---

## Step 4: Validate with Smoke Tests

**Run existing tests**:
```bash
# Simple loops（LoopForm v2 は既定 ON）
tools/smokes/v2/run.sh --profile quick --filter "loop_simple"

# Multi-carrier loops
tools/smokes/v2/run.sh --profile quick --filter "multi_carrier"

# All loop tests
tools/smokes/v2/run.sh --profile quick --filter "loop"
```

**Create comparison test**:
```bash
#!/bin/bash
# compare_loopform.sh

for test in local_tests/loop_*.hako; do
    echo "Testing: $test"

    # Old implementation
    NYASH_LOOPFORM_PHI_V2=0 ./target/release/nyash "$test" > /tmp/old.out 2>&1
    OLD_EXIT=$?

    # New implementation
    NYASH_LOOPFORM_PHI_V2=1 ./target/release/nyash "$test" > /tmp/new.out 2>&1
    NEW_EXIT=$?

    # Compare
    if [ $OLD_EXIT -eq 0 ] && [ $NEW_EXIT -eq 0 ]; then
        if diff -q /tmp/old.out /tmp/new.out > /dev/null; then
            echo "  ✅ PASS (outputs match)"
        else
            echo "  ⚠️  WARN (outputs differ)"
            diff /tmp/old.out /tmp/new.out
        fi
    elif [ $OLD_EXIT -ne 0 ] && [ $NEW_EXIT -eq 0 ]; then
        echo "  🎉 FIX (old failed, new works!)"
    elif [ $OLD_EXIT -eq 0 ] && [ $NEW_EXIT -ne 0 ]; then
        echo "  ❌ REGRESSION (old worked, new fails!)"
        cat /tmp/new.out
    else
        echo "  🤷 BOTH FAIL"
    fi
done
```

---

## Step 5: Selfhost Compiler Integration (Optional)

**File**: `lang/src/mir/builder/func_body/basic_lower_box.hako`

**Approach**: The selfhost compiler uses JSON-based MIR construction, not direct Rust API.

**Option A**: Let selfhost use Rust provider for multi-carrier loops (current behavior)

**Option B**: Implement LoopForm logic in Hakorune itself:
- Create `lang/src/mir/builder/internal/loopform_builder_box.hako`
- Implement carrier/pinned separation in Hakorune code
- Use `LoopFormBox.build2()` with explicit carrier metadata

**Recommendation**: Start with Option A (Rust provider fallback) for Phase 25.1b, implement Option B in Phase 25.2.

---

## Step 6: Performance Validation

**Benchmark**:
```bash
# Create benchmark test
cat > bench/loop_heavy.hako <<'EOF'
static box Main {
    main() {
        i = 0
        sum = 0
        loop(i < 10000) {
            sum = sum + i
            i = i + 1
        }
        print(sum)
    }
}
EOF

# Compare performance
hyperfine --warmup 3 \
  'NYASH_LOOPFORM_PHI_V2=0 ./target/release/nyash bench/loop_heavy.hako' \
  'NYASH_LOOPFORM_PHI_V2=1 ./target/release/nyash bench/loop_heavy.hako'
```

**Expected**: < 5% difference (allocation overhead is negligible)

---

## Step 7: Documentation Updates

**Files to Update**:

1. **`CURRENT_TASK.md`**:
   - Add entry: "✅ Phase 25.1b: LoopForm PHI circular dependency resolved"

2. **`docs/private/roadmap2/phases/phase-25.1b/README.md`**:
   - Document LoopFormBuilder implementation
   - Add testing results

3. **`docs/development/architecture/loops/loopform_ssot.md`**:
   - Update with LoopFormBuilder as new SSOT

4. **`CLAUDE.md`**:
   - Add to "Recent Updates" section

---

## Troubleshooting

### Issue: `is_parameter()` always returns false

**Solution**: Implement parameter tracking in MirBuilder:
```rust
pub struct MirBuilder {
    function_params: HashSet<String>,
    // ... existing fields
}

impl MirBuilder {
    pub fn set_function_params(&mut self, params: &[String]) {
        self.function_params = params.iter().cloned().collect();
    }
}
```

Call this when entering function scope:
```rust
self.builder.set_function_params(&["me", "param1", "param2"]);
```

### Issue: `update_phi_inputs()` not implemented

**Solution**: Add method to MirBuilder:
```rust
pub fn update_phi_instruction(
    &mut self,
    block: BasicBlockId,
    phi_id: ValueId,
    new_inputs: Vec<(BasicBlockId, ValueId)>,
) -> Result<(), String> {
    let block_data = self.blocks.get_mut(&block)
        .ok_or("Block not found")?;

    // Find PHI instruction with matching dst
    for inst in &mut block_data.instructions {
        if let MirInstruction::Phi { dst, inputs } = inst {
            if *dst == phi_id {
                *inputs = new_inputs;
                return Ok(());
            }
        }
    }

    Err(format!("PHI instruction {} not found in block {}", phi_id, block))
}
```

### Issue: Tests fail with "use of undefined value"

**Debug**:
```bash
# Dump MIR to see exact structure
NYASH_LOOPFORM_PHI_V2=1 ./target/release/nyash --dump-mir test.hako 2>&1 | less

# Check for:
# 1. All preheader copies present
# 2. Header PHIs reference correct preheader values
# 3. No forward references (%14 used before defined)
```

**Common fix**: Ensure `emit_copy_at_preheader()` inserts at **end** of preheader block, not current position.

---

## Success Metrics

### Week 2 Goals
- [ ] `fib_multi_carrier.hako` outputs correct result (8)
- [ ] All smoke tests pass with `NYASH_LOOPFORM_PHI_V2=1`
- [ ] No performance regression (< 5% slowdown)
- [ ] MIR dump shows correct PHI structure (no forward refs)

### Week 3 Goals
- [ ] Feature flag enabled by default
- [ ] Old `prepare_loop_variables_with()` marked deprecated
- [ ] Documentation updated

### Week 4 Goals
- [ ] Old code path removed
- [ ] All tests pass without feature flag
- [ ] Phase 25.1b marked COMPLETE ✅

---

## Rollback Plan

If integration fails:

1. **Immediate**: Set `NYASH_LOOPFORM_PHI_V2=0` in environment
2. **Short-term**: Comment out feature flag check, force old path
3. **Debug**: Use MIR dumps to identify incompatibility
4. **Iterate**: Fix LoopFormBuilder implementation, retry

**No risk to production**: Old code path remains intact until Week 4.

---

## Next Actions (Priority Order)

1. **Implement `LoopFormOps` for `LoopBuilder`** (Step 1)
2. **Add feature flag to `build_loop()`** (Step 2)
3. **Test fibonacci example** (Step 3)
4. **Run smoke tests** (Step 4)
5. **Validate performance** (Step 6)
6. **Update documentation** (Step 7)

**Estimated Time**: 2-4 hours for integration, 1-2 hours for testing and validation.

---

**Document Status**: READY FOR IMPLEMENTATION ✅
**Next Assignee**: ChatGPT (implementation) or User (manual integration)
