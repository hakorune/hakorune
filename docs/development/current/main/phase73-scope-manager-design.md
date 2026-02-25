# Phase 73: JoinIR ScopeManager → BindingId-Based Design

**Status**: Design Phase (No Production Code Changes)
**Date**: 2025-12-13
**Purpose**: SSOT document for migrating JoinIR lowering's name-based lookup to BindingId-based scope management

---

## Executive Summary

### Problem
JoinIR lowering currently uses **name-based variable lookup** (`String → ValueId` maps) while MIR builder uses **BindingId-based lexical scope tracking** (Phase 68-69). This mismatch creates potential bugs:

1. **Shadowing Confusion**: Same variable name in nested scopes can reference different bindings
2. **Future Bug Source**: As lexical scope becomes more sophisticated, name-only lookup will break
3. **Inconsistent Mental Model**: Developers must track two different scope systems

### Solution Direction
Introduce **BindingId** into JoinIR lowering's scope management to align with MIR's lexical scope model.

### Non-Goal (Phase 73)
- ❌ No production code changes
- ❌ No breaking changes to existing APIs
- ✅ Design-only: Document current state, proposed architecture, migration path

---

## Current State Analysis

### 1. MIR Builder: BindingId + LexicalScope (Phase 68-69)

**Location**: `src/mir/builder/vars/lexical_scope.rs`

**Key Structures**:
```rust
// Conceptual model (from ast_analyzer.rs - dev-only)
struct BindingId(u32);  // Unique ID for each variable binding

struct LexicalScopeFrame {
    declared: BTreeSet<String>,              // Names declared in this scope
    restore: BTreeMap<String, Option<ValueId>>, // Shadowing restoration map
}
```

**How It Works**:
1. Each `local x` declaration creates a new **binding** with unique BindingId
2. LexicalScopeGuard tracks scope entry/exit via RAII
3. On scope exit, shadowed bindings are restored via `restore` map
4. `variable_map: HashMap<String, ValueId>` is the SSA resolution map (name → current ValueId)

**Shadowing Example**:
```nyash
local x = 1;        // BindingId(0) → ValueId(5)
{
    local x = 2;    // BindingId(1) → ValueId(10) (shadows BindingId(0))
    print(x);       // Resolves to ValueId(10)
}
print(x);           // Restores to ValueId(5)
```

**Key Insight**: MIR builder uses **name → ValueId** for SSA conversion, but **BindingId** for scope tracking (declared/restore).

---

### 2. JoinIR Lowering: Name-Based Lookup (Current)

**Location**: `src/mir/join_ir/lowering/`

**Key Structures**:

#### 2.1 `ConditionEnv` (condition_env.rs)
```rust
pub struct ConditionEnv {
    name_to_join: BTreeMap<String, ValueId>,  // Loop params + condition-only vars
    captured: BTreeMap<String, ValueId>,       // Captured function-scoped vars
}
```
- Maps variable **names** to JoinIR-local ValueIds
- Used for loop condition lowering (`i < n`, `p < s.length()`)

#### 2.2 `LoopBodyLocalEnv` (loop_body_local_env.rs)
```rust
pub struct LoopBodyLocalEnv {
    locals: BTreeMap<String, ValueId>,  // Body-local variables
}
```
- Maps body-local variable **names** to ValueIds
- Example: `local temp = i * 2` inside loop body

#### 2.3 `CarrierInfo` (carrier_info.rs)
```rust
pub struct CarrierInfo {
    loop_var_name: String,
    loop_var_id: ValueId,
    carriers: Vec<CarrierVar>,
    promoted_loopbodylocals: Vec<String>,  // Phase 224: Promoted variable names
}

pub struct CarrierVar {
    name: String,
    host_id: ValueId,   // HOST function's ValueId
    join_id: Option<ValueId>,  // JoinIR-local ValueId
    role: CarrierRole,
    init: CarrierInit,
}
```
- Tracks carrier variables (loop state, condition-only)
- Uses **naming convention** for promoted variables:
  - DigitPos pattern: `"digit_pos"` → `"is_digit_pos"`
  - Trim pattern: `"ch"` → `"is_ch_match"`
- Relies on **string matching** (`resolve_promoted_join_id`)

