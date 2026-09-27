# MIRBUILDER-GATE1-STATIC-RESULT-CALLER-COVERAGE-S1

Status: landed
Date: 2026-09-27
Emission: `mirbuilder-gate1-static-result-caller-coverage-d0-2026-09-27.md`
rule 2 (`BirthConstructor` co-seal + arm, atomic).
Selection proof: `MIRBUILDER-GATE1-STATIC-RESULT-CALLER-COVERAGE-S1`
-> workstream row H.
MirBuilder goal row 1 (sole caller key issuance into `rows_by_key`),
row 3 (sole target/result ingress), row 4 (sole verified admission —
catalog declaration membership), row 6 (exact consume/drain via
`finish_empty` — mint and consume land in one slice).

## Landed boundary

- `issue_source_backed_same_module_callable_catalog_v1`
  (`source_backed.rs`): after the callable row scan, joins
  `with_constructor_semantic_syntax` (the same loan
  `instance_constructor_semantic.rs` uses) and mints
  `birth_constructor(owner, arity)` `rows_by_key` rows carrying the
  parser-normalized `FunctionDeclaration` body — the same body the
  lowering port lowers. `Init`/`Pack` kinds mint no row.
  `validate_parameters` + `DuplicateCanonicalKey` discipline
  preserved; the constructor loan failure lands as the typed
  `ConstructorSyntax` issue (new variant; error type re-exported
  through `parser/public_api.rs`).
- `seal_statements` (`catalog.rs`): iterates
  `BoxDeclaration.constructors` and mints `birth_constructor` rows
  for `birth`/`FunctionDeclaration` members — parity with the
  source-backed issuer; record/sync boxes and non-`birth`
  constructor kinds stay unrowed.
- `classify_source_context_v1`
  (`static_result_publication_ingress.rs`): `InstanceConstructor(key)`
  arm — `source_backed` -> `declarations` absent ->
  `DeclarationCatalogUnavailable`; `key.published_birth_key()`
  `None` (init/pack) or `catalog.declaration(&birth_key)` unresolved
  -> `ForeignLineage`; resolved AND `declaration_for(StaticBoxMethod,
  owner, method, argc)` `Some` -> `Cataloged{caller: decl.key(),
  site}`; probe `None` -> `Unavailable` (mirrors `Cataloged`/`TopLevel`).
- `NormalInstanceConstructorSourceKeyV1` gained
  `published_birth_key()` (`pub(in crate::mir::builder)`);
  `from_physical_source` widened to the same visibility for ingress
  fixtures.
- `src/mir/builder/README.md` lineage paragraph updated:
  `InstanceConstructor` moved to the admitted lineage list;
  `NestedBoxMethod`/`ScriptRoot` remain foreign.
- `static_result_publication_ingress.rs` reached the 800-line hard
  stop mid-slice; the `#[cfg(test)]` module moved to
  `static_result_publication_ingress_tests.rs` via `#[path]` (same
  module nesting — `super::super` paths unchanged). The two
  guards pinning test tokens inside the production file
  (`script_static_result_publication_ingress_guard.sh`,
  `mirbuilder_qualified_route_scope_guard.sh`) follow the move.

## Out of scope (unchanged)

- `NestedBoxMethod`/`ScriptRoot` — permanent exclusion.
- `init`/`pack` constructors — permanent exclusion (no canonical key).
- Solver/proof candidacy — stays `StaticBoxMethod`-only.
- Constructor lowering/demand/manifest changes.
- Any runtime/VM/legacy-path change.

## Evidence

- `cargo test --lib -- static_result_publication_ingress` —
  22/22 (includes 6 `InstanceConstructor` pins: declared admission,
  absent catalog -> `DeclarationCatalogUnavailable`, `None`
  published key -> `ForeignLineage`, unrowed key -> `ForeignLineage`,
  non-declaration target -> `Unavailable`, no ledger -> `Unavailable`).
- `cargo test --lib -- callable_declaration_catalog` — 24/24
  (seal parity: `excludes_non_catalog_callable_surfaces` now expects
  the `free_function` + `birth_constructor` pair;
  `birth_constructor_rows_carry_the_normalized_body_and_skip_other_kinds`
  pins body carry + init/pack exclusion;
  `source_backed_issuer_co_seals_birth_constructor_rows` pins the
  production issuer's row on a real parsed `box Holder { birth(seed) }`).
- `cargo test --lib -- source_call_target callable_result_representation
  callable_parameter_contract` — 188 pass / 1 known baseline
  (`declared_box_names_project_as_handle_not_opaque_or_exact`,
  `MapBox -> Map` arm from `98582ecc22`).
- `cargo test --lib -- program_root_work_plan selected_projection` —
  6 pass / 4 baseline `cohort-missing` (pre-existing, confirmed at
  baseline before this lane).
- `cargo test --lib -- normal_callable` — 492 pass / 5 baseline
  (dynamic x2 + semantic-package x3, all in the red-baseline manifest).
- Guards: `script_static_result_publication_ingress_guard.sh`,
  `mirbuilder_qualified_route_scope_guard.sh`,
  `mir_call_d1b_selected_normal_duplicate_projection_guard.sh`,
  `script_direct_static_source_reown_window_r0_guard.sh`,
  `script_instance_box_transfer_guard.sh`,
  `current_state_pointer_guard.sh` — all green.
- Stale-path baseline debt (fails identically on HEAD, recorded not
  fixed — different lane): `exact_callable_bare_function_call_location_guard.sh`
  and `cut0_i0_root0_..._public_ingress0_guard.py` still grep
  `src/mir/builder/raw_invocation_source_transport.rs` (moved to a
  directory by `dc81a64dc7`).

## Production measurement

`./target/release/hakorune apps/boxtorrent-mini/main.hako` on the
landed tree: the observed
`[freeze:contract][static-result-ingress/foreign-lineage]` terminal is
gone — `InstanceConstructor` callers (generated field-initializer
stores calling `LayoutBox.class_*`) now reach the verified-catalog
ingress. The next named terminal is
`[freeze:contract][ordinary-new/argument-source-unavailable]` at
`new ContentChunk(cid, data, alloc_handle)` (line 78): a constructor
argument bound to an instance-call result is outside the
`ordinary-new` trivial-argument inventory
(`Integer|Bool|Local|Handle`). Different family — this slice does
not claim it.

## Next selected row

`MIRBUILDER-GATE1-ORDINARY-NEW-ARGUMENT-SOURCE-D0` — `ordinary-new`
argument-source coverage: constructor arguments whose value is an
exact binding produced by a non-trivial expression (e.g.,
`alloc_handle = me.allocator.allocate(..)` feeding
`new ContentChunk(..)`). Requires a census before any
implementation: the trivial-argument inventory is a shared
admission boundary, and the correct fix is source-authorized
argument provenance, not a wider kind list.
