# Phase 142: Canonicalizer Route-Shape Extension

## Status
- P0: ✅ Complete (trim leading/trailing)
- P1: ✅ Complete (continue pattern)

## P0: trim leading/trailing (COMPLETE)

### Objective
Extend Canonicalizer to recognize trim leading/trailing patterns, enabling proper routing through the normalized loop pipeline.

### Target Patterns
- `tools/selfhost/test_pattern3_trim_leading.hako` - `start = start + 1` pattern
- `tools/selfhost/test_pattern3_trim_trailing.hako` - `end = end - 1` pattern

### Accepted Criteria (All Met ✅)
- ✅ Canonicalizer creates Skeleton for trim_leading/trailing
- ✅ `decision.chosen == LoopBreak` (`Pattern2Break` at the time, ExitContract priority)
- ✅ `decision.missing_caps == []` (no missing capabilities)
- ✅ Strict parity green (NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1)
- ✅ Default behavior unchanged
- ✅ Unit tests added
- ✅ Documentation created

### Implementation Summary

#### 1. Route-Shape Recognizer Generalization
**Current module surface**: `src/mir/builder/control_flow/plan/ast_feature_extractor.rs`

**Historical path token**: `src/mir/builder/control_flow/joinir/patterns/ast_feature_extractor.rs`

**Changes**:
- Extended `detect_skip_whitespace_shape()` to accept both `+` and `-` operators
- Added support for negative deltas (e.g., `-1` for `end = end - 1`)
- Maintained backward compatibility with existing skip_whitespace patterns

**Key Logic**:
```rust
// Phase 142 P0: Accept both Add (+1) and Subtract (-1)
let op_multiplier = match operator {
    BinaryOperator::Add => 1,
    BinaryOperator::Subtract => -1,
    _ => return None,
};

// Calculate delta with sign (e.g., +1 or -1)
let delta = const_val * op_multiplier;
```

**Recognized Patterns**:
- skip_whitespace: `p = p + 1` (delta = +1)
- trim_leading: `start = start + 1` (delta = +1)
- trim_trailing: `end = end - 1` (delta = -1)

#### 2. Unit Tests
**File**: `src/mir/loop_canonicalizer/canonicalizer.rs`

**Added Tests**:
- `test_trim_leading_pattern_recognized()` - Verifies `start = start + 1` pattern
- `test_trim_trailing_pattern_recognized()` - Verifies `end = end - 1` pattern

**Test Coverage**:
- Skeleton creation
- Carrier slot creation with correct delta (+1 or -1)
- ExitContract setup (has_break=true)
- RoutingDecision (chosen=LoopBreak, missing_caps=[])

**Test Results**:
```
running 2 tests
test mir::loop_canonicalizer::canonicalizer::tests::test_trim_leading_pattern_recognized ... ok
test mir::loop_canonicalizer::canonicalizer::tests::test_trim_trailing_pattern_recognized ... ok

test result: ok. 2 passed; 0 failed; 0 ignored
```

#### 3. Manual Verification
**Strict Parity Check**:
```bash
NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1 ./target/release/hakorune \
  tools/selfhost/test_pattern3_trim_leading.hako
```

**Historical debug output at the time** (trim_leading):
```
[loop_canonicalizer]   Decision: SUCCESS
[loop_canonicalizer]   Chosen pattern: Pattern2Break
[loop_canonicalizer]   Missing caps: []
[choose_pattern_kind/PARITY] OK: canonical and actual agree on Pattern2Break
[loop_canonicalizer/PARITY] OK in function 'main': canonical and actual agree on Pattern2Break
```

**Historical debug output at the time** (trim_trailing):
```
[loop_canonicalizer]   Decision: SUCCESS
[loop_canonicalizer]   Chosen pattern: Pattern2Break
[loop_canonicalizer]   Missing caps: []
[choose_pattern_kind/PARITY] OK: canonical and actual agree on Pattern2Break
[loop_canonicalizer/PARITY] OK in function 'main': canonical and actual agree on Pattern2Break
```

### Design Principles Applied

#### Box-First Modularization
- Extended existing `detect_skip_whitespace_shape()` instead of creating new functions
- Maintained SSOT (Single Source of Truth) architecture
- Preserved delegation pattern through the canonicalizer recognizer wrapper (`route_shape_recognizer.rs`)

