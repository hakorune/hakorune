---
Status: accepted design stop — source requirement is carried, no production physical consumer is issued
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-REQUIRED-ARGUMENT-CONSUMER-D0
Parent: docs/development/current/main/investigations/mirbuilder-compatibility-seam-final-ratchet-d0-2026-08-21.md
ProductionCaller: selected-normal Script direct-static bridge only; no new caller
ReplacementCell: name one source-bound physical argument consumer before validating or claiming required ordinals; otherwise retain NoSafeSlice
Classification: design stop; no new semantic receipt, source shape, physical route, or production switch
Execution row: SCRIPT-DIRECT-STATIC-REQUIRED-ARGUMENT-CONSUMER-D0
---

# SCRIPT-DIRECT-STATIC-REQUIRED-ARGUMENT-CONSUMER-D0

## Six-line brief

Decision: Keep `required_callee_i64_arguments` as a source-issued callee
contract, but do not pretend that the current Script bridge consumes it. The
field is carried through Bundle → PublicationOwner → Recipe → Join → claim;
this D0 must name a real physical consumer or remain `NoSafeSlice`.

Source authority + canonical issuer: the callee result disposition
`VerifiedCallableResultDispositionV1::ExactI64 { required_i64_arguments }`,
issued by `VerifiedSameModuleCallableResultCatalogV1`, is the sole authority
for callee-required ordinals. The caller-propagated requirement set in
`VerifiedCallableResultCallSiteV1` is a distinct fact and may not be replaced
by the callee list.

Non-authority: `ValueId`, `MirType`, argument count, target arity, AST names or
ordinals, generic Call emission, `unwrap_or_default`, finalizer type hints,
and a successful physical call cannot prove that a required source argument
is an exact integer.

Fail-fast boundary: before any future required-argument claim or typed
physical validation. If a selected row has a non-empty required-ordinal set
but no named source-bound consumer, stop at `NoSafeSlice`; never infer from a
lowered value, silently drop the set, or retry through the ordinary route.

Smallest next slice: read-only consumer census and source/physical contract
design for this row. Only after one issuer, one consumer, and one complete
argument representation receipt are named may a separate
`...REQUIRED-ARGUMENT-CONSUMER-I0` be opened.

Non-claims: no argument type publication, no new Recipe/Join/receipt, no
bridge rewrite, no `Call` representation change, no Compatibility/Deferred
repair, no raw retirement, no ABI/backend/performance/production claim.

## Classification-completeness receipt

Every required-argument observation is classified before a physical consumer
could act. `NoSafeSlice` is a development stop, not a source disposition.

| state | authority / issuer | before effects | allowed terminal / continuation | fallback |
|---|---|---|---|---|
| `ExactI64Empty` | callee `ExactI64` disposition with an empty ordinal set | existing selected bridge may proceed under its existing contract | existing generic Call/publication path | no required-argument inference |
| `ExactI64Required` | callee `ExactI64` disposition with one or more ordinals | retain the source row; do not claim a new typed physical fact | `NoSafeSlice` until a named consumer is issued | no `ValueId`/`MirType` inference, no ordinary retry |
| `ExactNominalBox` | result disposition | no Script ExactI64 selection | existing unselected/unsupported terminal | never coerce to integer |
| `Unavailable` | result catalog disposition or missing target result | no bridge claim or child-side physical assertion | explicit unselected/compatibility terminal | no empty requirement set |
| `Absent` (`NoCandidate`) | Script result bundle has no exact row at the site | no required-argument effect | existing no-row route | never fabricate a source row |
| `SourceMismatch` | Bundle/Recipe/Join identity and ordinal validation | reject before physical claim/effects | typed freeze | no AST/name re-pairing |
| `DetachedCandidateOnly` | `VerifiedScriptDirectStaticPhysicalInputV1::issue` plus the detached `direct_static_entry_kernel` test helper | no production effect; review evidence only | test-only terminal; never a production claim | cannot stand in for a required-ordinal consumer |
| `ConsumerReady` | future source-bound argument representation owner | validate exact source ordinals before its physical effect | separate future I0 only | no current consumer is implied |

Negative witnesses must map to exactly one row above. In particular,
`required_callee_i64_arguments().is_empty()` in a test is evidence only for
`ExactI64Empty`; it does not prove that non-empty rows have a consumer.

## Evidence census

The current source chain is:

```text
VerifiedCallableResultDispositionV1::ExactI64
  -> VerifiedScriptDirectStaticResultBundleV1
  -> VerifiedScriptDirectStaticResultPublicationOwnerV1
  -> VerifiedScriptDirectStaticRecipeDemandV1
  -> VerifiedScriptDirectStaticJoinRowV1
  -> ScriptDirectStaticClaimedRowV1
```

The bundle currently copies `disposition.required_i64_arguments()` into the
field named `required_callee_i64_arguments`. The same result subsystem also
has a different caller-side field, `VerifiedCallableResultCallSiteV1::required_i64_arguments()`;
the two lists have different authorities and must not be merged.

