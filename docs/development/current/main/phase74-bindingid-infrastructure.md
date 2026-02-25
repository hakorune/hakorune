# Phase 74: BindingId Infrastructure

**Status**: ✅ Complete
**Date**: 2025-12-13
**Goal**: Establish BindingId infrastructure on main branch with zero production impact

---

## 1. Executive Summary

Phase 74 establishes the **BindingId infrastructure** as a parallel identifier system to ValueId, enabling future ScopeManager migration (Phase 75+) without breaking existing SSA functionality.

### Key Deliverables
- ✅ `src/mir/binding_id.rs` - BindingId type definition with overflow protection
- ✅ `src/mir/builder.rs` - Parallel allocation infrastructure (`allocate_binding_id()`)
- ✅ `src/mir/builder/vars/lexical_scope.rs` - Shadowing restoration for BindingId
- ✅ Unit tests (4 tests) - Initialization, sequential allocation, shadowing, parallel allocation
- ✅ Documentation (~300 lines)

### Acceptance Criteria
- ✅ `cargo build --lib` succeeds (no production path changes)
- ✅ `cargo test --release --lib` all tests pass (no regressions)
- ✅ `cargo test --features normalized_dev --test normalized_joinir_min` passes
- ✅ New unit tests validate BindingId behavior

---

## 2. Architecture: ValueId vs BindingId

### 2.1 Role Separation

| Aspect | ValueId | BindingId |
|--------|---------|-----------|
| **Purpose** | SSA value identity | Lexical binding identity |
| **Scope** | Single definition, multiple uses | Lexical scope lifetime |
| **Renaming** | PHI nodes create new ValueIds | Stable across PHI transformations |
| **Shadowing** | New ValueId per assignment | New BindingId per shadowing level |
| **Restoration** | Managed by SSA/PHI | Managed by lexical scope stack |

### 2.2 Parallel Allocation Example

```rust
// Outer scope: local x = 1;
let outer_vid = builder.next_value_id();      // ValueId(10)
let outer_bid = builder.allocate_binding_id(); // BindingId(0)
builder.variable_map.insert("x", outer_vid);
builder.binding_map.insert("x", outer_bid);

// Inner scope: { local x = 2; }
{
    let inner_vid = builder.next_value_id();      // ValueId(20)
    let inner_bid = builder.allocate_binding_id(); // BindingId(1)
    builder.variable_map.insert("x", inner_vid);   // Shadows outer ValueId
    builder.binding_map.insert("x", inner_bid);    // Shadows outer BindingId
}
// Scope exit: restore outer_vid and outer_bid
```

### 2.3 Why BindingId Matters

**Problem**: ValueId alone cannot distinguish between:
1. **SSA renaming** (PHI nodes merging values)
2. **Lexical shadowing** (new variable declaration with same name)

**Solution**: BindingId tracks the original lexical binding, enabling:
- **Shadowing detection** - Different BindingIds for outer/inner `x`
- **Scope restoration** - Restore correct BindingId on `}`
- **Future ScopeManager** - Stable binding identity for advanced scope analysis

---

## 3. Implementation Details

### 3.1 Core Data Structures

#### `src/mir/binding_id.rs`
```rust
pub struct BindingId(pub u32);

impl BindingId {
    pub fn new(id: u32) -> Self { /* overflow check */ }
    pub fn next(self) -> Self { /* sequential increment */ }
    pub fn raw(self) -> u32 { /* get raw value */ }
}
```

#### `src/mir/builder.rs` (MirBuilder fields)
```rust
pub struct MirBuilder {
    // Existing ValueId infrastructure
    value_gen: ValueIdGenerator,
    variable_map: BTreeMap<String, ValueId>,

    // Phase 74: Parallel BindingId infrastructure
    pub next_binding_id: u32,
    pub binding_map: BTreeMap<String, BindingId>,
}
```

#### `src/mir/builder.rs` (Allocation method)
```rust
pub fn allocate_binding_id(&mut self) -> BindingId {
    let id = BindingId::new(self.next_binding_id);
    self.next_binding_id = self.next_binding_id.saturating_add(1);
    debug_assert!(self.next_binding_id < u32::MAX, "overflow check");
    id
}
```

### 3.2 Lexical Scope Integration

#### `src/mir/builder/vars/lexical_scope.rs`
```rust
pub struct LexicalScopeFrame {
    declared: BTreeSet<String>,
    restore: BTreeMap<String, Option<ValueId>>,
    restore_binding: BTreeMap<String, Option<BindingId>>, // Phase 74
}
```

