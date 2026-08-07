---
Status: SSOT
Decision: accepted target; implementation parked behind the active MirBuilder lane
Date: 2026-08-07
Scope: package/ProviderSlot boundary, ring responsibilities, provider
  residency, Provider Box binding profiles, dispatch binding, provider-image
  lifetime, static embedding, and plugin Box lifecycle boundary.
Related:
  - docs/architecture/RINGS.md
  - docs/development/current/main/design/ring1-core-provider-scope-ssot.md
  - docs/development/current/main/design/box-callable-registry-ssot.md
  - docs/development/current/main/design/type-abi-naming-and-box-descriptor-ssot.md
  - docs/development/current/main/design/type-abi-view-and-plan-stamp-ssot.md
  - docs/development/current/main/design/type-abi-box-domain-ssot.md
  - docs/development/current/main/design/box-lifecycle-cprime-terminal-home-finalization-ssot.md
  - docs/reference/language/lifecycle.md
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
  who owns provider-image lifetime, provider-global lifetime,
  terminal-Home fini, and structural destruction
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

ABI domain:
  Core C ABI | Provider Box ABI family

Provider Box binding profile:
  TypeBox-TLV v2 | TypedFast exact-entry

dispatch binding:
  library-generic | per-Box-table | exact-method-pointer | direct-symbol

optimization outcome:
  none-observed | devirtualized | inlined
```

| Provider | Ring | Residency | Provider Box binding | Dispatch binding | Optimization |
| --- | --- | --- | --- | --- | --- |
| Console core | ring1 | embedded | in-process core | direct | not claimed |
| Array core | ring1 | embedded | in-process core | direct | not claimed |
| Net extension | ring2 | dynamic | TypeBox-TLV v2 | library-generic | none observed |
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

## 4. Target Provider Box Binding Model

Ring2 keeps one semantic provider contract and two explicit physical binding
profiles:

```text
TypeBox-TLV v2 / current BID-FFI:
  generic, portable, dynamic, compatibility and tooling route

TypedFast exact-entry:
  load/build-time verified, exact-signature hot execution route
```

TypedFast is not a third public semantic ABI. It is an exact-entry binding
inside the Provider Box ABI family. Both profiles consume the same selected
ProviderSlot contract, provider identity, ownership/effect contract, and
lifecycle capability.

They consume the same immutable callable, signature, ownership, effect, and
lifecycle-capability truth. They must not maintain two independent policies.
Mutable instance-lifecycle state is not registry truth; it belongs to the host
lifecycle controller described below.

After admission, the production runtime has exactly three decision
authorities. `VerifiedProviderSlotContractV1` remains the cold semantic API
authority consumed by admission; it is not a fourth runtime selector.

```text
what may be called:
  admitted BoxCallableRegistry

how the selected callable is reached:
  one route-binding authority with two separate products:
    semantic RoutePlan
    RuntimeExecutablePlan {
      semantic_plan,
      plan_stamp,
      provider_image_pin,
      function_address,
    }

whether the instance may be called now:
  host lifecycle controller / future ObjectCell authority
```

The rest are inputs, projections, or physical mechanisms, not additional
decision authorities:

```text
PluginLoaderV2:
  observes and publishes provider image/export facts

ProviderAdmissionSeal:
  one-shot BoxCallableRegistry construction transaction
  verifies ABI/version/capabilities, init success, export collisions,
  signature/ownership/effect, and granted host capabilities
  publishes only an admitted registry or one typed rejection
  is not retained as a fourth runtime authority

BoxDescriptor / historical TypeAbi* views:
  read-only descriptor projections

Provider Box ABI family:
  selected external execution domain
  current TypeBox-TLV v2 or verified TypedFast exact-entry binding

runtime object substrate:
  physical ownership, lease drain, and reclamation mechanics
```

The minimal production path is:

```text
provider export facts
  -> one ProviderAdmissionSeal
  -> one admitted BoxCallableRegistry snapshot
  -> one semantic RoutePlan
  -> one RuntimeExecutablePlan
  -> one host lifecycle gate
  -> one selected invocation
