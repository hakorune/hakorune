# MIRBUILDER-GATE1-CALLABLE-LOOP-ROUTE-FRONT-S0

Status: landed — see Pinned evidence
Date: 2026-09-27
Emission: `mirbuilder-gate1-callable-loop-route-front-d0-2026-09-27.md`
Decision — admit `ArrayBox.get/1` (`pure_read`, LoopBody placement) as
a plain CoreMethod contract through the named-array ArrayBox issuer
arm, gated by field-residence receiver evidence; no write-requirement
product.
Selection proof: `MIRBUILDER-GATE1-CALLABLE-LOOP-ROUTE-FRONT-S0`
-> workstream row H.
MirBuilder goal row 1 (sole Facts issuance — `method_calls()` /
`initializer_relations()` stay the only item/receiver inventories),
row 3 (sole admission boundary — the probe's complete-or-reject
coverage, unchanged), row 4 (verified admission — residence +
integer-source evidence, not selector/name guessing).

## Exact boundary

- `src/mir/resolved_semantics/core_method_instance_target.rs`:
  `CoreMethodHomeSchemaV1::ArrayDynamicRead` arm — receiver
  `"ArrayBox"`, `CoreMethodEffectV1::PureRead`,
  `CoreMethodHomeAbiProfileV1::NamedArrayReadV1`; `(ArrayGet,1)`
  relation mapping — `NamedArrayReceiver`, `[I64Parameter]`, new
  `CoreMethodHomeResultRelationV1::DynamicToCaller`
  (`CoreMethodResultKindV1::Dynamic` manifest check);
  `CoreMethodInstanceTargetIssuerV1::array_dynamic_read` constructor.
- `src/mir/resolved_semantics/resolver_core_method_callable_contract.rs`:
  verify mirror — `allowed_target_placements` admits
  `(ArrayGet,1) -> [Body]`; result-relation mirror admits
  `(ArrayGet,1,DynamicToCaller)`. No other placement/op admitted.
- `src/mir/source_call_target/named_array_method.rs`: extend the
  ArrayBox arm — `ArrayPush` path unchanged; new bounded `ArrayGet`
  branch: `detect_field_residence_claim` must yield a claim with
  `NamedArrayFieldResidenceObjectV1::Receiver` (ForeignDeclared
  deferred); `constructors.field_declaration` declared type must be
  `None` or `"ArrayBox"`, non-weak; `resolve_birth_provider` must find
  the `me.<field> = new ArrayBox()` provider site in the owning box's
  birth ledger; `verify_field_residence_relations` applies unchanged
  (ReassignedReceiver + ValueDemand + single integer-source
  argument); LoopBody placement via the existing `nearest_loop`
  candidate filter; then issue a **plain** contract via
  `ResolverCoreMethodCallableContractIssuerV1::issue` with the
  `array_dynamic_read` target — no `NamedArrayRequirementV1` product,
  no `field_providers` row.
- `src/mir/builder/normal_callable_semantic_lowering_state/source_call_publication.rs`
  (`take_source_core_method_call`): `DynamicToCaller` ->
  MIR type arm for the contract result (joins the existing
  `I64ToCaller`/`TextToCaller`/`NoValue` mapping; the unconditional
  `named_array_requirement().is_none()` arm is already the consumer).
- No probe/item-disposition change: `CoreMethod` coverage rows flow
  through `CallableLoopSourceItemDispositionV1::CoreMethod` and the
  existing `core_methods` relation arm unchanged.
- `tools/checks/…` focused guard: pin the `ArrayGet`/`DynamicToCaller`
  vocabulary sites mirroring the existing CoreMethod vocabulary pins.

## Out of scope

- `MapBox.get/1` (`MapGet`) — no residence/declared-type evidence
  family selected for map receivers.
- User-box instance calls (`store.readData/release/put`,
  `manifest.addChunk/seal`) — parked DeclaredInstance lineage, a
  separate family.
