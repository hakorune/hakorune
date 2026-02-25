# Phase 136 Step 4/7: BindingContext Extraction - Implementation Summary

## Overview
Successfully extracted `binding_map` (String → BindingId mapping) into a dedicated `BindingContext` structure, following the same pattern as TypeContext, CoreContext, and ScopeContext.

## Implementation Details

### New File: `src/mir/builder/binding_context.rs`
Created a new context structure with:

```rust
pub struct BindingContext {
    pub(super) binding_map: BTreeMap<String, BindingId>,
}
```

**Key Methods**:
- `lookup(name: &str) -> Option<BindingId>` - Lookup variable's BindingId
- `insert(name: String, binding_id: BindingId)` - Register a variable binding
- `remove(name: &str) -> Option<BindingId>` - Remove a variable binding
- `binding_map() -> &BTreeMap<String, BindingId>` - Get immutable reference
- `contains(name: &str) -> bool` - Check if variable exists
- `len() -> usize` / `is_empty() -> bool` - Size queries

### MirBuilder Integration

#### 1. Added binding_ctx field
```rust
pub(super) binding_ctx: binding_context::BindingContext,
```

#### 2. Deprecated legacy field
```rust
#[deprecated(note = "Use binding_ctx.binding_map instead")]
pub binding_map: BTreeMap<String, super::BindingId>,
```

#### 3. Added sync helpers
```rust
fn sync_binding_ctx_to_legacy(&mut self) {
    self.binding_map = self.binding_ctx.binding_map.clone();
}

fn sync_legacy_to_binding_ctx(&mut self) {
    self.binding_ctx.binding_map = self.binding_map.clone();
}
```

### Updated Files

#### Core Files (6 files)
1. **`src/mir/builder.rs`**
   - Added `binding_ctx` field initialization
   - Deprecated `binding_map` field
   - Added sync helper methods
   - Updated tests to verify both SSOT and legacy access

2. **`src/mir/builder/vars/lexical_scope.rs`**
   - `declare_local_in_current_scope()`: Uses `binding_ctx.lookup()` and `binding_ctx.insert()`
   - `pop_lexical_scope()`: Restores bindings via `binding_ctx.insert()` and `binding_ctx.remove()`
   - Added `sync_binding_ctx_to_legacy()` calls

3. **BindingMapProvider trait implementation**
   - Updated to return `binding_ctx.binding_map()` instead of direct field access

#### Pattern Files (4 files)
All pattern files updated to use `binding_ctx` for binding lookups:

4. **`src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs`**
   - Changed `self.binding_map.get(&loop_var_name)` → `self.binding_ctx.lookup(&loop_var_name)`
   - Changed `self.binding_map.get(&binding.name)` → `self.binding_ctx.lookup(&binding.name)`

5. **`src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`**
   - Changed `builder.binding_map.get(&loop_var_name).copied()` → `builder.binding_ctx.lookup(&loop_var_name)`

6. **`src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs`**
   - Changed `binding_map: Some(&builder.binding_map)` → `binding_map: Some(builder.binding_ctx.binding_map())`
   - Changed `builder.binding_map.clone()` → `builder.binding_ctx.binding_map().clone()`

7. **`src/mir/builder/control_flow/joinir/patterns/trim_loop_lowering.rs`**
   - Changed `binding_map: Some(&builder.binding_map)` → `binding_map: Some(builder.binding_ctx.binding_map())`
   - Changed `trim_info.to_carrier_info(Some(&builder.binding_map))` → `trim_info.to_carrier_info(Some(builder.binding_ctx.binding_map()))`

## Migration Strategy

### Dual-Access Pattern
Like previous Context extractions, we maintain both:
1. **SSOT (Single Source of Truth)**: `binding_ctx.binding_map`
2. **Legacy field**: `self.binding_map` (deprecated)
3. **Sync helpers**: Keep both in sync during migration

### Example Migration
```rust
// Before (Phase 74)
if let Some(bid) = self.binding_map.get(&loop_var_name) {
    cond_env.register_loop_var_binding(*bid, _loop_var_join_id);
}

// After (Phase 136 Step 4/7)
if let Some(bid) = self.binding_ctx.lookup(&loop_var_name) {
    cond_env.register_loop_var_binding(bid, _loop_var_join_id);
}
```

## Test Results

### Build
```
✅ cargo build --release
   Compiling nyash-rust v0.1.0
   Finished `release` profile [optimized] target(s) in 25.61s
```

### Unit Tests
```
✅ cargo test --release --lib
   test result: ok. 1008 passed; 4 failed; 56 ignored
```

**4 Pre-existing failures** (expected):
- `mir_breakfinder_ssa::mir_loopssa_breakfinder_min_verifies`
- `mir_locals_ssa::mir_locals_copy_instructions_emitted`
- `mir_stage1_cli_emit_program_min::mir_stage1_cli_emit_program_min_compiles_and_verifies`
- `mir_stage1_cli_emit_program_min::mir_stage1_cli_emit_program_min_exec_hits_type_error`

