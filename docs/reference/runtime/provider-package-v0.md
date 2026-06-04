# Hakorune Provider Package v0

Status: Active
Scope: user-facing provider package command, current staged build modes,
output layout, and smoke/preflight path.
Related:
- docs/development/current/main/design/provider-abi-v1-ssot.md
- docs/development/current/main/design/provider-package-artifact-ssot.md
- docs/development/current/main/design/provider-runtime-load-ssot.md

## Purpose

Provider package v0 turns a shared-library provider artifact into a Hakorune
provider package. The package is the formal artifact. The shared library is
only one file inside that package.

The current implementation has staged build modes:

| Mode | Status | Boundary |
| --- | --- | --- |
| existing-binary package | supported | packages an already-built `.so` / `.dll` / `.dylib` with manifest metadata |
| selected fixture provider build | supported | Hakorune-owned selected fixture only; no arbitrary shell build |
| selected `.hako`-derived provider package | supported | selected fixtures only; not arbitrary `.hako` to DLL lowering |
| semantic provider codegen | supported narrowly | only listed semantic modes are valid |
| metadata/load/API/call smokes | supported by tools | package command itself stays no-load |
| process allocator replacement | closed | no activation, hooks, or global allocator integration |
| provider-backed `LD_PRELOAD` | closed | current `LD_PRELOAD` shim lane is probe-only unless a later row opens replacement |

Do not read `.hako`-derived package support as arbitrary `.hako` to allocator
DLL generation. Each semantic mode is a narrow selected entrypoint contract.
The package command itself does not activate the provider, replace the process
allocator, install hooks, use global allocator integration, or claim benchmark
winners.

## Package Command

Build the Hakorune CLI and package an existing shared library:

```bash
cargo build --bin hakorune

target/debug/hakorune \
  --provider-package-existing-binary ./libhakorune_provider.so \
  --provider-package-out-dir dist/hakorune-provider \
  --provider-package-id org.hakorune.provider.mimalloc.v0 \
  --provider-package-name mimalloc-explicit \
  --provider-package-target-triple x86_64-unknown-linux-gnu \
  --provider-package-platform linux
```

`hakorune` is the current CLI binary name. `nyash` may still exist as a
legacy-compatible alias in some local builds, but new provider package docs and
automation should use `hakorune`.

The command emits this stable report contract:

```text
output_contract=hakorune-provider-package-existing-binary-manifest-v0
package_mode=existing-binary-manifest
schema_version=hakorune-provider-package-v1
shared_library_load_executed=0
required_export_resolved=0
descriptor_read_executed=0
provider_call_executed=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Required Flags

```text
--provider-package-existing-binary FILE
--provider-package-out-dir DIR
--provider-package-id ID
--provider-package-name NAME
--provider-package-target-triple TRIPLE
--provider-package-platform PLATFORM
```

## Optional Flags

```text
--provider-package-artifact-name FILE
--provider-package-kind KIND
--provider-package-version VERSION
--provider-package-profile speed|diagnostic
--provider-package-provider-call-allowed
--provider-package-force
```

Defaults:

```text
provider_kind=allocator
provider_version=0.1.0
profile=speed
provider_call_allowed=0
required_exports=hakorune_provider_descriptor_v1
capabilities=descriptor,explicit_allocator_api
```

`--provider-package-provider-call-allowed` only changes the manifest activation
policy for explicit provider-call rows. It does not activate the provider and
does not allow process allocator replacement, hooks, or global allocator use.

## Output Layout

```text
dist/hakorune-provider/
  hakorune_provider.json
  hakorune_provider.sha256
  libhakorune_provider.so | hakorune_provider.dll | libhakorune_provider.dylib
```

The artifact name defaults to the input file name. Use
`--provider-package-artifact-name` to normalize it for distribution.

## Manifest Contract

The generated manifest uses:

```text
schema_version=hakorune-provider-package-v1
abi_version=hakorune-provider-abi-v1
artifact.path=<package-relative shared library>
artifact.sha256=<sha256 of artifact bytes>
artifact.size_bytes=<artifact byte size>
contract_hash=<normalized provider contract hash>
required_exports=hakorune_provider_descriptor_v1
activation.allocator_replacement_allowed=false
activation.hook_allowed=false
activation.global_allocator_allowed=false
```

The manifest path is the only discovery input. Hakorune must not search
`PATH`, `LD_LIBRARY_PATH`, the current working directory, or platform loader
fallback locations for an unlisted provider binary.

## Metadata Preflight

Validate the generated manifest without loading the shared library:

```bash
python3 tools/allocator/provider_package_metadata_preflight.py \
  --manifest dist/hakorune-provider/hakorune_provider.json