```

`BoxCallableRegistry::seal` may physically implement `ProviderAdmissionSeal`,
but the admission checks must remain an explicit all-or-nothing stage before
registry publication.  Raw export facts are never callable registry truth.
`RoutePlan` contains IDs and route shape only; executable address, image pin,
and `PlanStamp` belong exclusively to `RuntimeExecutablePlan`.

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

## 6. ProviderSlot And Contract Authorities

`library = one API contract` is too coarse. A package may contain ordinary
Hako code and more than one independently replaceable capability. Selection
is therefore owned by a complete `ProviderSlot`, never by an entire package
and never by an individual method.

```text
package/library:
  public APIs, ordinary Hako implementation, dependencies

ProviderSlot:
  one versioned, complete replaceable capability contract

provider:
  one implementation of one ProviderSlot
```

V1 requires the complete method set of one slot to come from one provider.
Method-by-method provider mixing is rejected.

The durable products are deliberately separate:

```text
VerifiedProviderSlotContractV1:
  semantic API id/version/profile
  callable role, receiver, ordered parameter/result types
  parameter Home demands and result Home relation
  Result/Option/Fault mapping and effects
  suspension, thread-affinity, reentrancy, and lifecycle capabilities

AdmittedProviderImplementationV1:
  selected ProviderId/version
  exact implemented contract id and semantic profile
  provider export/binding table
  ABI version/capabilities and host grants
  cross-boundary memory owner/releaser law

RuntimeExecutablePlanV1:
  semantic RoutePlan
  exact ProviderImageId/artifact digest and image pin
  target/residency/binding profile
  exact function address/table/method id and PlanStamp
```

`BoxCallableRegistry` contains only admitted, selected rows. Candidate sets
belong to cold admission input and are never published into the live registry.
Each selected registry row co-seals its ProviderSlot contract identity,
selected provider identity, and callable target. It does not become a second
contract catalog.

Keep these identities distinct:

```text
semantic contract id/hash:
  API meaning and normalized semantic profile

wire signature hash:
  physical ABI compatibility only

ProviderId:
  semantic implementation identity

ProviderImageId / artifact hash:
  exact executable image, target, and build artifact
```

Dynamic and embedded artifacts may count as the same provider only when
ProviderId/version, ProviderSlot contract hash, and semantic profile match.
The executable plan and lock record still pin one exact ProviderImageId.

## 7. Selection And No-Retry Contract

An alternative compatibility transport may be selected before execution. It
is never selected as an after-failure retry.

The V1 binding epoch is either static build/link or one eager startup/load
transaction before application effects. Lazy call-time load and reselection
are outside V1. The generated lock record preserves the exact selection.

```text
production / exact AOT requiring TypedFast:
  missing or mismatched TypedFast -> build/load failure

explicit compatibility profile:
  select BID-TLV before execution

forbidden:
  TypedFast call fails -> invoke the same operation again through BID-TLV
```

This prevents duplicate effects and keeps the selected RoutePlan authoritative.

## 8. Plugin Box Lifecycle: Current Truth

The current `PluginBoxV2` uses `Arc<PluginHandleInner>`:

```text
share_box:
  share the same inner handle and instance_id

clone_box:
  currently assumes method zero is birth and creates a fresh inner handle

finalize_now / Drop:
  AtomicBool suppresses repeated fini calls for one inner handle
```

This provides a practical SharedV1 RAII model, but it is not the final C′ Box
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

## 9. Target C′ Lifecycle ABI

Provider lifecycle has four independent lifetimes:

```text
provider image lifetime:
  exact dynamic image pinned by plans, instances, and callbacks

provider-global lifetime:
  init before admission; shutdown only after plans/instances/callbacks drain

Box instance storage lifetime:
  birth creates storage; structural destroy reclaims it exactly once

C′ semantic lifetime:
  the last Home release alone enters the terminal fini transaction
```

Stateless/static capabilities such as scalar Math have no instance handle and
therefore no birth, fini, or structural destroy route.

For a stateful provider Box, the only accepted lifecycle is:

```text
Constructing
  successful birth -> Alive
  failed birth -> reverse release initialized children; no parent fini

