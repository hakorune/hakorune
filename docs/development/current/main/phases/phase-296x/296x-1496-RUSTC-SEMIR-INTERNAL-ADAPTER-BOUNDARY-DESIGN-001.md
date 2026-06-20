# 296x-1496 RUSTC-SEMIR-INTERNAL-ADAPTER-BOUNDARY-DESIGN-001

Status: closed
Date: 2026-06-20

## Purpose

Design the boundary for replacing source-shape lifecycle extraction probes with
a real rustc semantic adapter fact source.

This row is docs/design only. It must not invoke rustc internals or generate
new lifecycle facts.

## Selected By

```text
296x-1495-POST-RUSTC-SEMIR-EXTRACTED-FACTS-VERIFIER-PARITY-OWNER-SELECTION-001
```

## Scope

```text
input_candidates=HIR,THIR,MIR,borrowck,drop-elaboration,Instance graph
output_schema=RustLifecycleAdapterFacts-v0 or successor sidecar
consumer=existing lifecycle verifier / HakoLifecyclePlan resolver path
```

Questions to decide:

```text
which rustc layers are read for item identity and module provenance
which rustc layers are read for typed body / method target facts
which rustc layers are read for borrow escape / move / Drop facts
how rustc-internal IDs are normalized into repo-owned stable IDs
how toolchain version / rustc_private instability is isolated
how source-shape extractors are demoted to probes after rustc facts exist
```

Forbidden:

```text
rustc_internal_adapter_implementation
new generated RustLifecycleAdapterFacts
HakoLifecyclePlan-v0 output
.hako source output
backend behavior change
wider context extraction
```

## Acceptance

```text
rustc_semir_adapter_boundary_documented=1
stable_schema_boundary_documented=1
raw_rustc_dump_as_schema=0
adapter_policy_owner=0
implementation_started=0
backend_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Design

```text
design_ssot=docs/development/current/main/design/rustc-semir-internal-adapter-boundary.md
```

Decision:

```text
HIR owns crate/module/item/source provenance.
THIR owns typed structured body and resolved method/operator shape.
MIR + borrowck own copy/move/borrow/initializedness facts.
Drop elaboration owns Drop obligations.
Instance graph owns concrete generic/trait/drop-glue targets.
```

Stable handoff:

```text
repo-owned RustLifecycleAdapterFacts JSON
raw rustc IDs/dumps are forbidden as schema
adapter remains target-neutral and policy-free
```

## Closeout

```text
rustc_semir_adapter_boundary_documented=1
stable_schema_boundary_documented=1
raw_rustc_dump_as_schema=0
adapter_policy_owner=0
implementation_started=0
backend_behavior_changed=0
```

Next:

```text
POST-RUSTC-SEMIR-INTERNAL-ADAPTER-BOUNDARY-DESIGN-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_invoke_rustc_internals_in_design=1
do_not_generate_facts_in_design=1
do_not_choose_Hako_representation_in_adapter=1
do_not_start_wider_context_extraction=1
```
