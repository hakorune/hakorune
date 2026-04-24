---
Status: Landed
Date: 2026-04-24
Scope: Generate and guard the CoreMethodContract manifest derived from the `.hako` owner.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-132-core-method-contract-seed-card.md
  - lang/src/runtime/meta/core_method_contract_box.hako
  - lang/src/runtime/meta/generated/core_method_contract_manifest.json
  - tools/core_method_contract_manifest_codegen.py
  - tools/checks/core_method_contract_manifest_guard.sh
  - docs/tools/check-scripts-index.md
---

# 291x-133 CoreMethodContract Manifest Guard Card

## Goal

Land HCM-2: add a derived manifest and a focused drift guard without moving
backend consumers yet.

This is a BoxShape implementation card. It does not add a CoreBox row, parser
rule, `.inc` lowering branch, MIR carrier, environment variable, runtime
helper, or hot inline lowering.

## Implementation

- Added `tools/core_method_contract_manifest_codegen.py`.
- Generated `lang/src/runtime/meta/generated/core_method_contract_manifest.json`.
- Added `tools/checks/core_method_contract_manifest_guard.sh`.
- Wired the guard into `tools/checks/dev_gate.sh quick`.
- Added `lang/src/runtime/meta/generated/README.md`.
- Updated `docs/tools/check-scripts-index.md`.

The generator reads the narrow seed-row shape in
`CoreMethodContractBox`. The `.hako` box remains the source owner; the JSON is
a derived artifact.

## Manifest Shape

```text
schema: core_method_contract_manifest/v0
source: lang/src/runtime/meta/core_method_contract_box.hako
fields: schema_fields() result
row_count: 11
rows: derived rows with id / aliases / effect / core_op / lowering placeholders
```

## Boundary

- `.inc` still does not consume the manifest.
- MIR still does not carry CoreMethodOp metadata.
- The generated manifest is a drift target for the next no-growth guard.
- `CoreMethodContractBox` remains the semantic owner.

## Proof

```bash
python3 tools/core_method_contract_manifest_codegen.py --write
bash tools/checks/core_method_contract_manifest_guard.sh
bash tools/checks/current_state_pointer_guard.sh
cargo fmt -- --check
tools/checks/dev_gate.sh quick
git diff --check
```

## Next

- HCM-3: reject new `.inc` method-name classifier growth unless it has a
  CoreMethodContract row and deletion condition.
- Keep `.inc` table consumption for a later one-family card.
