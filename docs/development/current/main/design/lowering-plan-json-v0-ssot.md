---
Status: SSOT
Decision: accepted
Date: 2026-05-01
Scope: backend-facing LoweringPlan JSON v0 contract for pure-first ny-llvmc.
Related:
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/design/backend-recipe-route-profile-ssot.md
  - docs/development/current/main/design/ai-handoff-and-debug-contract.md
  - src/mir/core_method_op.rs
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/root.rs
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
---

# LoweringPlan JSON v0 SSOT

## Purpose

Move ny-llvmc toward emit-only behavior without a wide rewrite.

The backend must not keep discovering semantic shape from raw MIR. The
backend-facing contract is:

```text
CoreOp is semantics.
LoweringPlan is backend contract.
ny-llvmc is an emitter.
ColdRuntime is explicit unoptimized lowering, not compat replay.
HotInline is proof-only optimization.
```

## Stop Line

- ny-llvmc may consume `metadata.lowering_plan`.
- ny-llvmc must not invent a `LoweringPlan` entry from raw MIR.
- if a site has a valid `LoweringPlan` entry, ny-llvmc should prefer that entry
  over legacy route metadata for the same site.
- `Unsupported` belongs to the plan-builder side. The backend may report it, but
  it must not lower it.
- `ColdRuntime` is allowed only when the plan names an explicit runtime ABI
  symbol. It is not `HAKO_BACKEND_COMPAT_REPLAY=harness`.

## JSON v0 Shape

`metadata.lowering_plan` is an array of flat entries. The flat form is
intentional: C consumers can read it without nested shape interpretation.

Required fields:

| field | meaning |
| --- | --- |
| `site` | stable display id such as `b0.i2` |
| `block` | MIR block id |
| `instruction_index` | MIR instruction index within the block |
| `source` | builder source family, initially `generic_method_routes` |
| `source_route_id` | original source route id, for example `generic_method.get` |
| `core_op` | resolved semantic op, for example `MapGet` |
| `tier` | `HotInline`, `DirectAbi`, `ColdRuntime`, or `Unsupported` |
| `emit_kind` | `inline_ir`, `direct_abi_call`, `runtime_call`, or `unsupported` |
| `symbol` | ABI/helper symbol for call-based entries, or `null` |
| `proof` | semantic proof, initially `core_method_contract_manifest` |
| `route_proof` | source route proof, for example `get_surface_policy` |
| `route_kind` | source route kind needed by current emitters during migration |
| `perf_proof` | true only for keeper hot-path proof |

Optional fields may carry operands and result values:

| field | meaning |
| --- | --- |
| `receiver_value` | receiver value id |
| `receiver_origin_box` | receiver origin family when known |
| `arity` | method arity for method-derived plan entries |
| `key_route` | key/index route, or `null` |
| `key_value` | first key/index value id, or `null` |
| `value_value` | second extern/store value id, or `null` |
| `result_value` | result value id, or `null` |
| `return_shape` | semantic result shape, or `null` |
| `value_demand` | value demand expected by emitter/runtime |
| `publication_policy` | publication/objectization policy, or `null` |
| `effects` | stable effect tags |

Example:

```json
{
  "site": "b0.i2",
  "block": 0,
  "instruction_index": 2,
  "source": "generic_method_routes",
  "source_route_id": "generic_method.get",
  "core_op": "MapGet",
  "tier": "ColdRuntime",
  "emit_kind": "runtime_call",
  "symbol": "nyash.runtime_data.get_hh",
  "proof": "core_method_contract_manifest",
  "route_proof": "get_surface_policy",
  "route_kind": "runtime_data_load_any",
  "perf_proof": false,
  "receiver_value": 1,
  "key_value": 2,
  "result_value": 3,
  "return_shape": "mixed_runtime_i64_or_handle",
  "value_demand": "runtime_i64_or_handle",
  "publication_policy": "runtime_data_facade",
  "effects": ["read.key"]
}
```

## Tier Mapping

v0 bridges existing CoreMethodContract vocabulary into backend-facing tiers:

