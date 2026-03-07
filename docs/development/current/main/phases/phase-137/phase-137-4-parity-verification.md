# Phase 137-4: Loop Canonicalizer Router Parity Verification

**Status**: ✅ Complete
**Date**: 2025-12-16

## Summary

Dev-only verification that Loop Canonicalizer and Router pattern detection agree on pattern classification. On mismatch, provides detailed diagnostic with Fail-Fast option.

## Implementation

### Location

`src/mir/builder/control_flow/joinir/routing.rs`

### Components

1. **Parity Verification Function** (`verify_router_parity`)
   - Runs canonicalizer on the loop AST
   - Compares canonicalizer's chosen pattern with router's `ctx.pattern_kind`
   - Logs match/mismatch results
   - In strict mode, returns error on mismatch

2. **Integration Point**
   - Called in `cf_loop_joinir_impl` after `LoopPatternContext` is created
   - Only runs when `joinir_dev_enabled()` returns true
   - Deferred until after ctx creation to access `ctx.pattern_kind`

### Behavior Modes

#### Debug Mode (Default)
```bash
HAKO_JOINIR_DEBUG=1 ./target/release/hakorune program.hako
```
- Logs parity check results to stderr
- On mismatch: Logs warning but continues execution
- On match: Logs success message

#### Strict Mode
```bash
HAKO_JOINIR_STRICT=1 ./target/release/hakorune program.hako
```
- Same as debug mode, but on mismatch:
- Returns error and stops compilation
- Provides detailed mismatch diagnostic

### Output Examples

#### Match (Success)
```
[loop_canonicalizer/PARITY] OK in function 'foo':
  canonical and actual agree on Pattern1SimpleWhile
```

#### Mismatch (Warning in Debug)
```
[loop_canonicalizer/PARITY] MISMATCH in function 'main':
  canonical=Pattern3IfPhi, actual=Pattern2Break
```

#### Mismatch (Error in Strict)
```
[ERROR] ❌ MIR compilation error:
  [loop_canonicalizer/PARITY] MISMATCH in function 'main':
  canonical=Pattern3IfPhi, actual=Pattern2Break
```

#### Canonicalizer Failure
```
[loop_canonicalizer/PARITY] Canonicalizer failed for 'bar':
  Phase 3: Loop does not match skip_whitespace pattern
```

## Test Coverage

### Unit Tests

1. **`test_parity_check_mismatch_detected`**
   - Verifies mismatch detection on skip_whitespace pattern
   - Canonicalizer: if_phi_join route (`Pattern3IfPhi` legacy label; recognizes if-else structure)
   - Router: loop_break route (`Pattern2Break` legacy label; sees `has_break` flag)
   - Asserts inequality to document expected mismatch

2. **`test_parity_check_match_simple_while`**
   - Verifies canonicalizer fails on loop_simple_while route (not yet implemented at that phase)
   - Router: `LoopSimpleWhile` route (`Pattern1SimpleWhile` legacy label)
   - Canonicalizer: Fail-Fast (only supports skip_whitespace in Phase 3)

### Integration Tests

```bash
# Verify mismatch detection (debug mode)
HAKO_JOINIR_DEBUG=1 ./target/release/hakorune \
  tools/selfhost/test_pattern3_skip_whitespace.hako

# Verify strict mode error
HAKO_JOINIR_STRICT=1 ./target/release/hakorune \
  tools/selfhost/test_pattern3_skip_whitespace.hako
# Expected: Exit with error due to mismatch
```

## Known Mismatches

### skip_whitespace route shape

**Structure**:
```nyash
loop(p < len) {
  if is_ws == 1 {
    p = p + 1
  } else {
    break
  }
}
```

**Mismatch**:
- **Canonicalizer**: if_phi_join route shape (`Pattern3IfPhi` legacy label)
  - Recognizes specific if-else structure with conditional carrier update
  - Sees: `if cond { carrier += 1 } else { break }` as an if-phi style route shape

- **Router**: Pattern2Break
  - Classification based on `has_break` flag
  - Priority: break detection takes precedence over if-else structure

**Resolution Strategy**:
- Phase 4: Document and observe (current)
- Phase 5+: Refine classification rules to handle hybrid patterns
- Option A: Extend Pattern3 to include "break-in-else" variant
- Option B: Create new Pattern6 for this specific structure
- Option C: Make router defer to canonicalizer's decision

## Design Rationale

### Why Two Systems?

1. **Router (Existing)**
   - Feature-based classification (`has_break`, `has_continue`, etc.)
   - Fast, simple flags
   - Priority-based (e.g., break > if-else)

2. **Canonicalizer (New)**
   - Structure-based pattern matching
   - Deep AST analysis
   - Recognizes specific code idioms

### Why Parity Check?

- **Incremental Migration**: Allows canonicalizer development without breaking router
- **Safety Net**: Catches classification divergence early
- **Documentation**: Explicitly records where systems disagree
- **Flexibility**: Dev-only, no production overhead

### Why Dev-Only?

- **No Performance Impact**: Zero cost in release builds (flag-gated)
- **Development Insight**: Helps refine both systems
- **Fail-Fast Option**: Strict mode for CI/validation
- **Graceful Degradation**: Debug mode allows execution to continue

## Acceptance Criteria

- ✅ Flag OFF: No behavioral change
- ✅ Dev-only: Match/mismatch observable
- ✅ Strict mode: Mismatch stops compilation
- ✅ Debug mode: Mismatch logs warning
- ✅ Unit tests: 2 tests passing
- ✅ Integration test: skip_whitespace mismatch detected
- ✅ All tests: `cargo test --release --lib` passes (1046/1046)

## Future Work

### Phase 5: Pattern Classification Refinement

- Resolve skip_whitespace classification (Pattern2 vs Pattern3)
- Extend Pattern3 to handle break-in-else variant
- OR: Create Pattern6 for "conditional update with early exit"

### Phase 6: Canonicalizer Expansion

- Add Pattern1 (Simple While) to canonicalizer
- Add Pattern2 (Conditional Break) variants
- Add Pattern4 (Continue) support
- Add Pattern5 (Infinite Early Exit) support

### Phase 7: Router Migration

- Consider migrating router to use canonicalizer decisions
- Option: Make `ctx.pattern_kind = decision.chosen` if canonicalizer succeeds
- Gradual transition from feature-based to structure-based routing

## References

- **Phase 137-2**: Loop Canonicalizer observation (dev-only logging)
- **Phase 137-3**: skip_whitespace pattern recognition
- **Design SSOT**: `docs/development/current/main/design/loop-canonicalizer.md`
- **JoinIR Architecture**: `docs/development/current/main/joinir-architecture-overview.md`
