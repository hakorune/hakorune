# Modularization Quick Start Checklist

This is the **TL;DR** version of the full implementation plan. Use this as a quick reference during execution.

**Full Plan**: [modularization-implementation-plan.md](./modularization-implementation-plan.md)

---

## 1. control_flow.rs Modularization (12.5 hours)

### Phase 1: Debug Utilities (30 min)
- [ ] Create `src/mir/builder/control_flow/debug.rs`
- [ ] Move `trace_varmap()`
- [ ] Update `mod.rs` imports
- [ ] Run: `cargo build --release && cargo test --lib`
- [ ] Commit: `git commit -m "refactor: Extract debug utilities from control_flow.rs"`

### Phase 2: Pattern Lowerers (2 hours)
- [ ] Create `control_flow/joinir/patterns/` directory
- [ ] Move `cf_loop_pattern1_minimal()` to `pattern1_minimal.rs`
- [ ] Move `cf_loop_pattern2_with_break()` to `pattern2_with_break.rs`
- [ ] Move `cf_loop_pattern3_with_if_phi()` to `pattern3_with_if_phi.rs`
- [ ] Create `patterns/mod.rs` dispatcher
- [ ] Run: `cargo build --release && cargo test --lib -- --include-ignored`
- [ ] Test: `NYASH_OPTION_C_DEBUG=1 ./target/release/nyash apps/tests/loop_min_while.hako`
- [ ] Commit: `git commit -m "refactor: Extract pattern lowerers from control_flow.rs"`

### Phase 3: JoinIR Routing (1.5 hours)
- [ ] Create `control_flow/joinir/routing.rs`
- [ ] Move `try_cf_loop_joinir()`
- [ ] Move `cf_loop_joinir_impl()`
- [ ] Update imports
- [ ] Run: `cargo build --release && cargo test --release`
- [ ] Commit: `git commit -m "refactor: Extract JoinIR routing logic"`

### Phase 4: merge_joinir_mir_blocks (6 hours) ⚠️ CRITICAL
- [ ] Create `control_flow/joinir/merge/` directory
- [ ] Extract `id_remapper.rs` (150 lines)
- [ ] Extract `block_allocator.rs` (100 lines)
- [ ] Extract `value_collector.rs` (100 lines)
- [ ] Extract `instruction_rewriter.rs` (150 lines)
- [ ] Extract `exit_phi_builder.rs` (100 lines)
- [ ] Create `merge/mod.rs` coordinator (100 lines)
- [ ] Update all imports
- [ ] Run: `cargo build --release && cargo test --lib`
- [ ] Run: `tools/smokes/v2/run.sh --profile quick`
- [ ] Run: `NYASH_OPTION_C_DEBUG=1 ./target/release/nyash apps/tests/loop_min_while.hako 2>&1 | grep "merge_joinir"`
- [ ] Run determinism test (3x):
  ```bash
  for i in 1 2 3; do echo "=== Run $i ==="; cargo test --release test_loop_patterns; done
  ```
- [ ] Commit: `git commit -m "refactor: Break down merge_joinir_mir_blocks into modules"`

### Phase 5: Exception Handling (1 hour)
- [ ] Create `control_flow/exception/` directory
- [ ] Move `cf_try_catch()` to `try_catch.rs`
- [ ] Move `cf_throw()` to `throw.rs`
- [ ] Create `exception/mod.rs`
- [ ] Run: `cargo build --release && cargo test --lib -- exception`
- [ ] Commit: `git commit -m "refactor: Extract exception handling"`

### Phase 6: Utilities (30 min)
- [ ] Create `control_flow/utils.rs`
- [ ] Move `extract_loop_variable_from_condition()`
- [ ] Run: `cargo build --release && cargo test --lib`
- [ ] Commit: `git commit -m "refactor: Extract utilities"`

### Phase 7: Final Cleanup (1 hour)
- [ ] Add module-level documentation to all files
- [ ] Review `control_flow/mod.rs` for clarity
- [ ] Verify all `pub(super)` visibility
- [ ] Run: `cargo build --release --all-features`
- [ ] Run: `cargo test --release`
- [ ] Run: `cargo clippy --all-targets`
- [ ] Run: `tools/smokes/v2/run.sh --profile integration`
- [ ] Update `CLAUDE.md` with new structure
- [ ] Commit: `git commit -m "docs: Update control_flow module documentation"`

---

## 2. generic_case_a.rs Modularization (3.5 hours)

### Phase 1: Directory Setup (15 min)
- [ ] Create `src/mir/join_ir/lowering/generic_case_a/` directory
- [ ] Create `mod.rs` with public API
- [ ] Move `generic_case_a_entry_builder.rs` into directory
- [ ] Move `generic_case_a_whitespace_check.rs` into directory
- [ ] Update parent `mod.rs` imports
- [ ] Run: `cargo build --release`
- [ ] Commit: `git commit -m "refactor: Create generic_case_a directory structure"`

### Phase 2: Extract skip_ws (45 min)
- [ ] Create `generic_case_a/skip_ws.rs`
- [ ] Move `lower_case_a_skip_ws_with_scope()` and `_core()`
- [ ] Add module documentation
- [ ] Run: `cargo build --release && cargo test --lib -- skip_ws`
- [ ] Commit: `git commit -m "refactor: Extract skip_ws lowerer"`

