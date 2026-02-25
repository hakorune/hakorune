# Phase 136 Follow-up: Context Box Extraction Progress

## Overview
Phase 136 follow-up extracts builder context into dedicated Context structures for better organization and SSOT enforcement.

## Progress Tracking

### ✅ Step 1/7: TypeContext extraction (Complete)
- **Status**: ✅ Complete (commit 076f193f)
- **Extracted fields**:
  - `value_types: BTreeMap<ValueId, MirType>`
  - `value_kinds: HashMap<ValueId, MirValueKind>`
  - `value_origin_newbox: BTreeMap<ValueId, String>`
- **New file**: `src/mir/builder/type_context.rs`
- **Tests**: All passing
- **Migration**: Dual-access pattern (ctx + legacy field sync)

### ✅ Step 2/7: CoreContext extraction (Complete)
- **Status**: ✅ Complete (commits 81d79161, 89edf116)
- **Extracted fields**:
  - `value_gen: ValueIdGenerator`
  - `block_gen: BasicBlockIdGenerator`
  - `next_binding_id: u32`
  - `temp_slot_counter: usize`
  - `debug_join_counter: u32`
- **New file**: `src/mir/builder/core_context.rs`
- **Tests**: All passing
- **Migration**: Dual-access pattern with helper methods

### ✅ Step 3/7: ScopeContext extraction (Complete)
- **Status**: ✅ Complete (commit 3127ebb7)
- **Extracted fields**:
  - `lexical_scope_stack: Vec<LexicalScopeFrame>`
  - `loop_header_stack: Vec<BasicBlockId>`
  - `loop_exit_stack: Vec<BasicBlockId>`
  - `if_merge_stack: Vec<BasicBlockId>`
  - `current_function: Option<MirFunction>`
  - `function_param_names: HashSet<String>`
  - `debug_scope_stack: Vec<String>`
- **New file**: `src/mir/builder/scope_context.rs`
- **Tests**: All passing
- **Migration**: Dual-access pattern with sync helpers

### ✅ Step 4/7: BindingContext extraction (Complete)
- **Status**: ✅ Complete (current commit)
- **Extracted fields**:
  - `binding_map: BTreeMap<String, BindingId>`
- **New file**: `src/mir/builder/binding_context.rs`
- **Key methods**:
  - `lookup(name: &str) -> Option<BindingId>` - Variable lookup
  - `insert(name: String, binding_id: BindingId)` - Register binding
  - `remove(name: &str) -> Option<BindingId>` - Remove binding
  - `binding_map() -> &BTreeMap<String, BindingId>` - Get reference
- **Updated files**:
  - `src/mir/builder.rs` - Added binding_ctx field, sync helpers
  - `src/mir/builder/vars/lexical_scope.rs` - Use binding_ctx SSOT
  - `src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs` - Use binding_ctx.lookup()
  - `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs` - Use binding_ctx.lookup()
  - `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs` - Use binding_ctx.binding_map()
  - `src/mir/builder/control_flow/joinir/patterns/trim_loop_lowering.rs` - Use binding_ctx.binding_map()
- **Tests**: All passing (1008 passed; 4 pre-existing failures)
- **Migration**: Dual-access pattern with sync helpers
- **Acceptance criteria**:
  - ✅ `cargo build --release` - Success
  - ✅ `cargo test --release --lib` - 1008 passed (4 pre-existing failures OK)
  - ✅ Public API unchanged
  - ✅ Documentation updated

### 🔲 Step 5/7: VariableContext extraction (Pending)
- **Status**: 🔲 Not started
- **Target fields**:
  - `variable_map: BTreeMap<String, ValueId>`
  - (Possibly other variable-related fields)
- **Estimated effort**: Medium
- **Dependencies**: None (can proceed independently)

### 🔲 Step 6/7: MetadataContext extraction (Pending)
- **Status**: 🔲 Not started
- **Target fields**:
  - `user_defined_boxes: HashSet<String>`
  - `weak_fields_by_box: HashMap<String, HashSet<String>>`
  - `property_getters_by_box: HashMap<String, HashMap<String, PropertyKind>>`
  - `field_origin_class: HashMap<(ValueId, String), String>`
  - `field_origin_by_box: HashMap<(String, String), String>`
- **Estimated effort**: Large (many fields)
- **Dependencies**: None (can proceed independently)

### 🔲 Step 7/7: RegionContext extraction (Pending)
- **Status**: 🔲 Not started
- **Target fields**:
  - `current_slot_registry: Option<FunctionSlotRegistry>`
  - `current_region_stack: Vec<RegionId>`
  - (Possibly other region-related fields)
- **Estimated effort**: Small
- **Dependencies**: None (can proceed independently)

## Overall Status
- **Completed**: 4/7 steps (57%)
- **Remaining**: 3/7 steps (43%)
- **Next step**: Step 5/7 - VariableContext extraction

## Migration Strategy
All steps follow the same pattern:
1. Create new Context struct with extracted fields
2. Add as `pub(super)` field in MirBuilder
3. Deprecate legacy fields with `#[deprecated]` annotation
4. Add sync helper methods for backward compatibility
5. Update tests to verify both SSOT and legacy access
6. Verify build + tests pass

## Benefits Achieved (Steps 1-4)
- ✅ Better code organization (grouped related fields)
- ✅ Clearer SSOT ownership (each Context owns its data)
- ✅ Easier testing (can test Context independently)
- ✅ Backward compatibility maintained (legacy fields still work)
- ✅ Gradual migration path (no big-bang changes)

## Notes
- All contexts use `pub(super)` visibility for MirBuilder-only access
- Legacy fields are kept during migration with `#[deprecated]` warnings
- Sync helpers maintain consistency between SSOT and legacy fields
- Pre-existing test failures are acceptable (4 known failures)
