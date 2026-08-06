---
Status: Closed implementation receipt / caller-zero
Date: 2026-08-06
Decision: LOOP-FAMILY-LOOPTRUE-OBSERVATION-S1
Authority: docs/development/current/main/design/loop-family-observation-policy-ssot.md
---

# LoopTrue observation S1 design task

## Purpose

Define the bounded source-to-neutral-observation boundary for the LoopTrue
`loop(true)` plus explicit `break`/`continue` branch shape. This task is a
design product only. Its bounded implementation slice is now landed. It must
not open selector, Recipe, Builder/MIR, physical, retry, fallback, production,
or legacy-retirement work.

## Sole source authority

`src/mir/compiler/loop_true_break_continue_projection.rs::issue_loop_true_break_continue_source_projection_v1`
is the only syntax observer. It may observe the natural
`FunctionSourceViewV1`, resolver-issued `VerifiedResolvedLoopSourceV1`,
`BindingRefV1`, loop/if region records, and resolved exit records. Its sealed
projection is AST-free and move-only.

The existing `src/mir/loop_route_policy/loop_true_break_continue.rs` is a
legacy schedule/cursor/winner demand owner. It is not an input to S1 and is
not deleted until the common family selector and physical production cutover
prove zero non-historical callers.

## Exact candidate contract

The S1 candidate is only an exact bounded source projection containing:

```text
loop condition: literal true
loop body: exactly one If
If: explicit else; both arms exactly one statement
then arm: direct Break
else arm: direct Continue
branch condition: Equal(local BindingRef, integer literal)
break/continue: resolver exit records target the same loop region
root: resolver-issued loop binding and execution frame
identity: owner + function origin + source kind + loop site + frame
```

Before a policy candidate can be sealed, the source product or a dedicated
AST-free accessor must expose function origin/source kind/site together with
the frame. An adapter-only precheck is not sufficient as the long-term
identity contract. This identity extension is part of the bounded source
product slice; it is not a policy, Recipe, or Builder concern.

## Neutral transport and disposition

Add a move-only source-attempt DTO under `loop_structural_facts` and a pure
observer under `loop_route_policy`. The policy receives one source attempt and
one separately sealed owner/origin/kind/site/frame + mode + coverage context.
It emits exactly one of:

```text
Candidate(VerifiedLoopTrueBreakContinueSourceProjectionV1)
Declined(NotLoopTrueBreakContinueShape)
Unresolved(typed missing/opaque source fact)
Rejected(typed identity/binding/control conflict)
```

The mapping is fixed:

| Source/context evidence | S1 disposition |
| --- | --- |
| complete, sealed mode, matching identity/frame, exact projection | `Candidate` |
| known syntactic non-shape: non-true root, body/branch arity, branch/else shape, non-Equal condition, non-integer bound | `Declined` |
| incomplete or unsealed coverage/mode; source navigation/lookup failure; missing binding or exit fact | `Unresolved` |
| foreign owner/origin/kind/site/frame; mode mismatch; upvar; exit target mismatch; source-binding structural/owner conflict; candidate/context mismatch | `Rejected` |

`ExitResolution` is a missing resolver fact (`Unresolved`), while
`ExitTargetMismatch` is a structural conflict (`Rejected`). Root source
binding errors preserve owner/structural provenance. No source distinction may
be collapsed into a permissive decline.

The three modes are evidence snapshots only. They do not select a route,
suppress another family, retry, or invoke fallback. Incomplete coverage is
never a decline because unseen source can change the result.

## Finite implementation slice

1. Extend the source projection with an AST-free identity accessor/product if
   needed; preserve the existing typed source/lookup distinctions at the
   adapter boundary.
2. Add the neutral LoopTrue source-attempt/identity/mode/coverage DTO.
3. Add a `#![cfg(test)]` compiler adapter that prechecks identity and maps
   every projection outcome without exporting compiler reject enums.
4. Add one pure policy observer and focused tests for the positive three-mode
   case, each known decline family, incomplete/unsealed inputs, foreign/frame
   and mode mismatch, missing fact, upvar, binding conflict, and exit-target
   conflict.
5. Extend the existing shared recursive authority/line/caller-zero guard;
   do not add a row-specific shell guard.
6. Update the exact reference receipt, compiler/structural-facts/policy
   READMEs, current pointer, and workstream in the same implementation
   commit. The implementation commit must state that the reference documents
   were updated; no later documentation debt is allowed.

## Implementation receipt

The bounded S1 slice is landed. The source projection now carries the complete
AST-free identity (`owner + function origin + source kind + loop site + frame`)
and preserves the original three-part `into_parts` compatibility view. A
`#![cfg(test)]` compiler adapter maps typed projection outcomes into the
neutral source-attempt DTO without exporting compiler rejection enums. The
policy observer is pure and consumes only that DTO plus a separately sealed
identity/mode/coverage context.

Nine focused policy tests and eight projection tests are green. The shared
recursive authority/line/caller-zero guard is green. The implementation commit
also updates the design SSOT, reference matrix, compiler/structural-facts/
policy READMEs, current pointer, and workstream; there is no deferred reference
documentation debt for this slice.

## Stop lines and completion

This design does not authorize `family_selection.rs`, the legacy policy
demand, `LoopRouteId`, frozen schedules/cursors, Recipe/JoinSig/BindingKey,
Builder/MIR/ValueId/PHI, physical lowering, retry/fallback, a production
caller, or deletion of the old LoopTrue route. The design row is complete when
the source identity contract, disposition table, finite implementation slice,
and caller-zero/non-claims are recorded in the design SSOT and current
pointer. This row is now complete; stop at the next common family-selection/
admission-window design boundary.
