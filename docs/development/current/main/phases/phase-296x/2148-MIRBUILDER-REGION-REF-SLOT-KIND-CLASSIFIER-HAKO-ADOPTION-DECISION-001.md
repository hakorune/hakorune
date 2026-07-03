# 2148 - MIRBUILDER-REGION-REF-SLOT-KIND-CLASSIFIER-HAKO-ADOPTION-DECISION-001

## Token

```text
MIRBUILDER-REGION-REF-SLOT-KIND-CLASSIFIER-HAKO-ADOPTION-DECISION-001
```

## Purpose

Adopt `region_ref_slot_kind_classifier` as the tenth narrow Rust-oracle parity
pilot owner after a green 10-row `.hako` EXE parity gate.

This decision adopts only the pure type classifier surface. It does not adopt
Region construction, slot metadata collection, GC retain/release insertion,
Region trace/log emission, MIR mutation, Source Selfhost, or full MirBuilder
conversion.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-region-ref-slot-kind-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/region_ref_slot_kind_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_region_ref_slot_kind_classifier_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-region-ref-slot-kind-classifier-hako-adoption-decision-v0.json
```

## Acceptance

```text
parity_gate = green
parity_rows = 10
decision = Adopt
hako_adopted = 1
rust_bootstrap_retained = 1
rust_oracle_retained = 1

source_selfhost_claim = 0
rust_deletion = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
generated_artifact_as_native_edit_authority = 0
region_construction_migration = 0
slot_metadata_collection_migration = 0
gc_retain_release_migration = 0
region_trace_log_migration = 0
mir_mutation_migration = 0
```

## Decision

```text
decision:
  Adopt

reason_token:
  RegionRefSlotKindClassifierRustOracleParityGateGreen

selected_next_card:
  MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-010
```

## Non-Claims

```text
no Source Selfhost claim
no Rust deletion
no runtime fallback
no new backend route
no new ABI
no generated artifact edit authority
no Region construction migration
no slot metadata collection migration
no GC retain/release migration
no Region trace/log migration
no MIR mutation migration
```
