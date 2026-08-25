---
Status: SSOT
Scope: current MIR call-site authority and retirement contract
Decision: accepted policy; R6 core field cutover remains design-gated
Updated: 2026-08-25
Related:
- docs/development/current/main/CURRENT_STATE.toml
- docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
- docs/development/current/main/investigations/mir-call-core-r6-d1-manifest-2026-08-25.toml
- docs/development/current/main/investigations/mir-call-core-r6-d1b-method-none-manifest-2026-08-25.toml
- docs/development/current/main/investigations/mir-call-core-r6-d1c1-generic-function-call-handoff-2026-08-25.toml
- docs/development/current/main/investigations/mir-call-core-r6-d1e-normal-main-thunk-issuer-2026-08-25.toml
- docs/development/current/main/investigations/mir-call-core-r6-d1f-typed-method-issuer-2026-08-25.toml
- docs/development/current/main/investigations/mir-call-core-r6-d1g-builder-emit-receiver-reconstruction-2026-08-25.toml
- docs/development/current/main/investigations/mir-call-core-r6-d1h-phi-call-rematerialization-d0-2026-08-25.toml
- docs/development/current/main/investigations/mir-call-core-r6-d1i-concat3-extern-rewrite-issuer-2026-08-25.toml
- docs/development/current/main/investigations/mir-call-core-r6-d1k-retained-len-extern-issuer-2026-08-25.toml
- docs/development/current/main/investigations/mir-call-core-r6-d1l-direct-substring-len-extern-issuer-2026-08-25.toml
- docs/development/current/main/investigations/mir-call-core-r6-d1m-method-set-value-rewrite-issuer-2026-08-25.toml
- docs/development/current/main/investigations/mir-call-core-r6-d1-next-edge-census-2026-08-25.toml
- docs/development/current/main/investigations/mir-call-core-r6-d1n-user-box-publication-call-operand-projection-d0-2026-08-25.toml
- docs/development/current/main/investigations/mir-call-core-r6-d1o-publication-host-extern-issuer-i0-2026-08-25.toml
- docs/development/current/main/investigations/mir-call-core-r6-d1q-publication-return-extern-issuer-i0-2026-08-25.toml
- docs/development/current/main/investigations/mir-call-core-r6-d1r-materialization-store-extern-issuer-i0-2026-08-25.toml
- docs/development/current/main/investigations/mir-call-core-r6-d1s-publication-substring-extern-issuer-i0-2026-08-25.toml
- docs/development/current/main/investigations/mir-call-final-shape-and-ingress-boundary-design-2026-08-25.md
- docs/development/current/main/design/archive/mir-canonical-callsite-lane-history-2026-08-25.md
---

# MIR Canonical Callsite Lane

This is the compact current owner. Landed chronology, full inventories, and
consultation transcripts live in the linked historical ledger and finite
investigation manifests. The active row is selected only by `CURRENT_STATE.toml`
and the rolling workstream card.

## Durable decision

The end state is one typed call target and one physical issuer:

```text
typed producer or owner-private JSON-v0 ingress
  -> exact Callee
  -> MirInstruction::call
  -> optimizer/printer/wire/backend projection or execution
```

The final core shape is intended to be:

```rust
Call {
    dst: Option<ValueId>,
    callee: Callee,
    args: Vec<ValueId>,
    effects: EffectMask,
}
```

The current migration shape still contains `func: ValueId` and
`callee: Option<Callee>`. `MirInstruction::call` is the thin canonical helper;
it currently stores the transitional `INVALID`/`Some` representation. No field
deletion is authorized until the R6 writer and consumer blockers are closed.

## Authority map

| Boundary | Sole authority | Explicit non-authority |
|---|---|---|
| typed producer | resolver/source target that already knows the exact `Callee` | backend lookup, text reconstruction, default target |
| physical issuer | `MirInstruction::call(dst, callee, args, effects)` | target classification or retry inside the constructor |
| MIR JSON-v0 | private `JsonV0CallInput` and its one-shot resolver | public `LegacyCall`, core `None`, optimizer scan |
| JSON-v1 | explicit typed callee object | legacy decoration or malformed-target retry |
| optimizer | stored `Callee` plus ValueId remap | `Const(String)` target inference or target reclassification |
| interpreter/backend | typed dispatch of stored `Callee` | `func` register load, by-name fallback, registry retry |
| JSON/native egress | projection of stored target and profile | `receiver.unwrap_or(func)`, metadata/name retry |

`Callee` owns target ValueId operands through its immutable projection:

```text
Method.receiver; Value(value); Closure.captures in stored order, then me_capture
then Call.args in stored order; duplicates are preserved.
```

Escape, ownership, query, and JoinIR policies may reuse that enumeration but
retain their own barrier/ownership decisions. `Callee::Method(None)` is an open
legacy state, not a static-call authority. Static calls must eventually use a
qualified `Global(owner.method/arity)`; instance calls require `Method(receiver)`.

## Fail-fast contract

Resolve the target exactly once before the first irreversible boundary:

```text
target/source relation
  -> validation and finite state decision
  -> exact Callee
  -> argument/block/wire/object/backend effect
```

Malformed explicit targets, missing/ambiguous/foreign legacy relations, missing
receivers, duplicate or consumed claims, and profile-incompatible shapes reject
before publication. A rejected target never retries through another name,
receiver, registry, metadata, optimizer, or backend route.

## Current selected row

`CURRENT_STATE.toml` records the current row:

