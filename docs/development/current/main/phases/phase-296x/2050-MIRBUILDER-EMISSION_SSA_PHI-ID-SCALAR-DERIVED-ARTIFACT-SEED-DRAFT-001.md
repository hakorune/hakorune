# 2050 - MIRBUILDER-EMISSION_SSA_PHI-ID-SCALAR-DERIVED-ARTIFACT-SEED-DRAFT-001

## Token

```text
MIRBUILDER-EMISSION_SSA_PHI-ID-SCALAR-DERIVED-ARTIFACT-SEED-DRAFT-001
```

## Purpose

Materialize a `DerivedArtifactSeedDraftInput` for the verified emission_ssa_phi
ID scalar SourcePlanAndRecipe.

This is not native edit authority. It does not materialize a native source
seed, generate Hako, adopt Hako, or claim Source Selfhost.

## Result

```text
state = DerivedArtifactSeedDraftInput
derived_artifact_seed_draft_materialization = 1
generated_artifact_as_native_edit_authority = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0

decision = DerivedArtifactSeedDraftInputMaterialized
reason_token = EmissionSsaPhiIdScalarDerivedArtifactSeedDraftInputMaterialized
selected_next_card =
  MIRBUILDER-ID-SCALAR-DOMAIN-SEED-READINESS-RESOLUTION-003
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_emission_ssa_phi_id_scalar_derived_artifact_seed_draft_guard.sh
```
