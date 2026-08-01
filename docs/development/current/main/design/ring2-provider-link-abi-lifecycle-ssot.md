---
Status: SSOT
Decision: accepted direction; consolidation required before implementation
Date: 2026-08-02
Scope: ring responsibilities, provider residency, ring2 ABI transports,
  dispatch binding, provider-image lifetime, static embedding, and plugin Box
  lifecycle boundary.
Related:
  - docs/architecture/RINGS.md
  - docs/development/current/main/design/ring1-core-provider-scope-ssot.md
  - docs/development/current/main/design/box-callable-registry-ssot.md
  - docs/development/current/main/design/type-abi-naming-and-box-descriptor-ssot.md
  - docs/development/current/main/design/type-abi-view-and-plan-stamp-ssot.md
  - docs/development/current/main/design/type-abi-box-domain-ssot.md
  - docs/development/current/main/design/box-lifecycle-bprime-tombstone-adaptive-ownership-ssot.md
  - docs/reference/plugin-system/bid-ffi-v1-actual-specification.md
  - docs/reference/plugin-system/plugin_lifecycle.md
  - include/nyash_abi.h
  - src/box_callable/model.rs
  - src/box_callable/route_plan.rs
  - src/runtime/ring0/mod.rs
  - src/runtime/plugin_loader_v2/enabled/host_bridge.rs
  - src/runtime/plugin_loader_v2/enabled/instance_manager.rs
  - src/runtime/plugin_loader_v2/enabled/loader/library.rs
  - src/runtime/plugin_loader_v2/enabled/loader/specs.rs
  - src/runtime/plugin_loader_v2/enabled/types.rs
---

# Ring2 Provider Link, ABI, And Lifecycle SSOT

## Purpose

Keep these independent questions separate:

```text
ring:
  who owns and guarantees the capability

provider residency:
  whether the implementation is dynamically loaded or embedded

ABI transport:
  how arguments and results cross the provider boundary

dispatch binding:
  how the already-selected implementation entry is reached

optimization outcome:
  whether the toolchain later devirtualizes or inlines the call

lifecycle:
  who owns provider-image lifetime, logical fini, and structural destruction
```

Static linking does not promote an extension into ring1. A fast ABI does not
change its trust ring. A direct symbol is a binding form, not an ABI transport.
LTO is an observed optimization result, not a link or ABI contract. A
descriptor is not a hot execution path. These axes must not be collapsed into
one `plugin` or `Type ABI` flag.

This document records the accepted architecture only. It does not activate a
new implementation lane, change the current provider route, or authorize an
implicit fallback.

## 1. Runtime Rings

### Compiler core is outside the runtime ring classification

Parser, semantic analysis, MIR construction, verification, and backend
lowering are required Hakorune facilities, but they are not ring0 providers.

```text
source -> parser -> semantic facts -> MIR -> backend
```

### ring0: host kernel boundary

ring0 is the Box-unaware, language-unaware host substrate. The current
`Ring0Context` owns abstractions for memory, I/O, time, logging, filesystem,
and threads.

ring0 does not own Array, Map, FileBox, network, GUI, or application Box
semantics. Moving collection or plugin policy into ring0 is forbidden.

### ring1: Hakorune-guaranteed core providers

ring1 contains minimal, trusted, reproducible providers that Hakorune itself
guarantees. The current accepted domains are `file`, `array`, `map`, `path`,
and `console`.

The exact current implementation and wiring remain owned by
`ring1-core-provider-scope-ssot.md`. ring1 must not depend on ring2.

### ring2: extension providers

ring2 contains application, ecosystem, or replaceable providers. Examples
include network, GUI, database, Python, and application-specific Boxes.

A ring2 provider may be dynamically loaded or statically embedded. Its ring is
defined by responsibility and trust ownership, not by whether it lives in a
shared library.

## 2. Classification Matrix

```text
provider trust / responsibility:
  ring1 | ring2

provider residency:
  dynamic | embedded-static

ABI transport:
  BID-TLV | TypedFast

dispatch binding:
  library-generic | per-Box-table | exact-method-pointer | direct-symbol

optimization outcome:
  none-observed | devirtualized | inlined
```