```text
MIR-CALL-CORE-R6-D1S-PUBLICATION-SUBSTRING-EXTERN-ISSUER-I0
```

D1M landed at `f7a442f524` after two upper-worker audits. It was limited to
the one shared method-set value rewrite writer; its boundary and receipt are
in:

```text
docs/development/current/main/investigations/
  mir-call-core-r6-d1m-method-set-value-rewrite-issuer-2026-08-25.toml
```

D1M's two existing route tests plus direct helper positive/negative matrix,
shared corridor guard, pointer/reference-route guards, rustfmt, and diff checks
are green; the quick profile's 441 warnings remain the pre-existing baseline.
The matcher is the sole eligibility authority, and whole-pass rollback is not
claimed. Full Call field retirement and string-corridor family retirement
remain separate.

The independent recursion-depth restore candidate landed at `74829ee3d2` after
two upper-worker audits. Both overflow return windows restore the exact entry
depth; focused overflow/success/error tests, the scoped guard, pointer/reference
guards, rustfmt, and diff checks are green. Panic/unwind restoration is not
claimed.

D1N landed at `0738d722a3` after the operand-SSOT audit. The single user-box
publication consumer now delegates Call membership to `MirInstruction::used_values()`
and deletes its local `func`/receiver matcher; typed Method/Value/Closure/arg
positives, stale-func/dst-only negatives, legacy-func parity, and the 5-test
module suite are green. D1O's plan-owned publication host Extern writer landed
at `d2826802f3`; selected Call writers are now 15 (publication family 3). D1P's
Store/FieldSet publication writer landed at `6912edb287`; the existing
PublicationWriteBoundaryPlan now delegates to `MirInstruction::call`, with
the two boundary proofs preserving destination, five-argument order, READ
effects, copy removal, and adjacency. The receipt closes this writer while
full Call retirement remains separate. D1Q landed at `491e212e77`: the two
PublicationReturnPlan writers (instruction Return and terminator Return)
delegate to `MirInstruction::call` as one atomic family. Selected writers are
now 12 and the publication Return family is 0; the focused six-test proof and
shared guard are green. D1R landed at `82be5daa97`: the one
`MaterializationStorePlan` writer in `concat_corridor_apply.rs` now delegates to
`MirInstruction::call`. D1S landed at `5d5c564e82`: the one
`PublicationHelperSubstringPlan` writer in the same file delegates to
`MirInstruction::call`, preserving composed start/end Add roots and
plan-owned effects. Selected writers are 11 -> 10 and the concat family is
8 -> 7; no other concat, unified, array, or canonicalizer writer is authorized.

D1B `Method(None)` and D1C1 bare `FunctionCall` remain separate
`NoSafeSlice`/`CutoverBlockerOpen` boundaries; D1H does not authorize PHI
admission/purity/rollback changes, field deletion, `Method(None)` repair,
other Callee variants, or generic FunctionCall changes.

## R6 retirement order

The order is producer first, then canonical consumers, then atomic schema:

```text
R1 exact qualified producers
R2 owner-private MIR-v0 input state
R3 pre-core legacy resolution and late-issuer retirement
R4 Callee operand/remap/semantic consumer SSOT
R5 optimizer/interpreter/printer/JSON/selected backend closure
R6 Call { func, Option<Callee> } -> mandatory-Callee core
R7 impossible-state guards and reference/docs closeout
```

R6 is not selected while any `Method(None)`, Closure/Constructor construction
edge, `MirCall`/`CallFlags` transport reader, JSON/VM/native fallback, or direct
writer census row remains open. The exact writer inventory is the D1 manifest:

```text
docs/development/current/main/investigations/
  mir-call-core-r6-d1-manifest-2026-08-25.toml
```

## Finite state boundary

```text
CanonicalReady                  exact typed Callee -> Call
CompatibilityReady              owner-private legacy relation resolved once
InvalidExplicit                 typed reject; no legacy retry
MissingTarget                   typed reject before publication
MissingOrAmbiguousReceiver      typed reject; no static/instance guess
ForeignOrDuplicateClaim        typed reject; no re-consume
MethodNoneLegacy                open R6 blocker, never canonical authority
ClosureOrConstructorShape       construction boundary, not generic target repair
OutsideSelectedBackend          ParkedSealed until CURRENT_STATE reselects it
```

Positive evidence compares `callee`, receiver/target operands, args order, dst,
effects, and execution result. Numeric `func` sentinels, target strings,
printer output, or compatibility fixtures are not semantic parity evidence.

## Parked and non-goal boundaries

- PyVM, reference/Python/WASM, and non-selected backend lanes are `ParkedSealed`.
- Closure/NewClosure, Constructor/NewBox, `MirCall`/`CallFlags`, JoinIR remap,
  normal-root projection cleanup, physical-type layout, and warning retirement
  are separate rows unless `CURRENT_STATE.toml` selects them.
- The 2,595-line normal-root manifest is a dedicated owner and is not copied,
  slimmed, or used as R6 Call authority.

## Reusable evidence

```bash
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mir_call_canonical_corridor_guard.sh
git diff --check
```

Cargo gates are run only by an accepted fast/closeout card. Design-stop audits
must not be converted into implementation permission by local green alone.

## Historical owner

The superseded long-form SSOT, including R1-R6 history and closed design rows,
is preserved at:

```text
docs/development/current/main/design/archive/
  mir-canonical-callsite-lane-history-2026-08-25.md
```

It is traceability-only; current decisions and next actions belong here and in
the active pointer/card.
