# MIRBUILDER-UNTYPED-OBJECT-STORAGE-D0

Status: closed__2026-09-25__decision-accepted
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D1 (closed,
  NoSafeSlice -> this card). Materializes the SSOT-named owner
  `MIRBUILDER-UNTYPED-OBJECT-STORAGE-D0` (final-pipeline-ssot
  acceptance recheck, ParkedSealed until source-backed storage is
  migrated to an explicit supported type or issued with a tagged
  dynamic/opaque ABI)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table;
  D1 audit (worker 0b3a5cac) — TwoOwners split + untyped-storage
  family is the highest-weight remaining class.

## Slice

Design only. Fix the authority question for inferred/untyped object
storage on the ordinary-new lane: which issuer owns
destruction/Home-end availability and field-read result type for
fields whose `declared_type_name` is absent (init-only fields fed by
birth parameters). Emit one accepted Decision + one bounded S-card,
or a documented ParkedSealed/NoSafeSlice with observable reopen
trigger.

## The edge (from D1 audit)

- `typed_object_birth_param_min`: `Page { init { page_id, capacity } }`
  fields project `declared_type_name = None` ->
  `object_definition.rs:107-132` issues
  `Destruction::Unavailable(FieldType)` (declaration-shape only,
  deliberately does not consult inference) -> `local_entry.rs:82-91`
  `end_available() == false` -> `root_home.rs:167-184` exit
  `Unavailable` -> `root_call_entry.rs:185`
  `root-call-entry-missing`.
- `typed_object_untyped_field_min`: `holder.count` untyped read ->
  `FieldGet{ declared_type: None }` -> `post_success.rs:84-92`
  publishes no `value_types` entry -> `return_type_strategy.rs`
  `resolve_known_return_definition_type` has no FieldGet arm ->
  debug panic `ValueId(15)` (release path continues to the parked
  `Invoke` transport terminal).
- `binary_trees` (downstream): after its own claim-census/`null`-arg
  gaps are designed, tree field ends hit the same family.

## Questions to answer

1. Is inferred storage in-scope for the selected EXE lane at all, or
   is `declared-typed` storage the contract? Check the untyped-storage
   parking records (`NoSafeSlice`/`ParkedSealed` history) before
   designing an issuer.
2. If in-scope: which existing receipt owns field-type observation
   (Facts) — must stay AST-free and must not mint Recipe keys. Where
   does the `usize`/`i64` birth-param inference live, and is it a
   semantic authority or a physical guess?
3. Masked edge: `sum()` has no declared return — the
   root-instance-call disposition needs `results.row == I64`. Is a
   result-contract issuer also missing for inferred returns?
4. Does fixing destruction authority alone produce a passing entry,
   or do all three (destruction, result type, field-read type) have
   to land together? Pick the minimal honest unit.

## Boundary

- Includes: `object_definition.rs`, `local_entry.rs`, `root_home.rs`,
  `root_call_entry.rs`, `fields/post_success.rs`,
  `return_type_strategy.rs`, `ordinary_new_arguments.rs` (null kind),
  claim census (`ordinary_new_coseal_*`) — audit and authority naming.
- Excludes: implementation; usize parameter contract; NamedArray;
  String-result ABI; loop Facts; newbox toolchain.

## Exit

- [ ] In-scope/out-of-scope decision for inferred storage recorded
      with authority.
- [ ] One accepted Decision + bounded S-card, or ParkedSealed with
      reopen trigger.

## Decision (accepted 2026-09-25)

```text
Decision: split the family by where untypedness is essential.
  birth_param_min migrates to declared storage (SSOT-sanctioned
  branch: source-backed storage migrated to an explicit supported
  type) and moves its smoke caller to the --emit-exe lifecycle
  route (MIRBUILDER-INVOKE-LIFECYCLE-PHYSICAL-V2-CUTOVER-I0
  precedent — the selfhost --mir route is retired for lifecycle
  cases and must keep rejecting Invoke). untyped_field_min stays
  red as the untyped/dynamic-storage sentinel.
Source authority + canonical issuer:
  declared field types (`page_id: i64`/`capacity: i64`) +
  declared result `sum(): i64` are the existing authority;
  PlainI64NoHook destruction + root-call entry + lifecycle V4
  physical emit are existing owners — no new Facts/Recipe.
Non-authority: inference-driven storage (phase-293x MIR-observation
  route is retired), generic MIR JSON Invoke transport (intentional
  rejection), `.hako` workaround scope (sanctioned by the SSOT
  branch, not an evasion — the selfhost corpus has zero `init{}`
  uses; `init{}` is documented legacy-compat).
Fail-fast boundary: storage FieldType stop, root-call-entry-missing,
  unsupported_newbox_type, no_lowering_variant all remain named.
Smallest next slice:
  MIRBUILDER-UNTYPED-OBJECT-STORAGE-S0 — birth_param_min: app
  migration to declared fields + declared `sum(): i64` return +
  smoke script moved to the --emit-exe lifecycle caller. Proven:
  typed clone at /tmp compiles via lifecycle V4 and exits 30.
Non-claims: does not fix untyped_field_min, does not admit handle-
  typed fields into newbox, does not admit usize/NamedArray/loop
  facts, does not change Invoke transport policy.
```

## Evidence (main-investigator probes, release+quick binaries)

- `lang/**/*.hako` uses zero `init {}` forms — the selfhost corpus
  already migrated to declared `field: Type`; `init{}` is legacy
  compat (EBNF :984-996).
- Typed-clone probe of birth_param (`page_id/capacity: i64`,
  `sum(): i64`): `--emit-exe` -> lifecycle V4 ok -> EXE exits **30**
  (matches the pinned expectation). On the retired `--mir` route the
  same source dies at `unsupported terminator Invoke` — confirming
  the smoke route must move, not the transport.
- untyped_field typed clones still fail: `new Holder()`/`new Items()`/
  `new ArrayBox()` -> `unsupported_newbox_type` (boxes with
  handle-typed fields are not admitted at newbox), and minimal
  i64-field `new Holder()` reaches `no_lowering_variant` — the pure
  lane's newbox coverage gap (same class as newbox_min). Three
  stacked gaps = design family, not a slice.
- untyped_field_min therefore stays the untyped-storage sentinel:
  `NoSafeSlice` — reopen when a tagged dynamic/opaque slot-ABI
  D-card or handle-field newbox admission owner is selected.

## Exit

- [x] In-scope decision recorded: declared-storage migration for
      birth_param; untyped_field stays sealed.
- [x] One bounded S-card emitted:
      `MIRBUILDER-UNTYPED-OBJECT-STORAGE-S0`.