```

Expected preflight stop line:

```text
output_contract=hakorune-provider-package-metadata-preflight-v0
dll_mode=metadata-preflight
manifest_ready=1
descriptor_ready=0
binary_hash_ready=1
shared_library_load_executed=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Current Smoke Ladder

Use this ladder to verify the package path without treating it as allocator
replacement.

```bash
python3 tools/allocator/provider_package_metadata_preflight.py \
  --manifest dist/hakorune-provider/hakorune_provider.json

python3 tools/allocator/provider_package_descriptor_smoke.py \
  --manifest dist/hakorune-provider/hakorune_provider.json

python3 tools/allocator/provider_package_api_bind_smoke.py \
  --manifest dist/hakorune-provider/hakorune_provider.json

python3 tools/allocator/provider_package_alloc_free_smoke.py \
  --manifest dist/hakorune-provider/hakorune_provider.json
```

The smokes may load the shared library and call explicit provider APIs. They do
not make the provider active as the process allocator:

```text
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

The `LD_PRELOAD` shim lane is separate and currently probe-only. It may prove
that a shim loads and delegates through the platform loader, but it is not a
provider-backed allocator replacement contract.

## Provider Front vs Replacement Front

Allocator-facing packages have two different fronts:

```text
HakoAllocProviderFront:
  explicit provider boundary
  descriptor / manifest / ABI table / safe loader path
  good for plugin/provider calls
  intentionally thicker

HakoAllocReplacementFront:
  future thin malloc/free ABI boundary
  benchmark/product replacement candidate only after a dedicated activation row
  must not dispatch through the provider API table on the hot path
```

Do not optimize the Provider Front by weakening its safety boundary. If C-like
allocator hot-path thinness is required, create a separate Replacement Front
whose report proves:

```text
provider_table_dispatch=0
function_pointer_hot_call=0
owns_check_hot_path=0
tracking_hot_path=0
direct_core_call=1
activation=0
benchmark_only=1
summary=ok
```

Until a dedicated activation row opens it, the Replacement Front is a benchmark
front only. It is not a provider activation, hook install, global allocator, or
winner claim.

Native-slot provider packages may export the private symbol
`hakorune_provider_usable_size_v0` for shim-cost measurement. This symbol is
not part of `hakorune-provider-abi-v1`, is not listed in the provider API
table, and must not be used as a product allocator contract. The current
LD_PRELOAD shim can opt into this path with
`HAKORUNE_PROVIDER_LDPRELOAD_USE_USABLE_SIZE=1` to bypass shim-side pointer
tracking and classify whether size tracking or the provider call boundary owns
the remaining cost. A stricter measurement-only mode,
`HAKORUNE_PROVIDER_LDPRELOAD_ASSUME_PROVIDER_OWNED=1`, may be combined with
usable-size mode to skip provider `owns` checks before `free`/`realloc`; this is
only valid for controlled benchmarks where all hot pointers are provider-owned.

## Phase B1 Selected Fixture Build

Phase B1 adds one Hakorune-owned selected build producer:

```bash
target/debug/hakorune \
  --provider-package-selected-binary-build-fixture \
  --provider-package-out-dir dist/hakorune-provider \
  --provider-package-id org.hakorune.provider.selected.fixture \
  --provider-package-name selected-fixture-provider \
  --provider-package-target-triple x86_64-unknown-linux-gnu \
  --provider-package-platform linux
```

This command builds the selected fixture provider binary and then emits the
same package layout:

```text
output_contract=hakorune-provider-package-selected-binary-build-v0
package_mode=selected-binary-build-package
build_mode=selected-fixture
build_command_executed=1
hako_shared_library_generation=0
arbitrary_shell_build_executed=0
shared_library_load_executed=0
required_export_resolved=0
descriptor_read_executed=0
provider_call_executed=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

The selected fixture build is not full `.hako` to shared-library generation.
It must not run arbitrary user shell commands or discover provider binaries
through `PATH`, `LD_LIBRARY_PATH`, the current working directory, or platform
loader fallback locations.

## Phase C0 .hako-Derived Fixture Build

Phase C0 adds the first selected `.hako` source producer:

