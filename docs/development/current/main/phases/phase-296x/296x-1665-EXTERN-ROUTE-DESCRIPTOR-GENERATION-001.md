---
Status: Complete
Date: 2026-06-24
Scope: Move extern-call route declaration need rows toward descriptor-owned data.
---

# EXTERN-ROUTE-DESCRIPTOR-GENERATION-001

## Decision

Select extern route descriptor generation after generic-method set-route value
shape generation. Same-module/global-call descriptor generation remains a later
separate owner.

## Target

The Rust MIR extern route planner already owns route metadata through
`ExternCallRouteSpec`:

```text
route_id
core_op
symbol / aliases
arity
value_arg_index
proof
return_shape
value_demand
effect_tags
accepts_void_result
```

The next slice moves the C declaration-need table for extern calls out of
handwritten `LoweringPlanExternNeedRule` rows and into generated descriptor data.
The backend must consume selected extern route facts; it must not infer route
meaning from symbol spelling.

## Minimal Slice

```text
source authority:
  src/mir/extern_call_route_plan/route_spec.rs

first generated consumer:
  lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_metadata_rules.inc

handwritten table to remove:
  LoweringPlanExternNeedRule rules[]
```

## Acceptance

```text
extern route descriptor generator owns need_kind rows for all routes that
currently have LoweringPlanExternNeedRule entries.

C shim consumes descriptor rows by:
  route_id
  core_op
  route_kind
  symbol
  tier

Generator rejects:
  unknown route kind
  unknown need kind
  duplicate route_id + symbol
  missing C need mapping for a consumed extern route

LoweringPlanExternNeedRule handwritten struct/table = 0
symbol/name fallback for declaration need = 0
existing extern route focused guards remain green
new backend route = 0
runtime fallback = 0
```

## Verification

```text
python3 tools/extern_call_route_descriptor_codegen.py --check
python3 tools/generic_method_route_descriptor_codegen.py --check
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
```

Note:

```text
k2_wide_mimalloc_atomic_load_exe_guard.sh currently fails later in
pure-first-route preflight on print global-call target publication, not on
extern route declaration need generation.
```

## Non-Claims

```text
same-module/global-call descriptor generation = 0
extern emit rule generation = 0
extern route source planner redesign = 0
canonical MIR instruction change = 0
foreign/unsafe capability redesign = 0
```