| source tier | plan tier | meaning |
| --- | --- | --- |
| `warm_direct_abi` | `DirectAbi` | direct helper ABI call is explicit and accepted |
| `cold_fallback` | `ColdRuntime` | runtime ABI call is explicit and not perf proof |
| future hot proof | `HotInline` | inline IR with keeper proof |
| no plan | absent / `Unsupported` | plan builder owns diagnosis |

## Migration Rule

v0 starts by deriving plan entries from `generic_method_routes` and narrow
extern-call route plans.

That is not the final architecture, but it creates the right boundary:

```text
generic_method_routes
  -> lowering_plan v0
  -> ny-llvmc reads plan first
  -> legacy route readers stay as fallback only during migration
```

Extern calls are not CoreMethodContract rows. They must use their own source:

```text
extern_call_routes
  -> lowering_plan v0
  -> ny-llvmc reads plan first
```

Do not force externs into `CoreMethodOp`; use a narrow `core_op` string such as
`EnvGet` / `EnvSet` plus `proof=extern_registry`.

Global user/static calls are also not a `generic_method_routes` or
`extern_call_routes` slice. Do not add one-off `.inc` matchers for concrete
global names such as `BuildBox.emit_program_json_v0/2`. Same-module global user
calls use the `global_call_routes` plan family. In v0 this family records
`tier=Unsupported` with call target, arity, target existence, target arity,
arity match state, result representation, proof, and reason. It is a diagnostic
contract only until a later card adds a real typed user/global-call emitter.

`global_call_routes.reason` distinguishes:

| reason | owner |
| --- | --- |
| `missing_multi_function_emitter` | ny-llvmc module/function emitter |
| `global_call_arity_mismatch` | MIR call target contract |
| `unknown_global_callee` | MIR resolver / module contents |

The backend may surface this reason, but it must not reclassify raw callee
names to decide it.

The generic pure program reader must treat `functions[]` as the module owner and
entry selection as a view. While the emitter is still entry-only, it must keep
module facts available through the program view so the multi-function emitter
can be added without inventing a second raw JSON scanner.

C consumers must read `global_call_routes` through a dedicated
`LoweringPlanGlobalCallView`, the same way generic method and extern plans have
typed views. The failure path may report `view.reason`; it must not hand-parse
`source/tier/reason` at the callsite.

Same-module global user calls must carry `target_symbol` when
`target_exists=true`. The v0 symbol is the quoted LLVM function symbol for the
target MIR function. Call emitters must use `target_symbol`; `callee_name`
remains diagnostic identity and resolver evidence.

When `target_exists=true` but `target_shape=null`, MIR must also carry
`target_shape_reason` when it can explain why the target did not match a
lowerable shape. This is classifier evidence only; ny-llvmc may report it, but
must not reinterpret raw callee names or body JSON to decide shape ownership.
The C `LoweringPlanGlobalCallView` must read this field as metadata and may
surface it on dev-only unsupported-shape traces; it must not use it as a
backend-local permission to emit a call.
When the reason is caused by a child same-module target, MIR should also carry
`target_shape_blocker_symbol` and `target_shape_blocker_reason` so the next
acceptance slice can be chosen without scanning raw target bodies in the
backend.
Reason strings should be specific enough to preserve ownership boundaries. In
particular, unsupported local instructions, method calls, unsupported extern or
backend-global surfaces, missing global targets, and child targets whose own
shape remains unknown are distinct causes. A `void` signature whose observed
returns are string-or-void sentinel values must use
`generic_string_return_void_sentinel_candidate`, not the broader
`generic_string_return_abi_not_handle_compatible`; it is still unsupported
until a separate shape/proof makes the sentinel ABI lowerable.
Non-string object returns, such as `box<MapBox>`, must use
`generic_string_return_object_abi_not_handle_compatible`. This marks an object
boundary for the next ownership slice instead of hiding it behind the broad
return ABI reason.
The same object reason applies when a `void` or `unknown` signature has observed
local return-profile evidence for a non-`StringBox` object, for example
`void|null` plus a returned `ArrayBox`. This remains diagnostic evidence only:
MIR must not make the parent target lowerable, and child-global object blockers
must continue to propagate through `target_shape_blocker_*` instead of being
collapsed into the parent.
String-or-void sentinel candidates may run the same MIR-owned body blocker scan
as generic pure string targets, with `null`/`void` sentinel constants allowed as
return-profile evidence only. If that scan finds a more specific unsupported
child target, method call, extern call, backend global, or instruction, MIR must
surface that blocker through `target_shape_reason` and
`target_shape_blocker_*` instead of stopping at the generic sentinel candidate
reason. This remains diagnostic evidence; it must not make the sentinel target
lowerable.
If the sentinel return-profile is almost present but the non-sentinel return is
an unknown same-module global call, MIR must report that returned child global
as the blocker instead of collapsing the parent into
`generic_string_return_abi_not_handle_compatible`. This keeps source-execution
triage on the next ownership edge and still does not authorize the parent body.
When that returned child global already carries its own blocker evidence, MIR
must propagate the deepest known blocker instead of stopping at the intermediate
wrapper. This keeps fail-fast traces aligned with the next concrete ownership
edge while preserving the unsupported parent route.
Outside that sentinel body scan, a `null`/`void` constant observed by the generic
pure string classifier must reject with
`generic_string_unsupported_void_sentinel_const` instead of the broad
`generic_string_unsupported_instruction`. This marks presence-probe helpers such
as env flag checks as a distinct next-slice blocker without accepting them as a
string body.
The generic pure string classifier may treat `null`/`void` constants as
comparison-only sentinels for `==`/`!=` against string values. Such sentinels do
not count as string returns and cannot flow through PHI, arithmetic, or returns.
This allows null-guarded string helpers to expose their real body blocker, such
as an unsupported method call, without hiding behind the null check.

