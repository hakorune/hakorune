# Phase 201: JoinValueSpace Design

## 1. Problem Statement

### 1.1 Root Cause (Phase 201-A Analysis)

Pattern 2 frontend と JoinIR lowering の間で ValueId 空間が分離されていないため、衝突が発生している。

```
Pattern 2 Frontend:                    JoinIR Lowering:
┌────────────────────────┐             ┌────────────────────────┐
│ alloc_join_value()     │             │ alloc_value()          │
│ → env['v'] = ValueId(7)│             │ → const 100 dst=ValueId(7)│
└────────────────────────┘             └────────────────────────┘
                    │                             │
                    └─────────── Collision! ──────┘
                                │
                                ▼
                    remapper → Both → ValueId(12)
                                │
                                ▼
                    PHI corruption: %12 = phi [...], %12 = const 100
```

### 1.2 Affected Components

| Component | Current ValueId Source | Issue |
|-----------|------------------------|-------|
| ConditionEnv | `alloc_join_value()` | Param IDs may collide with local IDs |
| CarrierInfo.join_id | `alloc_join_value()` | Same allocator as ConditionEnv |
| CapturedEnv | `alloc_join_value()` | Same allocator |
| Pattern lowerers | `alloc_value()` (starts from 0) | Collides with param IDs |
| LoopHeaderPhiBuilder | Uses remapped IDs | PHI dst may be overwritten |

## 2. Solution: JoinValueSpace

### 2.1 Design Goals

1. **Single Source of Truth**: All JoinIR ValueId allocation goes through one box
2. **Disjoint Regions**: Param IDs, Local IDs, and PHI dst never overlap
3. **Contract Enforcement**: Debug-mode assertions catch violations
4. **Backward Compatible**: Existing APIs continue to work

### 2.2 ValueId Space Layout

```
JoinValueSpace Memory Layout:

 0          100        1000                     u32::MAX
 ├──────────┼──────────┼──────────────────────────┤
 │  PHI     │  Param   │       Local             │
 │  Reserved│  Region  │       Region            │
 └──────────┴──────────┴──────────────────────────┘

PHI Reserved (0-99):
  - Pre-reserved for LoopHeader PHI dst
  - reserve_phi(id) marks specific IDs

Param Region (100-999):
  - alloc_param() allocates here
  - Used by: ConditionEnv, CarrierInfo.join_id, CapturedEnv

Local Region (1000+):
  - alloc_local() allocates here
  - Used by: Pattern lowerers (Const, BinOp, etc.)
```

### 2.3 API Design

```rust
/// Single source of truth for JoinIR ValueId allocation
pub struct JoinValueSpace {
    /// Next available param ID (starts at PARAM_BASE)
    next_param: u32,
    /// Next available local ID (starts at LOCAL_BASE)
    next_local: u32,
    /// Reserved PHI dst IDs (debug verification only)
    reserved_phi: HashSet<u32>,
}

impl JoinValueSpace {
    /// Create a new JoinValueSpace with default regions
    pub fn new() -> Self;

    /// Allocate a parameter ValueId (for ConditionEnv, CarrierInfo, etc.)
    /// Returns ValueId in Param Region (100-999)
    pub fn alloc_param(&mut self) -> ValueId;

    /// Allocate a local ValueId (for Const, BinOp, etc. in lowerers)
    /// Returns ValueId in Local Region (1000+)
    pub fn alloc_local(&mut self) -> ValueId;

    /// Reserve a PHI dst ValueId (called by PHI builder before allocation)
    /// No allocation - just marks the ID as reserved for PHI use
    pub fn reserve_phi(&mut self, id: ValueId);

    /// Check if a ValueId is in a specific region (debug use)
    pub fn region_of(&self, id: ValueId) -> Region;

    /// Verify no overlap between regions (debug assertion)
    #[cfg(debug_assertions)]
    pub fn verify_no_overlap(&self) -> Result<(), String>;
}

pub enum Region {
    PhiReserved,
    Param,
    Local,
    Unknown,
}
```

### 2.4 Constants

```rust
// Region boundaries (can be tuned based on actual usage)
const PHI_MAX: u32 = 99;      // PHI dst range: 0-99
const PARAM_BASE: u32 = 100;  // Param range: 100-999
const LOCAL_BASE: u32 = 1000; // Local range: 1000+
```

## 3. Integration Points

### 3.1 ConditionEnv / CapturedEnv

```rust
// Before (collision-prone):
let mut env = ConditionEnv::new();
let join_id = alloc_join_value(); // Could be 0, 1, 2...
env.insert("i".to_string(), join_id);

// After (JoinValueSpace-based):
let mut space = JoinValueSpace::new();
let mut env = ConditionEnv::new();
let join_id = space.alloc_param(); // Always 100+
env.insert("i".to_string(), join_id);
```

### 3.2 CarrierInfo.join_id

```rust
// Before:
carrier.join_id = Some(alloc_join_value()); // Could collide

// After:
carrier.join_id = Some(space.alloc_param()); // Safe in Param region
```

### 3.3 Pattern Lowerers

```rust
// Before (loop_with_break_minimal.rs):
let mut value_counter = 0u32;
let mut alloc_value = || {
    let id = ValueId(value_counter);
    value_counter += 1;
    id
}; // Starts from 0 - collides with env!

// After:
let mut alloc_value = || space.alloc_local(); // Starts from 1000
```

### 3.4 LoopHeaderPhiBuilder

```rust
// Before merge:
space.reserve_phi(phi_dst); // Mark PHI dst as reserved

// After finalization:
// verify_no_overlap() checks no local overwrote PHI dst
```

## 4. Migration Plan

### Phase 201-2: JoinValueSpace Box

