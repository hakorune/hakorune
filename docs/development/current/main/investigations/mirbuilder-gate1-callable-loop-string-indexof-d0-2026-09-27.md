# MIRBUILDER-GATE1-CALLABLE-LOOP-STRING-INDEXOF-D0

Status: closed — Decision accepted 2026-09-27 (bounded S0 slice emitted)
Parent: workstream row H / callable-loop route-front lane; follows
`MIRBUILDER-GATE1-CALLABLE-LOOP-ROUTE-FRONT-S0` (landed — field-resident
`ArrayBox.get/1` plain contract; `chunkListText` cleared).

Selected by the family-local action scheduler: the observed next
boxtorrent terminal `materialize/1` (`store.readData(cid)`) belongs to
the ParkedSealed DeclaredInstance family
(`MIR-CALL-ME-DECLARED-INSTANCE-SELECTED-C-ADMISSION-D0`), which must not
resume without explicit selection. The queued non-parked family in the
same route-front lane is `StringIndexOf` — `alphabet.indexOf(ch)` in
`ContentHash.digest` LoopBody (apps/boxtorrent-mini/main.hako:18).

## Observed terminal and family boundary

Same emission path as the route-front D0:
`CallableLoopSourceTargetProbeV1::into_selected_relation` —
`SourceCallOutsideSelectedFamily` — the designed
unsupported-family terminal, unchanged. `digest`'s loop items are
`text.substring(i, i+1)` (already armed StringSubstring/2) and
`alphabet.indexOf(ch)` (uncovered — `StringIndexOf` has no placement
arm today). This D0 covers only the indexOf/1 item; the loop's
remaining coverage is already sealed.

## Manifest and uniqueness census

`("StringBox","indexOf",*)` — generated manifest row
(`core_method_contract_rows.rs:178-191`, source
`core_method_contract_box.hako` `_string_indexof_row`):

- `op = StringIndexOf`, `result_kind = I64Value`,
  `effect = PureRead`, `lowering_tier = WarmDirectAbi`,
  `aliases = ["find"]`, `arities = [1, 2]` — `issue_core_method_
  manifest_row_ref_v2` specializes by exact arity.
- **Catalog-unique**: no other `receiver_box` row carries `indexOf`
  in the CoreMethodContractBox manifest (unlike `get/1` —
  ArrayGet vs MapGet — which forced receiver evidence for op
  selection).

BUT uniqueness in the *CoreMethodContractBox manifest* is not the
whole story — the runtime router catalog accepts
`("ArrayBox","indexOf",1)` (`router/catalog.rs`) and the legacy type
inference treats `ArrayBox.indexOf/1` as i64-returning
(`types/annotation.rs`). `local a = new ArrayBox()` (or any
non-string local) + `a.indexOf(x)` in a loop body would falsely mint
`StringIndexOf` under an unconditional arm — selector-only minting,
explicitly prohibited by the design stop's own wording.

## Collateral census (why unconditional arm is rejected)

`parser_scan_loop_box.hako:10-11` (`skip_while`): `pred_chars.
indexOf(ch)` — `Lexical(Local)` **parameter** receiver in a loop
body, today on the Dynamic-member lane (fixtures pin
`indexOf` Dynamic rows: `normal_callable_semantic_package/
tests.rs:523-627`, `source_call_target/README.md`). An
unconditional `(StringIndexOf,1)->Body` arm mints a CoreMethod
contract there too, silently re-routing a parked family.

`find` alias: the verifier accepts canonical-or-alias spelling —
arming `indexOf` arms `x.find(y)` on the same receiver class.

`StringIndexOf/2` (`indexOf(needle, start)`): same union row; the
`(op, arity)` placement mirror leaves arity-2 unarmed naturally.

`text.length()` (`digest` Body, outside the loop): never a probe
item — unaffected.

## Decision

Admit `("StringBox","indexOf",1)` -> `StringIndexOf` in LoopBody
placement **only under receiver-text evidence**, mirroring the
ArrayGet residence-gate precedent:

