# 1827 - MIRBUILDER-CRATE-WIDE-UNCONVERTED-SURFACE-REPORT-HARDENING-001

## Token

```text
MIRBUILDER-CRATE-WIDE-UNCONVERTED-SURFACE-REPORT-HARDENING-001
```

## Purpose

Harden the crate-wide unconverted Rust source surface report so it can be used
as a stable diagnostic map rather than a one-off scan.

This keeps the report diagnostic-only. It does not emit Hako, infer new
projection policy, select a family, or claim Source Selfhost.

## Delta

```text
provenance hashes:
  source_root_hash
  native_owner_seed_capability_survey_hash
  source_selfhost_family_guard_manifest_hash
  variable_context_reference_projection_contract_hash

reverse evidence checks:
  orphan_source_surface_count
  orphan_evidence_row_count
  orphan_evidence_rows

reason token table:
  stable medium-grained reason vocabulary for reported source surfaces

guard hardening:
  owner_edge_confidence required
  heuristic owner-edge joins cannot select an owner
  public IgnoredNonSemanticHelper is forbidden
  every item reason_token must appear in reason_token_table
```

## Result

```text
decision = KeepStopped
reason_token = AmbiguousUnconvertedSurfaceCandidates

known_owner_edge_count = 3
orphan_source_surface_count = 1563
orphan_evidence_row_count = 7
reason_token_count = 11
```

The report now exposes both directions:

```text
Rust source surface with no owner evidence
route/adoption evidence row with no exact source-surface join
```

These are diagnostic signals only. They do not select a family or prove
conversion eligibility.

## Acceptance

```text
tool_output_matches_checked_in_fixture = 1
source_root_hash_present = 1
input_fixture_hashes_present = 1
reverse_evidence_checks_present = 1
reason_token_table_present = 1
every_reason_token_is_stable = 1
owner_edge_confidence_recorded = 1
heuristic_owner_edge_not_selectable = 1
public_ignored_requires_reason = 1
manual_family_selection = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Non-Claims

```text
no Source Selfhost claim
no owner selection
no Hako generation
no HakoAdopted decision
no new borrow policy
no route repair
```