The physical census found no production consumer of the carried field:

- `ScriptDirectStaticClaimedRowV1::required_callee_i64_arguments()` is an
  accessor used only by its definition/tests; the physical bridge does not
  read it.
- `calls/script_direct_static_physical_bridge.rs` validates target namespace
  and arity, lowers ordered arguments, emits the existing receipt-required
  Call, and publishes ExactI64. It does not validate required ordinals.
- `calls/static_result_publication_physical_bridge.rs` destructures the
  required list as `_required_i64_arguments`; it is therefore not a consumer.
- `VerifiedScriptDirectStaticPhysicalInputV1::issue` does co-seal the Join with
  `VerifiedScriptDirectStaticScalarOperandRecipeV1`, and
  `script_physical_exit/direct_static_entry_kernel.rs` can lower that input,
  but the repository census finds no production caller for the kernel (only
  its own focused unit test). The helper also never reads
  `required_callee_i64_arguments`; its presence is therefore a candidate-only
  physical path, not the missing required-ordinal consumer.
- `ValueId`/`MirType` after argument descent are physical observations, not a
  source issuer for an argument representation contract.
- `VerifiedCallableResultActivationSourceSiteV1` and the older activation
  gate are a separate callable-result lane; they do not consume ScriptRoot
  direct-static claims and cannot be silently reused as this consumer.

The `unwrap_or_default()` at Script bundle projection is also not a consumer.
For an ExactI64 disposition it should be unreachable as `None`, while a
nominal/unsupported disposition must not be converted into an empty required
set. A later implementation row must choose a typed mismatch or a proven
representation path; this D0 does not repair it.

## NoSafeSlice boundary

This row remains a design stop when any of the following is true:

1. The intended consumer is only a generic Call emitter, finalizer, or
   `ValueId`/`MirType` lookup.
2. The caller-side and callee-side required ordinal sets cannot be kept as
   separate source facts.
3. A missing consumer is represented as an empty list, `None`, or a successful
   compatibility route.
4. Required-argument validation happens only after argument effects or Call
   emission and cannot poison/discard the isolated candidate.
5. A new physical argument receipt would need to be inferred from lowered MIR
   instead of being issued by the source/Facts owner.
6. The proposed consumer would widen Script accepted shapes, repair
   Compatibility/Deferred/RawLegacy, or change ABI/Call representation.
7. Any implementation would grow a 760-line owner toward the 800-line hard
   stop rather than split by responsibility.
8. The only apparent physical-input/kernel path is test-only or drops the
   required ordinal set; it cannot be promoted by wiring a caller or by
   treating scalar operand lowering as proof of the callee contract.

The next implementation card, if ever opened, must name the exact source
argument representation owner, the physical consumer, its finite state table,
and the old non-consuming edge it retires. Until then this card is accepted
as `NoSafeSlice`, not as a hidden requirement to add a default check.

The existing scalar operand Recipe is a useful candidate input for that future
row, but it only proves an AST-free integer expression tree at each argument
site. A future consumer must still co-seal the required ordinal set with those
argument trees and validate that every required ordinal selects an exact
integer representation before the physical Call. Operand-tree lowering alone
is not that proof.

## Review receipt

- finite state table includes selected, neutral, unsupported, absent, and
  identity-mismatch outcomes;
- source and caller-required ordinal authorities are explicitly distinct;
- `rg` census shows no current physical consumer of the carried field;
- the existing physical-input/detached-kernel pair is explicitly classified
  as `DetachedCandidateOnly`, not production evidence;
- no compiler, fixture, semantic receipt, fallback, or production route was
  changed by this D0;
- the reusable classification-completeness guard is the focused review gate;
- compatibility cohort admission remains independently parked at its own
  `NoSafeSlice` card.

## D0 closeout and next design stop

The read-only consumer census is complete. The existing
`VerifiedScriptDirectStaticPhysicalInputV1` / detached-kernel pair is
`DetachedCandidateOnly`: both production-caller searches are zero, the helper
does not expose or validate `required_callee_i64_arguments`, and its only call
is its focused unit test. The selected-normal lowering input also carries no
scalar operand Recipe, so this candidate cannot be promoted by wiring a
caller.

The callable `CallProofContextV1` cannot fill the gap: it requires a callable
owner key, callable parameter environment, and callable `call_result` row;
ScriptRoot has no such caller identity. A synthetic key or a `CallerOutside-
Catalog`-to-`Absent` conversion would create a second authority.

This D0 remains an accepted `NoSafeSlice` for physical consumption. The next
bounded design row is
[`SCRIPT-DIRECT-STATIC-REQUIRED-ARGUMENT-SOURCE-PROOF-D0`](script-direct-static-required-argument-source-proof-d0-2026-08-21.md),
which decides whether the existing resolver scalar operand facts can issue a
complete ScriptRoot argument representation proof. No I0 is opened until
that source proof and its physical consumer are both named.