Same-module global user-call target evidence must also include
`target_return_type` when `target_exists=true`. This is the compact MIR
signature return label (`i64`, `str`, `void`, `box<Name>`, etc.) for diagnostics
and next-slice selection only. It is not a backend-local permission bit:
`target_shape`, `tier`, proof, arity, and same-module definition availability
remain the legality contract.
C consumers must read this through `LoweringPlanGlobalCallView`; callsites and
diagnostics must not fetch it by hand from raw JSON.

If MIR naming normalization rewrites a diagnostic call name to the canonical
function symbol, `callee_name` must keep the original observed call name and
`target_symbol` must carry the canonical MIR function name. Example:
`callee_name="main._helper/0"` may resolve to
`target_symbol="Main._helper/0"` through the MIR NamingBox policy.

Before any `UserGlobalCall` emitter is enabled, ny-llvmc must validate the
direct target through `LoweringPlanGlobalCallView`: route id, `UserGlobalCall`,
`target_symbol`, target existence, arity match, and quoted-symbol safety. A
site that passes this validator but still has `tier=Unsupported` is a
`missing_multi_function_emitter` stop, not a permission to externalize the call.

The first lowerable same-module user/global-call target shape is
`numeric_i64_leaf`. The second lowerable shape is
`generic_pure_string_body` for the narrow generic pure string/env/global-call
subset. The third lowerable shape is `generic_i64_body` for narrow i64 helpers
that use the same generic function emitter contract but return `ScalarI64`.
The fourth lowerable shape is `generic_string_or_void_sentinel_body` for the
same string body subset when canonical returns are string handles or a void/null
sentinel. It uses the generic string function emitter and reports
`return_shape=string_handle_or_null`.
The fifth lowerable shape is `program_json_emit_body` for exact Program(JSON v0)
emit wrappers. It accepts `BuildBox._emit_program_json_from_scan_src/1` and the
Stage1 raw wrapper that calls `BuildBox.emit_program_json_v0(source, null)`.
It does not accept general `BuildBox.emit_program_json_v0/2` calls, MapBox
options, or bundle paths. It uses the same Stage1 Program(JSON v0) handle
export as `parser_program_json_body`, but the MIR proof must come from the
wrapper shape, not from backend by-name matching.
MIR owns these classifications and records them as `target_shape`.
The string-or-void sentinel return-profile scan may classify
`RuntimeDataBox.substring(i64, i64)` / `StringBox.substring(i64, i64)` as a
string return only when the receiver is already known string and both arguments
are scalar-like values. The body scan and LoweringPlan `generic_method.substring`
metadata remain the authority for actual emission.
Generic string scans and generic i64 scans seed value classes from existing MIR
`value_types` and declared signatures; unknown parameters must not be treated
as string by default. A string concat surface such as `"" + value` may prove
only the concat result is string, while scalar substring bounds must still come
from typed value evidence. Relational i64 comparisons, exact MIR copies, and PHI
destinations with known typed evidence may propagate scalar evidence inside the
analysis; method rejection may only be deferred while the value-class fixpoint
is still changing.
String return-profile scans are weaker than body acceptance scans and exist to
surface the next owner boundary. They must not treat raw scalar `value_types`
metadata as semantic non-string proof for string handles. A string concat
surface proves its result is a string handle, and loop-carried PHIs may carry
that return-profile string class when they have observed string evidence and no
observed non-string evidence.
Within `generic_i64_body`, MIR may infer a string receiver for
`RuntimeDataBox.length()` / `StringBox.length()` and
`RuntimeDataBox.substring(i64, i64)` / `StringBox.substring(i64, i64)`, and
may accept string `Lt` / `Gt` comparisons that produce a boolean for
digit-range scanners such as `StringHelpers.to_i64/1`.
`length()` produces an i64 value. `substring` produces a string value only after
both bounds resolve to i64; pending unknown bounds may defer during the
fixpoint, but non-i64 bounds reject the shape. Ordered string compares are
lowered through the existing `nyash.string.lt_hh` helper and must not become a
backend-local by-name route for specific helpers.
`generic_i64_body` may also accept a `?` return signature when every canonical
return value is proven i64 by the body scan. This supports thin i64 wrappers
such as `StringScanBox.find_quote/2` without treating unknown return signatures
as scalar by default.
Within generic string scans, direct child route facts are stronger than stale
`void` value metadata for the call result. Exact copies and all-string/all-i64
PHI destinations may carry that proven class through stale `void` destinations.
This override is limited to proven direct-route or exact-flow evidence and must
not infer unrelated unknown parameters as string.
Same-module target classification must iterate in deterministic function-name
order so the shape fixpoint is stable across equivalent module map orders.
Return-profile blocker propagation is diagnostic-only: missing child targets
produce `generic_string_global_target_missing`, unknown child targets propagate
their blocker, and already-direct child targets must not create blockers.
For return-profile analysis, a direct
`generic_string_or_void_sentinel_body` child call produces a `StringOrVoid`
value class. That class may merge string/null evidence and counts as both string
and void when deciding whether a parent is also a string-or-void sentinel body;
it must not make arbitrary `void` values string-compatible.
Within `generic_pure_string_body`, MIR may accept string-class
`RuntimeDataBox.length()` and `RuntimeDataBox.substring(i64, i64)` only when the
receiver is already classified as a string value, or when an existing string
corridor fact for the same method-call result proves `str.len` / `str.slice`.
That corridor proof may seed the exact receiver as `StringBox` for unknown
receiver values such as `StringScanBox.read_char/2`; it must not classify
unrelated unknown parameters as string by default. These methods must also
carry the matching `generic_method.len` / `StringLen` or
`generic_method.substring` / `StringSubstring` LoweringPlan entry before
ny-llvmc emits `nyash.string.len_h` or `nyash.string.substring_hii`; backend
shims must not infer this from the raw method name alone. This does not accept
other string methods.
`generic_pure_string_body` may also contain the existing supported backend
global `print` as a no-result debug side-effect. That surface is not a
same-module user/global call and must not create a `global.user_call`
LoweringPlan entry or externalize to an unresolved function symbol.
The lowerable v0 rows are:

| route | target_shape | tier | emit_kind | proof |
| --- | --- | --- | --- | --- |
| `global.user_call` | `numeric_i64_leaf` | `DirectAbi` | `direct_function_call` | `typed_global_call_leaf_numeric_i64` |
| `global.user_call` | `generic_pure_string_body` | `DirectAbi` | `direct_function_call` | `typed_global_call_generic_pure_string` |
| `global.user_call` | `generic_string_or_void_sentinel_body` | `DirectAbi` | `direct_function_call` | `typed_global_call_generic_string_or_void_sentinel` |
| `global.user_call` | `generic_i64_body` | `DirectAbi` | `direct_function_call` | `typed_global_call_generic_i64` |
| `global.user_call` | `program_json_emit_body` | `DirectAbi` | `direct_function_call` | `typed_global_call_program_json_emit` |

ny-llvmc may emit a direct call only after it has emitted the target function as
a definition in the same LLVM module. Calling a same-module `target_symbol`
that only has a declaration is forbidden because it externalizes the MIR
function and hides the missing multi-function emitter.

For `generic_pure_string_body` and `generic_i64_body`, ny-llvmc must seed
definition emission from the selected entry function and follow only the
transitive closure of direct generic calls. It must not scan the whole module
and define every string-looking or i64-looking helper, because that widens the
active authority surface beyond the current entry route.

