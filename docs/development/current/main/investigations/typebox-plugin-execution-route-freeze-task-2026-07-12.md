# TypeBox Plugin Execution Route Freeze — Parked Task

Status: Parked BoxShape task; not the active lane.
Date: 2026-07-12

## Purpose

Make the existing TypeBox ABI v2 plugin call path match the repository's
planned `BoxCallableRegistry -> RoutePlan -> execution` ownership direction.
Resolve plugin callable routes when the loader state changes, publish one
immutable execution table, and keep ordinary calls off config, name, registry
reconstruction, and long-held loader locks.

This task does not create a third ABI and does not replace TypeBox ABI v2.

## Box versus leaf boundary

This task owns external stateful/dynamic **Box** execution only. It does not
make TypeBox ABI the universal low-level call path.

```text
TypeBox ABI v2:
  external stateful/dynamic objects
  identity + lifecycle + multiple methods
  cross-language/plugin dispatch

backend-private native leaf:
  static hot scalar operations
  instruction-like capability operations
  no external Box identity or dynamic method family

TargetRecipe / intrinsic lowering:
  compile-time backend selection
  never runtime TypeBox dispatch
```

The source-facing API may still be one ordinary typed Hako capability library.
Its implementation projection is selected below that source API:

```text
ordinary Hako library call
  -> pure Hako model
  -> backend-private native leaf
  -> TypeBox plugin when the implementation is genuinely Box-like/dynamic
  -> backend capability rejection
```

Do not route `ctz`, one CAS, one byte load, or allocator-loop scalar work
through TypeBox TLV merely to obtain a uniform implementation mechanism.
Conversely, do not disguise a stateful plugin object as a collection of
unowned native leaves.

## Current evidence

Load time already owns:

```text
config parsing
dynamic library loading
nyash_typebox_<Box> symbol lookup
TypeBox ABI validation
invoke_id / resolve function-pointer capture
```

The named VM call path still performs the following per invocation:

```text
global PluginHost read lock
PluginLoaderV2 read lock
BoxCallableRegistry snapshot reconstruction
config/spec/method-name lookup
type_id -> invoke pointer resolution
TLV encode
plugin invoke
TLV decode
```

Code anchors:

```text
src/runtime/plugin_loader_unified.rs
src/runtime/plugin_loader_v2/enabled/method_route_plan.rs
src/runtime/plugin_loader_v2/enabled/box_callable_registry.rs
src/runtime/plugin_loader_v2/enabled/route_resolver.rs
src/runtime/plugin_loader_v2/enabled/runtime_invoke_boundary.rs
src/runtime/plugin_loader_v2/enabled/ffi_bridge.rs
src/backend/mir_interpreter/handlers/boxes_plugin.rs
```

The existing `PluginMethodExecutionPlan` is the starting executable value.
Do not introduce a parallel `TypeBoxCallPlanV0` truth without first proving
that the existing plan cannot own the required frozen fields.

## Authority

```text
external ABI authority:
  Core C ABI
  TypeBox ABI v2

internal callable authority:
  BoxCallableRegistry

executable route authority:
  domain-owned RoutePlan / PluginMethodExecutionPlan

runtime binding substrate:
  PluginLoaderV2 function-pointer and library-lifetime state
```

`TypeBoxContract` is not introduced as a new authority. If that name is used
later, it must be either an internal callable-contract view attached to
`BoxCallableRegistry` or a read-only projection from existing authority.

## Target structure

```text
PluginLoader load / direct-autoload / explicit reload
  -> rebuild BoxCallableRegistry-derived plugin routes once
  -> bind invoke pointer and explicit compatibility policy
  -> publish immutable Arc<PluginExecutionRoutes>

ordinary plugin call
  -> route-key lookup
  -> copy/clone immutable PluginMethodExecutionPlan
  -> release host/loader guards
  -> TLV encode
  -> invoke_id
  -> TLV decode
```

The route key must preserve the existing callable domain distinction. A
single untyped `method_id` must not merge internal slots, plugin method IDs,
function IDs, intrinsic IDs, or lifecycle IDs.

## Smallest implementation slice

### R0 — executable route inventory

- Enumerate every mutation point for config, loaded libraries, box specs,
  TypeBox exports, direct autoload, and compatibility policy.
- Fix the publication/rebuild boundary and plugin-library lifetime rule.
- Record whether plugin unload or reload currently exists. Do not invent hot
  reload semantics.

### R1 — immutable route publication

- Add one loader-owned immutable route table, preferably derived from the
  existing `BoxCallableRegistry` projection.
- Rebuild only after a loader-state mutation completes successfully.
- Publish atomically as one complete value; no partially rebuilt table may be
  observed.
- Include the already resolved `invoke_box_fn`, shim pointer, explicit compat
  policy, `type_id`, `method_id`, and `returns_result` needed by the existing
  execution plan.

### R2 — named VM call adoption

- Replace per-call registry snapshot construction and config/name traversal
  with immutable route lookup.