```bash
target/debug/hakorune \
  --provider-package-hako-derived-build-fixture apps/provider-package/hako-derived-allocator-fixture/main.hako \
  --provider-package-out-dir dist/hakorune-provider \
  --provider-package-id org.hakorune.provider.hako.fixture \
  --provider-package-name hako-derived-fixture-provider \
  --provider-package-target-triple x86_64-unknown-linux-gnu \
  --provider-package-platform linux
```

This command verifies the `.hako` source path, emits MIR JSON, records the
source and MIR JSON hashes in package metadata, then builds a Hakorune-owned
provider ABI wrapper artifact:

```text
output_contract=hakorune-provider-package-hako-derived-build-v0
package_mode=hako-derived-provider-package
build_mode=hako-derived-selected-fixture
hako_source_checked=1
hako_mir_json_emitted=1
hako_semantic_provider_codegen=0
shared_library_artifact_generated=1
shared_library_load_executed=0
required_export_resolved=0
descriptor_read_executed=0
provider_call_executed=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

`hako_semantic_provider_codegen=0` means the package is tied to real `.hako`
source/MIR evidence, but native provider entrypoint semantics are still supplied
by the wrapper until a later semantic-codegen row opens.

The first semantic-codegen mode is `ping-literal-v0`:

```bash
target/debug/hakorune \
  --provider-package-hako-derived-build-fixture apps/provider-package/hako-derived-allocator-fixture/main.hako \
  --provider-package-hako-semantic-codegen ping-literal-v0 \
  --provider-package-out-dir dist/hakorune-provider \
  --provider-package-id org.hakorune.provider.hako.fixture \
  --provider-package-name hako-derived-fixture-provider \
  --provider-package-target-triple x86_64-unknown-linux-gnu \
  --provider-package-platform linux \
  --provider-package-provider-call-allowed
```

This mode maps only `.hako` `HakoProvider.ping/0 -> i64 literal` into the
provider `hako_ping()` entrypoint:

```text
hako_semantic_provider_codegen=ping-literal-v0
hako_provider_ping_codegen=1
hako_provider_ping_value=7
provider_noop_call_result=7
```

Allocator entrypoints remain wrapper-owned until a later semantic-codegen row.

The first allocator-entrypoint semantic-codegen mode is
`alloc-free-owns-literal-v0`:

```bash
target/debug/hakorune \
  --provider-package-hako-derived-build-fixture apps/provider-package/hako-derived-allocator-fixture/main.hako \
  --provider-package-hako-semantic-codegen alloc-free-owns-literal-v0 \
  --provider-package-out-dir dist/hakorune-provider \
  --provider-package-id org.hakorune.provider.hako.fixture \
  --provider-package-name hako-derived-fixture-provider \
  --provider-package-target-triple x86_64-unknown-linux-gnu \
  --provider-package-platform linux \
  --provider-package-provider-call-allowed
```

This mode maps `.hako` `HakoProvider.ownsAllocated/0 -> i64 literal 0|1`
into provider `hako_owns(non_null_ptr)`, then verifies it through explicit
provider alloc/free smoke:

```text
hako_semantic_provider_codegen=alloc-free-owns-literal-v0
hako_provider_owns_codegen=1
hako_provider_owns_value=1
provider_alloc_executed=1
provider_free_executed=1
provider_owns_result=1
allocator_entrypoint_called=1
```

Native pointer allocation/free mechanics still belong to the generated wrapper
in this mode. Provider activation and process allocator replacement remain
closed.

The selected object-lifecycle semantic-codegen mode is
`object-lifecycle-small-alloc-release-v0`:

```bash
target/debug/hakorune \
  --provider-package-hako-derived-build-fixture apps/provider-package/hako-derived-mimalloc-real-entrypoint-fixture/main.hako \
  --provider-package-hako-semantic-codegen object-lifecycle-small-alloc-release-v0 \
  --provider-package-out-dir dist/hakorune-provider \
  --provider-package-id org.hakorune.provider.hako.mimalloc.real-entrypoint \
  --provider-package-name hako-mimalloc-real-entrypoint-provider \
  --provider-package-target-triple x86_64-unknown-linux-gnu \
  --provider-package-platform linux \
  --provider-package-provider-call-allowed
```

This mode verifies the selected `.hako` mimalloc entrypoint call chain:

```text
HakoProvider.objectLifecycleSmallAllocReleaseOk/0
  -> HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
  -> HakoAllocPageModel.acquireFreshSmall/1