#### 2.4 `ScopeManager` Trait (scope_manager.rs - Phase 231)
```rust
pub trait ScopeManager {
    fn lookup(&self, name: &str) -> Option<ValueId>;
    fn scope_of(&self, name: &str) -> Option<VarScopeKind>;
}

pub struct Pattern2ScopeManager<'a> {
    condition_env: &'a ConditionEnv,
    loop_body_local_env: Option<&'a LoopBodyLocalEnv>,
    captured_env: Option<&'a CapturedEnv>,
    carrier_info: &'a CarrierInfo,
}
```

**Lookup Order** (Pattern2ScopeManager):
1. ConditionEnv (loop var, carriers, condition-only)
2. LoopBodyLocalEnv (body-local variables)
3. CapturedEnv (function-scoped captured variables)
4. Promoted LoopBodyLocal → Carrier (via naming convention)

**Current Issues**:
- ✅ **Works for current patterns**: No shadowing within JoinIR fragments
- ⚠️ **Fragile**: Relies on **naming convention** (`is_digit_pos`) and **string matching**
- ⚠️ **Shadowing-Unaware**: If same name appears in multiple scopes, last match wins
- ⚠️ **Mismatch with MIR**: MIR uses BindingId for shadowing, JoinIR uses name-only

---

### 3. Where Shadowing Can Go Wrong

#### 3.1 Current Patterns (Safe for Now)
- **Pattern 1-4**: No shadowing within single JoinIR fragment
- **Carrier promotion**: Naming convention avoids conflicts (`digit_pos` → `is_digit_pos`)
- **Captured vars**: Function-scoped, no re-declaration

#### 3.2 Future Risks
**Scenario**: Nested loops with shadowing
```nyash
local i = 0;
loop(i < 10) {
    local i = i * 2;  // BindingId(1) shadows BindingId(0)
    print(i);         // Which ValueId does ScopeManager return?
}
```

**Current Behavior**: `ScopeManager::lookup("i")` would return the **first match** in ConditionEnv, ignoring inner scope.

**Expected Behavior**: Should respect lexical scope like MIR builder does.

#### 3.3 Promoted Variable Naming Brittleness
```rust
// CarrierInfo::resolve_promoted_join_id (lines 432-464)
let candidates = [
    format!("is_{}", original_name),       // DigitPos pattern
    format!("is_{}_match", original_name), // Trim pattern
];
for carrier_name in &candidates {
    if let Some(carrier) = self.carriers.iter().find(|c| c.name == *carrier_name) {
        return carrier.join_id;
    }
}
```
- **Fragile**: Relies on string prefixes (`is_`, `is_*_match`)
- **Not Future-Proof**: New patterns require new naming conventions
- **BindingId Alternative**: Store original BindingId → promoted BindingId mapping

---

## Proposed Architecture

### Phase 73 Goals
1. **Document** the BindingId-based design
2. **Identify** minimal changes needed
3. **Define** migration path (phased approach)
4. **No production code changes** (design-only)

---

### Design Option A: Parallel BindingId Layer (Recommended)

**Strategy**: Add BindingId alongside existing name-based lookup, gradually migrate.

#### A.1 Enhanced ConditionEnv
```rust
pub struct ConditionEnv {
    // Phase 73: Legacy name-based (keep for backward compatibility)
    name_to_join: BTreeMap<String, ValueId>,
    captured: BTreeMap<String, ValueId>,

    // Phase 73+: NEW - BindingId-based tracking
    binding_to_join: BTreeMap<BindingId, ValueId>,  // BindingId → JoinIR ValueId
    name_to_binding: BTreeMap<String, BindingId>,   // Name → current BindingId (for shadowing)
}
```

**Benefits**:
- ✅ Backward compatible (legacy code uses `name_to_join`)
- ✅ Gradual migration (new code uses `binding_to_join`)
- ✅ Shadowing-aware (`name_to_binding` tracks current binding)

**Implementation Path**:
1. Add `binding_to_join` and `name_to_binding` fields (initially empty)
2. Update `get()` to check `binding_to_join` first, fall back to `name_to_join`
3. Migrate one pattern at a time (Pattern 1 → 2 → 3 → 4)
4. Remove legacy fields after full migration

---

#### A.2 Enhanced CarrierInfo
```rust
pub struct CarrierVar {
    name: String,
    host_id: ValueId,
    join_id: Option<ValueId>,
    role: CarrierRole,
    init: CarrierInit,

    // Phase 73+: NEW
    host_binding: Option<BindingId>,  // HOST function's BindingId
}

pub struct CarrierInfo {
    loop_var_name: String,
    loop_var_id: ValueId,
    carriers: Vec<CarrierVar>,
    trim_helper: Option<TrimLoopHelper>,

    // Phase 73+: Replace string list with BindingId map
    promoted_bindings: BTreeMap<BindingId, BindingId>,  // Original → Promoted
}
```