| Provider | Ring | Residency | ABI transport | Binding | Optimization |
| --- | --- | --- | --- | --- | --- |
| Console core | ring1 | embedded | in-process core | direct | not claimed |
| Array core | ring1 | embedded | in-process core | direct | not claimed |
| Net extension | ring2 | dynamic | BID-TLV | library-generic | none observed |
| Net extension | ring2 | dynamic | TypedFast | exact method pointer | none observed |
| Net extension | ring2 | embedded | TypedFast | per-Box table | none observed |
| App-specific Box | ring2 | embedded | TypedFast | direct symbol | optional measured LTO |

Forbidden inference:

```text
static extension -> ring1
```

The correct classification is `static extension -> embedded ring2 provider`.
Only Hakorune's tracked ring1 authority may promote a domain into ring1. An
application manifest cannot grant itself ring1 trust.

## 3. Current Ring2 Transport

The current working compatibility transport is BID-FFI v1:

```text
type_id
method_id
instance_id
TLV arguments
TLV result
```

Its single generic entry is `nyash_plugin_invoke`. It is the portable,
dynamic, compatibility-oriented route.

The current mainline can also resolve a per-Box `BoxInvokeFn` and prefer it to
the library-wide compatibility shim. This removes repeated library/type
routing, but the current VM path may still perform method-plan lookup, TLV
encoding, output allocation, TLV decoding, and Box materialization.

Therefore the current per-Box route is not a zero-overhead typed method call.

## 4. Target Two-Transport Model

Ring2 keeps two explicit transports:

```text
BID-FFI / TLV:
  generic, portable, dynamic, compatibility and tooling route

Typed Fast ABI:
  load/build-time verified, exact-signature hot execution route
```

They consume the same immutable callable, signature, ownership, effect, and
lifecycle-capability truth. They must not maintain two independent policies.
Mutable instance-lifecycle state is not registry truth; it belongs to the host
lifecycle controller described below.

Accepted target ownership graph:

```text
provider image and exported-address observation:
  PluginLoaderV2 provider boundary

callable identity and immutable lifecycle capability truth:
  BoxCallableRegistry

descriptor projection:
  BoxDescriptor / historical TypeAbi* views

selected execution and binding:
  sealed RoutePlan + plan stamp + provider image pin

mutable instance-call legality:
  host lifecycle controller / future ObjectCell authority

external plugin execution ABI:
  TypeBox ABI v2 and its selected transport

physical ownership and lease drain:
  runtime object substrate, not the registry or descriptor view
```

### Typed Fast ABI target

At load or build time, verify and seal provider identity, ABI version, type
and method identity, exact signature and signature hash, ownership, effect,
error contract, backend capability, and lifecycle entries.

At execution time, a dynamic Typed Fast call becomes conceptually:

```text
cached_typed_function(instance, arg0, arg1)
```

The hot path must not repeat symbol-name or method-name lookup, registry
lookup, type/method switches, a generic invoke wrapper, TLV conversion,
temporary result-buffer allocation, generic result-tag dispatch, or
unnecessary Box materialization.

For a dynamically loaded provider, one indirect call to the actual plugin
implementation remains. This is the necessary implementation call, not an
extra wrapper call.

The current per-Box `invoke_id` path still uses method IDs, TLV payloads, and a
variable output buffer. It is a narrower BID dispatch path, not TypedFast.

## 5. Static Ring2 Embedding

`hako.toml` may eventually select an extension for static embedding, but the
manifest selects provider and link policy; it does not assign trust rings.

Illustrative target vocabulary, not a current accepted config schema:

```toml
[providers.net]
class = "extension"
source = "plugins/nyash-net-plugin"
link = "static"
transport = "typed-fast"
dispatch = "direct"
```

Do not expose user-controlled `ring = 1` promotion.

### Static forms and call cost

```text
shared library + BID-TLV:
  generic routing and transport cost

dynamic TypedFast:
  one indirect call to implementation

static function table:
  one indirect call to implementation

static direct symbol:
  one direct call to implementation

static direct symbol + successful LTO inline:
  implementation call may disappear
```

Static embedding alone does not remove an indirect call. Direct symbol
binding is a separate selection. LTO may remove the implementation call, but
inlining is an optimization outcome rather than an ABI guarantee.

For `static-direct`, the build must fail if exact binding is unavailable. It
must not silently lower the site to BID-TLV or a generic function table.

## 6. Selection And No-Retry Contract

