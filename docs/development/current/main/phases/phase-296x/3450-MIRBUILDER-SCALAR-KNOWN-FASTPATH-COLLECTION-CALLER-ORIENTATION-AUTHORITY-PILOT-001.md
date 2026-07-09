# 3450 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-AUTHORITY-PILOT-001

## Status

Ready implementation task. Authority remains zero until implementation and
guards are green.

## Required Delta

1. Replace the Collection assertion-only caller with an authority validator
   accepting only `policy_row_id` and returning `Unit`.
2. Resolve and exhaustively validate the exact four generated caller-contract
   rows and matching generated Collection policy rows.
3. Validate homogeneous lowering/result/value/publication/effect/proof fields
   plus each explicit receiver domain inside the generated policy boundary.
4. Retain the existing Collection route decision and Rust oracle fail-fast
   comparison; caller orientation must not choose or return a route.
5. Add exact-set, unknown-row, missing/extra-row, metadata-drift, receiver-
   domain-drift, `AnyLength -> Box`, and no-authority-leak tests.
6. Record a deterministic implementation fixture for the 3451 rerun.

## Stop Conditions

Stop if implementation needs receiver domain, route kind, core operation,
route decision, runtime value, non-Unit output, MIR/ValueId emission,
runtime/backend consumption, mutation/publication, fallback, Write, Delete,
wide, or Source Selfhost scope.

## Allowed Completion Claims

```text
collection_caller_orientation_authority_pilot = 1
collection_caller_orientation_authority_scope = policy_row_id_contract_only
collection_caller_orientation_consumer_unit_only = 1
collection_exact_four_row_scope = 1
collection_mixed_receiver_domain_guarded = 1
collection_anylength_box_explicit_row_guarded = 1
collection_hako_route_decision_authority_retained = 1
collection_rust_oracle_compat_checker_retained = 1
collection_mismatch_fail_fast = 1
no_new_route_authority = 1
```

All runtime/backend/mutation/publication/Write/Delete/wide/fallback/Source
Selfhost claims remain zero.

## Implementation Result

The live Collection validator now accepts only `policy_row_id`, resolves the
exact four generated contract rows, validates their generated policy metadata,
and returns `Unit`. Receiver domains are checked inside the generated policy
boundary; they are not caller inputs. The existing Collection route decision
and Rust oracle comparison remain the authority/veto path.

```text
collection_caller_orientation_authority_pilot = 1
collection_caller_orientation_authority_scope = policy_row_id_contract_only
collection_caller_orientation_consumer_unit_only = 1
collection_exact_four_row_scope = 1
collection_mixed_receiver_domain_guarded = 1
collection_anylength_box_explicit_row_guarded = 1
collection_hako_route_decision_authority_retained = 1
collection_rust_oracle_compat_checker_retained = 1
collection_mismatch_fail_fast = 1
no_new_route_authority = 1
```