**Benefits**:
- ✅ No more naming convention hacks (`is_digit_pos`, `is_ch_match`)
- ✅ Direct BindingId → BindingId mapping for promoted variables
- ✅ Type-safe promotion tracking

**Migration**:
```rust
// Phase 73+: Promoted variable resolution
fn resolve_promoted_binding(&self, original: BindingId) -> Option<BindingId> {
    self.promoted_bindings.get(&original).copied()
}

// Legacy fallback (Phase 73 transition only)
fn resolve_promoted_join_id(&self, name: &str) -> Option<ValueId> {
    // OLD: String matching
    // NEW: BindingId lookup
}
```

---

#### A.3 Enhanced ScopeManager
```rust
pub trait ScopeManager {
    // Phase 73+: NEW - BindingId-based lookup
    fn lookup_binding(&self, binding: BindingId) -> Option<ValueId>;

    // Legacy (keep for backward compatibility)
    fn lookup(&self, name: &str) -> Option<ValueId>;
    fn scope_of(&self, name: &str) -> Option<VarScopeKind>;
}

pub struct Pattern2ScopeManager<'a> {
    condition_env: &'a ConditionEnv,
    loop_body_local_env: Option<&'a LoopBodyLocalEnv>,
    captured_env: Option<&'a CapturedEnv>,
    carrier_info: &'a CarrierInfo,

    // Phase 73+: NEW - BindingId context from HOST
    host_bindings: Option<&'a BTreeMap<String, BindingId>>,
}

impl<'a> ScopeManager for Pattern2ScopeManager<'a> {
    fn lookup_binding(&self, binding: BindingId) -> Option<ValueId> {
        // 1. Check condition_env.binding_to_join
        if let Some(id) = self.condition_env.binding_to_join.get(&binding) {
            return Some(*id);
        }

        // 2. Check promoted bindings
        if let Some(promoted) = self.carrier_info.resolve_promoted_binding(binding) {
            return self.condition_env.binding_to_join.get(&promoted).copied();
        }

        // 3. Fallback to legacy name-based lookup (transition only)
        None
    }
}
```

---

### Design Option B: Full BindingId Replacement (Not Recommended for Phase 73)

**Strategy**: Replace all name-based maps with BindingId-based maps in one go.

**Why Not Recommended**:
- ❌ High risk (breaks existing code)
- ❌ Requires simultaneous changes to MIR builder, JoinIR lowering, all patterns
- ❌ Hard to rollback if issues arise
- ❌ Violates Phase 73 constraint (design-only)

**When to Use**: Phase 80+ (after Option A migration complete)

---

## Integration with MIR Builder

### Challenge: BindingId Source of Truth

**Question**: Where do BindingIds come from in JoinIR lowering?

**Answer**: MIR builder's `variable_map` + `LexicalScopeFrame`

#### Current Flow (Phase 73)
1. **MIR builder** maintains `variable_map: HashMap<String, ValueId>`
2. **JoinIR lowering** receives `variable_map` and creates `ConditionEnv`
3. **ConditionEnv** uses names as keys (no BindingId tracking)

#### Proposed Flow (Phase 73+)
1. **MIR builder** maintains:
   - `variable_map: HashMap<String, ValueId>` (SSA conversion)
   - `binding_map: HashMap<String, BindingId>` (NEW - lexical scope tracking)
2. **JoinIR lowering** receives both maps
3. **ConditionEnv** builds:
   - `name_to_join: BTreeMap<String, ValueId>` (legacy)
   - `binding_to_join: BTreeMap<BindingId, ValueId>` (NEW - from binding_map)

---

### Required MIR Builder Changes

#### 1. Add `binding_map` to MirBuilder
```rust
// src/mir/builder.rs
pub struct MirBuilder {
    pub variable_map: HashMap<String, ValueId>,

    // Phase 73+: NEW
    pub binding_map: HashMap<String, BindingId>,  // Current BindingId per name
    next_binding_id: u32,

    // Existing fields...
}
```