An alternative compatibility transport may be selected before execution. It
is never selected as an after-failure retry.

```text
production / exact AOT requiring TypedFast:
  missing or mismatched TypedFast -> build/load failure

explicit compatibility profile:
  select BID-TLV before execution

forbidden:
  TypedFast call fails -> invoke the same operation again through BID-TLV
```

This prevents duplicate effects and keeps the selected RoutePlan authoritative.

## 7. Plugin Box Lifecycle: Current Truth

The current `PluginBoxV2` uses `Arc<PluginHandleInner>`:

```text
share_box:
  share the same inner handle and instance_id

clone_box:
  currently assumes method zero is birth and creates a fresh inner handle

finalize_now / Drop:
  AtomicBool suppresses repeated fini calls for one inner handle
```

This provides a practical SharedV1 RAII model, but it is not the final B′ Box
lifecycle contract.

Known gaps:

1. The common invocation boundary does not consistently reject every method
   call after `finalized` becomes true.
2. `PluginHandleInner::drop` invokes the user `fini` route, mixing logical
   finalization with structural instance destruction.
3. `finalize_now` marks the handle finalized before calling the plugin and
   does not propagate the lifecycle call result.
4. Singleton shutdown finalizes immediately only when `Arc::try_unwrap`
   succeeds; outstanding shares defer finalization to the last drop.
5. Individual plugins may reject a removed `instance_id`, but that is not a
   substitute for one host-owned use-after-fini boundary.
6. `clone_box` calls hard-coded method/instance zero rather than consuming the
   registry-selected birth route.
7. A `PluginHandleInner` copies plugin function pointers without pinning the
   `libloading::Library`; replacing the loader map entry can drop the old image
   while existing handles still retain its addresses.
8. The registry projection drops `lib_name` from callable identity, so
   duplicate Box exports have no stable provider-scoped winner or rejection.
9. `NyashTypeBoxFfi.version` and `capabilities` are present but not currently
   negotiated or enforced. The provider init result is also not an admission
   authority.
10. Reentrancy, output-buffer bounds, panic/exception containment, and
    cross-boundary allocation ownership do not yet have one common ABI law.

## 8. Target Lifecycle ABI

Typed Fast ABI adoption requires distinct lifecycle operations:

```text
birth:
  create a structural plugin instance

fini:
  explicit logical user finalization and eager external-resource release

destroy:
  structural instance destruction/reclamation; never a second user fini
```

The logical state machine is the B-prime lifecycle authority from
`box-lifecycle-bprime-tombstone-adaptive-ownership-ssot.md`:

```text
Alive
  method under an ordinary lease -> Alive
  one fini winner -> Finalizing

Finalizing
  reject new ordinary leases
  drain outstanding ordinary leases
  run the user fini hook at most once under a privileged finalizer lease
  attempt remaining teardown even when a hook or cleanup step fails
  preserve the primary typed error
  publish payload absent and Dead; never reopen partial Alive

Dead
  ordinary payload method -> fail-fast: use after fini
  fini -> idempotent result
  last structural owner -> provider destroy exactly once
```

Logical `Dead`, provider-instance destruction, and provider-image unloading are
three different facts. `Destroyed` is not a second logical state beside the
B-prime authority. Lifecycle state has one host-side authority. Plugins
implement selected hooks but do not independently decide whether a call is
legal.

Required invariants:

```text
one birth creates one structural instance identity
share does not create another instance
clone creates a fresh instance through birth
the user fini hook is attempted at most once for one lifecycle transaction
fini failure is observable, remaining teardown still runs, and partial Alive is never republished
method calls after Finalizing begins fail before plugin method execution
destroy is called exactly once when structural ownership ends
destroy never invokes user fini implicitly
singleton shutdown has an explicit outstanding-share disposition
static and dynamic ring2 use the same lifecycle state machine
```

The first implementation decision must also define clone through the selected
birth route, concurrent/reentrant fini behavior, and whether singleton shutdown
rejects, drains, or retains outstanding shares. `Arc::try_unwrap` success is not
a lifecycle policy.

## 9. Provider Image Lifetime And Identity

A dynamic executable address is legal only while its exact provider image is
alive. A copied function pointer is not a lifetime proof.

```text
sealed executable plan
  -> provider identity + provider generation
  -> exact ABI/callable contract
  -> PlanStamp checked at plan/cache boundary
  -> provider image pin
  -> selected function address
```

