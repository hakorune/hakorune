---
Status: accepted__CanonicalSsaS6cStateBoxShape__2026-09-11
Date: 2026-09-11
Decision: MIRBUILDER-CANONICAL-SSA-S6C-STATE-BOXSHAPE-D0
Parent: mirbuilder-post-r0-failopen-audit-d0-2026-09-11
---

# MIRBUILDER-CANONICAL-SSA-S6C-STATE-BOXSHAPE-D0

Implementation row: `MIRBUILDER-CANONICAL-SSA-S6C-STATE-BOXSHAPE-R0`.

## Six-line brief

```text
Decision: isolate the existing S6C cursor-seal and pinned-Text residence
  lifecycle fields behind one private canonical-session state child.
Source authority + canonical issuer: CanonicalSsaFunctionSessionV2 remains the
  sole mutable SSA/CFG/PHI owner; its existing lifecycle methods issue state.
Non-authority: the new child must not infer source meaning, allocate ValueId or
  CFG/PHI products, select callers, or rename the existing V1/V2 contracts.
Fail-fast boundary: preserve the current duplicate/missing-state errors while
  routing all accesses through the private child.
Smallest next slice: add owner README, move only two fields, add structural
  guard checks, and run focused compile/guard evidence.
Non-claims: no semantic behavior change, new receipt, caller move, backend
  change, S6C meaning change, GenericLoop promotion, or legacy retirement.
```

## BoxShape boundary

The finite canonical SSA inventory is the existing `canonical_ssa/` module and
its private session children. `CanonicalSsaFunctionSessionV2` remains the
canonical mutable owner for SSA/CFG/PHI publication. `ResolvedSsaIdentityStateV2`
remains the identity/binding authority, and `CanonicalCfgSessionV1` remains the
physical CFG authority. The new `session/s6c_state.rs` is only a private storage
child for state already issued by the parent lifecycle methods.

The slice moves exactly these parent fields:

- `deferred_s6c_cursor_blocks`
- `pinned_text_residence`

`physical_entry_sidecar`, `physical_entry_execution`, and
`physical_entry_seal_deferred` remain in the parent because they form the
physical-entry boundary itself. The child exposes storage accessors only; it
does not validate, reinterpret, or publish a semantic product.

## Caller and authority inventory

The existing canonical session callers remain unchanged: callable lowering,
direct accumulation, trivial SSA, nested predicate, generic G0, selected
dynamic lowering, and the common V2/S6C session. S6C lifecycle methods in
`physical_entry_boundary.rs` and `residence_lifecycle.rs` remain the only
mutation boundaries for the two moved fields. No caller is added or removed.

The existing V1/V2 naming is retained. V1 types are physical receipts or
state records; V2 names the canonical function session. Renaming or merging
those contracts would be a separate design decision.

## Must-not and acceptance

The child must not use `#[path]` glue from the `canonical_ssa/mod.rs` facade,
must not expose runtime/raw/MIR authorities, and must not create a second S6C
owner. `common_v2_s6c_structure_guard.sh` must require the README and child,
reject direct parent-field access from the two lifecycle children, and retain
the below-800-line guard.

Acceptance is: targeted Rust formatting is green; the canonical SSA library
compile succeeds; the existing canonical session tests remain green; the
common S6C structure guard and current-state pointer guard are green; and the
diff shows no change to emitted behavior or caller selection. Any repository-
wide formatting drift remains baseline evidence and is not folded into this
slice.

## Worker consultation receipt

A read-only worker audit confirmed this is a BoxShape-only slice with an
existing owner and no `NoSafeSlice`. It identified the two fields above as the
smallest isolated state, while recommending that S6C meaning, callers, and the
V1/V2 contracts remain untouched. The audit did not edit files or run Cargo.

## R0 implementation receipt (2026-09-11)

`CanonicalSsaFunctionSessionV2` now stores the existing S6C cursor-seal and
pinned-Text lifecycle state through the private `session/s6c_state.rs` child.
The two lifecycle children use storage accessors, while duplicate and missing
state checks retain their original owner methods and error text. No caller,
semantic product, physical CFG/SSA/PHI operation, or V1/V2 contract changed.

`canonical_ssa/README.md` records the owner, caller, and must-not boundary. The
common S6C structure guard now requires the README/child, checks the session
facade, rejects direct moved-field access from lifecycle children, and includes
the canonical session in the below-800-line inventory.

Evidence is green: targeted rustfmt, `common_v2_s6c_structure_guard.sh`, and
`current_state_pointer_guard.sh`; canonical SSA tests 10/10; common S6C cursor
tests 7/7; canonical CFG Residence tests 4/4; and the targeted library compile
with `cargo test --profile quick --lib
'mir::builder::resolved_lowering::canonical_ssa' --no-run --jobs 2`.
The compile produced the known whole-library warning baseline (493 warnings)
and no compile error. The repository-wide formatting check remains outside
this slice because of unrelated pre-existing drift.