#### 2. Update `declare_local_in_current_scope`
```rust
// src/mir/builder/vars/lexical_scope.rs
pub fn declare_local_in_current_scope(
    &mut self,
    name: &str,
    value: ValueId,
) -> Result<BindingId, String> {  // Phase 73+: Return BindingId
    let frame = self.lexical_scope_stack.last_mut()
        .ok_or("COMPILER BUG: local declaration outside lexical scope")?;

    // Allocate new BindingId
    let binding = BindingId(self.next_binding_id);
    self.next_binding_id += 1;

    if frame.declared.insert(name.to_string()) {
        let previous_value = self.variable_map.get(name).copied();
        let previous_binding = self.binding_map.get(name).copied();  // Phase 73+
        frame.restore.insert(name.to_string(), previous_value);
        frame.restore_bindings.insert(name.to_string(), previous_binding);  // Phase 73+
    }

    self.variable_map.insert(name.to_string(), value);
    self.binding_map.insert(name.to_string(), binding);  // Phase 73+
    Ok(binding)
}
```

#### 3. Update `pop_lexical_scope`
```rust
pub fn pop_lexical_scope(&mut self) {
    let frame = self.lexical_scope_stack.pop()
        .expect("COMPILER BUG: pop_lexical_scope without push_lexical_scope");

    for (name, previous) in frame.restore {
        match previous {
            Some(prev_id) => { self.variable_map.insert(name, prev_id); }
            None => { self.variable_map.remove(&name); }
        }
    }

    // Phase 73+: Restore BindingIds
    for (name, previous_binding) in frame.restore_bindings {
        match previous_binding {
            Some(prev_binding) => { self.binding_map.insert(name, prev_binding); }
            None => { self.binding_map.remove(&name); }
        }
    }
}
```

---

## Migration Path (Phased Approach)

### Phase 73 (Current - Design Only)
- ✅ This document (SSOT)
- ✅ No production code changes
- ✅ Define acceptance criteria for Phase 74+

---

### Phase 74 (Infrastructure)
**Goal**: Add BindingId infrastructure without breaking existing code

**Tasks**:
1. Add `binding_map` to `MirBuilder` (default empty)
2. Add `binding_to_join` to `ConditionEnv` (default empty)
3. Add `host_binding` to `CarrierVar` (default None)
4. Update `declare_local_in_current_scope` to return `BindingId`
5. Add `#[cfg(feature = "normalized_dev")]` gated BindingId tests

**Acceptance Criteria**:
- ✅ All existing tests pass (no behavior change)
- ✅ `binding_map` populated during local declarations
- ✅ BindingId allocator works (unit tests)

---

### Phase 75 (Pattern 1 Pilot)
**Goal**: Migrate Pattern 1 (Simple While Minimal) to use BindingId

**Why Pattern 1?**
- Simplest pattern (no carriers, no shadowing)
- Low risk (easy to validate)
- Proves BindingId integration works

**Tasks**:
1. Update `CarrierInfo::from_variable_map` to accept `binding_map`
2. Update `Pattern1ScopeManager` (if exists) to use `lookup_binding`
3. Add E2E test with Pattern 1 + BindingId

**Acceptance Criteria**:
- ✅ Pattern 1 tests pass with BindingId lookup
- ✅ Legacy name-based lookup still works (fallback)

---

### Phase 76 (Pattern 2 - Carrier Promotion)
**Goal**: Migrate Pattern 2 (with promoted LoopBodyLocal) to BindingId

**Challenges**:
- Promoted variable tracking (`digit_pos` → `is_digit_pos`)
- Replace `promoted_loopbodylocals: Vec<String>` with `promoted_bindings: BTreeMap<BindingId, BindingId>`

**Tasks**:
1. Add `promoted_bindings` to `CarrierInfo`
2. Update `resolve_promoted_join_id` to use BindingId
3. Update Pattern 2 lowering to populate `promoted_bindings`

**Acceptance Criteria**:
- ✅ Pattern 2 tests pass (DigitPos pattern)
- ✅ No more naming convention hacks (`is_*`, `is_*_match`)

---

### Phase 77 (Pattern 3 & 4)
**Goal**: Complete migration for remaining patterns

**Tasks**:
1. Migrate Pattern 3 (multi-carrier)
2. Migrate Pattern 4 (generic case A)
3. Remove legacy `name_to_join` fallbacks

**Acceptance Criteria**:
- ✅ All patterns use BindingId exclusively
- ✅ Legacy code paths removed
- ✅ Full test suite passes

---

