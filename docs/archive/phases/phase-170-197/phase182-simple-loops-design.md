# Phase 182-1: JsonParser Simple Loops Design Memo

## Overview
Phase 182 implements three simple JsonParser loops using existing P2/P1 JoinIR patterns.
This follows Phase 181's design investigation.

## Target Loops (3)

### 1. _parse_number (P2 Break)
- **Loop condition**: `p < s.length()` (LoopParam only)
- **Carriers**: `p`, `num_str` (2 carriers)
- **Control flow**: `break` on `digit_pos < 0`
- **Pattern**: P2 Break
- **JoinIR support**: ✅ Existing P2 lowerer sufficient
- **AST characteristics**:
  - Single break point
  - No continue
  - No nested control flow
  - Carriers updated unconditionally before break check

### 2. _atoi (P2 Break)
- **Loop condition**: `i < n` (LoopParam + OuterLocal)
- **Carriers**: `v`, `i` (2 carriers)
- **Control flow**: `break` on `ch < "0" || ch > "9"` + `pos < 0`
- **Pattern**: P2 Break
- **JoinIR support**: ✅ Existing P2 lowerer sufficient
- **AST characteristics**:
  - Multiple break conditions (combined with OR)
  - No continue
  - No nested control flow
  - Carriers updated before and after break check

### 3. _match_literal (P1 Simple)
- **Loop condition**: `i < len` (LoopParam + OuterLocal)
- **Carriers**: `i` (1 carrier)
- **Control flow**: `return` (early exit)
- **Pattern**: P1 Simple
- **JoinIR support**: ✅ Existing P1 lowerer sufficient
- **AST characteristics**:
  - No break/continue
  - Early return instead
  - Single carrier increment
  - Simple conditional logic

## Pipeline Integration Strategy

### Existing Infrastructure (Reuse)
- **PatternPipelineContext**: Already handles P1/P2 detection
- **build_pattern_context()**: Existing logic sufficient
- **P1 lowerer**: `lower_simple_while_minimal()` ready
- **P2 lowerer**: `lower_loop_with_break_minimal()` ready
- **LoopFeatures**: Shape detection already identifies P1/P2

### Minimal Additions Required
1. **Routing whitelist**: Ensure 3 functions are in JOINIR_TARGETS
2. **LoopUpdateAnalyzer**: May need to skip string-heavy operations (gradual enablement)
3. **Tracing**: Use NYASH_JOINIR_STRUCTURE_ONLY=1 for verification

### NOT Required
- ❌ New pattern types
- ❌ New lowerer functions
- ❌ Special carrier handling
- ❌ Custom PHI generation

## Verification Plan

### Phase 182-2: Routing Check
```bash
# Verify functions are in whitelist
grep -E "(_parse_number|_atoi|_match_literal)" \
  src/mir/builder/control_flow/joinir/routing.rs
```

### Phase 182-3: Pattern Detection Tracing
```bash
# Verify correct pattern routing
NYASH_JOINIR_STRUCTURE_ONLY=1 NYASH_JOINIR_DEBUG=1 \
  ./target/release/hakorune apps/selfhost-runtime/jsonparser.hako 2>&1 | \
  grep -E "(pattern|route|_parse_number|_atoi|_match)"
```

Expected output:
- `_parse_number` → [trace:pattern] route: Pattern2_Break MATCHED
- `_atoi` → [trace:pattern] route: Pattern2_Break MATCHED
- `_match_literal` → [trace:pattern] route: Pattern1_Minimal MATCHED
- No `[joinir/freeze]` errors

### Phase 182-5: Representative Tests
Create 3 minimal test files:
1. `local_tests/test_jsonparser_parse_number_min.hako` - number parsing
2. `local_tests/test_jsonparser_atoi_min.hako` - string to integer
3. `local_tests/test_jsonparser_match_literal_min.hako` - literal matching

Success criteria:
- ✅ RC = 0 (normal exit)
- ✅ Output matches expected values
- ✅ No `[joinir/freeze]` errors
- ✅ Same results as Rust JsonParser

## Implementation Notes

### Design Principles
1. **Fit into existing framework** - Don't add special cases
2. **Minimal additions** - Only adjust filters if necessary
3. **Gradual enablement** - String operations can be phased in
4. **Reuse pattern pipeline** - PatternPipelineContext handles everything

### Potential Issues
- **String operations**: May need LoopUpdateAnalyzer filtering
- **Carrier complexity**: P2 has 2 carriers, but existing code handles this
- **Break conditions**: Multiple conditions in _atoi, but OR combination is standard

### Success Metrics
- All 3 loops route to correct patterns (P2/P1)
- No new special-case code required
- Tests pass with NYASH_JOINIR_CORE=1
- Performance comparable to existing patterns

## Next Steps (Phase 183+)
- _parse_array (P4 Continue candidate)
- _parse_object (P4 Continue candidate)
- Complex nested loops (future phases)

## References
- Phase 170: Loop header PHI design
- Phase 181: JsonParser loop analysis
- Pattern detection: `src/mir/builder/control_flow/joinir/patterns/detection.rs`
- P1/P2 lowerers: `src/mir/builder/control_flow/joinir/patterns/pattern{1,2}_*.rs`
Status: Historical