#### Incremental Implementation
- Focused on recognizer generalization only
- Did not modify routing or lowering logic
- Kept scope minimal (P0 only)

#### ExitContract Priority
- Route choice determined by ExitContract (has_break=true)
- Routes to `LoopBreak` (historical label: `Pattern2Break`), not `IfPhiJoin`
- Consistent with existing SSOT policy

### Files Modified
1. `src/mir/builder/control_flow/plan/ast_feature_extractor.rs` (+35 lines, improved comments)
   - same historical path token as the Route-Shape Recognizer section above (`src/mir/builder/control_flow/joinir/patterns/ast_feature_extractor.rs`)
2. `src/mir/loop_canonicalizer/canonicalizer.rs` (+178 lines, 2 new tests)

### Statistics
- **Total changes**: +213 lines
- **Unit tests**: 2 new tests (100% pass)
- **Manual tests**: 2 patterns verified (strict parity green)
- **Build status**: ✅ No errors, no warnings (lib)

### SSOT References
- **Design**: `docs/development/current/main/design/loop-canonicalizer.md`
- **JoinIR Architecture**: `docs/development/current/main/joinir-architecture-overview.md`
- **Route-shape detection**: `ast_feature_extractor.rs` (Phase 140-P4-A SSOT)

### Known Limitations
- loop_break variable promotion (A-3 Trim promotion) not yet implemented
- This is expected - Phase 142 P0 only targets recognizer extension
- Promotion will be addressed in future phases

### Next Steps (Future Phases)
- Phase 142 P1: Implement A-3 Trim promotion in Pattern2 handler
- Phase 142 P2: Extend to other route shapes (`IfPhiJoin` / `LoopContinueOnly`)
- Phase 142 P3: Add more complex carrier update patterns

### Verification Commands
```bash
# Unit tests
cargo test --release loop_canonicalizer::canonicalizer::tests::test_trim --lib

# Manual verification (trim_leading)
NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1 ./target/release/hakorune \
  tools/selfhost/test_pattern3_trim_leading.hako

# Manual verification (trim_trailing)
NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1 ./target/release/hakorune \
  tools/selfhost/test_pattern3_trim_trailing.hako
```

### Conclusion
Phase 142 P0 successfully extends the Canonicalizer to recognize trim leading/trailing patterns. The implementation:
- Maintains SSOT architecture
- Passes all unit tests
- Achieves strict parity agreement
- Preserves existing behavior
- Sets foundation for future pattern extensions

All acceptance criteria met. ✅

---

## P1: continue pattern (COMPLETE)

### Objective
Extend Canonicalizer to recognize continue patterns, enabling proper routing through the normalized loop pipeline.

### Target Pattern
- `tools/selfhost/test_pattern4_simple_continue.hako` - Simple continue pattern with carrier update

### Accepted Criteria (All Met ✅)
- ✅ Canonicalizer creates Skeleton for continue pattern
- ✅ `decision.chosen == LoopContinueOnly` (`Pattern4Continue` at the time, router agreement)
- ✅ `decision.missing_caps == []` (no missing capabilities)
- ✅ Strict parity green (NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1)
- ✅ Default behavior unchanged
- ✅ Unit tests added
- ✅ Documentation updated

### Implementation Summary

#### 1. Continue Route-Shape Detection
**Current module surface**: `src/mir/builder/control_flow/plan/ast_feature_extractor.rs`

**Historical path token**: same as above

**Current function**: `detect_continue_shape()`

**Pattern Structure**:
```rust
loop(cond) {
    // ... optional body statements (Body)
    if skip_cond {
        carrier = carrier + const  // Optional update before continue
        continue
    }
    // ... rest of body statements (Rest)
    carrier = carrier + const  // Carrier update
}
```

**Example** (from test_pattern4_simple_continue.hako):
```nyash
loop(i < n) {
  if is_even == 1 {
    i = i + 1  // Update before continue
    continue
  }
  sum = sum + i  // Rest statements
  i = i + 1      // Carrier update
}
```

**Key Logic**:
- Finds if statement containing continue in then_body
- Extracts body statements before the if
- Extracts rest statements after the if
- Detects carrier update (last statement in rest_stmts)
- Returns the continue route-shape tuple with carrier name, delta, body_stmts, and rest_stmts