**Scope exit logic** (`pop_lexical_scope()`):
```rust
// Restore ValueId mappings (existing)
for (name, previous) in frame.restore {
    match previous {
        Some(prev_id) => self.variable_map.insert(name, prev_id),
        None => self.variable_map.remove(&name),
    };
}

// Phase 74: Restore BindingId mappings (parallel)
for (name, previous_binding) in frame.restore_binding {
    match previous_binding {
        Some(prev_bid) => self.binding_map.insert(name, prev_bid),
        None => self.binding_map.remove(&name),
    };
}
```

**Local declaration** (`declare_local_in_current_scope()`):
```rust
// Capture previous state (for restoration on scope exit)
let previous = self.variable_map.get(name).copied();
frame.restore.insert(name.to_string(), previous);

let previous_binding = self.binding_map.get(name).copied();
frame.restore_binding.insert(name.to_string(), previous_binding);

// Allocate new ValueId and BindingId
self.variable_map.insert(name.to_string(), value);
let binding_id = self.allocate_binding_id();
self.binding_map.insert(name.to_string(), binding_id);
```

---

## 4. Test Strategy

### 4.1 Unit Tests (src/mir/builder.rs)

#### Test 1: Initialization
```rust
#[test]
fn test_binding_map_initialization() {
    let builder = MirBuilder::new();
    assert_eq!(builder.next_binding_id, 0);
    assert!(builder.binding_map.is_empty());
}
```
**Validates**: Fresh builder starts with BindingId(0), empty binding_map.

#### Test 2: Sequential Allocation
```rust
#[test]
fn test_binding_allocation_sequential() {
    let mut builder = MirBuilder::new();
    let bid0 = builder.allocate_binding_id();
    let bid1 = builder.allocate_binding_id();
    let bid2 = builder.allocate_binding_id();

    assert_eq!(bid0.raw(), 0);
    assert_eq!(bid1.raw(), 1);
    assert_eq!(bid2.raw(), 2);
    assert_eq!(builder.next_binding_id, 3);
}
```
**Validates**: Sequential BindingId allocation (0, 1, 2, ...).

#### Test 3: Shadowing Restoration
```rust
#[test]
fn test_shadowing_binding_restore() {
    let mut builder = MirBuilder::new();
    builder.push_lexical_scope();

    // Outer x -> BindingId(0)
    let outer_vid = builder.value_gen.next();
    builder.declare_local_in_current_scope("x", outer_vid).unwrap();
    let outer_bid = *builder.binding_map.get("x").unwrap();
    assert_eq!(outer_bid.raw(), 0);

    // Inner x -> BindingId(1)
    builder.push_lexical_scope();
    let inner_vid = builder.value_gen.next();
    builder.declare_local_in_current_scope("x", inner_vid).unwrap();
    let inner_bid = *builder.binding_map.get("x").unwrap();
    assert_eq!(inner_bid.raw(), 1);

    // Scope exit -> restore BindingId(0)
    builder.pop_lexical_scope();
    let restored_bid = *builder.binding_map.get("x").unwrap();
    assert_eq!(restored_bid, outer_bid);
}
```
**Validates**: Shadowing creates new BindingId, scope exit restores outer BindingId.

#### Test 4: Parallel Allocation Independence
```rust
#[test]
fn test_valueid_binding_parallel_allocation() {
    let mut builder = MirBuilder::new();

    let vid0 = builder.value_gen.next();  // ValueId(0)
    let bid0 = builder.allocate_binding_id(); // BindingId(0)
    let vid1 = builder.value_gen.next();  // ValueId(1)
    let bid1 = builder.allocate_binding_id(); // BindingId(1)

    assert_eq!(vid0.0, 0);
    assert_eq!(bid0.raw(), 0);

    // Allocate more ValueIds -> BindingId counter unaffected
    let _ = builder.value_gen.next();
    let _ = builder.value_gen.next();
    let bid2 = builder.allocate_binding_id();
    assert_eq!(bid2.raw(), 2); // Still sequential

    // Allocate more BindingIds -> ValueId counter unaffected
    let _ = builder.allocate_binding_id();
    let _ = builder.allocate_binding_id();
    let vid2 = builder.value_gen.next();
    assert_eq!(vid2.0, 4); // Continues from where we left off
}
```
**Validates**: ValueId and BindingId allocation are completely independent.

