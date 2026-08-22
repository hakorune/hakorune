Status: I0 implementation complete; terminal-only F2 accepted, no ordinary Outside consumer opened
Task: MIR-CALLABLE-LOOP-OUTSIDE-OBSERVED-CLASS-D0
Date: 2026-08-23
Priority: separate source observation from Ready admission without opening an ordinary Outside consumer
Parent: MIR-CALLABLE-LOOP-STRUCTURAL-LEASE-RETIRE-D0
Current execution row: MIR-CALLABLE-LOOP-OUTSIDE-OBSERVED-CLASS-I0 (closeout)
CurrentCard: docs/development/current/main/investigations/mirbuilder-callable-loop-outside-observed-class-d0-2026-08-23.md
NextCard: MIR-CALLABLE-LOOP-UNPUBLISHED-SESSION-CAPABILITY-D0 (design stop)
---

# Callable Loop Outside observed rows and Ready classes D0

## Six-line brief

Decision: Accept. Keep `Outside(BodyOnlyRebind)` terminal and use separate observed-row, Ready-class, and Outside-kind vocabularies. A body-only rebind is observation evidence, not proof that the binding belongs to the one-carrier Ready cohort. I0 is complete with the row/class split and private remainder validator; no ordinary consumer opens.
Source authority + canonical issuer: `CallableLoopSourceProjectionV1::project_disposition` owns the exact resolver-issued variable/assignment/source-site rows; `VerifiedCallableSemanticLoopBindingScheduleV1::seal` remains the sole issuer of the admitted Ready schedule. A bounded private validator may validate the Ready remainder but must not issue a throwaway Verified product.
Non-authority: `has_rebind` as a complete classifier, `CallableLoopReadyBindingClassV1::Carrier` for Outside rows, binding/name/ordinal joins, Builder state, AST inference, ValueId/MIR facts, ordinary JoinIR lowering, and a default/empty row.
Fail-fast boundary: after source rows are collected and grouped by exact `BindingRefV1`, classify complete body-only rebind rows before Ready schedule issuance. Outside returns typed terminal evidence with zero Builder effect; malformed/foreign/duplicate coverage rejects before any consumer.
Smallest next slice: introduce observed-row and admitted-class vocabulary in the existing handoff owner, preserve binding-to-receipt relation in `CallableLoopOutsideReasonV1`, and replace the Outside remainder `VerifiedCallableSemanticLoopBindingScheduleV1::seal(...)?` with a private validation function. The existing terminal Outside and Ready production consumers remain unchanged.
Non-claims: no ordinary Outside consumer, no source rescan, no new physical receipt, no Ready cohort expansion, no `LoopRouteContext`, no Builder capability, no pure-plan split, no publication, fallback, or generic Loop activation.

## Current source boundary

The existing source handoff is one-way:

```text
resolver source projection
  -> exact variable/assignment source receipts
  -> rows grouped by BindingRefV1
  -> Ready(schedule) or Outside(reason)
  -> raw entry terminal/semantic Recipe branch
```

The production Outside consumer is intentionally terminal:

```text
Outside(reason) -> typed String at outer API -> Builder effect = 0
```

There is no accepted ordinary source-aware Outside consumer. This D0 must not
turn the new row vocabulary into permission to lower `Outside` through
`lower_loop_or_freeze_v1` or `PlanLowerer`.

## Worker Decision — Dirac (read-only)

The worker audited the current source handoff and found no NoSafeSlice for the
terminal-only slice:

```text
source authority       = CallableSemanticLoweringState locals/variables/assignments
projection issuer      = CallableLoopSourceProjectionV1::project_disposition
Ready consumer         = one semantic Recipe -> physical adapter
Outside consumer       = one terminal error path, Builder effect = 0
```

The worker confirmed that the old shared row combined
observed receipts with Ready class, and that the Outside remainder calls
`VerifiedCallableSemanticLoopBindingScheduleV1::seal(...)` only to validate then
drops it. The accepted I0 is to split observed/Ready/Outside row vocabulary and
replace that throwaway issuance with a private validator. Builder, AST, MIR,
fallback, and ordinary Outside lowering remain outside the slice.

## Problem in the current vocabulary

The former shared row builder derived:

```text
iteration local -> IterationLocal
else has BodyRebind -> Carrier
else -> ReadOnlyOperand
```

That is adequate for a Ready schedule whose complete carrier is later checked,
but it is misleading inside `Outside(BodyOnlyRebind)`: `out` and `handled` have
BodyRebind evidence yet no `ConditionRead`, so they are deliberately outside
the current admitted cohort. The row proves source observation, not Ready
admission.

The current Outside branch also calls
`VerifiedCallableSemanticLoopBindingScheduleV1::seal(...)` on the remaining
rows only to validate them, then drops the returned product. That creates a
Verified semantic product without a consumer and makes the Outside path appear
to issue a Ready authority.

## Proposed authority/type shape

Keep one source scan and one exact grouped relation:

