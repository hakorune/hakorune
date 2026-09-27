# MIRBUILDER-GATE1-STATIC-RESULT-CALLER-COVERAGE-D0

Status: closed — Decision accepted 2026-09-27
Parent: workstream row H / Gate-1 internal owner series; follows
`MIRBUILDER-GATE1-STATIC-RESULT-INGRESS-LINEAGE-D0`+`S0` (landed —
`Main` arm). Selected by the lineage D0: the observed boxtorrent
terminal is `InstanceConstructor`-rooted, and the lineage D0 ruled
that `InstanceConstructor`/`TopLevel` may only be admitted once the
sealed caller inventory covers their callers.

## Question

Can `BirthConstructor` and `FreeFunction` callers enter the
`rows_by_key` declaration set so `observe_all_calls` mints
`(caller, site)` rows for their bodies — and which order makes
mint ⇄ consume atomic without violating `finish_empty` or
misreporting `NoExactStaticTarget`?

## Census (worker + direct verification)

### Row sources and gaps

- `rows_by_key` is minted by two issuers:
  - `seal_statements` (`callable_declaration_catalog/catalog.rs:
    92-261`): iterates `BoxDeclaration.methods` only. Top-level
    `FunctionDeclaration` statements record a `SelectedTopLevel`
    source row and `continue` (:116-124) — no `rows_by_key` row.
    `constructors` is never iterated.
  - `issue_source_backed_same_module_callable_catalog_v1`
    (`source_backed.rs:114-343`): `FinalCallableDeclarationModeV1::
    TopLevel` already mints a **`free_function` row with body**
    (:148-184). The callable loan has **no constructor mode** —
    `BirthConstructor` rows cannot appear from it today.
- `VerifiedSourceMethodCallSiteV1::verify` requires
  `catalog.declaration(caller)` — `rows_by_key` membership is the
  structural gate (`source_method_call_site.rs:35-39`); the body it
  borrows must be the same parser-normalized body the lowering port
  lowers (constructor semantic rows verify pointer-identity against
  `BoxDeclaration.constructors`, `instance_constructor_semantic.rs:
  390-399`).
- `SameModuleCallableSourceReceiverPolicyV1::from_namespace`
  already maps `BirthConstructor`→`DeclaredInstance` and
  `FreeFunction`→`Absent` (`callable_receiver_policy.rs:18-25`) —
  the observer vocabulary is ready.
- `published_birth_key` is `Some(birth_constructor(box, arity))`
  exactly when `kind == Birth` (`instance_constructor_semantic.rs:
  601-621` → `admission.rs:134-141`); `init`/`pack` have `None` —
  no canonical identity exists for them, they must stay foreign.
- `SelectedTopLevelFunctionKeyV1{declared_name, declared_arity}`
  projects to `free_function(name, arity)` — the same key the
  source-backed issuer already minted (`normal_top_level_function_
  admission.rs:40-63` precedent).
- Result catalog: `prove_function` runs only over
  `static_declarations()`; `call_result`/`disposition` stay
  static-only, but `validate_target_pairing` accepts any caller in
  `all_keys` and owner `issue()` projects namespace-agnostically
  (`solver.rs:33-114`, owner `issue` :157-262). Nothing downstream
  of `classify_source_context_v1` is caller-namespace-dependent.
- Mint ⇄ consume: the owner is installed only on the
  SelectedNormal/app installed lane; on that lane every cataloged
  body lowers under a callable ledger — constructors once per
  source row via demand ticket (`README.md:367-387`), top-level
  functions via `with_selected_source_scope`. `init`/`pack` and
  compat lanes never mint.
- **Latent freeze today**: `FreeFunction` rows already mint on the
  production catalog — a top-level function containing
  `Owner.method()` freezes at `foreign-lineage` (or
  `UnconsumedSelected` at drain) right now.
- Alternative authorities: named-array field providers already
  carry `provider_caller = birth.published_birth_key()` +
  `provider_site` (`named_array_method.rs:222-256`) — `BirthConstructor`
  caller identity is already exercised end-to-end in an adjacent
  relation; no authority mints general target/result rows for these
  callers.

## D0 Decision

Admission rule (restated from lineage D0): a located lineage admits
iff it yields a verified `CanonicalSameModuleCallableKeyV1` **and**
that caller holds a `rows_by_key` declaration (so
`observe_all_calls` inventoried its body). Both gates are probes
against the installed catalog — never reconstruction.

1. `FreeFunction` coverage is **already minted on the production
   catalog**; only two gaps remain: `seal_statements` parity (compat
   /test catalogs mint no `free_function` rows — inconsistent with
   the source-backed issuer) and the `TopLevel` lineage arm. Land
   both in bounded **S0**: mint `free_function(name, arity)` rows
   for top-level `FunctionDeclaration` statements in
   `seal_statements` (same `validate_parameters` +
   `DuplicateCanonicalKey` discipline as the source-backed issuer)
   and add the `TopLevel` arm:
   `catalog.declaration(&free_function(name, arity))` resolves AND
   `declaration_for(StaticBoxMethod, owner, method, argc)` resolves
   -> `Cataloged{caller: decl.key(), site}`; caller-membership
   failure -> `ForeignLineage`; target-probe failure ->
   `Unavailable` (same rule as the `Cataloged` arm); absent catalog
   -> `DeclarationCatalogUnavailable`; `!source_backed` ->
   `Unavailable`.
