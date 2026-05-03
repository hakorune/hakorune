---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv dev-gate generated CoreMethodContract manifest sync
Related:
  - docs/development/current/main/phases/phase-29cv/P328A-METHODIZE-NO-CALL-IDENTITY.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# P329A: CoreMethodContract Manifest Sync

## Problem

After P328A, the source-execution checks pass, but `tools/checks/dev_gate.sh quick`
stops at the generated CoreMethodContract manifest guard:

```text
[core-method-contract-manifest] generated manifest is stale; run:
python3 tools/core_method_contract_manifest_codegen.py --write
```

The `.hako` owner remains the SSOT. The JSON manifest is a generated mirror
used by the gate.

## Boundary

Do not edit CoreMethodContract rows by hand.

Do not change route policy or runtime behavior.

This card only refreshes the generated manifest from the existing `.hako`
owner and records the gate evidence.

## Implementation

Run:

```text
python3 tools/core_method_contract_manifest_codegen.py --write
```

Then rerun `tools/checks/dev_gate.sh quick`.

## Acceptance

```text
python3 tools/core_method_contract_manifest_codegen.py --check
tools/checks/dev_gate.sh quick
```

Both pass.

## Result

Observed:

```text
python3 tools/core_method_contract_manifest_codegen.py --check
[core-method-contract-manifest] ok

tools/checks/dev_gate.sh quick
[dev-gate] profile=quick ok
```

The manifest sync updated only the generated `row_count` mirror.