### Phase 3: Extract trim (1 hour)
- [ ] Create `generic_case_a/trim.rs`
- [ ] Move `lower_case_a_trim_with_scope()` and `_core()`
- [ ] Add module documentation
- [ ] Run: `cargo build --release && cargo test --lib -- trim`
- [ ] Commit: `git commit -m "refactor: Extract trim lowerer"`

### Phase 4: Extract append_defs & stage1 (1 hour)
- [ ] Create `generic_case_a/append_defs.rs`
- [ ] Create `generic_case_a/stage1_using_resolver.rs`
- [ ] Move respective functions
- [ ] Add module documentation
- [ ] Run: `cargo build --release && cargo test --release`
- [ ] Run: `tools/smokes/v2/run.sh --profile quick --filter "funcscanner_*"`
- [ ] Commit: `git commit -m "refactor: Extract append_defs and stage1 lowerers"`

### Phase 5: Final Cleanup (30 min)
- [ ] Add module-level documentation to all files
- [ ] Verify all imports are clean
- [ ] Run: `cargo build --release --all-features && cargo test --release`
- [ ] Commit: `git commit -m "docs: Complete generic_case_a modularization"`

---

## 3. loopform_builder.rs Modularization (4 hours) - FUTURE

**Status**: Already partially modularized in Phase 191. Remaining work is lower priority.

### Phase 1: Directory Setup (30 min)
- [ ] Create `src/mir/phi_core/loopform/` directory
- [ ] Move existing modular files into directory
- [ ] Create `mod.rs` with re-exports
- [ ] Run: `cargo build --release && cargo test --lib -- loopform`

### Phase 2: Extract 4-Pass Architecture (2 hours)
- [ ] Create `loopform/passes/` directory
- [ ] Extract Pass 1: Variable discovery
- [ ] Extract Pass 2: Header PHI construction
- [ ] Extract Pass 3: Latch block processing
- [ ] Extract Pass 4: Exit PHI construction
- [ ] Create `passes/mod.rs` coordinator
- [ ] Run: `cargo build --release && cargo test --release -- loopform`

### Phase 3: Extract Core Builder (1 hour)
- [ ] Create `loopform/builder_core.rs`
- [ ] Move remaining builder logic
- [ ] Run: `cargo build --release && cargo test --release`

### Phase 4: Final Cleanup (30 min)
- [ ] Add module documentation
- [ ] Run: `cargo build --release --all-features`
- [ ] Run: `tools/smokes/v2/run.sh --profile quick --filter "phi_*"`

---

## Emergency Rollback Commands

```bash
# Rollback control_flow.rs modularization
rm -rf src/mir/builder/control_flow/
git checkout src/mir/builder/control_flow.rs
cargo build --release && cargo test --lib

# Rollback generic_case_a.rs modularization
rm -rf src/mir/join_ir/lowering/generic_case_a/
git checkout src/mir/join_ir/lowering/generic_case_a*.rs
cargo build --release && cargo test --lib

# Rollback loopform_builder.rs modularization
rm -rf src/mir/phi_core/loopform/
git checkout src/mir/phi_core/loopform*.rs
cargo build --release && cargo test --lib
```

---

## Verification Commands

### Quick Verification (after each phase)
```bash
cargo build --release
cargo test --lib
```

### Comprehensive Verification (after critical phases)
```bash
cargo build --release --all-features
cargo test --release
cargo clippy --all-targets
tools/smokes/v2/run.sh --profile quick
```

### Debug Trace Verification (Phase 4 only)
```bash
NYASH_OPTION_C_DEBUG=1 ./target/release/nyash apps/tests/loop_min_while.hako 2>&1 | grep "merge_joinir"
```

### Determinism Check (Phase 4 only)
```bash
for i in 1 2 3; do
  echo "=== Run $i ==="
  cargo test --release test_loop_patterns 2>&1 | grep "test result"
done
```

---

## Success Criteria

- ✅ All 267+ tests pass
- ✅ Build time ≤ current
- ✅ `control_flow/mod.rs` < 200 lines (vs 1,632)
- ✅ Largest file < 250 lines (vs 714)
- ✅ Debug traces still work
- ✅ No HashMap non-determinism
- ✅ Code is easier to navigate

---

## Timeline

### Week 1: control_flow.rs Phases 1-3 (Low Risk)
- **Total**: 4 hours
- **Risk**: Low
- **Deliverable**: Pattern lowerers and routing isolated

### Week 2: control_flow.rs Phase 4 (High Risk)
- **Total**: 6 hours
- **Risk**: Medium
- **Deliverable**: merge_joinir_mir_blocks modularized

### Week 3: control_flow.rs Phases 5-7 + generic_case_a.rs
- **Total**: 6 hours
- **Risk**: Low
- **Deliverable**: control_flow.rs complete, generic_case_a.rs modularized

---

## Questions?

**Full Plan**: [modularization-implementation-plan.md](./modularization-implementation-plan.md)

**Open Issues**:
- Create a GitHub issue if you encounter blockers
- Tag with `refactoring` label

**Need Help?**:
- Check the full plan for detailed explanations
- Review the risk/mitigation matrix
- Use the emergency rollback procedure if needed

---

**Last Updated**: 2025-12-05
**Status**: Ready to execute