Alive
  ordinary method under an admitted lease -> Alive
  terminal Home winner -> Finalizing

Finalizing
  reject new ordinary leases before provider execution
  drain already-issued leases
  invoke the non-callable provider fini hook at most once
  release host-owned fields in reverse declaration order
  invoke structural destroy exactly once
  -> PayloadDropped

PayloadDropped
  no ordinary method, fini, or destroy route remains callable
```

The C′ terminal Home DropPlan is the sole transition owner. Neither Rust
`Drop`, plugin code, a receiver call, nor provider-global shutdown may invoke
the user `fini` hook independently.

```text
birth:
  instance construction hook selected by the lifecycle contract

fini:
  non-callable, parameterless, non-suspending last-Home hook
  no Result channel, resurrection, hidden share, or receiver escape

destroy:
  structural native payload reclamation after fini and field release
  exactly once; never invokes fini

close / shutdown / commit / abort:
  ordinary domain methods; may return Result while the Box remains Alive
```

If a terminal hook or transport boundary faults, preserve the first terminal
failure, continue remaining field release and destroy best-effort, and never
republish Alive. A recoverable close failure belongs to an ordinary domain
method, not to `fini` or `destroy`.

Required invariants:

```text
one birth creates one structural instance identity
share adds a Home; it does not create another instance
clone creates a fresh instance only through the selected birth contract
only the terminal Home winner enters fini
new ordinary leases fail before provider execution once Finalizing begins
already-issued leases drain before fini
destroy occurs exactly once after the terminal teardown sequence
provider-global shutdown never substitutes for instance fini
static and dynamic artifacts of one provider obey the same C′ contract
```

The first lifecycle implementation must also fix clone through the selected
birth route, concurrent/reentrant terminalization, and the singleton
outstanding-share policy. `Arc::try_unwrap` success is not lifecycle policy.

## 10. Provider Image Lifetime And Identity

A dynamic executable address is legal only while its exact provider image is
alive. A copied function pointer is not a lifetime proof.

```text
sealed executable plan
  -> provider identity
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
`box_type` or `u32 instance_id` is not globally unique across providers.
Duplicate provider-scoped exports are either rejected deterministically or
selected by one sealed, explicit policy before execution. A provider
generation becomes necessary only if a later row enables reload; the initial
process-pinned model must not add an unused generation authority.

## 11. TypedFast Wire Contract Obligations

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

## 12. Structural Boundaries

Do not put plugin invocation or lifecycle policy into ring0.

Do not duplicate callable, signature, effect, or ownership truth in both the
BID and TypedFast implementations. Both transports consume a sealed plan.

Do not use historical `TypeAbiView`, `TypeAbiCatalog`, or `TypeAbiPack` as a
hot execution ABI. They remain descriptor projections.

Do not add unconditional fallback, by-name hot dispatch, or per-plugin
lifecycle exceptions.

Do not claim plugin C′ terminal-Home adoption until non-callable `fini` and
structural `destroy` have separate ABI owners and post-terminal use is
rejected at the common boundary.

## 13. Parked Implementation Task Order

The architecture does not need a rewrite. The following is one dependency
ordered future lane. It remains parked and does not change
`CURRENT_STATE.toml`, the current blocker, or the active MirBuilder lane.

### Mandatory reference closeout law

Every implementation or retirement row below must update the affected
`docs/reference/**` pages in the same implementation slice. A row may not
close with code and design docs only.

At minimum, update the applicable lifecycle, plugin ABI, ABI boundary,
manifest/user workflow, diagnostic, and migration reference pages. The final
closeout performs a zero-drift census; it is not a substitute for per-row
reference updates.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
reference_update_in_same_slice = required
```

### 0. `RING2-PROVIDER-BOUNDARY-CONTRACT0-D0` — accepted here

```text
Change:
  fix ProviderSlot selection, contract/provider/image identities,
  semantic/executable plans, C′ lifecycle, and no-retry boundaries
Contract:
  production/current-lane delta = 0; no new competing SSOT
Done:
  this document is the one Ring2 architecture and task-order authority
Stop:
  implementation remains parked until CURRENT_STATE explicitly reopens it