- `StringIndexOf` (`alphabet.indexOf`) — manifest row exists, no
  placement arm selected in this slice.
- `Construction`-alias receivers (`local a = new ArrayBox(); a.get`),
  `ForeignDeclared` residence objects, `me.chunk_ids.get` unaliased,
  non-`Lexical(Local)` receivers, Condition placement.
- `DynamicMember` catalog arm and its consumers — untouched.
- Any write-requirement (`NamedArrayRequirementV1`) minting for reads.
- Any runtime/VM/legacy-path change; no probe or route selection
  change.

## Negative/deletion proof (acceptance shape)

- `local ids = me.chunk_ids` + loop-body `ids.get(i)` ->
  `CoreMethod` row with `ArrayGet` target, `PureRead`,
  `NamedArrayReceiver` + `I64Parameter` + `DynamicToCaller` ->
  emitted via the unconditional consumer arm
  (`read_variable(receiver_site)` + slot-load call).
- `m.get(k)` on a `me`-field `MapBox` residence -> no contract ->
  `SourceCallOutsideSelectedFamily` (selector-only minting is not
  added).
- `local a = <non-FieldAccess>` + `a.get(0)`; `me.chunk_ids.get(i)`
  unaliased; `ids.get` in a loop condition -> same outside-family
  terminal.
- `ids` rebound anywhere in the function ->
  `ReassignedReceiver` -> no contract -> outside family.
- No silent `Ok(None)` added to any production semantic path; the
  probe stays complete-or-reject.

## Pinned evidence (landed)

- Focused tests
  (`cargo test --release core_method_source_tests`, 6/6):
  `field_resident_array_get_issues_plain_core_method_contract`
  (ArrayGet + `DynamicToCaller` + Body placement + no
  requirement), `field_resident_get_rejects_receiver_rebind_and_
  non_integer_index` (`ReassignedReceiver`,
  `IntegerSourceMissing`), `non_resident_get_stays_unarmed`
  (construction-alias, unaliased `me` field, Condition placement).
  Regression: `named_array` 31/31, `source_call` 87,
  `source_call_target` 83, `core_method` 69, `callable_contract`
  18, `instance_constructor` 37 — all green.
- One boundary refinement vs the emitted card:
  `verify_field_residence_relations` gained a
  `NamedArrayResultDemandV1` parameter — the write arm keeps
  `NoValue` (statement-position check); the read arm passes
  `ReadResult` so a demanded `pure_read` result is not rejected.
  No other relation check changed.
- boxtorrent re-measurement (`--dump-mir`, fresh binary):
  `BoxTorrentManifest.chunkListText/0` advances past
  `ids.get(i)`; the only uncovered site is now
  `materialize/1` `[Body(4), LoopBody(1), Initializer(0)]` —
  `local data = store.readData(cid)`, a user-instance call that
  stays `SourceCallOutsideSelectedFamily` by design (parked
  DeclaredInstance family; recorded, not claimed).
- Guards: `mirbuilder_qualified_route_scope_guard.sh` pins
  `ArrayDynamicRead` / `array_dynamic_read` / `DynamicToCaller` /
  `issue_array_get_read_contract` / `NamedArrayResultDemandV1::
  ReadResult` / `(ArrayGet,1)->[Body]` / `MirType::Unknown` /
  the three test names; `current_state_pointer_guard.sh` ok.

## Next selected row (after landing)

The user-instance callable-loop family
(`store.readData/release/put`, `manifest.addChunk/seal`) is the
observed next terminal but remains ParkedSealed
(`MIR-CALL-ME-DECLARED-INSTANCE-SELECTED-C-ADMISSION-D0` —
published target + backend coverage missing; resume requires
explicit selection). The queued non-parked family in the same
route-front lane is the `StringIndexOf` placement arm
(`alphabet.indexOf(ch)` in `ContentHash.digest` LoopBody —
manifest row exists, placement arm absent), selected as
`MIRBUILDER-GATE1-CALLABLE-LOOP-STRING-INDEXOF-D0`.