HakoProvider.objectLifecycleSmallAllocReleaseOk/0
  -> HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
  -> HakoAllocPageModel.releaseLocalKnownLive/1
```

The generated provider ABI wrapper still owns native pointer allocation/free in
this mode. The `.hako` side is used as selected source/MIR evidence for the
real object-lifecycle entrypoint; it is not arbitrary `.hako` to allocator-DLL
lowering, and it does not activate provider replacement.

The first experimental object-lifecycle native bridge mode is
`object-lifecycle-native-slot-bridge-v0`:

```bash
target/debug/hakorune \
  --provider-package-hako-derived-build-fixture apps/provider-package/hako-derived-mimalloc-real-entrypoint-fixture/main.hako \
  --provider-package-hako-semantic-codegen object-lifecycle-native-slot-bridge-v0 \
  --provider-package-out-dir dist/hakorune-provider \
  --provider-package-id org.hakorune.provider.hako.mimalloc.real-entrypoint \
  --provider-package-name hako-mimalloc-real-entrypoint-provider \
  --provider-package-target-triple x86_64-unknown-linux-gnu \
  --provider-package-platform linux \
  --provider-package-provider-call-allowed
```

This mode still verifies the selected `.hako` object-lifecycle call chain, but
the generated provider `alloc/free/owns` entrypoints use a native static slot
bridge derived from that selected lifecycle shape:

```text
hako_semantic_provider_codegen=object-lifecycle-native-slot-bridge-v0
hako_provider_object_lifecycle_entrypoint_verified=1
hako_provider_alloc_free_route=native_static_slot_bridge_from_object_lifecycle_shape
hako_provider_alloc_free_uses_host_malloc=0
hako_provider_alloc_free_uses_hako_object_lifecycle=1
hako_provider_object_lifecycle_entrypoint_usage=native_shape_codegen
```

This is an explicit provider-package experiment. It is not product allocator
activation, not hook installation, not global allocator selection, and not a
winner claim. The bridge is intentionally narrow: it provides a fixed native
slot arena for provider API smoke and LD_PRELOAD bridge evidence before the
full `.hako` object-lifecycle lowering is connected.

## Provider ABI Claim Operations

The v0 compatibility API still exposes:

```text
alloc
free
owns
```

Allocator-provider measurements should not treat `owns + free` plus shim-side
tracking as the long-term hot ownership boundary. The forward ABI direction is
claim-style operations where the provider owns provider-pointer lifecycle truth:

```text
free_claim(ptr) -> handled | not_owned
usable_size_claim(ptr, out_size) -> owned(size) | not_owned
realloc_claim(ptr, new_size, out_ptr) -> handled(ptr) | not_owned | failed
```

Current generated providers append `free_claim`, `usable_size_claim`, and
`realloc_claim` as optional tail entries after the compatibility fields. Existing
`alloc/free/owns` remain supported for compatibility, while LD_PRELOAD shim
mainline may prefer claim operations when the tail entries are present.

`usable_size_claim` is route-specific. Host-backed adapters currently return
`not_owned` because they do not own host usable-size truth until a future
`HostAllocatorV0` row. Native-slot generated providers return owned requested
size for provider-owned slots.

`realloc_claim` is route-specific for the same reason. Host-backed adapters
currently return `not_owned` until `HostAllocatorV0` supplies host realloc
truth. Native-slot generated providers handle provider-owned pointers in place
when the new size still fits the fixed slot, return null handled for size zero,
and report failed for oversized provider-owned realloc requests.

Report fields:

```text
provider_allocator_kind=pure_allocator|host_backed_adapter
provider_abi_claim_ops_v1=1
provider_free_claim_enabled=1
provider_realloc_claim_enabled=0|1
provider_usable_size_claim_enabled=0|1
compat_alloc_free_owns_still_supported=1
compat_owns_free_mainline=0
host_allocator_vtable_init=0
```

`HostAllocatorV0` is a future row. Host-backed adapters must eventually receive
host allocator operations through an explicit vtable instead of depending on
LD_PRELOAD symbol reentry or nonportable libc-private symbols.

## v0 Stop Line

Provider package v0 is complete when the CLI creates the package and metadata
preflight accepts the generated manifest.

Still closed in v0:

```text
arbitrary .hako-to-provider semantic lowering
shared-library load by the package command
export resolution by the package command
descriptor read by the package command
provider API bind by the package command
explicit allocator call by the package command
provider activation
process allocator replacement
hook installation
global allocator integration
benchmark winner claims
```