```

### 1. `MATH-PROVIDER-CONTRACT0-D0`

```text
Change:
  fix one complete hako.math.scalar@1 ProviderSlot as the first canary;
  decide source surface, method set, numeric and failure semantics
Contract:
  shared MathBox naming is not provider parity; exact semantic profile owns
  NaN/domain behavior, signed zero, rounding, precision, and Result/Fault
Done:
  root/plugin ID and behavior mismatches are explicit counterexamples and
  one contract is accepted for the canary
Stop:
  static/instance surface, method set, or special-value semantics remain
  ambiguous
```

Current evidence already rejects parity: the root app manifest uses Math IDs
4-7 while the plugin owns 1-4; builtin negative `sqrt` returns a String error
while the plugin follows floating NaN behavior; builtin `round` returns an
Integer while the plugin returns `f64`.

### 2. `PROVIDER-CONTRACT-ARTIFACT0-S0`

```text
Change:
  generate the normalized contract artifact, semantic/wire hashes, IDs,
  C header, Rust binding, and BoxDescriptor projection from one source
Contract:
  no hand-maintained duplicate method table; no runtime provider publication
Done:
  provider and host consume the same generated artifact
Stop:
  a second source authority or an ABI/runtime fallback is required
```

### 3. `RING2-PLUGIN-LIFECYCLE-CPRIME0`

```text
Change:
  separate birth, terminal-Home fini, reverse field release, structural
  destroy, provider-global shutdown, and image lifetime on the existing TLV
  route
Contract:
  C′ terminal Home owner is sole fini authority; Arc/refcount is not source
  Home authority
Done:
  Drop->fini, AtomicBool lifecycle truth, birth-zero clone, and
  Arc::try_unwrap shutdown policy are retired or quarantined
Stop:
  canonical terminal-Home receipt is unavailable, or a second lifecycle owner
  is needed
```

### 4. `RING2-CALLABLE-LINK-PLAN0`

```text
Change:
  make the existing TLV route the first live consumer of:

    provider facts -> ProviderAdmissionSeal
      -> immutable admitted BoxCallableRegistry
      -> semantic RoutePlan
      -> RuntimeExecutablePlan + exact image pin
      -> invoke exactly once

Contract:
  one selected ProviderSlot, one image, one executable route, no call-time
  re-selection or fallback
Done:
  collisions, per-call snapshot reconstruction, arity-zero lookup, and
  primary-plus-shim plans have zero production callers
Stop:
  caller-zero proof plan, second registry, or hot-path PlanStamp check appears
```

Reject duplicate/colliding exports before atomic publication. Retire mutable
config/spec reads after seal, provider identity loss, and unpinned addresses.

### 5. `MIRBUILDER-PROVIDER-CONTRACT-INPUT0`

```text
Change:
  replace MirBuilder CWD/nyash_box.toml reads with a resolved sealed callable
  contract input
Contract:
  MirBuilder does not own DLL paths, provider manifests, TypeBox name
  resolution, or provider selection
Done:
  plugin_sigs is no longer semantic authority and the sealed input is the
  only provider contract source
Stop:
  MirBuilder must reopen provider selection or manifest parsing
```

### 6. `RING2-TYPEDFAST0`

```text
Change:
  open BoxCount after rows 3-5: dynamic exact signature -> embedded table
  -> static direct symbol -> measured assembly/perf evidence
Contract:
  TypedFast failure never retries TLV; no hot name/registry/config lookup,
  TLV conversion, temporary allocation, or per-call PlanStamp check
Done:
  one exact dynamic canary and subsequent static rows have measured route
  evidence; lifecycle gates are accounted for
Stop:
  E_SHORT requires effectful re-invocation, or performance claim lacks a
  pre-edit perf/assembly baseline
```

```text
dynamic one exact signature
  -> embedded static table
  -> static direct symbol
  -> assembly/perf evidence
