---
Status: closed__2026-09-14
Task: MIR-CALL-STATIC-COMPATIBILITY-I0-A2-BODY-CALL-CONSTRUCTOR-GUARDS
Date: 2026-09-14
Parent: mir-call-static-compatibility-a2-body-call-constructor-d1-2026-09-14.md
NextCard: none__a2_body_call_constructor_guard_closeout
Implementation permission: true for focused guard evidence in existing owners; no new semantic product
---

# A2 body, direct-call, and constructor guards I0

## Six-line brief

```text
Decision: close the finite A2 body/direct-call/constructor relation with existing owner checks and reject terminals.
Source authority + canonical issuer: parser final syntax loan, constructor catalog, resolver callable index, and existing package co-seal remain the sole issuers.
Non-authority: AST rescans, names/arity as keys, slot reconstruction, MIR/ValueId, compatibility fallback, publication, StringBox, and import lineage.
Fail-fast boundary: preserve typed parser/resolver/package rejects for missing, duplicate, foreign, orphan, target/header, arity, parent, and generated-origin drift.
Smallest next slice: add focused positive/negative coverage in the existing test owners and update the owner evidence only.
Non-claims: no MixedProgram admission, nested-owner policy change, caller switch, fallback retirement, Windows proof, or R7.
```

## Acceptance

- Positive evidence covers one ordinary/static body and direct-call source-index
  path plus one Birth/no-Birth constructor pair.
- Negative evidence names the existing terminal for missing/duplicate/foreign/
  orphan body rows, unissued/nested/wrong-arity/missing-header direct calls,
  foreign-parent or transformed-parent constructor rows, and mixed generated
  method/Birth provenance.
- The current production caller remains the normal root catalog lifecycle
  path; no new semantic receipt, fallback, or CI lane is introduced.
- Every changed source stays below the 760-line design boundary and
  `git diff --check` plus the pointer guard pass.

## Explicit non-claims

This row does not admit MixedProgram, change nested-owner target policy, alter
import-lineage transport, publish/cut over a backend, remove compatibility
edges, fix StringBox readers, prove Windows lifecycle, or close R7.

## Receipt — existing focused owner guards

The finite guard inventory was already covered by the existing owner tests, so
this row adds no semantic product or duplicate classifier. The focused direct
test binary passed these positive and negative witnesses:

- parser source/body and constructor preservation: `exact_static_callable_set_survives_one_transform`, `parser_program_source_authority_lends_one_paired_body_cursor`, `ordinary_constructor_source_catalog_survives_normal_source_transform`, and `foreign_transform_output_is_rejected_by_parser_session`;
- body-owner and source-index/header co-seal: `mixed_direct_methods_resolve_once_in_exact_source_order`, `freestatic_target_batch_lends_one_owner_matched_index_and_header`, `missing_row_rejects_before_any_lowering_input_is_lent`, and `lowering_input_borrows_the_same_forest_owner_and_parameter_binding`;
- direct-call terminals: `cataloged_typed_static_direct_call_uses_source_index_target`, `cataloged_nested_lambda_direct_call_observation_rejects_before_install`, `app_main_direct_call_wrong_arity_rejects_before_install`, `app_main_non_freestatic_direct_call_rejects_before_install`, and `app_main_nested_direct_call_observation_rejects_before_install`;
- constructor Birth/no-Birth and parent/completion guards: `instance_constructor_semantics_keep_parser_identity_and_nested_brand_relations`, `construction_plan_rejects_foreign_parent_and_birth_arity_not_as_no_birth`, `constructor_lookup_rejects_foreign_or_mismatched_parent_not_as_no_birth`, and `constructor_loan_rejects_lost_shape_and_completion`;
- generated method provenance: `delegate_generator_rejects_foreign_source_relation`, `property_generator_rejects_missing_duplicate_and_foreign_receipts`, and `delegate_anchor_coverage_rejects_missing_and_duplicate_relations`.

The production caller remains the existing normal root catalog lifecycle. No
new CI lane was introduced. The one `ordinary_batch_preflight_checks_candidates_before_that_owners_completion`
red remains the pre-existing `SourceAuthorityUnavailable(PostpassNotSourceBacked)`
fixture failure recorded on the parent A2 split card; it is outside this guard
receipt and is not attributed to this closeout.