New backend work should add a `LoweringPlan` entry before adding a new raw
`.inc` matcher. Existing route metadata may stay until the matching plan
consumer is proven.

## Consumer Rule

`.inc` consumers must read the common generic-method plan fields through the
shared LoweringPlan metadata view before applying family-specific legality.
Consumers may validate operands, proofs, effects, and helper symbols for their
own family, but they should not duplicate the generic source/tier/proof/site
field parsing.

Need-kind declaration rules should be table rows keyed by LoweringPlan view
fields. Do not add one-off `strcmp` ladders for every new proven plan slice.
Route-state declaration rules follow the same policy: plan-first route
selection should be table rows keyed by the shared LoweringPlan view, with
legacy route metadata retained only as the migration fallback.
Generic-method emit-kind selection follows the same policy: plan-first
`generic_method_emit` rows should be table rows keyed by the shared
LoweringPlan view instead of per-op branch ladders.
Set-route declaration rules also use table rows. Value-shape-specific set
variants must be added as rows rather than extending ad hoc branch ladders.
Rows for concrete set helpers must validate observed value shape before
selecting a helper.

## Proven v0 Slices

| slice | tier | symbol | proof |
| --- | --- | --- | --- |
| `MapGet` | `ColdRuntime` | `nyash.runtime_data.get_hh` | P70 plan-only fixture |
| `MapGet` | `DirectAbi` | `nyash.map.slot_load_hh` | P80 plan-only fixture |
| `MapHas` | `DirectAbi` | `nyash.map.probe_hi` | P72 plan-only fixture |
| `MapHas` any | `DirectAbi` | `nyash.map.probe_hh` | P94 plan-only fixture |
| `ArrayHas` | `DirectAbi` | `nyash.array.has_hh` | P82 plan-only fixture |
| `MapLen` | `DirectAbi` | `nyash.map.entry_count_i64` | P75 plan-only fixture |
| `ArrayLen` | `DirectAbi` | `nyash.array.slot_len_h` | P76 plan-only fixture |
| `StringLen` | `DirectAbi` | `nyash.string.len_h` | P77 plan-only fixture |
| `ArrayGet` | `DirectAbi` | `nyash.array.slot_load_hi` | P78 plan-only fixture |
| `ArrayPush` | `ColdRuntime` | `nyash.array.slot_append_hh` | P83 plan-only fixture |
| `MapSet` | `ColdRuntime` | `nyash.map.slot_store_hhh` | P85 plan-only fixture |
| `ArraySet` i64 | `ColdRuntime` | `nyash.array.slot_store_hii` | P88 plan-only fixture |
| `ArraySet` handle | `ColdRuntime` | `nyash.array.slot_store_hih` | P89 plan-only fixture |
| `ArraySet` string | `ColdRuntime` | `nyash.array.set_his` | P91 plan-only fixture |
| `StringSubstring` | `DirectAbi` | `nyash.string.substring_hii` | P92 plan-only fixture |
| `StringIndexOf` | `DirectAbi` | `nyash.string.indexOf_hh` | P93 plan-only fixture |
| `EnvGet` | `ColdRuntime` | `nyash.env.get` | P108 plan-only fixture |
| `EnvSet` | `ColdRuntime` | `nyash.env.set` | P157 Stage1 using-resolver guard |
| `UserGlobalCall` | `Unsupported` | `null` | P112/P113 plan-only diagnostic |

## Non-goals

- no broad `CoreOp` expansion in this card
- no hidden compat replay
- no new environment variable
- no promise that `ColdRuntime` is a perf keeper