### Phase 78+ (Future Enhancements)
**Optional Improvements**:
- Nested loop shadowing support
- BindingId-based ownership analysis (Phase 63 integration)
- BindingId-based SSA optimization (dead code elimination)

---

## Acceptance Criteria (Phase 73)

### Design Document Complete
- ✅ Current state analysis (MIR + JoinIR scope systems)
- ✅ Proposed architecture (Option A: Parallel BindingId Layer)
- ✅ Integration points (MirBuilder changes)
- ✅ Migration path (Phases 74-77)

### No Production Code Changes
- ✅ No changes to `src/mir/builder.rs`
- ✅ No changes to `src/mir/join_ir/lowering/*.rs`
- ✅ Optional: Minimal PoC in `#[cfg(feature = "normalized_dev")]`

### Stakeholder Review
- ⏰ User review (confirm design makes sense)
- ⏰ Identify any missed edge cases

---

## Open Questions

### Q1: Should BindingId be global or per-function?
**Current Assumption**: Per-function (like ValueId)

**Reasoning**:
- Each function has independent binding scope
- No cross-function binding references
- Simpler allocation (no global state)

**Alternative**: Global BindingId pool (for Phase 63 ownership analysis)

---

### Q2: How to handle captured variables?
**Current**: `CapturedEnv` uses names, marks as immutable

**Proposed**: Add `binding_id` to `CapturedVar`
```rust
pub struct CapturedVar {
    name: String,
    host_id: ValueId,
    host_binding: BindingId,  // Phase 73+
    is_immutable: bool,
}
```

---

### Q3: Performance impact of dual maps?
**Concern**: `binding_to_join` + `name_to_join` doubles memory

**Mitigation**:
- Phase 74-75: Both maps active (transition)
- Phase 76+: Remove `name_to_join` after migration
- BTreeMap overhead minimal for typical loop sizes (<10 variables)

---

## References

### Related Phases
- **Phase 68-69**: MIR lexical scope + shadowing (existing implementation)
- **Phase 63**: Ownership analysis (dev-only, uses BindingId)
- **Phase 231**: ScopeManager trait (current implementation)
- **Phase 238**: ExprLowerer scope boundaries (design doc)

### Key Files
- `src/mir/builder/vars/lexical_scope.rs` - MIR lexical scope implementation
- `src/mir/join_ir/lowering/scope_manager.rs` - JoinIR ScopeManager trait
- `src/mir/join_ir/lowering/condition_env.rs` - ConditionEnv (name-based)
- `src/mir/join_ir/lowering/carrier_info.rs` - CarrierInfo (name-based promotion)
- `src/mir/join_ir/ownership/ast_analyzer.rs` - BindingId usage (dev-only)

---

## Appendix: Example Scenarios

### A1: Shadowing Handling (Future)
```nyash
local sum = 0;
loop(i < n) {
    local sum = i * 2;  // BindingId(1) shadows BindingId(0)
    total = total + sum;
}
print(sum);  // BindingId(0) restored
```

**Expected Behavior**:
- Inner `sum` has BindingId(1)
- ScopeManager resolves `sum` to BindingId(1) inside loop
- Outer `sum` (BindingId(0)) restored after loop

---

### A2: Promoted Variable Tracking (Current)
```nyash
loop(p < len) {
    local digit_pos = digits.indexOf(ch);
    if digit_pos < 0 { break; }  // Promoted to carrier
}
```

**Current (Phase 73)**: String-based promotion
- `promoted_loopbodylocals: ["digit_pos"]`
- `resolve_promoted_join_id("digit_pos")` → searches for `"is_digit_pos"`

**Proposed (Phase 76+)**: BindingId-based promotion
- `promoted_bindings: { BindingId(5) → BindingId(10) }`
- `lookup_binding(BindingId(5))` → returns ValueId from BindingId(10)

---

## Conclusion

**Phase 73 Deliverable**: This design document serves as SSOT for BindingId migration.

**Next Steps**:
1. User review and approval
2. Phase 74: Infrastructure implementation (BindingId allocation)
3. Phase 75-77: Gradual pattern migration

**Estimated Total Effort**:
- Phase 73 (design): ✅ Complete
- Phase 74 (infra): 2-3 hours
- Phase 75 (Pattern 1): 1-2 hours
- Phase 76 (Pattern 2): 2-3 hours
- Phase 77 (Pattern 3-4): 2-3 hours
- **Total**: 8-12 hours

**Risk Level**: Low (gradual migration, backward compatible)
