# 293x-451 MIR-ROUTE-PREFLIGHT-001 Lowering-Plan Preflight

Status: ready
Date: 2026-05-16

## Decision

`MIR-ROUTE-PREFLIGHT-001` is the next compiler/selfhost sidecar after
`MIR-EMIT-SSOT-001`.

It adds a pure-first MIR metadata preflight so missing or unsupported lowering
routes fail before ny-llvmc / C shim emission.

SSOT:

```text
docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md
```

## Scope

- Add `tools/checks/pure_first_route_preflight.py`.
- Read MIR JSON `functions[].metadata.lowering_plan`.
- Treat `docs/development/current/main/design/lowering-plan-json-v0-ssot.md`
  as the schema owner.
- Classify known route-miss conditions before backend emission.
- Integrate preflight into pure-first guards after the same-artifact route is
  available.
- Add a narrow guard fixture for a route hit and a route miss.

## Required Reason Vocabulary

```text
lowering_plan_missing
unsupported_tier
typed_user_box_method_contract_missing
typed_global_call_contract_missing
target_body_supported=false
target_exists=false
arity_mismatch
return_shape_missing
value_demand_mismatch
```

These are classifier reasons. They must be derived from actual LoweringPlan
JSON v0 fields such as `site`, `source`, `tier`, `emit_kind`, `target_exists`,
`target_body_supported`, `arity_matches`, `return_shape`, `value_demand`, and
`reason`. Do not invent reason strings that cannot be traced to the current
MIR JSON schema.

## Output Contract

Failures should be single-line or compact multi-line records with stable tags:

```text
[pure-first-route][fail]
function=<name>
site=<block.instruction>
callee=<source callee when available>
reason=<reason>
owner=<route owner when available>
suggestion=<narrow next action>
```

## Stop Lines

- Do not change route selection.
- Do not add C shim matchers.
- Do not add allocator behavior.
- Do not implement expected return-type propagation in this row.
- Do not hide missing routes behind fallback execution.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `451.1` | Inventory current LoweringPlan JSON v0 fields from the SSOT and one generated MIR artifact. | Preflight has a narrow schema contract and fails cleanly on missing metadata. | no broad MIR parser |
| `451.2` | Implement the preflight classifier. | Known missing route reasons produce stable output. | no backend emit |
| `451.3` | Add route hit/miss guard fixtures. | A supported route passes and a deliberately unsupported route fails before ny-llvmc. | no app-specific name matching |
| `451.4` | Wire pure-first guards after artifact exactness. | Guards run preflight on the same MIR file they pass to `--mir-in`. | no duplicate MIR emission |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/pure_first_route_preflight_guard.sh
tools/checks/dev_gate.sh quick
```

## Return Condition

This row closes when a missing route is classified from MIR metadata before the
C shim would produce a late `mir_call_no_route`-style failure.
