# 3456 - MIRBUILDER-MAPSTORE-ROUTE-POLICY-KEY-VALUE-DOMAIN-BOXSHAPE-001

## Status

Active by explicit user priority change on 2026-07-12. Language v1
conformance closeout remains parked; this card is the selected selfhost
resume task. No language-v1 behavior, scope, evidence, or acceptance claim is
changed by the reprioritization. The common-row design was accepted on
2026-07-12.

This card changes representation and validation structure only. It must not
add an accepted route or move authority.

## Accepted Design

Use one hand-authored Hako `RoutePolicyRow` table as the policy SSOT. The
existing I64/Any classifier boxes remain compatibility projections during the
migration; they must not become independent policy owners.

```text
common Hako RoutePolicyRow
        |
        +--> I64/Any classifier projections
        +--> one generator -> typed Rust decision payload
        +--> caller/shadow shared validator

Rust route matcher/oracle remains independent
```

The row separates `key_domain` from `stored_value_domain`. `value_boundary` is
removed from the MapStore contract rather than renamed. Existing guards and
fixtures may be migrated in bounded batches, but no compatibility copy may
remain as an authority source.

Decision: accepted. This is a BoxShape-only migration; route selection,
runtime mutation, backend lowering, and Source Selfhost claims remain zero.

## Final Design Refinements

The accepted design is tightened by the following rules:

1. The generator parses the common Hako source exactly once into a typed
   `PolicyTable` model, validates that model, and renders every artifact from
   the in-memory model. No renderer reparses a generated projection.
2. Structural/schema validation belongs to the generator. Semantic parity is
   an independent Rust-oracle veto; the generator must not become a second
   route-selection oracle.
3. Caller contracts are generated from the row plus one explicit
   `MetadataOnlySingleSurfaceV1` projection profile. Route-name conditionals
   must not encode caller policy.
4. The generated Rust model uses typed enums for MapStore domains and closed
   policy fields. Pipe-delimited Hako text is wire format only.

```text
hand-authored Hako RoutePolicyRow
        -> parse once
        -> typed PolicyTable
        -> structural validator
        +-> Rust policy artifact
        +-> Hako classifier projections
        +-> caller-contract projections

independent Rust matcher/oracle -> parity veto only
```

The generator exposes `--check` and atomic `--write` modes. It renders all
outputs in memory before comparing or replacing any artifact, so partial
projection state cannot be committed.

## Problem

`MapStoreI64` names the key domain, not the stored-value domain:

```text
MapStoreI64: key_domain = I64, stored_value_domain = Any
MapStoreAny: key_domain = Any, stored_value_domain = Any
```

The old `value_boundary=ScalarI64` tuple merged independent semantic axes. Its
duplication across source, generator, generated Rust, validators, and fixtures
allowed the error to remain internally consistent.

## Structural Delta

1. Define one typed Hako `RoutePolicyRow` SSOT for MapStore policy data with:
   `policy_row_id`, `route_kind`, `key_domain`, `stored_value_domain`,
   `result_shape`, `effect_class`, `mutation_class`, `publication_policy`, and
   `authority_kind`.
2. Keep row data owned by the hand-authored Hako source. Use one generator to
   emit the typed decision payload and classifier/caller projections.
3. Keep the Rust route matcher and Rust oracle independent.
4. Replace caller/shadow tuple comparisons with one shared validator entry;
   projections must not validate policy independently.
5. Remove the ambiguous `value_boundary` field from this MapStore contract.

Expected touchpoints:

```text
lang/src/compiler/lib/write_set_mapstore_route_policy.hako
lang/src/compiler/lib/write_set_mapstore_i64_policy_classifier.hako
lang/src/compiler/lib/write_set_mapstore_any_policy_classifier.hako
tools/rust_lifecycle/generate_write_set_mapstore_route_policy.py
tools/rust_lifecycle/generate_write_set_mapstore_*_hako_policy.py
src/mir/generic_method_route_plan/generated/write_set_mapstore_route_policy.rs
src/mir/generic_method_route_plan/generated/write_set_mapstore_*_hako_policy.rs
src/mir/generic_method_route_plan/caller_orientation.rs
src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs
src/mir/generic_method_route_plan/write_routes.rs
src/mir/generic_method_route_plan/tests/map_set_routes/
```

`write_routes.rs` is an oracle/test touchpoint, not a route-matching rewrite
target for this card.

## Implementation Progress

The first implementation slice is now materialized:

