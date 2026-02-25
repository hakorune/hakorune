# Phase 70-A: Relay Runtime Guard

**Status**: Implementation Phase
**Date**: 2025-12-13

---

## Overview

Phase 66 enabled multihop relay acceptance at the analysis/plan layer (`plan_to_p2_inputs_with_relay` / `plan_to_p3_inputs_with_relay`). However, the **runtime lowering path** (actual MIR generation) does not yet support multihop execution.

This phase establishes a **Fail-Fast boundary** with a standardized error tag to clearly indicate "analysis OK, runtime not yet supported."

---

## Current State

| Layer | Multihop Support | Status |
|-------|-----------------|--------|
| `plan_to_p2_inputs_with_relay` | ✅ Accepts | Phase 66 complete |
| `plan_to_p3_inputs_with_relay` | ✅ Accepts | Phase 66 complete |
| **P3 runtime lowering** | ❌ Rejects | Phase 70-A guard |
| P2 runtime lowering | ❌ Not integrated | Future |

---

## Fail-Fast Tag

**Standard Tag**: `[ownership/relay:runtime_unsupported]`

When multihop relay (`relay_path.len() > 1`) is detected at runtime:

```
[ownership/relay:runtime_unsupported] Multihop relay not executable yet: var='sum', owner=ScopeId(0), relay_path=[ScopeId(2), ScopeId(1)]
```

**Diagnostic fields**:
- Variable name being relayed
- Owner scope ID
- Relay path (inner → outer)
- Relay path length

---

## Implementation Location

**File**: `src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs`

**Function**: `check_ownership_plan_consistency()`

**Behavior**: On `relay_path.len() > 1`, return `Err` with standardized tag.

---

## Release Conditions (Phase 70-B+)

The `[ownership/relay:runtime_unsupported]` guard can be removed when:

1. **Exit PHI merge** implemented at owner scope
2. **Boundary carrier propagation** for intermediate scopes
3. **Integration tests** passing for 3+ layer nested loops
4. **No regression** in existing Pattern3 tests

---

## Test Coverage

**Test**: `test_phase70a_multihop_relay_runtime_unsupported_tag`

Verifies:
- 3-layer nested loop AST (L1 owns `sum`, L3 writes `sum`)
- Runtime path returns `Err`
- Error message contains `[ownership/relay:runtime_unsupported]`

---

## Phase 70 Series

| Phase | Status | Description |
|-------|--------|-------------|
| Phase 70-A | ✅ Complete | Runtime guard with standard tag `[ownership/relay:runtime_unsupported]` |
| Phase 70-B | ✅ Complete | Simple passthrough multihop support (contiguous path, no self-updates) |
| Phase 70-C | ✅ Complete | Merge relay detection (multiple inner loops → same owner) |
| Phase 70-D+ | 🚧 Future | Full runtime execution support (exit PHI merge, carrier propagation) |

---

## Related Documents

- [Phase 65: Multihop Design](phase65-ownership-relay-multihop-design.md)
- [Phase 66: Multihop Implementation](phase65-ownership-relay-multihop-design.md#phase-66-implementation-status)
- [Phase 70-C: Merge Relay](phase70c-merge-relay.md)
- [Phase 56: Ownership-Relay Architecture](phase56-ownership-relay-design.md)

---

## Changelog

- **2025-12-13**: Phase 70-C completed - Merge relay detection and validation
- **2025-12-13**: Phase 70-A created - Fail-Fast tag standardization