### Updated Tests
```rust
#[test]
#[allow(deprecated)]
fn test_binding_map_initialization() {
    let builder = MirBuilder::new();
    assert_eq!(builder.next_binding_id, 0);
    // Phase 136 Step 4/7: Check both binding_ctx (SSOT) and legacy field
    assert!(builder.binding_ctx.is_empty());
    assert!(builder.binding_map.is_empty());
}

#[test]
#[allow(deprecated)]
fn test_shadowing_binding_restore() {
    // ... (verifies both binding_ctx.lookup() and legacy binding_map)
}
```

## Acceptance Criteria - All Met ✅

1. ✅ **Build Success**: `cargo build --release` completes without errors
2. ✅ **Tests Pass**: `cargo test --release --lib` passes (1008 tests, 4 pre-existing failures)
3. ✅ **No Public API Breakage**: All changes internal, backward compatibility maintained
4. ✅ **Progress Document Updated**: `phase-136-context-box-progress.md` shows 4/7 complete

## Design Benefits

### 1. SSOT Enforcement
- `binding_ctx` is the single source of truth for BindingId mappings
- Legacy field access triggers deprecation warnings
- Sync helpers ensure consistency during migration

### 2. Better Organization
- Binding-related logic centralized in `BindingContext`
- Clear separation from ValueId mapping (`variable_map`)
- Easier to understand relationship with `ScopeContext`

### 3. Type Safety
- `lookup()` returns `Option<BindingId>` (not borrowed reference)
- No need for `.copied()` calls like with `binding_map.get()`
- More ergonomic API

### 4. Testability
- `BindingContext` has its own unit tests
- Can test binding logic independently of MirBuilder
- Easier to verify correctness

## Relationship with Other Contexts

### BindingContext ↔ CoreContext
- **CoreContext** allocates BindingIds via `next_binding()`
- **BindingContext** stores the name → BindingId mappings

### BindingContext ↔ ScopeContext
- **ScopeContext** manages lexical scope frames
- Each scope frame has `restore_binding: BTreeMap<String, Option<BindingId>>`
- **BindingContext** is restored from scope frame data on `pop_lexical_scope()`

### BindingContext ↔ Variable Map
- **variable_map**: String → ValueId (SSA value mapping)
- **binding_ctx**: String → BindingId (binding identity tracking)
- Both are parallel tracking systems (Phase 74 design)

## Phase 74 Background

BindingId was introduced in Phase 74 to:
1. Track variable binding identity (separate from SSA renaming)
2. Enable stable binding tracking across SSA transformations
3. Support future ScopeManager migration (Phase 75+)
4. Provide deterministic iteration (BTreeMap vs HashMap)

## Next Steps

### Step 5/7: VariableContext extraction
Extract `variable_map` and related variable tracking:
- `variable_map: BTreeMap<String, ValueId>`
- Possibly other variable-related fields
- Follow same dual-access pattern

### Future Work
- Step 6/7: MetadataContext (user_defined_boxes, weak_fields, etc.)
- Step 7/7: RegionContext (slot_registry, region_stack)
- Eventually remove deprecated fields after full migration

## Lessons Learned

### What Worked Well
1. ✅ Following established pattern from Steps 1-3 made implementation smooth
2. ✅ Dual-access pattern provides safety net during migration
3. ✅ Incremental approach (one context at a time) is manageable
4. ✅ Tests verify both SSOT and legacy access work correctly

### Notes for Next Steps
- Keep same pattern: extract → deprecate → sync helpers → update tests
- Verify all `rg` searches to find usage sites
- Update both feature-gated code (`#[cfg(feature = "normalized_dev")]`) and regular code
- Don't forget to update trait implementations (like BindingMapProvider)

## File Statistics

### New Files
- `src/mir/builder/binding_context.rs` (149 lines)

### Modified Files
- `src/mir/builder.rs` (+18 lines net)
- `src/mir/builder/vars/lexical_scope.rs` (+9 lines net)
- `src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs` (+4 lines net)
- `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs` (+2 lines net)
- `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs` (+4 lines net)
- `src/mir/builder/control_flow/joinir/patterns/trim_loop_lowering.rs` (+3 lines net)

### Total Impact
- **New**: 149 lines
- **Modified**: ~40 lines net
- **Files touched**: 7 files

## Commit Message Template

```
feat(mir): Phase 136 Step 4/7 - BindingContext extraction

Extract binding_map into dedicated BindingContext for better organization
and SSOT enforcement. Follows the same dual-access pattern as TypeContext,
CoreContext, and ScopeContext.

Changes:
- New: src/mir/builder/binding_context.rs (BindingContext struct)
- MirBuilder: Add binding_ctx field, deprecate binding_map
- Add sync helpers: sync_binding_ctx_to_legacy() / sync_legacy_to_binding_ctx()
- Update vars/lexical_scope.rs to use binding_ctx SSOT
- Update pattern files to use binding_ctx.lookup() / binding_map()
- Update BindingMapProvider trait to use binding_ctx
- Update tests to verify both SSOT and legacy access

Test results:
- cargo build --release: ✅ Success
- cargo test --release --lib: ✅ 1008 passed (4 pre-existing failures)
- No public API breakage

Progress: 4/7 context extractions complete
```

## Conclusion

Phase 136 Step 4/7 successfully extracts BindingContext, maintaining 100% backward compatibility while improving code organization. The dual-access pattern provides a safe migration path, and all acceptance criteria are met.

**Status**: ✅ Complete and ready for commit