- common Hako `RoutePolicyRow` source contains both MapStore rows;
- one generator emits the typed Rust row artifact;
- live MapStore shadow decisions consume that shared artifact;
- one neutral validator is shared by caller-orientation and shadow consumers;
- shared domain/metadata validation rejects row drift;
- independent key/stored-value axis tests are fixed in the Rust shadow suite.
- the common generator now renders the typed Rust row, classifier compatibility
  projections, and caller-contract projections from one parsed row table;
- `--check` verifies every generated artifact, while `--write` renders and
  validates the complete artifact set before atomic per-file replacement;
- generated MapStore artifacts are now owned by the common generator, while
  classifier projections no longer carry the ambiguous `value_boundary` field;
- MapStore shadow checks now validate only the independent key/stored-value
  domains and shared policy metadata.

The older I64/Any classifier artifacts remain compatibility projections for
existing historical gates. They are not consumed by the live MapStore shadow
decision and are scheduled for bounded projection migration before 3456
closeout.

## Projection Boundary Finding

Directly importing the common policy Box from the classifier source was probed
and rejected by the existing Hako merge/parser contract: the imported static
Box is merged into the classifier compilation unit and fails before the
classifier body is parsed. The probe was reverted; the existing parity gate
remains green and no parser workaround is allowed in this card.

The next projection slice must therefore use the generator boundary:

```text
common Hako RoutePolicyRow
        -> one generator
        -> typed row + legacy classifier projection artifacts
```

The generator must validate the common row first, then emit the legacy output
shape as a derived compatibility view. Classifier source text must no longer
be treated as policy authority. Existing historical gates can be migrated in
the same bounded slice to assert the common source and generated projection.
This is a design boundary finding, not a parser or runtime expansion.

The direct Hako `using` projection probe remains rejected. Projection
generation must not depend on Hako merge/parser composition.

Focused verification for this slice is:

```bash
python3 tools/rust_lifecycle/generate_write_set_mapstore_route_policy.py \
  > /tmp/mapstore_route_policy.rs
diff -u src/mir/generic_method_route_plan/generated/write_set_mapstore_route_policy.rs \
  /tmp/mapstore_route_policy.rs
cargo test -q --lib scalar_known_hako_shadow
```

## Required Tests

```text
I64 key + Any stored value -> MapStoreI64
Any key + the same stored value -> MapStoreAny
changing only the stored-value type does not change route kind
unknown, missing, extra, or drifting typed fields -> fail-fast
caller and shadow consumers use the same validator
```

Tests must vary key and stored-value domains independently. A fixture that only
copies the generated tuple is not an independent oracle.

Migration guards must reject:

```text
missing key_domain
missing stored_value_domain
value_boundary present in the MapStore row
classifier projection used as policy authority
caller/shadow validator bypass
generator parse count greater than one
generated artifact used as generator input
Rust oracle used as projection source
partial multi-artifact write
```

## Authority Boundary

```text
route matching = Rust write_routes.rs
policy row edit source = hand-authored Hako
decision payload = Rust artifact generated from Hako
compatibility veto = independent Rust validator / oracle
mutation and backend = downstream Rust
caller orientation = policy-row contract acceptance or rejection only
```

## Acceptance

```text
mapstore_route_policy_typed_row_ssot = 1
mapstore_key_stored_value_domains_separated = 1
mapstore_shared_policy_validator = 1
mapstore_independent_axis_tests = 1
route_behavior_change = 0
route_selection_authority_switch = 0
caller_orientation_authority_pilot = 0
runtime_mutation_authority = 0
backend_lowering_authority = 0
publication_execution = 0
source_selfhost_claim = 0
```

Additional closeout claims:

```text
common_hako_policy_authority_count = 1
common_generator_count = 1
common_source_parse_count_per_run = 1
hand_authored_i64_classifier = 0
hand_authored_any_classifier = 0
hand_authored_mapstore_caller_contract = 0
legacy_generator_reads_classifier_source = 0
legacy_generator_reads_caller_contract_source = 0
value_boundary_mapstore_residual = 0
generated_projection_used_as_policy_authority = 0
rust_route_matcher_independent = 1
rust_oracle_generated_from_hako = 0
```

## Next

Implementation order:

```text
1. common parser + typed PolicyTable + schema validator
2. multi-artifact generator with atomic --write/--check
3. generated I64/Any classifier projections
4. caller contracts from the fixed projection profile
5. live/shared validator migration
6. old generators become delegators/checkers
7. historical parity gates move to common-row evidence
8. value_boundary and legacy-source residuals reach zero
```

After this order, the independent Rust oracle comparison, and all required
tests are green, resume 3454. After a green 3454 fixture-backed rerun, enter
3455 and park caller orientation before the focused Fact/Plan/Boundary
inventory.