- Obtain an owned/copyable execution plan, then drop PluginHost and loader
  guards before plugin native code runs.
- Preserve TLV wire bytes, error mapping, birth/fini behavior, handle
  lifecycle, and accepted callable shapes.

### R3 — gates and measurement

- Prove route-table rebuild count changes only at declared mutation points.
- Prove ordinary calls perform zero registry snapshot rebuilds and zero typed
  config deserializations.
- Prove plugin code is invoked without PluginHost/PluginLoader read guards.
- Preserve existing plugin ABI and behavior fixtures.
- Measure route-only call overhead before considering TLV specialization.

## Deferred slices

The following are separate decisions and must not be mixed into R0-R3:

```text
signature-specialized TLV adapters
stack/scratch TLV buffers
public or plugin-exported typed fast entries
TypeBox ABI struct/capability widening
new stable String/Box APIs
bulk plugin API redesign
reverse-call slot redesign
plugin hot reload semantics
```

Backend-internal hidden leaves remain governed by their existing manifest and
AOT SSOTs. A typed entry exported across a plugin library boundary requires a
separate TypeBox ABI v2 version/capability decision.

## Required gates

1. **Authority dependency gate**
   - route publication consumes `BoxCallableRegistry` and loader bindings;
   - no `TypeAbiPack` or serialized descriptor becomes planner/runtime truth.

2. **Rebuild boundary gate**
   - load/direct-autoload mutation causes exactly one successful publication;
   - failed mutation leaves the previous complete table intact or fails the
     loader before calls are accepted.

3. **Hot-path isolation gate**
   - ordinary call performs no `box_callable_registry_snapshot()`;
   - no `get_box_config()` clone/deserialize;
   - no `find_library_for_box()` scan/sort;
   - no TypeBox name resolver fallback unless the selected plan explicitly
     records a compatibility route.

4. **Lock lifetime gate**
   - host/loader guards are released before `invoke_id` enters plugin code;
   - reentrant calls do not depend on a silent `Ok(None)` escape.

5. **Exact behavior gate**
   - same `type_id`, `method_id`, TLV bytes, result decoding, error domain,
     birth/fini behavior, and handle ownership as the pre-change route.

6. **Fail-fast gate**
   - missing, stale, or incompatible route does not silently fall back to a
     newly reconstructed generic route.

7. **Box/leaf classification gate**
   - each candidate is classified as stateful/dynamic Box, backend-private
     native leaf, or compile-time recipe before route work starts;
   - static hot scalar leaves do not enter TypeBox TLV dispatch;
   - stateful plugin identity/lifecycle does not escape into ad-hoc leaf
     handles.

8. **Source facade isolation gate**
   - ordinary Hako callers depend on a typed capability/library API, not raw
     plugin symbols, hidden leaf names, or target recipe IDs;
   - implementation selection does not add source syntax or a third public
     ABI.

## Implementation may claim

```text
plugin callable routes are resolved and frozen at loader-state publication
ordinary named plugin calls do not rebuild the callable registry
ordinary named plugin calls do not traverse config to select a method
plugin native execution does not retain PluginHost/PluginLoader read guards
TypeBox ABI v2 wire format changed = 0
canonical ABI surface count changed = 0
accepted plugin callable vocabulary changed = 0
```

## Implementation must not claim

```text
TLV encode/decode is eliminated
all plugin calls are allocation-free
typed plugin fast entry is standardized
TypeBoxContract is a third canonical ABI
all backends use the frozen route
plugin hot reload is supported
plugin call semantics changed
performance win without a focused measurement
TypeBox ABI is the universal low-level function/instruction boundary
backend recipes are selected through runtime plugin dispatch
```

## Stop conditions

Stop and return to design if:

1. the route table becomes a second callable authority beside
   `BoxCallableRegistry`;
2. execution is rebuilt from `TypeAbiPack` or another serialized projection;
3. publication cannot keep the plugin library alive for every copied function
   pointer;
4. stale plans are repaired by silent fallback;
5. host/loader locks must remain held during native plugin execution;
6. the slice requires TypeBox ABI layout, TLV wire, method ID, or ownership
   changes;
7. a plugin-exported typed fast entry is required;
8. BoxCount behavior changes are mixed into this BoxShape task;
9. one file approaches 800 lines instead of splitting publication, lookup,
   and execution responsibilities.
10. a static hot scalar operation is moved into TypeBox TLV solely for API
    uniformity;
11. a compile-time target recipe becomes a runtime plugin-dispatch decision.

## Resume condition

This parked task may become active only after `CURRENT_STATE.toml` explicitly
selects it or the current design-stop decision returns to another workstream.
Before implementation, run a focused baseline that counts route rebuilds,
config resolution, and locks per named plugin call. The baseline is evidence,
not permission to widen the ABI.

The related native-leaf and target-recipe work remains parked in
`low-level-fast-path-v0-task-2026-07-12.md`; neither parked task activates the
other.