#### 2. Canonicalizer Integration
**File**: `src/mir/loop_canonicalizer/canonicalizer.rs`

**Changes**:
- Added `try_extract_continue_shape()` call before skip_whitespace check
- Build skeleton with continue pattern structure
- Set `ExitContract` with `has_continue=true, has_break=false`
- Route to `LoopContinueOnly`

**Skeleton Structure**:
1. HeaderCond - Loop condition
2. Body - Optional body statements before continue check
3. Body - Rest statements (excluding carrier update)
4. Update - Carrier update step

#### 3. Historical module re-export chain
**Files Modified** (historical re-export chain at the time):
- same historical re-export lane as the recognizer path token above (`src/mir/builder/control_flow/joinir/patterns/mod.rs`) - Added historical re-export for the continue helper (`detect_continue_pattern`, `ContinuePatternInfo`) at the time
- `src/mir/builder/control_flow/joinir/mod.rs` - Re-export to joinir level
- `src/mir/builder/control_flow/mod.rs` - Re-export to control_flow level
- `src/mir/builder.rs` - Re-export to builder level
- `src/mir/mod.rs` - Re-export to crate level

**Pattern**: Followed existing SSOT pattern from Phase 140-P4-A

#### 4. Route-Shape Recognizer Wrapper
**File**: `src/mir/loop_canonicalizer/route_shape_recognizer.rs`

**New Function**: `try_extract_continue_shape()`
- Delegates to `detect_continue_shape()` from ast_feature_extractor
- Returns tuple: `(carrier_name, delta, body_stmts, rest_stmts)`
- Maintains backward compatibility with existing callsites

#### 5. Unit Tests
**File**: `src/mir/loop_canonicalizer/canonicalizer.rs`

**Added Test**: `test_simple_continue_pattern_recognized()`
- Builds AST: `loop(i < n) { if is_even { i = i + 1; continue } sum = sum + i; i = i + 1 }`
- Verifies skeleton creation with correct structure
- Checks carrier slot (name="i", delta=1)
- Validates ExitContract (has_continue=true, has_break=false)
- Confirms routing decision (LoopContinueOnly, missing_caps=[])

**Test Results**:
```
running 8 tests
test mir::loop_canonicalizer::canonicalizer::tests::test_simple_continue_pattern_recognized ... ok
test result: ok. 8 passed; 0 failed; 0 ignored
```

#### 6. Manual Verification
**Strict Parity Check**:
```bash
NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1 ./target/release/hakorune \
  tools/selfhost/test_pattern4_simple_continue.hako
```

**Historical debug output at the time**:
```
[loop_canonicalizer] Function: main
[loop_canonicalizer]   Skeleton steps: 4
[loop_canonicalizer]   Carriers: 1
[loop_canonicalizer]   Has exits: true
[loop_canonicalizer]   Decision: SUCCESS
[loop_canonicalizer]   Chosen pattern: Pattern4Continue
[loop_canonicalizer]   Missing caps: []
[choose_pattern_kind/PARITY] OK: canonical and actual agree on Pattern4Continue
[loop_canonicalizer/PARITY] OK in function 'main': canonical and actual agree on Pattern4Continue
```

**Status**: ✅ Strict parity green!

### Design Principles Applied

#### Box-First Modularization
- Created dedicated `detect_continue_shape()` function in ast_feature_extractor
- Maintained SSOT architecture with proper re-export chain
- Followed existing pattern from skip_whitespace detection

#### Incremental Implementation
- Focused on route-shape recognition only (P1 scope)
- Did not modify lowering logic (expected promotion errors)
- Kept changes minimal and focused

#### ExitContract Priority
- Route choice determined by ExitContract (has_continue=true, has_break=false)
- Routes to `LoopContinueOnly` (historical label: `Pattern4Continue`)
- Consistent with existing SSOT policy from Phase 137-5

### Files Modified
1. `src/mir/builder/control_flow/plan/ast_feature_extractor.rs` (+167 lines, new function)
   - same historical path token as above
