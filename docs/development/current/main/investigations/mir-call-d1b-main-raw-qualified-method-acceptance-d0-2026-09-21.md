---
Status: design_stop__2026-09-21__MainQualifiedMethodAcceptance
Task: MIR-CALL-D1B-MAIN-RAW-QUALIFIED-METHOD-ACCEPTANCE-D0
Date: 2026-09-21
Parent: mir-call-d1b-main-raw-qualified-method-handoff-i0-2026-09-21.md
Implementation permission: false; design decision only
NextCard: MIR-CALL-D1B-MAIN-RAW-QUALIFIED-METHOD-ACCEPTANCE-I0
---

# Main raw qualified MethodCall acceptance D0

## Six-line brief

```text
Decision: define the smallest source-backed Main acceptance window for the
  qualified StaticBoxMethod handoff that is now physically wired.
Source authority + canonical issuer: the resolver-owned qualified relation,
  its installed Main adapter take, and the existing target-only physical
  terminal; acceptance must not add a second source inventory.
Non-authority: compatibility/VM lanes, Script whole-source lookup, raw name
  lookup, result ABI publication, and any fallback retry.
Fail-fast boundary: one Cataloged Main call must consume one relation row,
  validate ordered argument sites, emit one canonical global target call, and
  close the relation with no residual rows.
Smallest next slice: reproduce the selected Main acceptance boundary, classify
  the existing canonical-function cleanup red, and choose the existing owner
  that can close resolved binding authority before source-to-MIR is claimed.
Non-claims: no production cutover, other callers, result typing, VM repair,
  backend parity, or LegacyCallV0 deletion.
```

## Boundary and current evidence

The I0 implementation landed at `d6ee865368`. Relation-level evidence is green:
ordered argument sites, receiver mismatch before consumption, exact one-shot
take, residual completion, and second-take rejection. `cargo check
--profile quick --lib` and `cargo fmt --check` are green with the repository
warning baseline.

The full source-backed Main call is not yet an acceptance receipt. The existing
qualified/direct Main call family reaches the known
`canonical_function_session/cleanup_failed` state-imbalance baseline around
resolved binding authority. This D0 must identify the existing canonical owner
and terminal that closes that authority; it must not hide the red by routing to
compatibility or by weakening cleanup checks.

The finite inventory is only qualified direct-owner/import-alias calls in the
Cataloged App Main root. It excludes bare functions, `me.method`, instance
methods, ScriptRoot, VM, nested-owner inheritance, result publication, and
legacy retirement.

## Required decision evidence

* name the owner that performs the source-to-MIR acceptance close and its
  existing completion receipt;
* replay the direct and alias fixtures at the selected terminal, preserving the
  named cleanup failure if the owner is still absent;
* keep the I0 relation and target-only bridge as the sole physical path; and
* record the exact acceptance command and the known baseline classification.

No implementation is authorized until this owner/terminal tuple is accepted.
