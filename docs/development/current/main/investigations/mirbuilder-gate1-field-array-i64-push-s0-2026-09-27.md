# MIRBUILDER-GATE1-FIELD-ARRAY-I64-PUSH-S0

Status: in-progress
Date: 2026-09-27
Parent: MIRBUILDER-GATE1-FIELD-ARRAY-I64-PUSH-D0 (closed,
  decision accepted) — implements the accepted relation from
  `mirbuilder-gate1-field-array-i64-push-d0-2026-09-27.md`.
  Workstream row H / Gate-1 series row 2 (source contract + physical
  connection).
Mode: fast — one responsibility, bounded to the accepted relation.

## Responsibility

Issue a `NamedArrayFieldResidenceRequirementV1` + `ArrayIntegerAppend`
contract for `local a = me.f; a.push(int)` where `f` is a declared
`ArrayBox` field with a generated-birth `new ArrayBox()` provider, and
carry the residence correspondence through lowering to the retained
physical marker — consuming the four `HakoAllocPage.seedBlocks/0` push
sites through the existing LoopCond coverage and writer.

## Boundary (from the accepted Decision)

- Claim: alias initializer is a `me`-field read of a declared,
  non-weak `ArrayBox` field. Once claimed, every failure is typed
  (`NamedArrayFieldResidenceIssueV1`) — no silent omission inside the
  claim, no generic retry.
- Provider: the field's `StoredFieldInitializer` generated-birth `new`
  proven in the birth ledger (`FieldWrite` + `construction_source` =
  bare `new ArrayBox()`). Package joins caller ledger -> field ref ->
  birth ledger; no cross-ledger guessing.
- Integer argument meaning: bounded source predicate (integer literal;
  local binding whose initializer + all rebind RHSs are
  integer-producing; same-owner `I64ToCaller` contract; integer-
  declared `me`-field read). `TextSourceMissing` on a construction
  claim may fall through to the integer arm before rejecting.
- Physical: marker `allocation` becomes `NamedArrayAllocationRefV1`
  (`LocalValue` | `FieldResidence{provider_owner, provider_site}`);
  provider ValueId recorded by birth lowering into module-visible
  residence rows; `validate_*` resolves it at module level.
  `NamedAllocationConsumer::Array` binds on the birth-side `NewBox`.
- Caller consumption: `take_source_array_push`, `CoreEffectPlan::
  NamedArrayPush`, `effect_emission` write path unchanged; coverage
  and obligation validators extended for the residence ref only.
- No deletion: the silent-omission set shrinks only where the claim
  now issues; generic escape arms stay for other callers.

## Non-claims

- No production caller switch, no retirement, no Gate-1 green, no
  whole-app pass — the armed-loop freeze is replaced by contract
  coverage for this membership only.
- No `me.f.push` direct receiver, no Text-on-residence, no mixed
  element family, no ordinary-new eligibility change.
- No new authority: reuses `CoreMethodInstanceTargetIssuerV1`,
  `ResolverCoreMethodCallableContractIssuerV1`, `instance_constructors`
  object definitions, and constructor trigger membership.

## Acceptance

1. `apps/boxtorrent-mini` `--emit-mir-json` no longer dies at
   `SourceCallOutsideSelectedFamily` for `seedBlocks` (the terminal
   either closes or moves to the next named family — record which).
2. Focused positive pins: field-resident alias push (var + literal)
   reaches retained write marker + provider-bound `NewBox`; alpha-
   renamed alias; zero/one/multiple iterations; I64 argument through
   an `I64ToCaller` nested call.
3. Focused negative pins: shadow ArrayBox, foreign field owner,
   missing/weak/provider-less residence, reassigned alias, Text arg
   on I64 family, value-position push, residual source row — all
   typed rejections before physical writes; existing Text pins stay
   green.
4. Line-boundary + pointer guards green; red classification recorded;
   README/reference rows updated only where this contract changed.

## Evidence

- D0 card records the verified census (issue/contract/marker/
  consumption seams) and the observed first terminal
  (`SourceCallOutsideSelectedFamily` on `seedBlocks/0`, body items
  0-3).
- Worker `c7ee89b0` census; primary re-verified
  `named_array_requirement.rs`, `core_method_instance_target.rs`,
  `resolver_core_method_callable_contract.rs`,
  `named_array_obligation.rs`, `emission.rs`, `named_array.rs`,
  `constructor_source.rs`, `object_definition.rs`,
  `instance_constructor_semantic.rs`, `issuer.rs:425/464`, and the
  boxtorrent-mini probe.
