# Lifecycle kernel artifact

This staticlib owns the selected normalized-status process `main`. The kernel
core owns startup, runtime state and flushing. The generated `ny_main` runs once;
its `0..=255` status passes through, and any other value becomes 70. This entry
never interprets a status as a host handle.

Build the legacy kernel first with its documented default-feature command,
then build this artifact separately:

```sh
CARGO_BUILD_JOBS=4 cargo build -p nyash_lifecycle_kernel --release --target-dir target/lifecycle-kernel
python3 crates/nyash_lifecycle_kernel/tests/launcher.py target/lifecycle-kernel/release/libnyash_lifecycle_kernel.a target/release/libnyash_kernel.a
```

`legacy-entry` and `lifecycle-core` cannot be enabled together. Generic workspace
builds exclude this package. Its private target directory keeps a core-only
`libnyash_kernel.a` from overwriting the legacy artifact.

The entry owner emits `.nyash.entry_abi.v1`: 16 bytes consisting of `NYENTRY1`,
little-endian revision 1 and normalized-i64 entry kind 1. The runtime descriptor
remains the core's independent layout authority. Host admission must inspect
both records in the exact archive it retains for linking.

The launcher test links this real archive to an ABI stub. It proves the native
entry boundary only, not constructor LLVM emission or Pair source execution.

The optional second test argument checks the legacy archive's symbols and its
unchanged status/truncation behavior. Entry-record rejection tests live with
`src/host_providers/llvm_codegen/runtime_abi_descriptor_tests.rs`; the ignored
actual-archive test requires both release outputs above. Mixed-feature rejection
is exercised by `cargo build -p nyash_kernel --release --features lifecycle-core
--target-dir target/lifecycle-kernel` (expected compile error, not a build recipe).
