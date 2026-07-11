---
Status: Active implementation task
Date: 2026-07-12
Owner: 3505-LANGV1-FAILURE-OUTCOME-RELATION-INVENTORY-001
Decision: accepted
---

# 3507 - LANGV1-FAILURE-OUTCOME-S2-PROJECTION-BINDING-001

## Objective

Implement the smallest S2 inventory-only slice that separates semantic
authority from runtime/backend representation. Runtime activation remains
zero and no existing carrier or backend behavior changes.

## Accepted Policy

```text
semantic authority = classified OperationOutcomeSite
projection mechanics = ProjectionBinding with backend projection owner
boundary transport = BoundaryObservation -> BoundaryAdapter -> OperationOutcomeSite
physical encoding never infers semantic meaning
```

Projection bindings must reference an already classified operation-outcome
site:

```text
projects_site -> site_kind=operation_outcome
projects_site -> review_status=classified
projects_site -> semantic_class/target_carrier/semantic_owner are complete
```

Forbidden projection sources are carriers, pending sites, synthetic
`source_observation` placeholders, other projections, route names, and helper
names. Synthetic source observations may remain as diagnostic evidence but have
`synthetic_source_is_authority = 0`.

## Schema Boxes

```text
OperationOutcomeSite:
  site_id, layer, owner_domain, operation, outcome_branch
  authority_kind, semantic_owner, semantic_class, target_carrier, profile
  evidence_refs

ProjectionBinding:
  projection_id, projection_owner, projects_site
  backend, route_id, encoding, payload_policy, collision_policy
  observability, capability, profile, evidence_refs, resolution

BoundaryObservation:
  observation_id, boundary_id, boundary_kind, direction
  transport_outcome, boundary_carrier, observation_owner, evidence_refs

BoundaryAdapter:
  adapter_id, adapter_owner, consumes_observation, maps_to_site
  declared_policy, profile, evidence_refs
```

`ProjectionBinding` derives semantic class and target carrier from
`projects_site`; it does not duplicate them as independent authority fields.
S2 may use `BoundInventoryOnly`, `Pending`, and `Unsupported` resolutions. It
must not add `Supported` or activate a runtime route.

## Ordered Slice

```text
P1 schema separation:
   keep OperationOutcomeSite, ProjectionBinding, BoundaryObservation, and
   BoundaryAdapter as distinct manifest collections

P2 synthetic-source demotion:
   retain source_observation only as diagnostic placeholder; never as authority

P3 positive corridor:
   bind hako_mem_free.success -> Unit -> VoidSentinelI64Zero when the explicit
   public API contract, no-payload policy, collision policy, and consumer
   discard behavior are all evidenced

P4 negative split:
   inventory ConstValue::Void and ConstValue::Null projection candidates as
   separate rows; do not infer Null as Unit or CompatNull without profile

P5 provider fallback:
   record the six unwrap_or(Ok(VMValue::Void)) rows as pending
   ProviderContractMissing/ApiContractMissing; do not map to Unit/None/Err

P6 guards:
   reject missing/pending/synthetic/projection-chain sources, class drift,
   zero-collision-unproven, provider-missing-to-Unit, missing payload policy,
   missing ForeignNull adapter, and compatibility profile omission
```

## Guard Contract

```text
projection source missing -> reject
projection source not classified authority -> reject
projection to another projection -> reject
projection semantic-class drift -> reject
zero projection collision proof missing -> reject
provider missing -> Unit -> reject
missing payload policy missing -> reject
ForeignNull adapter missing -> reject
implicit boundary mapping -> reject
compatibility_only profile missing -> reject
synthetic source authority -> reject
runtime activation != 0 -> reject
```

## Acceptance

```text
all four schema collections are machine-readable
projection source references are explicit and one-way
no projection derives meaning from zero/null/route/encoding
hako_mem_free corridor is bound only from its public API contract
ConstValue::Null and ConstValue::Void remain separate candidates
provider fallback rows remain pending with closed reasons
ForeignNull remains boundary-observation-only
S1 evidence queue and semantic-site graph remain green
parser/MIR/runtime/backend behavior changed = 0
runtime/provider activation = 0
```

## Non-Claims

```text
Unit/Option::None/Result::Err/Fault activation = 0
Weak upgrade behavior change = 0
Canonical null migration = 0
Compat2025 profile change = 0
ForeignNull language carrier = 0
Wasm/LLVM/AOT projection semantic completion = 0
provider fallback correction = 0
backend support widening = 0
selfhost claim = 0
```

## Commands

```text
python3 tools/docs/failure_outcome_site_inventory.py --check --strict
python3 tools/docs/failure_outcome_semantic_site_graph.py --check
python3 tools/docs/failure_outcome_projection_binding.py --check
python3 -m unittest tools/docs/test_failure_outcome_projection_binding.py
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Stop Boundary

If `hako_mem_free` lacks one complete API/consumer/collision evidence chain, or
if any projection source still requires semantic owner selection, stop at a
focused consultation. Do not use route names, zero encodings, or VM agreement
as a substitute for the missing authority.