```rust
struct CallableLoopObservedBindingRowV1 {
    binding: BindingRefV1,
    receipts: Box<[CallableLoopBindingReceiptV1]>,
}

enum CallableLoopReadyBindingClassV1 {
    Carrier,
    ReadOnlyOperand,
    IterationLocal,
}

enum CallableLoopOutsideKindV1 {
    BodyOnlyRebind,
}

struct CallableLoopOutsideRowV1 {
    observed: CallableLoopObservedBindingRowV1,
    kind: CallableLoopOutsideKindV1,
}
```

The implementation keeps this shape in the existing handoff owner; it does not
create a second source scan or parallel projection.

The Ready class is issued only after the first-cohort predicate succeeds:

```text
Carrier       = ConditionRead + BodyRead + BodyRebind
ReadOnly      = source read, no rebind, not an admitted iteration local
IterationLocal = exact body-local ownership and complete source coverage
```

`BodyRebind` alone is never a Ready Carrier proof.

## Accepted implementation — MIR-CALLABLE-LOOP-OUTSIDE-OBSERVED-CLASS-I0

The implementation may proceed only within `normal_callable_loop_handoff.rs`
and its focused tests/guard:

```text
one exact source grouping pass
  -> observed rows
  -> Ready-only admitted class
  -> Outside-only kind
  -> private remainder validation
  -> existing Ready/Outside consumers
```

No new source scan, semantic issuer, Builder capability, ordinary Outside
consumer, or fallback is part of I0.

## I0 closeout evidence

The implementation now has:

```text
CallableLoopObservedBindingRowV1       = observed binding + exact receipts
CallableLoopReadyBindingClassV1        = Ready-only admission vocabulary
CallableLoopOutsideKindV1::BodyOnlyRebind = explicit terminal kind
validate_ready_remainder(...)          = private validation, no throwaway Verified schedule
```

The existing Ready semantic/physical consumer and the existing Outside
terminal consumer remain the only production consumers. The focused handoff
tests and the Outside disposition guard cover the separated vocabulary,
exact receipts, and zero-effect terminal boundary. The next design slice is
not opened by this I0.

## Finite state table

| State | Sole owner | Effect | Allowed next | Fallback |
| --- | --- | ---: | --- | --- |
| `ObservedRows` | source projection | none | validate grouping/classification | none |
| `ReadyAdmitted` | schedule issuer | none | `claim_all` -> Recipe | no Outside fallback |
| `OutsideBodyOnlyRebind` | source projection | none | typed terminal only | no ordinary lowering |
| `Incomplete` | source coverage validator | none | typed reject/discard | no empty/default row |
| `IntegrityInvalid` | source relation validator | none | typed reject/discard | no repair/re-pair |
| `NoSafeSlice` | design owner | none | stop and review | never guess |

The `OutsideBodyOnlyRebind` row is complete source evidence but not a
Candidate/Ready state. It must not be converted into `Absent`, `NonCandidate`,
or a compatibility retry.

## Acceptance evidence

The completed I0 names:

```text
one source grouping/observation owner
one Ready schedule issuer
one Outside row relation with binding + receipts + kind
one private Ready-remainder validator
zero Verified schedule construction on the Outside-only validation path
existing Outside terminal consumer count = 1
existing Ready production consumer count unchanged
```

Focused evidence covers:

```text
body-only rebind -> OutsideBodyOnlyRebind with exact receipts
condition+body+rebind -> Ready Carrier
read-only binding -> Ready ReadOnlyOperand or explicit current class
foreign/duplicate/missing source row -> typed reject
Outside path Builder effect = 0
no second source scan and no raw/ordinal re-pairing
```

Observed evidence:

```text
cargo test --profile quick --lib normal_callable_loop_handoff: 6 passed
cargo test --profile quick --lib normal_callable_loop_source_facts::tests: 7 passed
cargo test --profile quick --lib selected_dynamic_physical_emitter: 10 passed
bash tools/checks/rust_mirbuilder_callable_loop_outside_disposition_p0_guard.sh: PASS
bash tools/checks/rust_mirbuilder_callable_loop_source_facts_issuer_p0_guard.sh: PASS
bash tools/checks/rust_mirbuilder_callable_loop_generic_facts_policy_p0_guard.sh: PASS
bash tools/checks/current_state_pointer_guard.sh: PASS
git diff --check: PASS
```

The generic terminal-port guard remains a caller-zero historical guard; its
message currently says that a named consumer is required even though the
assertion requires zero production callers. It is not part of this F2 slice.

## NoSafeSlice

Remain at design stop if:

```text
observed rows cannot retain binding-to-receipt relation in one aggregate
Ready class semantics need a new source authority not present in the resolver
Outside requires an ordinary consumer before the vocabulary can be separated
the remainder validator must issue a Verified schedule to prove completeness
the current first-cohort class cannot be expressed without BodyRebind inference
the change needs Builder/MIR/ValueId or a second route classifier
```

Do not solve the gap with `Option`, empty/default catalogs, `has_rebind` alone,
AST rescans, or a new structural lease.

## Parked after this D0

`MIR-CALLABLE-LOOP-UNPUBLISHED-SESSION-CAPABILITY-D0` remains separate. The
physical adapter currently composes with `&mut MirBuilder` before PlanVerifier;
that is a later capability/pure-plan design and must not be mixed into this
source row vocabulary slice.
