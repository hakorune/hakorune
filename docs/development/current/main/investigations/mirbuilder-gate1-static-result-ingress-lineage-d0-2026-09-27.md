# MIRBUILDER-GATE1-STATIC-RESULT-INGRESS-LINEAGE-D0

Status: closed — Decision accepted 2026-09-27
Parent: workstream row H / Gate-1 internal owner series; follows
field-array D0..D2+S0..S2 (all landed). Selected by S2's "next
observed required terminal": the first app terminal on both lanes is
`[freeze:contract][static-result-ingress/foreign-lineage]`.

## Question

Which non-`Cataloged` `RawInvocationRootLineageV1` roots may admit
static-result publication, and how does the caller
`CanonicalSameModuleCallableKeyV1` derive without minting a second
identity or bypassing the publication owner?

## Census (worker + direct probes)

Boundary: `RawInvocationRootLineageV1` producers ->
`classify_source_context_v1` / `take_static_result_publication_ingress_v1`
-> `VerifiedStaticCallResultPublicationOwnerV1::take_for_source` ->
`member_route` / me-policy consumers. Covers all production
lineage+ledger combinations; excludes the sibling script-direct claim
lane internals.

### Owner rows and caller coverage

- `VerifiedWholeSourceStaticCallTargetInventoryV1::observe_all_calls`
  (`src/mir/source_call_target/whole_source_inventory.rs:180-241`)
  iterates `declarations.declarations()` — `rows_by_key` holds only
  `StaticBoxMethod` + `InstanceBoxMethod` keys
  (`declaration_for` returns `None` for `FreeFunction` /
  `BirthConstructor`). Constructor bodies, top-level functions,
  nested methods, and script statements are **never inventoried
  callers**: no `(caller, site)` rows exist for them.
- The publication owner keys every row by
  `(CanonicalSameModuleCallableKeyV1 caller, SourceExprSiteV1)`
  (`static_call_result_publication_owner.rs:117-134`) and
  `finish_empty` enforces full consumption.
- `Main.main/N` and other `Main.*` static children **are** catalog
  declarations (`seal_statements` keeps every `is_static` box method
  in `rows_by_key`; the `name != "Main"` clause only excludes Main
  from `selected_source_rows`). So `Main`-locator callers already
  have owner rows when their bodies contain exact static targets.

### Producer/ledger combinations

- `Cataloged(key)` + ledger: produced by `main_root.rs:276-292`,
  `loan_port.rs:432/671`, `with_cataloged_callable_source_scope`
  (instance methods) — the only admitted combination today.
- `Main(RawSourceLocatorV1{top_level_statement, box_name,
  method_name, symbol, arity})`: produced **only** by
  `pending_helpers.rs:40-48` (`complete_raw_root_static_child_branded`,
  the `compile_raw_published_v1` raw-root lane) — on a port whose
  `callable_ledger` is `None`. `Main`+ledger is production-unreachable
  today; the arm decided below is rule-complete but production-inert.
- `InstanceConstructor(NormalInstanceConstructorSourceKeyV1)` +
  ledger: produced by `normal_instance_constructor_admission.rs:
  493-496` inside `with_constructor_semantic_scope` — the reachable
  producer for every `new <UserBox>` birth body.
- `TopLevel(SelectedTopLevelFunctionKeyV1)` + ledger:
  `normal_callable_semantic_source.rs:240-245` via
  `with_callable_source_scope`.
- `NestedBoxMethod{parent_site, method_key}` + ledger: structurally
  possible when a box decl lowers inside an active callable scope;
  at top level the port is ledger-less.
- `ScriptRoot`: `callable_ledger` never `Some` — never reaches
  `ForeignLineage`; sibling `script_direct_static_claim` family owns
  it.

### Observed boxtorrent terminal attribution

boxtorrent-mini has no top-level functions; instance methods lower
under `Cataloged`; script root has no ledger. The only ledgered
non-`Cataloged` producer present is `InstanceConstructor`:
`new HakoAllocHeap()` inside `BoxTorrentStore.birth` lowers
`HakoAllocHeap`'s generated birth stores, whose field initializers
`new HakoAllocPage(0, LayoutBox.class_size(0),
LayoutBox.class_capacity(0))` evaluate `using`-aliased static calls
under the `InstanceConstructor` root -> `ForeignLineage`. The
observed terminal is `InstanceConstructor`-rooted, not `Main`.

### Caller-key facts

- `Main(locator)` — locator fields are sealed projection output
  (`RawSourceOriginV1::BareAst` / `VerifiedRawRootExpansionV1`).
  `declaration_for(StaticBoxMethod, locator.box_name(),
  locator.method_name(), locator.arity())` resolves through the
  catalog's own index and returns the **sealed** `decl.key()` —
  the same probe discipline the existing arm already applies to
  non-`StaticBoxMethod` `Cataloged` callers. Covers `Main.main` and
  static helpers uniformly (the `Main` lineage is minted for every
  raw-root static child). `locator.symbol()` is never consulted.
- `AppMainCatalogCoSealV1` carries opaque `parser_identity` +
  `catalog_key` + `catalog_brand` only — it cannot verify a
  `RawSourceLocatorV1`, and the catalog probe is sufficient without
  extending it.
- `TopLevel(SelectedTopLevelFunctionKeyV1)` — `free_function(
  declared_name, declared_arity)` derivable (`from_catalog_key`
  precedent exists) but `FreeFunction` is not a catalog row.
- `InstanceConstructor(key)` — `published_birth_key:
  Option<CanonicalSameModuleCallableKeyV1>` is already canonical
  when `Some`; `None` on compat rows.