### 4.2 Existing Smoke Tests

Phase 74 is **infrastructure-only** - no production code uses BindingId yet.
Existing smoke tests validate:
- ✅ No regressions in ValueId allocation
- ✅ Lexical scope restoration still works
- ✅ Shadowing behavior unchanged

---

## 5. Next Steps (Phase 75+)

### Phase 75: Pilot Integration
- **Goal**: Use BindingId in a single isolated component (e.g., local variable tracking)
- **Files**: Select 1-2 files to pilot BindingId usage (shadowing detection)
- **Validation**: Smoke tests confirm behavior unchanged, BindingId logged correctly

### Phase 76: Promotion
- **Goal**: Expand BindingId usage to critical paths (ScopeManager migration)
- **Files**: Migrate `declare_local_in_current_scope()` users to BindingId-aware APIs
- **Validation**: Full regression suite + shadowing edge cases

### Phase 77: Expansion
- **Goal**: Complete BindingId integration across MirBuilder
- **Files**: Replace remaining `variable_map.get()` calls with BindingId-aware equivalents
- **Validation**: Production smoke tests, performance benchmarks

---

## 6. Migration Checklist

### Phase 74 (Infrastructure) ✅
- [x] `src/mir/binding_id.rs` created
- [x] `src/mir/mod.rs` exports BindingId
- [x] `MirBuilder` fields added (next_binding_id, binding_map)
- [x] `allocate_binding_id()` method implemented
- [x] Lexical scope restoration extended
- [x] Unit tests written (4 tests)
- [x] Documentation completed

### Phase 75 (Pilot) - TODO
- [ ] Identify 1-2 files for pilot integration
- [ ] Add BindingId usage in isolated component
- [ ] Smoke tests validate behavior unchanged
- [ ] BindingId logging/debugging enabled

### Phase 76 (Promotion) - TODO
- [ ] Migrate ScopeManager to BindingId
- [ ] Update `declare_local_in_current_scope()` callers
- [ ] Full regression suite passes
- [ ] Shadowing edge cases tested

### Phase 77 (Expansion) - TODO
- [ ] Replace `variable_map.get()` with BindingId-aware APIs
- [ ] Production smoke tests pass
- [ ] Performance benchmarks show no regressions
- [ ] Documentation updated for public API

---

## 7. Risks and Mitigations

### Risk 1: Forgotten Restoration
**Issue**: Scope exit might forget to restore binding_map.
**Mitigation**: Symmetric logic in `pop_lexical_scope()` (ValueId + BindingId both restored).

### Risk 2: Allocation Divergence
**Issue**: BindingId allocation might diverge from expected sequence.
**Mitigation**: Unit test `test_binding_allocation_sequential()` validates monotonic increment.

### Risk 3: Production Impact
**Issue**: Infrastructure code might accidentally affect production paths.
**Mitigation**: Phase 74 is **allocation-only** (no production code uses BindingId), smoke tests confirm zero impact.

### Risk 4: Overflow
**Issue**: BindingId counter might overflow.
**Mitigation**: `debug_assert!` checks in `allocate_binding_id()` and `BindingId::next()`.

---

## 8. Conclusion

Phase 74 establishes a **production-ready BindingId infrastructure** with:
- ✅ **Zero production impact** (infrastructure-only, no behavior changes)
- ✅ **Parallel allocation** (ValueId and BindingId independent)
- ✅ **Scope restoration** (shadowing correctly handled)
- ✅ **Full test coverage** (4 unit tests + existing smoke tests)

**Ready for Phase 75 pilot integration** with confidence that the foundation is solid.

---

## Appendix A: File Diff Summary

```
src/mir/binding_id.rs                         +130 lines (new file)
src/mir/mod.rs                                 +2 lines (exports)
src/mir/builder.rs                             +85 lines (fields + methods + tests)
src/mir/builder/vars/lexical_scope.rs          +30 lines (restoration logic)
docs/development/current/main/phase74-*.md     +300 lines (this doc)
Total:                                         +547 lines
```

---

## Appendix B: References

- **Phase 73 PoC**: Validated BindingId design feasibility
- **Phase 25.1**: BTreeMap determinism strategy (applied to binding_map)
- **LANGUAGE_REFERENCE_2025**: Shadowing semantics (lexical scoping rules)
- **MirBuilder architecture**: ValueId allocation patterns
