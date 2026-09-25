# MIRBUILDER-EXE-ACCEPTANCE-UNTYPED-STORAGE-AUTHORITY-D0

Status: open__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D1 (closed,
  NoSafeSlice -> this card)
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
