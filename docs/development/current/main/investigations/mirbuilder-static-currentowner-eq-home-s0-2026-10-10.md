# Static CurrentOwner Eq Home S0

Status: selected implementation
Date: 2026-10-10
Scope: MIRBUILDER-STATIC-CURRENTOWNER-EQ-HOME-S0
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-static-currentowner-condition-cohort-d0-2026-10-10.md
  - src/mir/resolved_semantics/README.md

## Selected replacement

For the original `SizeClassBox.accepts` If condition
`me.size_to_bin(size) == me.huge_bin()`, the composed Home scalar owner must
issue two ordered `LocalCallObservationV1` rows. The Lhs one-input call uses
the existing original CurrentOwner Static claim and staged source actual;
the RHS uses its original zero-input Static claim. This replaces the exact
condition's source-only Home gap, without changing executable transport.

Admit by resolved If-region, exact Equal binary and child sites, CurrentOwner
receivers, one/zero arities, I64 source claims, and Lhs source-argument
projection. Check both children before publishing either Home observation.
The selected Eq evaluates Lhs then RHS on Normal; a Fault in either child
does not publish a later result. Home records source order but does not by
itself authorize physical evaluation. Nested-call arguments, changed sites,
missing siblings, and short-circuit operators remain unselected.

Use the existing `observe_scalar_expression` owner. Put the bounded Eq helper
in a responsibility-specific module instead of growing the 716-line scalar
owner toward its 800-line hard stop. Preserve the existing direct-value and
zero-input call issuers; do not create a second Static source authority.

## Acceptance

1. The unchanged `size_class_box.hako` source yields exactly two Home
   observations for `accepts`'s Eq, ordered Lhs/Rhs, with the original sites,
   result I64 classes and Lhs ordered argument.
2. A changed Lhs or RHS source, a missing RHS claim, a nested call argument,
   and an `&&`/`||` condition cannot publish a partial pair. Existing
   zero-input and `>= 0` condition evidence remains green.
3. The original incoming `size_to_bin` cohort remains two rows and
   source-only for executable actual/packet. No physical Eq or EXE success is
   claimed. Run focused Home positive/negative and pointer guard; classify
   regressions before closeout.

The physical integer Eq envelope, ordered Fault lowering, whole two-caller
finisher, and `size_to_bin` internal loop remain dependent work under the D0.
