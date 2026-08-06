---
Status: landed caller-zero source brand / assembler next
Date: 2026-08-06
Decision: LOOP-FAMILY-WINDOW-LEASE-ISSUER-S0
Authority: docs/development/current/main/design/loop-family-observation-policy-ssot.md
---

# Loop-family window lease issuer S0

## Purpose

Add the resolver-owned source identity brand required before the common
admission assembler. This is a caller-zero source seam. It does not assemble
rows, select a family, issue Recipe/JoinSig/BindingKey demand, or enter
Builder/MIR.

## Accepted boundary

The sole issuer is `VerifiedResolvedFunctionV1` in the resolved-semantics
layer. It consumes one exact `VerifiedResolvedLoopSourceV1` lookup and issues
one non-`Clone`, non-`Copy` `VerifiedLoopFamilyWindowLeaseV1` containing only:

```text
FunctionOwnerIdV1
VerifiedResolvedLoopSourceV1
private seal
```

The wrapped source token remains the authority for function origin, source
kind, root site, and execution frame. The lease contains no AST, source view,
forest, names, route IDs, schedules, cursors, Recipe data, Builder/MIR data,
or environment policy. Mode and coverage are policy-row evidence, not
resolver lease fields.

The lease may be consumed by the future five-row assembler. Family projector
fan-out is not opened by S0; future fan-out must be one explicit resolver
operation and may not clone or relookup this source token.

## Allowed implementation

```text
VerifiedResolvedFunctionV1
  -> exact resolved_loop_source(site)
  -> VerifiedLoopFamilyWindowLeaseV1
```

Missing/foreign source lookup remains a typed issuer error. No loose
`(owner, site, frame)` test constructor is exposed to production code.

## Stop lines

S0 must not:

```text
import AST or VerifiedResolvedSourceUnit
import shared_loop_source_window_tests.rs
import GenericSourceLeaseV1
assemble or normalize family rows
read mode/coverage from environment
select Candidate/Declined rows
call family_selection.rs or legacy policy
issue Recipe/JoinSig/BindingKey
enter Builder/MIR/physical routes
add retry/fallback/production caller
```

## Acceptance evidence

- exact loop site issues one lease with owner/origin/source-kind/site/frame
- missing/non-loop/foreign source is rejected before a lease is published
- lease has no `Clone`/`Copy` implementation and no AST-bearing field
- resolver module and focused tests remain below 800 lines
- focused lease tests and the standard current-state/in-place guards pass
- this task, the observation SSOT, current mirrors, and reference receipt are
  synchronized in the implementation commit

After S0 lands, the next bounded row is
`LOOP-FAMILY-COMMON-ADMISSION-ASSEMBLER-S1`. Selector, production caller,
legacy retirement, and physical cutover remain closed.

## Implementation receipt

`VerifiedResolvedFunctionV1::issue_loop_family_window_lease_v1` now consumes
the exact resolver loop-source lookup and publishes the sealed non-`Clone` /
non-`Copy` lease. Three focused issuer tests cover success, missing source,
and distinct resolver owner brands. The shared in-place replacement guard and
current-state pointer guard are green; the implementation and reference
documents were synchronized in the same commit. No family fan-out, assembler,
selector, Recipe, Builder/MIR, or production caller was opened.