The pin may initially be a process-lifetime `NoUnloadV1` policy. In that scope,
loading another generation under the same identity must fail; silent map
replacement is forbidden. Hot reload and unload are outside the first
TypedFast slice.

Before either is enabled, a separate decision must define quiescing, active
call leases, instance and callback lifetime, background threads/TLS, generation
invalidation, and the exact point at which the image may be released. A
`PlanStamp` is checked at plan/cache boundaries, never on every hot call.

Provider identity must be part of callable and object identity. A bare
`box_type` or `u32 instance_id` is not globally unique across providers or
provider generations. Duplicate provider-scoped exports are either rejected
deterministically or selected by one sealed, explicit policy before execution.

## 10. TypedFast Wire Contract Obligations

TypedFast cannot be implemented by adding another Rust function-pointer type.
Its D0 must fix one language-neutral C ABI contract covering all of:

```text
representation:
  extern C, fixed-width scalars, opaque handles, pointer + explicit length
  no Rust String, Vec, enum, trait object, or compiler-dependent layout

canonical signature descriptor:
  ABI version and algorithm id
  provider/type/method stable identity
  receiver, argument, and result wire types
  ownership, effect, error, thread/reentrancy, and lifecycle capability

version negotiation:
  incompatible major -> reject
  compatible minor only through struct_size + required capability checks

failure boundary:
  stable status and out-result/error validity
  no partial result publication
  no panic or C++ exception across the ABI

memory ownership:
  borrowed input lifetime
  returned buffer/handle owner and release function
  no allocator-crossing free

trust and capability:
  signature hash proves ABI compatibility, not trust or authorization
  requested host capability and granted provider capability are distinct
```

The exact handle width, hash algorithm, error representation, capability
vocabulary, and thread model remain D0 decisions. The current `version` and
reserved `capabilities` fields must not be described as negotiated proof.

## 11. Structural Boundaries

Do not put plugin invocation or lifecycle policy into ring0.

Do not duplicate callable, signature, effect, or ownership truth in both the
BID and TypedFast implementations. Both transports consume a sealed plan.

Do not use historical `TypeAbiView`, `TypeAbiCatalog`, or `TypeAbiPack` as a
hot execution ABI. They remain descriptor projections.

Do not add unconditional fallback, by-name hot dispatch, or per-plugin
lifecycle exceptions.

Do not claim plugin ObjectCell/B′ adoption until logical `fini` and structural
`destroy` have separate ABI owners and use-after-fini is enforced at the
common boundary.

## 12. Parked Consolidation Task Order

The architecture does not need a rewrite. It does require the following
consolidation before implementation. These tasks are parked inventory, not a
change to `CURRENT_STATE.toml` or the active MirBuilder lane.

### D0-1: `RING2-LIFECYCLE-BPRIME-ALIGNMENT0-D0`

Decide the exact host lifecycle transaction before changing runtime code.

```text
select:
  registry = immutable birth/fini/destroy capability truth
  host controller = mutable Alive/Finalizing/Dead legality
  provider = selected hook implementation only

seal:
  fini failure and teardown order
  concurrent and reentrant fini
  use-after-fini gate
  clone through selected birth
  outstanding singleton-share shutdown policy
  legacy BID lifecycle compatibility and retirement condition
```

Done means this document and the B-prime SSOT describe one state machine and
one failure law. No runtime edit is authorized by the D0 alone.

### D0-2: `RING2-PROVIDER-IMAGE-PIN-IDENTITY0-D0`

Choose the first provider-image lifetime policy and canonical provider-scoped
identity. The preferred first baseline is process-lifetime pinning with
same-identity reload rejected. General unload/hot reload is not required.

```text
seal:
  provider identity and generation
  image pin owner
  duplicate export rejection/selection
  provider init failure before publication
  object identity across provider/type/instance
  PlanStamp and cache invalidation boundary
```

Done means every retained executable address has a lifetime proof and no
loader map replacement can invalidate a live plan or instance.

### D0-3: `RING2-TYPEDFAST-WIRE-CONTRACT0-D0`

Fix the C ABI obligations from section 10: canonical signature bytes and hash,
version negotiation, status/error representation, unwind containment,
allocation ownership, thread/reentrancy law, capability grant, and lifecycle
entries. It must also decide the versioned treatment of existing plugins that
have birth/fini but no structural destroy.

