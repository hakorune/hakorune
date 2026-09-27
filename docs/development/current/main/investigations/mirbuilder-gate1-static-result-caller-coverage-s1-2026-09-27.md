# MIRBUILDER-GATE1-STATIC-RESULT-CALLER-COVERAGE-S1

Status: selected — implementation not yet landed
Date: 2026-09-27
Emission: `mirbuilder-gate1-static-result-caller-coverage-d0-2026-09-27.md`
rule 2 (`BirthConstructor` co-seal + arm, atomic).
Selection proof: `MIRBUILDER-GATE1-STATIC-RESULT-CALLER-COVERAGE-S1`
-> workstream row H.
MirBuilder goal row 1 (sole caller key issuance into `rows_by_key`),
row 3 (sole target/result ingress), row 4 (sole verified admission —
catalog declaration membership), row 6 (exact consume/drain via
`finish_empty` — mint and consume land in one slice).

## Exact boundary

- `issue_source_backed_same_module_callable_catalog_v1`: join the
  constructor syntax loan (`with_constructor_semantic_syntax`, the
  same loan `instance_constructor_semantic.rs:444-485` uses) and mint
  `birth_constructor(owner, arity)` `rows_by_key` rows carrying the
  parser-normalized `FunctionDeclaration` body — the same body the
  lowering port lowers. `init`/`pack` constructor kinds mint no row
  (no canonical key exists — `published_birth_key == None`).
  `validate_parameters` + `DuplicateCanonicalKey` discipline
  preserved; selected-source rows for constructor sites only if the
  existing inventory shape already owns such a site kind — otherwise
  constructor rows stay declaration-only.
- `seal_statements` (`catalog.rs`): iterate
  `BoxDeclaration.constructors` and mint `birth_constructor` rows for
  `birth`/`FunctionDeclaration` members — parity with the source-
  backed issuer; record/sync boxes and non-`birth` constructor kinds
  stay unrowed.
- `classify_source_context_v1`: `InstanceConstructor(key)` arm —
  `source_backed` -> `declarations` absent ->
  `DeclarationCatalogUnavailable`; `key.published_birth_key()`
  `None` (init/pack) or `catalog.declaration(&birth_key)` unresolved
  -> `ForeignLineage`; resolved AND `declaration_for(StaticBoxMethod,
  owner, method, argc)` `Some` -> `Cataloged{caller: decl.key(),
  site}`; probe `None` -> `Unavailable` (mirrors `Cataloged`/`TopLevel`).
- The grouped foreign arm keeps `ScriptRoot | NestedBoxMethod{..}`.
- `src/mir/builder/README.md` lineage paragraph update.

## Out of scope

- `NestedBoxMethod`/`ScriptRoot` — permanent exclusion.
- `init`/`pack` constructors — permanent exclusion (no canonical key).
- Solver/proof candidacy — stays `StaticBoxMethod`-only.
- Constructor lowering/demand/manifest changes — bodies already
  lower exactly once per source row on the minting lane.
- Any runtime/VM/legacy-path change.

## Pinned evidence (gate: focused test files only)

- Catalog: `seal_program` on a box with an explicit `birth` mints a
  `birth_constructor(owner, arity)` row carrying the normalized
  constructor body; generated field-initializer stores appear inside
  that body; non-birth constructor kinds mint no row.
- Ingress: `InstanceConstructor` + `source_backed` + catalog holding
  the `birth_constructor` row + `StaticBoxMethod` target decl ->
  `Cataloged{caller: birth_constructor(..)}`.
- Negative: `None` `published_birth_key` (init/pack key) ->
  `ForeignLineage`; absent catalog -> `DeclarationCatalog-
  Unavailable`; unrowed birth key -> `ForeignLineage`; resolved
  caller + unresolvable target -> `Unavailable`; `!source_backed` ->
  `Unavailable`.
- `observe_all_calls` on a catalog with a birth row inventories
  `LayoutBox.class_*` calls inside field-initializer argument sites
  (boxtorrent shape).
- `finish_empty` consistency: minted rows consumed exactly once on
  the minting lane — verified by existing drain pins plus a birth-
  body static-call drain test if one exists; otherwise the
  classification + inventory pins bound this slice.

## Negative/deletion proof

- No fallback: `None` published key stays `ForeignLineage`; absent
  catalog stays `DeclarationCatalogUnavailable`; unresolved target
  stays `Unavailable`.
- Atomicity: row minting and the lineage arm land in this one slice
  so `finish_empty` never sees minted-but-unadmittable rows and the
  arm never misreports `NoExactStaticTarget`.
- No deletion-set membership: shared seams untouched.
- Gate-1 or overall MirBuilder completion: not claimed.

## Next selected row

Determined at S1 closeout by the next observed terminal on the
boxtorrent measurement path.