```

Static-direct absence is a build failure. State-aware lifecycle gates may
remain; one-indirect-call and inlining claims require measured evidence. Each
optimization cell starts by recording the exact executable perf and assembly
baseline before code or API edits.

Before general effectful TLV use, close the `E_SHORT` exactly-once hole by
choosing contract-sized preallocation, a provider-owned result handle, or an
effect-free copy phase. Reinvoking an effectful operation for buffer sizing is
forbidden.

### 7. `RING2-PROVIDER-CONFIG-SCHEMA0-D0`

```text
Change:
  freeze hako.toml/provider-manifest/lock workflow only after one exact
  executable canary
Contract:
  app manifest contains intent only; one provider manifest authority emits
  generated IDs/ABI/hashes; lock pins exact ProviderImageId
Done:
  filename/schema and nyash_box.toml/using kind=dylib migration are explicit
Stop:
  schema needs a third manifest authority or runtime provider discovery
```

```text
hako.toml:
  dependency and explicit provider override/link intent only
provider authoring manifest:
  one source authority; exact filename selected here
generated artifact manifest:
  IDs, ABI, hashes, artifacts
hako.lock:
  ProviderSlot/ProviderId/ProviderImageId, hashes, target, residency, binding
```

Do not create a third manifest authority beside existing provider package
artifacts and migration input. `nyash_box.toml` and `using kind="dylib"` become
named compatibility inputs with retirement conditions.

### 8. `MATH-PROVIDER-CUTOVER0`

```text
Change:
  publish pure Hako, intrinsic/native, and external Math implementations
  only after exact ProviderSlot parity
Contract:
  select one complete provider before effects; optimizer cannot change
  ProviderId implicitly
Done:
  types, errors, special values, ownership, results, and generated IDs match
Stop:
  any provider needs semantic exception, fallback, or hidden identity change
```

### 9. `RING2-COMPAT-REFERENCE-CLOSEOUT0-G0`

```text
Change:
  perform the final caller-zero and docs/reference drift census
Contract:
  every implementation slice already updated its affected references;
  this row only proves no residue was missed
Done:
  all listed compatibility callers and old lifecycle claims are zero
Stop:
  any legacy route still owns production semantics
```

Require zero production callers for:

```text
per-call registry snapshot reconstruction
runtime name and arity-zero compatibility resolution
primary-plus-shim executable plans
hand-maintained app/plugin method IDs
MirBuilder manifest reads
using kind="dylib"
Plugin Drop/finalize_now -> user fini
old B′ provider lifecycle claims
```

Also finish the TypeBox four-surface naming census and verify that every
implemented behavior is reflected in `docs/reference/**`, examples,
diagnostics, and migration guidance. General lazy load, unload, hot reload,
and provider generation remain a later independent D0.

Keep BoxCount and BoxShape separate. Use one rolling workstream card when the
lane reopens; do not create one document or guard per subtask.

## 14. Hard Stops

```text
classifies direct-symbol as an ABI transport
classifies LTO/inlining as a link or ABI guarantee
uses TypeAbi* descriptor projections as hot callable truth
calls the current per-Box TLV invoke path TypedFast

keeps mutable lifecycle legality in BoxCallableRegistry or the plugin
returns Finalizing/partially torn-down state to Alive after terminal failure
calls user fini from Rust Drop or structural destroy
lets use-after-fini reach the plugin before host rejection
uses hard-coded method zero as clone birth

stores a dynamic function pointer without an exact provider image pin
allows same-identity reload while old plans/instances exist
drops provider identity from callable/object identity
adds provider generation while reload remains unsupported
checks PlanStamp on every hot call

calls a version field negotiation without major/minor/capability checks
lets Rust panic or C++ exception cross the ABI
frees provider memory with the host allocator or vice versa
treats a signature hash as trust or authorization
uses manifest thread_safe alone as host concurrency proof

retries one effectful operation through BID after TypedFast failure
reinvokes an effectful TLV operation after `E_SHORT`
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
- Current plugin lifecycle is not yet C′ complete.
- Dynamic provider pointers are not yet protected by a sealed image-lifetime
  contract.
- `NyashTypeBoxFfi.version` and `capabilities` are not currently proof of
  negotiated TypedFast compatibility.
- General plugin unload and hot reload are not accepted capabilities.
- This document does not change the active MirBuilder lane or authorize code
  implementation from the current design-stop state.