- Receiver must be `Lexical(ResolvedLexicalRefV1::Local)` whose
  binding has exactly one initializer relation whose initializer
  site is **text-producing**: `ledger.literal_source(site) ==
  ResolvedLiteralSourceV1::String(_)` (covers `local alphabet =
  "..."`), or a `TextToCaller` CoreMethod contract minted at that
  exact site in the same issuance (covers `local s = text.
  substring(..)` receivers). No initializer (parameters),
  multiple initializers, a non-text initializer, or a rebound
  receiver -> the call stays unarmed -> existing
  `SourceCallOutsideSelectedFamily` terminal.
- The argument `needle` is proven text-producing by the same
  bounded text-source check over `variable_ref -> initializer`
  (port of `integer_source_at` in `named_array_residence.rs`):
  string literal, or local binding whose single initializer is a
  string literal or a `TextToCaller` contract site. `ch` (a
  substring/2 result local) satisfies it. Non-text arguments ->
  unarmed, not an error.
- Result relation `I64ToCaller` (existing) -> `MirType::Integer`
  in the existing consumer arm — no consumer change.
- Plain contract: `named_array_requirement` `None` (the
  `(None, StringBoxText)` matcher arm already permits it). No
  write-requirement product, no provider table row.
- Parameter relation: the needle is *borrowed* text, not retained
  — `TextRetainedByReceiver` is the push-specific relation and is
  semantically wrong here; a new
  `CoreMethodHomeParameterRelationV1::TextParameter` variant is
  honest vocabulary (issuer + verifier mirror agree).

Fail-fast boundary: no silent `Ok(None)` relaxation added to the
probe; uncovered sites keep the designed terminal. No
DynamicMember widening, no receiver class widening.

## Bounded S0 emitted

`MIRBUILDER-GATE1-CALLABLE-LOOP-STRING-INDEXOF-S0` — exactly:

1. `core_method.rs::allowed_placements` —
   `(StringIndexOf,1) -> &[Body]` routed through a bounded
   text-evidence gate (new private predicate beside
   `issue_source_bound_core_method_calls_v1`, not the named-array
   arm — `alphabet` is a literal receiver, not a field residence).
2. `core_method_instance_target.rs::issue` — `(StringIndexOf,1)`
   arm: `StringBoxReceiver` + `[TextParameter]` + `I64ToCaller`;
   new `TextParameter` variant.
3. `resolver_core_method_callable_contract.rs` — placement mirror
   `(StringIndexOf,1)->[Body]`, result tuple `(StringIndexOf,1,
   I64ToCaller)`, `expected_parameters` `StringIndexOf ->
   &[TextParameter]`.
4. Text-source predicate: bounded `text_source_at` over
   literal-source + single-initializer local bindings +
   `TextToCaller` contract at the initializer site; receiver must
   also have no `BindingRebind` assignment targets (mirroring the
   ReassignedReceiver gate).
5. `mirbuilder_qualified_route_scope_guard.sh` — pin the new
   vocabulary and test names mirroring the ArrayGet block.
6. Focused tests: positive (`local alphabet = "..."` + loop-body
   `alphabet.indexOf(ch)` where `ch` is a substring-result local ->
   one `StringIndexOf`/`I64ToCaller`/Body contract row); negatives
   (param receiver `s.indexOf` unarmed, `local a = new ArrayBox()`
   + `a.indexOf` unarmed, non-text needle unarmed, rebound
   receiver unarmed, Condition placement unarmed).

## Non-claims

- `StringIndexOf/2` (`indexOf(needle, start)`), `lastIndexOf`,
  `find` on non-text-evidence receivers — unarmed.
- `pred_chars.indexOf` (`parser_scan_loop_box`) stays unarmed —
  the Dynamic-member lane keeps its family.
- `arr.indexOf` on any non-text receiver — unarmed; the runtime
  router's `ArrayBox.indexOf` row never leaks into this arm.
- No change to `text.substring`/`text.length` behavior; no probe,
  route-selection, or physical-path change; `MirType::Integer`
  comes from the existing `I64ToCaller` arm.
- Does not clear `materialize`/`releaseFrom`/`ingest` — the
  ParkedSealed DeclaredInstance family owns those sites.
- Not Gate-1 completion; not MirBuilder completion; no legacy
  retirement.
