---
Status: Accepted design handoff
Date: 2026-07-12
Owner: 3505-LANGV1-FAILURE-OUTCOME-RELATION-INVENTORY-001
Decision: accepted projection binding policy
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

## Design Consultation Stop

The read-only worker inventory found runtime/provider evidence that cannot be
linked to one S1 source semantic site without choosing an owner policy:

```text
Wasm i32.const 0 projections = 9
void_sentinel_i64_zero route projections = 16
extern missing-result unwrap_or(Ok(VMValue::Void)) = 6
NullBox/VoidBox/MissingBox runtime surfaces = 201
FFI/provider boundary candidates = 74
```

The direct Wasm zero and route-level sentinel rows have no line-independent S1
source site. Assigning them to `VMValue::Void`, `ConstValue::Null`, a backend
route, or an operation owner would be a semantic authority choice, not
inventory. S2 therefore stops before writing a runtime/provider manifest.

The consultation must decide:

```text
source authority for backend zero/null projections
whether route-level sentinel rows project an operation site or a carrier site
how FFI/provider status rows name their boundary owner
whether an unresolved projection is pending or a strict guard failure
```

Non-authority evidence includes source file, current carrier, numeric zero,
helper name, route name, and VM agreement alone. Runtime activation remains
zero.

## Next Stop

The consultation accepted the source/representation split and opened the
minimum implementation task:

```text
LANGV1-FAILURE-OUTCOME-S2-PROJECTION-BINDING-001
```

See
`docs/development/current/main/phases/phase-296x/3507-LANGV1-FAILURE-OUTCOME-S2-PROJECTION-BINDING-001.md`.