- `NestedBoxMethod` — `method_key` is a raw `Box<str>` (method name
  only, no owner/kind); nested methods are not catalog decls — no
  verified caller exists.
- `ScriptRoot` — no caller identity by construction.

## D0 Decision

Admission rule: a located lineage may admit static-result
publication iff **(a)** it carries or resolves a **verified**
`CanonicalSameModuleCallableKeyV1` — sealed payload or a sealed
catalog probe — never `locator.symbol()`, raw method-key strings,
or name/arity reconstruction; and **(b)** that caller is an
inventoried caller of the sealed static-call-target inventory.
Admitting a caller the inventory never observed would make the
take answer `NoExactStaticTarget` for sites whose targets exist —
a wrong classification, not a truthful reject.

1. `Cataloged` — admitted (unchanged).
2. `Main(locator)` — **admitted** by catalog probe:
   `declaration_for(StaticBoxMethod, locator.box_name(),
   locator.method_name(), locator.arity())` -> `Cataloged{caller:
   decl.key(), site}`. Probe failure -> `ForeignLineage`;
   `declarations == None` -> `DeclarationCatalogUnavailable` (the
   proof authority is absent — not the same as proven-foreign).
   Production-inert today (its sole producer is ledger-less); the
   arm is rule-complete, not a production claim.
3. `TopLevel` — **not admitted now**: key derivable but no
   inventoried `FreeFunction` callers exist. Admission is gated on
   caller-inventory coverage (separate family).
4. `InstanceConstructor` — **not admitted now**: `published_birth_key`
   is canonical when `Some`, but `BirthConstructor` callers are not
   inventoried. **This is the observed boxtorrent terminal**; the
   real unblock is constructor-caller coverage in the declaration
   catalog + `observe_all_calls` inventory (+ result-catalog
   coverage for `call_result(caller, site)`), a different owner
   family — not a lineage-gate change.
5. `NestedBoxMethod` — **rejected**: no verified caller key exists;
   stays `ForeignLineage` unless a verified caller relation is
   added at construction (parked, not this series).
6. `ScriptRoot` — **rejected permanently**: sibling
   `script_direct_static_claim` family owns script statics.
7. Fail-fast boundary unchanged apart from the new arm:
   `!source_backed` -> `Unavailable`; `UnlocatedCompatibility` +
   backed -> `SourceLocationLost`; `Main` + backed + no catalog ->
   `DeclarationCatalogUnavailable`; `Main` + unresolved probe ->
   `ForeignLineage`; all other non-`Cataloged` roots + backed ->
   `ForeignLineage`; owner/catalog take errors unchanged. No
   fallback: `Unavailable` is never manufactured for a source-backed
   located root.

## Six-line brief

```text
Decision: admit Main(locator) via catalog-probed sealed declaration key;
  TopLevel/InstanceConstructor stay foreign until the caller-inventory
  family covers FreeFunction/BirthConstructor; NestedBoxMethod/ScriptRoot
  stay foreign.
Source authority + canonical issuer: VerifiedSameModuleCallableDeclarationCatalogV1
  (declaration_for probe -> sealed decl.key()); the publication owner
  stays sole row authority.
Non-authority: locator.symbol(), NestedBoxMethod method_key, name/arity
  key reconstruction, physical MIR symbols.
Fail-fast boundary: Main+backed+no-catalog -> DeclarationCatalogUnavailable;
  unresolved probe -> ForeignLineage; source loss -> SourceLocationLost;
  owner take errors unchanged.
Smallest next slice: S0 — Main arm + classify-boundary pins.
Non-claims: boxtorrent unblock (InstanceConstructor coverage is a separate
  family); nested/script admission; Gate-1 completion; runtime execution.
```

## Bounded S0 (emitted)

`MIRBUILDER-GATE1-STATIC-RESULT-INGRESS-LINEAGE-S0` — one arm +
pins only:

- `classify_source_context_v1`: `Located{root: Main(locator)}` +
  backed -> `DeclarationCatalogUnavailable` when `declarations` is
  `None`; `declaration_for(StaticBoxMethod, box_name, method_name,
  arity)` -> `Cataloged{caller: decl.key().clone(), site}`; probe
  `None` -> `ForeignLineage`. All other roots unchanged.
- Pins in `static_result_publication_ingress.rs` tests: located
  `Main` + matching `Main.main`/`Main.<helper>` declaration ->
  `Cataloged` with the sealed key; no catalog ->
  `DeclarationCatalogUnavailable`; undeclared method ->
  `ForeignLineage`; `!source_backed` -> `Unavailable`.
- `src/mir/builder/README.md` (or owning reference): record the
  admitted-lineage table + the coverage dependency for
  `InstanceConstructor`/`TopLevel`.
- No producer changes, no inventory/catalog widening, no new
  callers, no switch.

## Next selected row after S0 lands

`MIRBUILDER-GATE1-STATIC-RESULT-CALLER-COVERAGE-D0` — design
whether/how `BirthConstructor` and `FreeFunction` callers enter the
declaration catalog, the `observe_all_calls` caller inventory, and
the result catalog so `InstanceConstructor`/`TopLevel` lineage
admission becomes truthful; that is the observed app terminal.

## Non-claims

- Boxtorrent unblock: `InstanceConstructor` coverage is a separate
  family; this slice only closes the lineage question.
- `Main` arm production effect: unreachable while its only producer
  stays ledger-less — pins prove the classification contract only.
- `NestedBoxMethod`/`ScriptRoot` admission: unchanged, excluded.
- Catalog/inventory/result-catalog widening: not this slice.
- Gate-1 or overall MirBuilder completion: not claimed.