Done means a provider and host can independently build the same compatibility
descriptor without using Rust layout or mutable loader state.

### Refactor Series 1: `RING2-PLUGIN-LIFECYCLE-BPRIME0`

Use a short live BoxShape series on the existing BID route only.

```text
S0:
  install one host lifecycle controller and common invocation gate

I0/R0:
  separate logical fini from structural destroy
  consume the registry-selected birth route for clone
  enforce the selected shutdown-share policy
  add sharing/clone/failure/reentrancy/use-after-fini/destroy-once tests

same-series retirement:
  Drop -> user fini
  standalone AtomicBool logical-state authority
  hard-coded clone birth method zero
  Arc::try_unwrap as shutdown policy
```

Do not add TypedFast in this series.

### Refactor Series 2: `RING2-CALLABLE-LINK-PLAN0`

Create the provider-scoped immutable callable/link plan with a live existing
BID consumer in the first commit. A caller-zero or proof-only generic plan is
forbidden.

```text
provider exports
  -> immutable provider-scoped BoxCallableRegistry snapshot
  -> sealed RoutePlan + PlanStamp + provider image pin
  -> existing BID execution exactly once
```

The series retires per-call reconstruction from mutable config/spec state and
fails deterministically on provider/type/callable collisions.

### BoxCount sequence

Only after both Refactor Series are green:

```text
RING2-TYPEDFAST-DYNAMIC-ONE-SHAPE0-I0-R0
  one dynamic provider + one exact signature
  selected before execution; BID retry after failure = 0

RING2-TYPEDFAST-STATIC-TABLE0-I0-R0
  same ABI plan, embedded residency, indirect table binding

RING2-TYPEDFAST-STATIC-DIRECT0-I0-R0
  exact direct symbol; unavailable binding = build failure

RING2-TYPEDFAST-ASM-EVIDENCE0-D0
  measure call count and assembly before any zero-wrapper/LTO claim

RING2-PROVIDER-CONFIG-SCHEMA0-D0
  only then select hako.toml vocabulary and user workflow
```

General unload/hot reload, if still wanted, is a later independent
`RING2-PROVIDER-UNLOAD-RELOAD0-D0`. It must not be smuggled into the first
dynamic TypedFast row.

Keep BoxCount and BoxShape separate. Keep task selection and closeout in one
workstream update; do not create one docs file per subdecision.

## 13. Hard Stops

```text
classifies direct-symbol as an ABI transport
classifies LTO/inlining as a link or ABI guarantee
uses TypeAbi* descriptor projections as hot callable truth
calls the current per-Box TLV invoke path TypedFast

keeps mutable lifecycle legality in BoxCallableRegistry or the plugin
returns Finalizing/partially torn-down state to Alive after fini failure
calls user fini from Rust Drop or structural destroy
lets use-after-fini reach the plugin before host rejection
uses hard-coded method zero as clone birth

stores a dynamic function pointer without an exact provider image pin
allows same-identity reload while old plans/instances exist
drops provider identity or generation from callable/object identity
checks PlanStamp on every hot call

calls a version field negotiation without major/minor/capability checks
lets Rust panic or C++ exception cross the ABI
frees provider memory with the host allocator or vice versa
treats a signature hash as trust or authorization
uses manifest thread_safe alone as host concurrency proof

retries one effectful operation through BID after TypedFast failure
publishes a provider after ignored init failure
lands a disconnected production plan with caller zero
mixes lifecycle BoxShape and TypedFast BoxCount in one series
changes the active MirBuilder lane from this parked design document
```

## Non-Claims

- The current runtime is not changed by this decision.
- Typed Fast ABI is not currently a production route.
- The illustrative `hako.toml` keys are not an accepted public schema.
- Static ring2 is not ring1.
- Static embedding alone does not guarantee direct calls.
- LTO does not guarantee inlining.
- Current plugin lifecycle is not yet B′ complete.
- Dynamic provider pointers are not yet protected by a sealed image-lifetime
  contract.
- `NyashTypeBoxFfi.version` and `capabilities` are not currently proof of
  negotiated TypedFast compatibility.
- General plugin unload and hot reload are not accepted capabilities.
- This document does not change the active MirBuilder lane or authorize code
  implementation from the current design-stop state.
