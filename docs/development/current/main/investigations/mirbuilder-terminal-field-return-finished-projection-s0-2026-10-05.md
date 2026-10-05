# Terminal field return finished projection S0

Status: Decision / owner mapping selected
Execution row: MIRBUILDER-TERMINAL-FIELD-RETURN-FINISHED-PROJECTION-S0
Scope: preserve the existing source-issued I64Field return through canonical CFG
finishing, using the same captured PhysicalBoundary/FinishedBindings projection.
Related:
- mirbuilder-declared-object-tagged-borrow-physical-s1-2026-10-05.md
- ../design/mirbuilder-final-pipeline-ssot.md
- ../../../RULES.md

## Decision

Source authority + canonical issuer: existing TerminalRelationV1::I64Field,
its exact source exit/read site and emitted dst/base/field/block. No new source
vocabulary, result claim, borrowed Home or class inference. Existing draft
emission captures the physical boundary; finishing already issues FinishedBindings.

Production consumers: validate_finalized_child_emissions before capture,
validate_root_body for initial/root finishing, and validate_finalized_child_functions
for ordinary-child finishing. The current terminal field validator looks up the
original block even after the canonical sole-predecessor CFG merge moves that
read. Select the existing check_binding mapping; no second scan/registry/remapper.
Keep exact dst/base/field, recorded result and direct Return.value checks, with
existing boundary.validate_complete preserving original CFG/sequence.

Non-authority: any field read with the same result type, diagnostic names,
a guessed surviving block, relaxing final source correspondence, AST rewrites.
Fail-fast boundary: wrong/deleted/duplicate/reordered read, result drift, foreign
field/base/value or return drift remain rejected by the terminal/boundary owners.

Smallest slice: lend the existing physical binding checker to the terminal read
validator at its three callers, keeping draft None and finishing Some(projection).
Read-only worker reviews exact interface/return obligations before implementation.
No additional visibility/authority or unchecked default callback is authorized.
Replacement: original-block-only terminal read check after finishing. Existing
draft checks, field-read coverage, source result contracts and boundary remain.

## Acceptance

- Original guarded direct formal.field return, null/object, both optimization
  settings through source->final MIR->physical JSON; execute both arms.
- Exact moved read allowed only through original boundary projection; reject
  orphan movement without canonical CFG, wrong field/base/dst/Return and extra
  read/redefinition. Focused positive/negative tests use existing owner fixtures.
- Retain S1 ingress/forwarding, physical boundary and source field families.
- Package red classification and selected scope pins; guard root1351 debt is
  HEAD-identical baseline, never full-green evidence. Source files below800.
- Owner README, pointer, whitespace, exact scoped staging; fresh unchanged app
  probe if a relevant production terminal changes. No .hako workaround.

## Evidence / non-claims

S1 landed/pushed fe0c9ad278: source20both-opt/C20Normal-Fault plus15 negatives,
entry21, source25, boundary11, JSON29; package622/exact known3, C3 regressions.
Fresh quick llvm-boundary PASS; unchanged mimalloc-lite rc1 at
ordinary-new/local-commit/artifact-unowned-lifecycle-site, no EXE.
Previous direct-return source witness optfalse published; opttrue failed at
ordinary-terminal-field-return/physical-drift. Read-only worker traced original
bb27 read moving into predecessor bb26; final opttrue coordinates not yet measured.
Protected C/Array/S2 changes remain uncommitted and must be preserved. No Cargo
live at selection. Required call-valued result/Provided and vocabulary obligations,
production switching/retirement and whole finite pipeline remain open.
