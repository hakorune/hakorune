# SSA-RC-RET-P0 Legacy Release Inventory Evidence

Status: Closed — behavior-neutral inventory and isolation guard

Date: 2026-07-14

Decision: classify every legacy lifecycle surface without changing opcode,
backend, optimizer, JSON, fixture, or canonical runtime behavior.

## Result

The dedicated machine ledger records every tracked source containing the exact
legacy lifecycle spellings and assigns one closed retirement disposition.

```text
tracked surfaces:                         118
exact token occurrences:                  266
canonical caller-zero delete:               1
legacy builder isolate:                     4
optional RC insertion isolate:              3
optimizer/CFG rewrite isolate:              11
backend/JSON compatibility isolate:         31
dead after repository caller zero:          68
semantic delta:                              0
canonical production ownership activation:  0
```

The single canonical producer is
`src/mir/builder/resolved_lowering/lowerer.rs`. The ledger does not delete or
reinterpret that caller; SSA-I1 owns the atomic caller-zero cutover.

## Artifacts

- `tools/checks/fixtures/legacy_release_strong_inventory_v1.json`
- `tools/checks/lib/resolved_ownership_legacy_release_inventory.py`
- `tools/checks/lib/resolved_ownership_legacy_release_contract.sh`
- private connection beneath
  `tools/checks/lib/resolved_binding_ssa_contract.sh`

The historical 92-row canonical SSA seam inventory remains unchanged. No new
public guard was added, and the bounded public authority guard did not grow.

## Verification

```text
inventory validator: green
shell syntax: green
resolved region-flow authority guard: green
authority focused Rust tests: 56/56 green
new/modified source and check files: <=217 lines
release build: green
dev_gate quick: 66/66 green
current-state pointer guard: green
```

## Non-claims

This row does not claim:

```text
legacy lifecycle opcode retirement
canonical caller zero
optional RC insertion retirement
backend or JSON vocabulary deletion
ownership transition planning
Binding SSA production activation
```

## Next row

`SSA-RC0` builds the disconnected pure ownership-transition planner. It emits
typed plans only, allocates no MIR value, and keeps production activation at
zero.
