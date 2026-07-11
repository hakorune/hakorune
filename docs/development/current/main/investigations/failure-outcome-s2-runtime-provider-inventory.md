---
Status: Active task
Date: 2026-07-12
Owner: 3505-LANGV1-FAILURE-OUTCOME-RELATION-INVENTORY-001
Decision: accepted inventory-only slice
---

# LANGV1-FAILURE-OUTCOME-S2-RUNTIME-PROVIDER-INVENTORY-001

## Decision

Extend the Failure/Outcome evidence inventory across runtime/provider
boundaries. Preserve the S1 semantic-site graph as the operation/outcome
identity source; S2 adds runtime/provider evidence and boundary categories only.

No semantic migration, carrier change, fallback addition, parser/MIR change,
or backend behavior change is allowed.

## Scope

Inventory these runtime/provider evidence families:

```text
VMValue/ConstValue conversion
Weak upgrade and dead/freed observation
NullBox/VoidBox/MissingBox compatibility surfaces
extern/provider dispatch and status branches
zero/missing-result synthesis
FFI/host boundary nullable or status carriers
```

Every row retains source location and links to an existing S1 semantic site
when the operation identity is already known. Unresolved owner/class/target
fields remain pending; the inventory must not infer semantic meaning from a
shared `VMValue::Void`, zero, or null bit pattern.

## Guard Contract

```text
duplicate runtime evidence id -> reject
unknown runtime evidence family -> reject
source location or evidence text missing -> reject
semantic activation != 0 -> reject
runtime/provider evidence without S1 site reference -> pending, not guessed
zero/null projection without source semantic site -> reject
provider-missing fallback classified as Unit -> reject
missing-result fallback classified as ordinary absence -> reject
FFI boundary without explicit boundary kind -> pending
```

## Acceptance

```text
all six runtime/provider families have deterministic evidence rows
all rows preserve source_path, line, token, and evidence_kind
S1 semantic-site references are stable and line-independent
unresolved rows remain pending with a reason
runtime/provider activation = 0
parser/MIR/runtime/backend behavior changed = 0
S1 graph and evidence queue remain green
```

## Commands

```text
python3 tools/docs/failure_outcome_site_inventory.py --check --strict
python3 tools/docs/failure_outcome_semantic_site_graph.py --check
python3 tools/docs/failure_outcome_runtime_provider_inventory.py --check
python3 -m unittest tools/docs/test_failure_outcome_semantic_site_graph.py
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next Stop

After the runtime/provider inventory is green, continue S3 control-flow
inventory. If one runtime/provider owner must be selected rather than merely
inventoried, stop at a focused consultation; do not activate a carrier from
this task.
