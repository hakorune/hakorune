# 1826 - MIRBUILDER-CRATE-WIDE-UNCONVERTED-SURFACE-REPORT-001

## Token

```text
MIRBUILDER-CRATE-WIDE-UNCONVERTED-SURFACE-REPORT-001
```

## Purpose

Report MirBuilder Rust source surfaces that are not yet tied to a conversion,
policy, verifier, route, or owner-edge classification.

This is a diagnostic / inventory card. It does not emit Hako, does not create a
new SemanticProjector, does not select a family by hand, and does not claim
Source Selfhost.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_crate_wide_unconverted_surface_report.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-crate-wide-unconverted-surface-report-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_crate_wide_unconverted_surface_report_guard.sh
```

## Result

```text
scan_unit = rust_function_or_method
join_unit = semantic_owner_edge
scan_method = regex_source_text_v0

decision = KeepStopped
reason_token = AmbiguousUnconvertedSurfaceCandidates
selected_next_card =
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001

scanned_surface_count = 1584
missing_projection_policy_count = 1396
borrow_policy_needed_count = 113
borrow_policy_known_count = 2
mapped_to_known_owner_count = 18
composite_suspected_count = 1
debug_only_count = 47
test_only_count = 7
```

The report exposes the current blocker more directly: there are many source
surfaces that still need projection policy, borrow policy, verifier/oracle
repair, decomposition, or owner-edge classification before a unique next
native owner seed can be derived.

Known VariableContext borrow surfaces are not treated as new blockers:

```text
variable_map()
  -> BorrowSurfacePolicyKnown
  -> OwnedReadSnapshotProjection

variable_map_mut()
  -> BorrowSurfacePolicyKnown
  -> ExplicitMutationApiOnly
```

## Acceptance

```text
tool_output_matches_checked_in_fixture = 1
scan_unit = rust_function_or_method
join_unit = semantic_owner_edge
scan_method = regex_source_text_v0
rust_ast_parser_required = 0
rustc_adapter_required = 0
semantic_inference_beyond_existing_ssot = 0

every_scanned_public_method_classified_exactly_once = 1
every_unconverted_item_has_reason_token = 1
multiple_candidates_keep_stopped = 1
borrow_policy_known_does_not_select_owner = 1
composite_suspected_is_not_decomposition_proof = 1
generated_artifact_only_is_not_native_edit_authority = 1
support_lane_only_is_not_hako_adoption_candidate = 1

manual_family_selection = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Next Allowed Moves

```text
if future report emits MissingProjectionPolicy exactly one:
  <OWNER>-PROJECTION-POLICY-001

if future report emits CompositeNeedsDecomposition exactly one:
  <OWNER>-DECOMPOSITION-001

if future report emits BorrowSurfaceNeedsPolicy exactly one:
  <OWNER>-BORROW-PROJECTION-POLICY-001

if future report emits MissingVerifierOrOracle exactly one:
  <OWNER>-VERIFIER-OR-ORACLE-REPAIR-001

if future report emits UnmappedRustSurface exactly one:
  <OWNER>-OWNER-EDGE-CLASSIFICATION-001

otherwise:
  keep SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

## Non-Claims

```text
no Source Selfhost claim
no family selection by hand
no Hako generation
no HakoAdopted decision
no native source owner seed materialization
no route repair
```
