# Rustc SemIR Internal Adapter Boundary

Status: Design
Scope: rustc semantic fact source for Rust-to-Hako lifecycle migration.

## Purpose

Replace source-shape lifecycle extraction probes with a real rustc semantic
adapter while keeping the stable handoff repo-owned.

This design does not implement the adapter.

## Boundary

```text
rustc internal layers
  -> adapter-owned normalization
  -> repo-owned RustLifecycleAdapterFacts JSON
  -> Hako lifecycle resolver / verifier / emitter
```

The adapter is a Rust facts producer. It is not a Hako policy owner.

## rustc Layers

### HIR

Use for stable source inventory:

```text
crate / module path
item identity
visibility
attributes relevant to layout / unsafe / cfg diagnostics
source span diagnostics
```

HIR owns item/module provenance. It does not own body lifecycle semantics.

### THIR

Use for typed structured body facts:

```text
typed expressions
structured if / loop / match
resolved method and operator calls
auto-ref / auto-deref evidence
pattern binding shape
temporary destruction scope hints
```

THIR is the preferred source for readable structure. It is not sufficient for
move / borrow / Drop parity by itself.

### MIR + Borrowck

Use for execution lifecycle facts:

```text
Place: local / field / index / deref projections
Operand: copy / move
borrow kind and region
borrow escape: CallOnly / LexicalScope / Returned / Stored / Unknown
definite / maybe initialized state
normal / unwind control-flow edges
```

MIR / borrowck owns move, borrow, and initializedness evidence.

### Drop Elaboration

Use for Drop obligations:

```text
TrivialMemory
StructuralOwned
CustomSemanticDrop
HostResource
Conditional
Open
```

Drop may be erased only from a positive `TrivialMemory` fact.

### Instance Graph

Use for concrete call and generic facts:

```text
concrete function instance id
monomorphized type arguments
resolved trait implementation
selected drop glue
```

Instance graph facts prevent the adapter from exporting unresolved trait or
generic syntax as lifecycle truth.

## Stable ID Normalization

The stable JSON handoff must not expose raw rustc IDs.

Allowed stable IDs:

```text
crate_path::module_path::item_name
function_instance_id derived from DefPath + normalized generic args
local/place ids scoped to function_instance_id
source span for diagnostics only
```

Forbidden stable IDs:

```text
raw HirId
raw DefId index/debug text
raw LocalDefId index
raw RegionVid
raw rustc MIR/THIR debug dump node ids
pretty-printed MIR as schema
```

The adapter may use rustc IDs internally, but must normalize before writing
repo-owned JSON.

## Output Contract

The first stable output remains target-neutral:

```text
kind=RustLifecycleAdapterFacts
schema_version=0
target_neutral.hako_policy_owner=false
target_neutral.hako_plan_kind_spelling_allowed=false
target_neutral.rendering_instruction_allowed=false
```

If `RustLifecycleAdapterFacts-v0` becomes too narrow, add a successor sidecar
through a design card. Do not smuggle rustc-private fields into v0.

## Toolchain Isolation

The adapter implementation must isolate rustc instability:

```text
adapter crate / tool owns rustc_private dependency
repo-owned JSON is the only checked-in stable handoff
toolchain version is reported in diagnostics
schema compatibility is tested by guards
source-shape extractors remain probes, not production truth
```

The product compiler, Hako resolver, verifier, emitter, and backend must not
depend on rustc internal crates.

## Source-Shape Probe Retirement

Current Python source-shape extractors are allowed only as probes.

Retirement rule:

```text
once rustc adapter emits equivalent facts for a subject:
  source-shape extractor may remain as regression probe
  but cannot be the authority for lifecycle facts
```

Guard wording should distinguish:

```text
source_shape_probe_green=1
rustc_semantic_adapter_green=1
```

## Stop Lines

```text
do_not_use_raw_rustc_dump_as_schema=1
do_not_let_adapter_choose_Hako_representation=1
do_not_emit_HakoLifecyclePlan_from_adapter=1
do_not_emit_hako_source_from_adapter=1
do_not_make_product_compiler_depend_on_rustc_private=1
do_not_treat_unknown_lifecycle_fact_as_box_fallback=1
```