1. Create `join_value_space.rs` in `src/mir/join_ir/lowering/`
2. Implement struct and core methods
3. Add unit tests for region separation
4. No integration yet - box only

### Phase 201-3: Param Region Migration

1. Modify `pattern2_with_break.rs` to pass JoinValueSpace
2. Update ConditionEnvBuilder to use `alloc_param()`
3. Update CarrierInfo initialization to use `alloc_param()`
4. Verify: Param IDs are now 100+

### Phase 201-4: PHI Reservation

1. Modify LoopHeaderPhiBuilder to call `reserve_phi()`
2. Add verification in merge/mod.rs
3. Verify: PHI dst is protected from overwrite

### Phase 201-5: Local Region Migration

1. Modify all pattern lowerers to use `alloc_local()`
2. Files: `loop_with_break_minimal.rs`, `loop_with_continue_minimal.rs`, etc.
3. Verify: Local IDs are now 1000+

### Phase 201-6: Testing

1. Run all existing tests (no regression)
2. Add `phase201_valueid_collision.hako` test
3. Verify `phase200d_capture_minimal.hako` outputs 30 (not 110)

## 5. Design Decisions

### 5.1 Why Fixed Regions?

Alternative: Dynamic start offset based on env.max_value_id()
- Pro: No wasted ID space
- Con: Complex, error-prone, requires coordination

Fixed regions are simpler:
- Clear boundaries (100, 1000)
- Easy to debug (看ID值就知道是Param还是Local)
- No coordination needed between allocators

### 5.2 Why reserve_phi() Instead of alloc_phi()?

PHI dst IDs come from MirBuilder (host side), not JoinValueSpace.
JoinValueSpace only needs to know "don't overwrite these IDs".
Hence `reserve_phi()` is a marker, not an allocator.

### 5.3 Relation to value_id_ranges.rs

`value_id_ranges.rs` is for **module-level isolation** (min_loop, skip_ws, etc.)
Each module gets a large fixed range (2000 IDs).

`JoinValueSpace` is for **intra-lowering isolation** (param vs local vs PHI).
It operates within a single lowering call.

They are complementary:
- Module-level: value_id_ranges.rs
- Intra-lowering: JoinValueSpace

## 6. Success Criteria

1. `phase200d_capture_minimal.hako` outputs **30** (not 110)
2. All existing tests pass (no regression)
3. Debug build asserts on ValueId collision
4. Architecture doc updated with JoinValueSpace section

## 7. File Changes Summary

| File | Change |
|------|--------|
| `join_value_space.rs` (NEW) | JoinValueSpace struct + methods |
| `condition_env.rs` | No change (env is storage, not allocator) |
| `condition_env_builder.rs` | Use JoinValueSpace.alloc_param() |
| `carrier_info.rs` | No change (storage only) |
| `pattern2_with_break.rs` | Pass JoinValueSpace, use alloc_param() |
| `loop_with_break_minimal.rs` | Use JoinValueSpace.alloc_local() |
| `loop_with_continue_minimal.rs` | Use JoinValueSpace.alloc_local() |
| `loop_with_if_phi_minimal.rs` | Use JoinValueSpace.alloc_local() |
| `loop_header_phi_builder.rs` | Call reserve_phi() |
| `merge/mod.rs` | Create JoinValueSpace, pass down |

## 8. Implementation Status (2025-12-09)

### 8.1 Completed Tasks

| Task | Status | Notes |
|------|--------|-------|
| 201-1: Design document | ✅ Complete | This document |
| 201-2: JoinValueSpace box | ✅ Complete | 10 unit tests, all passing |
| 201-3: Param region migration | ✅ Complete | ConditionEnvBuilder v2 API |
| 201-4: PHI reservation | ✅ Skipped | Not needed - lowerer uses ConditionEnv's ValueIds directly |
| 201-5: Local region migration | ✅ Complete | Pattern 2 lowerer updated |
| 201-6: Testing | ✅ Complete | 821 tests pass, E2E verified |
| 201-7: Documentation | ✅ Complete | This section |

### 8.2 Key Implementation Insight

The original plan assumed lowerers would allocate ALL ValueIds from JoinValueSpace.
The actual implementation is smarter:

```
Original Plan:
  - main params: alloc_local() → 1000+
  - loop_step params: alloc_local() → 1000+
  - intermediates: alloc_local() → 1000+

Actual Implementation:
  - main params: alloc_local() → 1000+ (entry point slots)
  - loop_step params: USE ConditionEnv's ValueIds → 100+ (CRITICAL!)
  - intermediates: alloc_local() → 1000+
```

Why? Because `lower_condition_to_joinir` uses ConditionEnv to resolve variable names.
If `loop_step.params[0]` (i_param) doesn't match `env.get("i")`, condition lowering fails.

### 8.3 Test Results

```
# Library tests
$ cargo test --release --lib
test result: ok. 821 passed; 0 failed

# E2E tests
$ ./target/release/hakorune apps/tests/phase200d_capture_minimal.hako
30  # ✓ Expected output

$ ./target/release/hakorune apps/tests/loop_continue_pattern4.hako
25  # ✓ Expected output

$ ./target/release/hakorune apps/tests/loop_continue_multi_carrier.hako
100
10  # ✓ Expected output (two carriers)
```

## 9. References

- Phase 201-A analysis: carrier PHI dst overwrite bug
- joinir-architecture-overview.md: JoinIR invariants
- value_id_ranges.rs: Module-level ValueId isolation
- Commits:
  - `1af53f82` feat(joinir): Phase 201 JoinValueSpace - unified ValueId allocation
  - `17152baf` feat(joinir): Phase 201-5 Pattern 2 lowerer uses JoinValueSpace
Status: Active  
Scope: Join Value Space 設計（JoinIR v2 ライン）
