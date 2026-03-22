# Ownership Analysis Module

Read first:

1. [`src/mir/README.md`](../../README.md)
2. [`src/mir/join_ir/README.md`](../README.md)
3. [`src/mir/join_ir/lowering/README.md`](../lowering/README.md)

## Packaging Status

Docs-first only for now. This module stays inside the JoinIR review lane and is
not a safe standalone crate split yet.

Reason:

- ownership analysis reads the same AST/ProgramJSON + runtime/env + MIR surface
  as the lowering path
- splitting it early would duplicate boundary logic before the bridge/lowering
  seam is stable

Landed substrate slice:

- `hakorune_mir_joinir::ownership_types` now owns the pure ownership type substrate
- `bridge/*` now owns validator / lowering-adapter glue under the ownership facade
- `analyzer.rs` + `ast_analyzer/*` remain the analysis core here

## Responsibility Boundary

This module is responsible for **analysis only**:
- ✅ Collecting reads/writes from AST/ProgramJSON
- ✅ Determining variable ownership (owned/relay/capture)
- ✅ Producing OwnershipPlan for downstream lowering

This module does NOT:
- ❌ Generate MIR instructions
- ❌ Modify JoinIR structures
- ❌ Perform lowering transformations

## Core Types

| Type | Purpose |
|------|---------|
| `ScopeId` | Unique scope identifier |
| `ScopeOwnedVar` | Variable defined in this scope |
| `RelayVar` | Write to ancestor-owned variable |
| `CapturedVar` | Read-only reference to ancestor |
| `OwnershipPlan` | Complete analysis result |

## Invariants

1. `carriers = owned_vars.filter(is_written)`
2. No variable in both owned and relay
3. No variable in both owned and captures
4. Relay implies ancestor ownership exists

## Design Philosophy

**「読むのは自由、管理は直下 owned だけ」**

- **Owned**: Variable defined in this scope (unique owner)
- **Carrier**: Owned AND written (managed as loop_step argument)
- **Capture**: Read-only reference to ancestor (via CapturedEnv)
- **Relay**: Write to ancestor → relay up to owner (exit PHI at owner)

## Phase Status

- Phase 56: ✅ Interface skeleton
- Phase 57: ⏳ OwnershipAnalyzer implementation
- Phase 58: ⏳ P2 plumbing
- Phase 59: ⏳ P3 plumbing
- Phase 60: ⏳ Cleanup dev heuristics
- Phase 61: ⏳ Canonical promotion decision

## Usage (Future)

```rust
let plan = OwnershipAnalyzer::analyze(&ast_node, parent_scope);
plan.verify_invariants()?;
let carriers: Vec<_> = plan.carriers().collect();
```

## Example

```nyash
local limit = 100      // owned by outer
loop {
    local sum = 0      // owned by loop
    if sum < limit {   // limit = capture (read-only)
        sum++          // sum = carrier (owned + written)
    }
}
```

**OwnershipPlan (loop scope)**:
- `owned_vars`: [`sum` (written), `limit` (read-only)]
- `relay_writes`: []
- `captures`: [`limit`]
- `condition_captures`: [`limit`]

## References

- Design Doc: [phase56-ownership-relay-design.md](../../../../docs/development/current/main/phase56-ownership-relay-design.md)
- JoinIR Architecture: [joinir-architecture-overview.md](../../../../docs/development/current/main/joinir-architecture-overview.md)
