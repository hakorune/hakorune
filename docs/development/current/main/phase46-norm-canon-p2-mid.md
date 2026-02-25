# Phase 46: Normalized Canonical P2-Mid Promotion

**Status**: Implemented ✅ (2025-12-12)
**Test Coverage**: 937/937 PASS

## Goal

Promote P2-Mid patterns (_atoi real, _parse_number real) to canonical Normalized→MIR(direct) route, alongside existing P2-Core patterns.

## Scope

**In Scope (P2 patterns)**:
- JsonparserAtoiReal → canonical Normalized
- JsonparserParseNumberReal → canonical Normalized

**Out of Scope (deferred to future phases)**:
- Pattern3 (if-sum) → NORM-P3
- Pattern4 (continue) → NORM-P4
- Selfhost complex loops → separate phases

## Rationale

With Phase 43/245B infrastructure complete, JsonParser P2-Mid loops (_atoi, _parse_number) are production-ready for canonical Normalized route:

1. **Proven infrastructure**: DigitPos dual-value, NumberAccumulation, StepSchedule all working
2. **Real-world validation**: JsonParser _atoi/_parse_number tests passing (937/937)
3. **Clear boundary**: P2 vs P3/P4 separation simplifies rollout

After Phase 46, P2 line becomes "Normalized-first" - Structured→MIR is legacy/comparison only.

## Implementation Checklist

### 1. Expand Canonical Set (shape_guard.rs)

✅ Update `is_canonical_shape()` to include:
- JsonparserAtoiReal
- JsonparserParseNumberReal

✅ Update doc comments:
- "Phase 41 canonical set" → "Phase 46 canonical set: P2-Core + P2-Mid"

### 2. Verify Bridge Routing (bridge.rs)

✅ Confirm `canonical_shapes()` routing unchanged (already calls `is_canonical_shape()`)

✅ Update comments:
- "Phase 41: P2-Core only" → "Phase 46: P2-Core + P2-Mid (_atoi/_parse_number real)"

### 3. Add/Update Tests

✅ Verify existing tests cover canonical routing:
- `normalized_pattern2_jsonparser_atoi_real_vm_bridge_direct_matches_structured`
- `normalized_pattern2_jsonparser_parse_number_real_vm_bridge_direct_matches_structured`

✅ Add new unit test (normalized_dev feature only):
- Verify canonical_shapes includes _atoi real / _parse_number real
- Verify bridge always routes to Normalized→MIR(direct)

### 4. Update Documentation

✅ Add Phase 46 section to `joinir-architecture-overview.md`:
- "JsonParser _skip_whitespace / _atoi / _parse_number now canonical Normalized"
- Link to P3/P4 future work (NORM-P3/NORM-P4)

✅ Add Phase 46 entry to `CURRENT_TASK.md`:
- Scope: P2-Core/P2-Mid canonical (P3/P4 out of scope)
- Done condition: shape_guard + bridge + tests + docs

## Canonical Set Evolution

| Phase | Canonical Patterns | Description |
|-------|-------------------|-------------|
| Phase 41 | P2-Core: Pattern2Mini, skip_ws mini/real, atoi mini | Initial canonical set |
| **Phase 46** | **+ P2-Mid: atoi real, parse_number real** | **JsonParser production patterns** |
| Future | P3/P4, Selfhost loops | Deferred to NORM-P3/NORM-P4 |

## Testing Strategy

**Existing coverage** (no new tests required):
- `normalized_joinir_min.rs` already tests _atoi real / _parse_number real
- VM output comparison verified (Normalized vs Structured)

**New unit test** (shape + bridge integration):
- Verify `canonical_shapes()` includes P2-Mid
- Feature-gated: `#[cfg(feature = "normalized_dev")]`

## Benefits

1. **Clear P2 boundary**: All JsonParser P2 loops now Normalized-first
2. **Simplified mental model**: P2 = Normalized canonical, P3/P4 = future work
3. **Production-ready**: _atoi/_parse_number real validated through Phase 246-EX/247-EX

## Next Steps (Out of Scope)

- **NORM-P3**: Pattern3 (if-sum) Normalized support
- **NORM-P4**: Pattern4 (continue) Normalized support
- **Selfhost**: Complex loops from selfhost compiler

## References

- **Completion Summary**: [PHASE_43_245B_NORMALIZED_COMPLETION.md](./PHASE_43_245B_NORMALIZED_COMPLETION.md)
- **Phase 44 Capabilities**: [phase44-shape-capabilities-design.md](./phase44-shape-capabilities-design.md)
- **Phase 45 Mode**: [phase45-norm-mode-design.md](./phase45-norm-mode-design.md)