2. `BirthConstructor` coverage needs a **co-seal**: constructor
   `FunctionDeclaration` bodies (`BoxDeclaration.constructors` /
   the constructor syntax loan) become `birth_constructor(owner,
   arity)` `rows_by_key` rows in **both** issuers (source-backed
   issuer joins the constructor loan; `seal_statements` iterates
   `constructors` for parity), landing **atomically** with the
   `InstanceConstructor` arm:
   `key.published_birth_key()` `Some` + `catalog.declaration` +
   `declaration_for(StaticBoxMethod, …)` -> `Cataloged`; `None`
   published key (init/pack) or unresolved membership ->
   `ForeignLineage`. Mint-without-arm leaves rows unconsumed at
   `finish_empty`; arm-without-rows misreports
   `NoExactStaticTarget` — hence atomic. This is the observed
   boxtorrent terminal owner; bounded as **S1**.
3. `NestedBoxMethod`/`ScriptRoot` stay foreign — no verified caller
   (bare method-name string) / sibling family.
4. `init`/`pack` constructors stay foreign permanently —
   `published_birth_key == None` is correct, not a gap.
5. Constructor body observability needs no new work: field-
   initializer generated stores live inside the same normalized
   `constructors` bodies the catalog will borrow, sites are
   `function_body`-rooted `SourcePathSegmentV1` paths, and the
   receiver policy already covers both namespaces.
6. Nothing downstream changes: owner, solver, verifier,
   `member_route`, physical bridge, and drain are already
   caller-namespace-agnostic.

## Six-line brief

```text
Decision: cover FreeFunction via seal parity + TopLevel arm (S0);
  cover BirthConstructor via constructor-row co-seal in both issuers
  + InstanceConstructor arm, atomic (S1); init/pack and nested/script
  roots stay foreign.
Source authority + canonical issuer: rows_by_key declaration rows
  (both catalog issuers) mint caller identity; the publication owner
  stays sole row authority.
Non-authority: NestedBoxMethod method_key, symbol/name re-derivation,
  AST re-walking for sites, physical MIR symbols.
Fail-fast boundary: caller-membership failure -> ForeignLineage;
  absent catalog -> DeclarationCatalogUnavailable; target-probe
  failure -> Unavailable (mirrors the Cataloged arm).
Smallest next slice: S0 — seal_statements free_function parity +
  TopLevel arm + pins.
Non-claims: BirthConstructor co-seal is S1 (atomic with its arm);
  no solver/proof candidacy widening; no Gate-1 completion.
```

## Bounded S0 (emitted)

`MIRBUILDER-GATE1-STATIC-RESULT-CALLER-COVERAGE-S0` — FreeFunction
caller coverage + `TopLevel` arm only:

- `seal_statements`: mint `free_function` `rows_by_key` rows for
  top-level `FunctionDeclaration` statements (parity with the
  source-backed issuer; `validate_parameters` +
  `DuplicateCanonicalKey` semantics preserved).
- `classify_source_context_v1`: `Located{root: TopLevel(key)}` +
  backed -> `DeclarationCatalogUnavailable` on absent catalog;
  `free_function(declared_name, declared_arity)` membership probe ->
  `ForeignLineage` when no declaration row; `declaration_for(
  StaticBoxMethod, owner, method, argc)` -> `Cataloged{caller:
  decl.key()}` / `Unavailable` when unresolved (mirrors Cataloged).
- Pins: top-level root + matching `free_function` row + static
  target decl -> `Cataloged`; absent catalog -> `DeclarationCatalog-
  Unavailable`; unrowed caller -> `ForeignLineage`; unresolved
  target -> `Unavailable`; `!source_backed` -> `Unavailable`.
- `src/mir/builder/README.md` lineage table update.
- No `InstanceConstructor`/`BirthConstructor` changes — that pair is
  atomic S1.

## Next selected row after S0 lands

`MIRBUILDER-GATE1-STATIC-RESULT-CALLER-COVERAGE-S1` —
`BirthConstructor` row co-seal (constructor loan join in the
source-backed issuer + `constructors` iteration in `seal_statements`)
+ `InstanceConstructor` arm (`published_birth_key` membership +
`StaticBoxMethod` target probe), landing atomically. Owns the
observed boxtorrent terminal (`LayoutBox.class_*` calls inside
`HakoAllocHeap` field-initializer generated stores).

## Non-claims

- `init`/`pack` constructors (`None` published key) stay foreign.
- `NestedBoxMethod`/`ScriptRoot` admission: unchanged, excluded.
- Solver/proof candidacy stays `StaticBoxMethod`-only.
- No member_route/owner/bridge/signature changes downstream.
- Gate-1 or overall MirBuilder completion: not claimed.
