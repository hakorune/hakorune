# 293x-274 DEL-003 Stage1 Delegate Exposes Lowering

Status: complete
Date: 2026-05-14

## Scope

Generate concrete forwarding methods from explicit delegate exposure metadata.

## Landed changes

- Added a Program-level delegate lowering pass after parsing.
- Resolves typed delegate fields against sibling box declarations.
- Resolves target methods and copies their signatures.
- Generates forwarding methods that call `me.<delegate_field>.<source_method>(...)`.
- Rejects missing fields, untyped fields, unknown target boxes, missing target methods, duplicate exposed names, and local method collisions.
- Keeps delegate metadata available for tooling and Program JSON.

## Non-goals

- No interface conformance.
- No wildcard exposes.
- No field/property forwarding.
- No cross-module or generic-substitution lowering.
- No legacy `from` / `override` migration.

## Guard

```bash
bash tools/checks/k2_wide_delegate_exposes_lowering_guard.sh
```

## Next selected row

`BRAND-001 Stage0 brand declaration metadata capsule`.

`LOOP-003 Stage1 LoopRange lowering` remains open and should be handled as a
JoinIR/CorePlan route, not as source-level range-loop desugar.
