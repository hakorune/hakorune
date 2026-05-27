# Hakorune Provider Package v0

Status: Active
Scope: user-facing provider package v0 command, output layout, and no-load preflight path.
Related:
- docs/development/current/main/design/provider-abi-v1-ssot.md
- docs/development/current/main/design/provider-package-artifact-ssot.md
- docs/development/current/main/design/provider-runtime-load-ssot.md

## Purpose

Provider package v0 turns an existing shared library into a Hakorune provider
package artifact. The package is the formal artifact. The shared library is
only one file inside that package.

v0 does not compile `.hako` into a shared library. It does not load the shared
library, resolve exports, read descriptors, call provider APIs, activate a
provider, replace the process allocator, install hooks, use global allocator
integration, or claim benchmark winners.

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