2. `src/mir/loop_canonicalizer/route_shape_recognizer.rs` (+35 lines, wrapper function)
3. `src/mir/loop_canonicalizer/canonicalizer.rs` (+103 lines, continue support + unit test)
4. same historical re-export lane as above (`mod.rs`, +3 lines, re-export)
5. `src/mir/builder/control_flow/joinir/mod.rs` (+3 lines, re-export)
6. `src/mir/builder/control_flow/mod.rs` (+3 lines, re-export)
7. `src/mir/builder.rs` (+2 lines, re-export)
8. `src/mir/mod.rs` (+2 lines, re-export)

### Statistics
- **Total changes**: +318 lines
- **Unit tests**: 1 new test (100% pass)
- **All canonicalizer tests**: 8 passed (100%)
- **Manual tests**: 1 pattern verified (strict parity green)
- **Build status**: ✅ No errors (warnings are pre-existing)

### SSOT References
- **Design**: `docs/development/current/main/design/loop-canonicalizer.md`
- **JoinIR Architecture**: `docs/development/current/main/joinir-architecture-overview.md`
- **Route-shape detection**: `ast_feature_extractor.rs` (Phase 140-P4-A SSOT)

### Known Limitations
- loop_continue_only variable promotion (A-3 Trim, A-4 DigitPos) not yet handling this pattern
- This is expected - Phase 142 P1 only targets recognizer extension
- Promotion will be addressed when loop_continue_only lowering is enhanced

### Next Steps (Future Phases)
- Phase 142 P2: Extend loop_continue_only lowering to handle recognized continue patterns
- Phase 142 P3: Add more complex continue patterns (multiple carriers, nested conditions)

### Verification Commands
```bash
# Unit tests
cargo test --release --lib loop_canonicalizer::canonicalizer::tests::test_simple_continue_pattern_recognized

# All canonicalizer tests
cargo test --release --lib loop_canonicalizer::canonicalizer::tests

# Manual verification
NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1 ./target/release/hakorune \
  tools/selfhost/test_pattern4_simple_continue.hako
```

### Conclusion
Phase 142 P1 successfully extends the Canonicalizer to recognize continue patterns. The implementation:
- Maintains SSOT architecture
- Passes all unit tests (8/8)
- Achieves strict parity agreement with router
- Preserves existing behavior
- Follows existing re-export pattern from Phase 140-P4-A

All acceptance criteria met. ✅

---

## P2: LoopContinueOnly Lowering Extension (historical design snapshot; numbered labels below follow the original phase notes)

### Objective
Extend loop_continue_only lowering to handle "continue + return" patterns found in parse_string/array/object.

### Target Pattern
- `tools/selfhost/test_pattern4_parse_string.hako` - Parse string with continue (escape) + return (quote)

### Pattern4 Lowering Contract (Phase 142 P2)

#### Accepted Minimum Structure

**Return Handling**:
- **Position**: Early return inside one or more if blocks
- **Type**: Scalar return values (complex returns are out of scope)
- **Constraint**: Only the last return in loop body is processed

**Continue Side Updates**:
- **Pattern**: `if cond { carrier = carrier ± 1; continue }`
- **Update**: Constant step only (+1, -1, +2, -2, etc.)
- **Constraint**: Multiple carriers not yet supported

**Carrier and Payload**:
- **Carrier**: Loop variable used in loop condition
- **Payload**: State updated on non-continue path (e.g., result string)

**Exit Contract**:
- `has_continue = true` (continue pattern exists)
- `has_return = true` (early return exists)
- Both must coexist

#### Unsupported (Fail-Fast)

The following patterns are rejected with explicit error messages:

- [ ] Multiple continue patterns (2+ continue statements)
- [ ] Nested continue-return (continue inside if inside if)
- [ ] Complex return values (returning multiple fields)
- [ ] Variable step updates (escape sequence handling, etc.)

### Implementation Strategy

**Step 1**: Clarify Pattern4 contract (this document)
**Step 2**: Add E2E test case
**Step 3**: Extend Pattern4 lowerer
**Step 4**: Consider box-ification / modularization
**Step 5**: Implementation and verification

### Progress

- [x] Step 1: Contract clarification
- [ ] Step 2: Add test case
- [ ] Step 3: Extend lowerer
- [ ] Step 4: Consider box-ification
- [ ] Step 5: Verification complete

### Acceptance Criteria

