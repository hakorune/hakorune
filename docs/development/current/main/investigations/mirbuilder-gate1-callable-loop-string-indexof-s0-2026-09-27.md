# MIRBUILDER-GATE1-CALLABLE-LOOP-STRING-INDEXOF-S0

Status: landed
Date: 2026-09-27
Emission: `mirbuilder-gate1-callable-loop-string-indexof-d0-2026-09-27.md`
Decision — admit `StringBox.indexOf/1` (`StringIndexOf`, I64Value,
pure_read, LoopBody placement) as a plain CoreMethod contract through
the existing StringBox issuer arm, gated by **receiver-text evidence**
(string-literal or `TextToCaller`-contract initializer, no rebind) and
a text-producing needle argument; no selector-only minting, no
write-requirement product.
Selection proof: `MIRBUILDER-GATE1-CALLABLE-LOOP-STRING-INDEXOF-S0`
-> workstream row H.
MirBuilder goal row 1 (sole Facts issuance — `method_calls()` /
`initializer_relations()` / `literal_source()` stay the only
inventories), row 3 (sole admission boundary — the probe's
complete-or-reject coverage, unchanged), row 4 (verified admission —
text evidence, not selector/name guessing).

## Exact boundary

- `src/mir/resolved_semantics/core_method_instance_target.rs`:
  new `CoreMethodHomeParameterRelationV1::TextParameter` variant
  (borrowed text argument — `TextRetainedByReceiver` stays
  push-specific); `(StringIndexOf,1)` arm in `issue` ->
  `StringBoxReceiver` + `[TextParameter]` + `I64ToCaller`
  (requires manifest `I64Value`, `ResultMismatch` otherwise).
- `src/mir/source_call_target/core_method.rs`:
  `allowed_placements` `(StringIndexOf,1) -> &[Body]`; a bounded
  private predicate `index_of_has_text_evidence` /
  `text_source_at` consulted only for `StringIndexOf`
  before target issue:
  - receiver `Lexical(Local)` -> exactly one `initializer_relation`
    -> initializer site text-producing
    (`literal_source == String` OR a minted `TextToCaller` contract
    row at that site); plus no `BindingRebind` assignment target on
    the receiver binding;
  - argument site: `literal_source == String`, or `variable_ref ->
    Local` with the same single-initializer text proof (bounded
    depth like `integer_source_at`);
  - failing evidence -> `continue` (unarmed), never Err.
- `src/mir/resolved_semantics/resolver_core_method_callable_contract.rs`:
  `allowed_target_placements` `(StringIndexOf,1) -> [Body]`;
  result tuple `(StringIndexOf,1,I64ToCaller)`;
  `expected_parameters` `StringIndexOf -> &[TextParameter]`.
  The `(None, StringBoxText)` plain-contract matcher already
  admits the schema — no named-array product.
- `source_call_publication.rs`: `I64ToCaller -> MirType::Integer`
  arm already exists — no consumer change.
- `tools/checks/mirbuilder_qualified_route_scope_guard.sh`: pin
  `StringIndexOf`, `TextParameter`, `(StringIndexOf,1)->[Body]`,
  and the new test names.

## Out of scope

- `StringIndexOf/2`, `lastIndexOf`, `find` on non-text receivers.
- Parameter/other receivers without a text-producing initializer
  (`pred_chars.indexOf` stays on the Dynamic-member lane).
- Non-text receiver evidence families (`new ArrayBox()` locals,
  `me` fields, qualified/other receivers).
- `length`/`substring` behavior changes (their existing arms are
  untouched; the runtime-router `ArrayBox.indexOf` row never
  leaks in).
- Any probe/route/physical-path change; no write-requirement
  product; no `me.<field>` receiver residence for strings.

## Negative/deletion proof (acceptance shape)

- `local alphabet = "..."` + `local ch = text.substring(i,i+1)` +
  loop-body `local idx = alphabet.indexOf(ch)` -> one contract row:
  `StringIndexOf`, `StringBoxReceiver`, `[TextParameter]`,
  `I64ToCaller`, Body placement, `named_array_requirement` None.
- `local a = new ArrayBox()` + `a.indexOf(x)` -> unarmed (no row).
- parameter receiver `s.indexOf(ch)` -> unarmed (no initializer).
- `local t = 1` + `alphabet.indexOf(t)` -> unarmed (non-text needle).
- `alphabet = "other"` anywhere -> unarmed (rebound receiver).
- `x.indexOf(ch)` in loop Condition -> unarmed (placement).
- `x.indexOf(ch, 0)` arity-2 -> unarmed (arity key).

## Pinned evidence (landed)

- `core_method_source_tests` 11/11 green:
  `literal_receiver_index_of_arms_body_with_text_parameter`
  (op/placement/`[TextParameter]`/`I64ToCaller` pinned),
  `substring_contract_supplies_index_of_needle_text`
  (TextToCaller row supplies the needle proof),
  `find_alias_follows_the_same_text_evidence_gate`,
  `index_of_stays_unarmed_without_text_evidence`
  (ArrayBox/integer receiver, non-text needle, rebound receiver,
  arity-2, `lastIndexOf`, parameter receiver `pred_chars`),
  `index_of_condition_placement_stays_rejected` (Ok, zero rows).
- Regressions: core_method 74, source_call_target 83, source_call 87,
  callable_contract 18, named_array 31 — all green.
- Production: a `ContentHash.digest`-shaped program
  (`local alphabet` literal + `text.substring` needle + loop-body
  `alphabet.indexOf(ch)`) compiles end-to-end; emitted MIR carries
  `indexOf` as a `RuntimeDataBox` MethodCall — the same
  WarmDirectAbi runtime-dispatch shape `length`/`substring` emit.
- boxtorrent re-measurement: the sole remaining stop is
  `materialize/1` `store.readData(cid)` — the parked
  DeclaredInstance terminal (`route-not-front-selected`,
  `SourceCallOutsideSelectedFamily`). Recorded, not claimed.
- Guards: `mirbuilder_qualified_route_scope_guard.sh` green
  (StringIndexOf pins added), pointer guard green.