- ✅ Representative test (parse_string or simple_continue) passes JoinIR lowering
- ✅ Execution results are correct in both VM and LLVM (scope to be determined)
- ✅ No regression in existing tests (phase132_exit_phi_parity, etc.)
- ✅ Unsupported patterns fail fast with reason (error_tags)
- ✅ No new environment variables added (dev-only observation only)
- ✅ Documentation updated

### Files to Modify

1. `docs/development/current/main/phases/phase-142/README.md` - Contract documentation
2. `tools/selfhost/test_pattern4_parse_string_lowering.hako` - Minimal E2E test (new)
3. `src/mir/join_ir/lowering/loop_routes/with_continue.rs` - current lowerer extension surface
   - historical path tokens: `src/mir/builder/control_flow/joinir/patterns/{pattern4_with_continue.rs,pattern4_carrier_analyzer.rs}`

### Step 3-A: Early Return Fail-Fast (COMPLETE ✅)

**Status**: ✅ COMPLETE - Return detection and explicit error implemented

**Implementation**: Added `has_return_in_body()` helper function to Pattern4 lowerer
- Recursively scans loop body for return statements
- Returns explicit Fail-Fast error when return is detected
- Error message references Phase 142 P2 for future lowering

**Test Results**: All 14 canonicalizer tests PASS (no regressions)

**Key Achievement**: Unsafe silent acceptance is now prevented - early returns explicitly surface as errors with actionable messages.

### Step 3-B: Return Path JoinIR Generation (DEFERRED)

**Status**: 🔄 DEFERRED for separate session - Large-scale implementation requires careful design

**Why separate**: JoinIR generation involves responsibility boundary decisions and ExitMeta/payload handling. Separating ensures cleaner cause analysis.

### 🔑 Design Decisions (FIXED for Phase 142 P2 Step 3-B)

#### 1. Return Responsibility Boundary: historical `Pattern5`

**Decision**: Return handling → historical `Pattern5` plan lane (not historical `Pattern4`)

**Rationale**:
- historical `Pattern4` responsibility at the time: "continue + update rules" only
- historical `Pattern5` responsibility at the time: "continue + early return" integration
- Prevents the historical `Pattern4` lane from bloating as parse_string/array/object family expands
- Aligns with canonicalizer SSOT ("structure is notes, chosen is final lowerer")

**Architecture**:
```
LoopContinueOnly (historical recognizer label: `Pattern4Continue`)
  ↓
historical `Pattern4Lowerer` (continue only)
  ↓ (has_return? → delegate)
historical `Pattern5Lowerer` (continue + early return)
```

#### 2. Return Payload Transport: Closed within Pattern5

**Decision**: ExitMeta expansion NOT needed (initially)

**Rationale**:
- **Judgment criteria**: "Does return need the same exit_line reconnection as break?"
- **Answer for parse_***: No - "return ends the function" (not carrier reconnect)
- **Therefore**: Close return payloads within Pattern5 lowerer (no cross-boundary phi)

**Payload Handling**:
1. Return value lives in a temporary (or direct function return)
2. No phi merge with continue-side carrier updates
3. Return path is self-contained within Pattern5

**ContinueReturn Asset Reuse**:
- ✅ Extract: Value consistency checks + transport logic
- ❌ Don't reuse: exit_line reconnection machinery (not needed)
- Reuse only the "payload consistency validation" parts

### 📋 Implementation Checklist for Step 3-B (Next Session)

**Preparation**:
- [ ] Design Pattern5 entry point (new or enhance existing Pattern5)
- [ ] Map return payload handling (temporary, no phi merge)
- [ ] Document ExitMeta usage (confirm unchanged)
- [ ] Identify ContinueReturn reusable parts (consistency checks)

**Implementation**:
- [ ] Create `pattern5_continue_return_minimal.rs` lowerer
- [ ] Add return path handling (closed within Pattern5)
- [ ] Use existing carrier reconnection from Pattern4 (for continue side)
- [ ] Test on parse_string minimal fixture
- [ ] Verify no ExitMeta changes needed

### SSOT References

- **Design**: `docs/development/current/main/design/loop-canonicalizer.md`
- **JoinIR Architecture**: `docs/development/current/main/joinir-architecture-overview.md`
- **LoopContinueOnly route implementation**: `src/mir/join_ir/lowering/loop_routes/with_continue.rs`
  - same historical path lane as above (`pattern4_with_continue.rs`)
