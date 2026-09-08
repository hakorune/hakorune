# Nyash Kernel

## Checked Map residence dependency

`exports/checked_map_residence.rs` privately prepares a SafeMutex indexed-object
residence after exact profile/type/identity validation. It does not clone or move
the indexed payload. Rejected-install wrapper disposal leaves the prior owner's
payload intact; consuming end invokes existing indexed reclaim once. TLS and
unsupported profiles refuse before residence creation. The process-wide store
provides the residence's static lifetime.

The eleven checked Map exports are implemented in `fault_checked_map.rs`;
`include/nyrt_fault_v1.h` declares them. Map/key/outcome regions use target-issued
opaque layouts and explicit consumption/disposal. Frame recording happens after
end callbacks. Host publication and compiler activation remain excluded. The prepare caller must have
source transfer/end authorization; physical identity validation cannot issue it.
See the [runtime contract](../../docs/reference/runtime/runtime-data-dispatch.md#checked-map-storage-and-indexed-residence).

## GC diagnostic output

Entry metrics consume the controller's finite last-completed reachability
observation. NonComplete results have null trial counts and explicit
status/reason, including unavailable controller; they never reuse old successful
counts. Text output also carries status/reason. See the
[GC reference](../../docs/reference/runtime/gc.md#native-reachability-observation).

## Primitive Array append

`array_compat::append_integer_raw` and the i64/Bool/F64 arms of
`array_slot_append::array_slot_append_any` delegate to ArrayBox atomic append.
The Array state owner determines the end position, validates and commits under
one write lock, returning committed length or zero on rejection. Kernel callers
do not pre-read length or compute a second result. Shared concurrent appends
preserve each committed element and return distinct positions.

This does not change generic value decoding or boxed/string/alternate storage
routes, and does not activate the typed Script C consumer. The [runtime contract](../../docs/reference/runtime/runtime-data-dispatch.md#primitive-array-state-write-outcomes)
owns the outcome and failure policy.


### Checked native Array ABI

`exports/fault_checked_array.rs` exports `nyash.array.checked_new_v1`,
`checked_claim_v1` and `checked_append_{i64,bool,f64}_v1`.
The existing Array claim/atomic append owns validation and state; the existing
FaultFrame owns diagnostics. Direct Array access avoids generic value decoding,
env-selected storage and name-based allocation. New publishes a nonzero handle
only on Normal. Native void handle release remains the residence owner.

[The runtime reference](../../docs/reference/runtime/runtime-data-dispatch.md#checked-native-array-abi-v1)
defines exact tags, payload lanes, reason/details and Normal/Fault/InvalidContract.
[The C header](../../include/nyrt_fault_v1.h) declares the same physical contract.
These exports are runtime dependency evidence; Script final input and selected
C/OBJ/EXE integration remain separate tasks.

**Minimal native/product runtime kernel for Nyash language - Plugin-First Architecture**

Generated: 2025-09-24
Architecture: Phase 2.4 NyRT→NyKernel Revolution Complete

## Overview

### Typed lifecycle read boundary

`nyash.object.checked_field_get_i64_v1` reads an exact indexed i64 field using
the caller's admitted storage profile, object type and slot. The same storage
owner validates identity and reads under one guard. Normal writes the out-slot;
InvalidContract leaves it unchanged, with no zero substitution, storage fallback
or source Fault. Supported profiles are SafeMutex and SingleThreadExact.
The trusted pointer contract is in `include/nyrt_fault_v1.h`; the ABI alone does
not activate a compiler/backend lifecycle consumer.

### Process entry artifacts

The default `legacy-entry` feature exports the compatibility `main`, including
its existing positive-handle decoding. The `lifecycle-core` feature instead
exposes the common startup/flush launcher to the separate
[`nyash_lifecycle_kernel`](../nyash_lifecycle_kernel/README.md) staticlib.
Enabling both features is a compile error; Cargo feature merging cannot silently
change the selected entry. Disabling both exports no process entry.

Build the legacy archive with `cargo build -p nyash_kernel --release`. Build the
lifecycle package separately in its documented target directory. Generic
workspace builds exclude `nyash_lifecycle_kernel`. Both artifacts reuse one
runtime implementation; only the lifecycle entry carries its normalized-status
ABI record. Its host session verifies that record and this kernel's runtime
layout descriptor from the same selected archive before object emission.

### Target-compiled ABI descriptor

`libnyash_kernel.a` retains exactly one `.nyash.runtime_abi.v2` ELF section.
Its fixed 236-byte little-endian record is emitted while compiling this target,
not inferred by the host. It records the Cargo target triple, pointer width,
Fault/status ABI revisions, and the Fault Diagnostic/Frame sizes, alignments,
and field offsets, followed by Map/key/outcome size/alignment/revision triples.
The descriptor export is `nyash_runtime_abi_descriptor_v2`; Fault/status ABI
remain revision1. A host reader must select this named section from the exact
archive and reject missing, duplicate, truncated, unsupported, or inconsistent
records before lifecycle object emission. Archive reading alone does not prove
that this ABI matches the selected LLVM session; that equality belongs to the
lifecycle invocation owner.

The Nyash Kernel (`nyash_kernel`) is the minimal native/product runtime core that replaced the legacy NyRT system. This represents a **42% reduction** in runtime complexity by moving from VM-dependent architecture to a unified Plugin-First system.

Scope note:
- `nyash_kernel` is the only surface in this repo that should be called `kernel`.
- `lang/src/vm` is a VM/reference cluster, not a product kernel.

## Architecture Revolution

### ✅ **From NyRT to NyKernel** (Phase 2.4 Complete)

**Before (Legacy NyRT)**:
- Mixed VM/Plugin dependencies
- `with_legacy_vm_args` scattered throughout codebase
- 58% essential + 42% deletable functions
- Complex shim layer for LLVM integration

**After (NyKernel)**:
- Pure Plugin-First architecture
- Zero legacy VM dependencies
- Only essential kernel functions remain
- Clean C ABI for LLVM integration

### 🏗️ **Core Components**

#### Essential Kernel Functions (58% - Kept)
- **GC Management**: Safepoints, write barriers, memory management
- **Handle Registry**: Object handle management for AOT/JIT
- **Plugin Host**: Unified plugin loading and method resolution
- **Process Entry**: Main entry point and runtime initialization

#### Birth Shim Surface (Kept)

The old "birth.rs removed" note was stale. `birth.rs` is still present as the
handle-based birth shim surface for AOT/JIT exports.

- `birth.rs` - handle-based birth exports kept for AOT/JIT linkage
- `with_legacy_vm_args` - legacy VM argument processing removed
- String/Box operation shims - moved to current kernel/plugin surfaces
- VM-specific encoding functions - removed from the legacy shim layer

#### Plugin Dispatch Compat Surface

`src/plugin/invoke/by_name.rs` restores the compat-only
`nyash.plugin.invoke_by_name_i64` export for stage1/bootstrap module-string
dispatch. It is a bridge surface, not the daily direct path, so callers that
already have a resolved direct callee should stay on the direct route and
reserve this export for the fallback lane.

Note:
- the `invoke.rs` row in the historical removal table should be read as an old
  removal snapshot, not as the current file presence
- the current plugin dispatch compat surface lives in `src/plugin/invoke.rs`
  and `src/plugin/invoke/by_name.rs`

## Build Output

```
Target: libnyash_kernel.a (static library)
Status: Clean build (0 errors, 0 warnings)
Integration: LLVM + VM unified
```

## Implementation Details

### Current Birth Shim Surface

| File | Locations | Status |
|------|-----------|---------|
| `encode.rs` | 1 | ✅ Removed |
| `birth.rs` | 1 | ✅ Kept as export shim |
| `future.rs` | 2 | ✅ Removed |
| `invoke.rs` | 6 | ✅ Removed |
| `invoke_core.rs` | 1 | ✅ Removed |
| **Total** | **11** | **Historical removal set; birth shim kept** |

### Plugin-First Integration

All Box operations now route through the unified plugin system:

```rust
// Before: VM-dependent
with_legacy_vm_args(|args| { ... })

// After: Plugin-First
let host = get_global_plugin_host().read()?;
host.create_box(type_name, &args)?
```

### 🔥 **ExternCall Print修正** (codex技術力)

**Phase 2.4で解決した重大問題**: LLVM EXEで`print()`出力されない

#### 問題の詳細
- **症状**: VM実行は正常、LLVM EXEは無音
- **根本原因**: `src/llvm_py/instructions/externcall.py`の引数変換バグ
- **技術詳細**: 文字列ハンドル→ポインタ変換後にnull上書き

#### 修正内容
```python
# src/llvm_py/instructions/externcall.py:152-154
else:
    # used_string_h2p was true: keep the resolved pointer (do not null it)
    pass
```

#### 検証結果
```bash
/tmp/direct_python_test_fixed
# 出力:
# 🎉 ExternCall print修正テスト！
# codex先生の名前解決修正確認
# Result: 0
```

## Usage

### For LLVM Backend
```bash
# Build with LLVM integration
cargo build --release -p nyash_kernel
# Output (workspace default):
#   target/release/libnyash_kernel.a
# Note: if you set `CARGO_TARGET_DIR`, the output is under `$CARGO_TARGET_DIR/release/`.
```

Notes:
- `libnyash_kernel.a` is required for **native executable linking** (AOT/`--emit-exe`/`ny-llvmc --emit exe`).
- The Python llvmlite **explicit compat/probe keep lane** (`NYASH_LLVM_USE_HARNESS=1`) does not require the static library.

### For VM-family integration
```bash
# Explicit keep/reference integration (automatic when those lanes are selected)
./target/release/hakorune program.hako
```

## Design Philosophy

**"Everything is Plugin"** - The kernel provides only the essential infrastructure for plugin management, leaving all Box implementations to the plugin system.

### Core Principles
1. **Minimal Surface**: Only GC, handles, plugins, and process entry
2. **Plugin-First**: All Box operations through unified plugin host
3. **C ABI Clean**: Stable interface for LLVM/native executable integration
4. **Zero Legacy**: Complete removal of VM-dependent code paths

## ChatGPT5 × codex × Claude Collaboration

This kernel represents a historic achievement in AI-assisted architecture design:
- **Design**: ChatGPT5 Pro architectural analysis (42% reduction strategy)
- **Implementation**: Claude systematic implementation (11 locations)
- **Debugging**: codex root cause analysis (ExternCall print fix)
- **Result**: 100% successful architecture revolution + critical bug resolution

## Integration

The Nyash Kernel integrates with:
- **LLVM/native executable route**: Static linking via libnyash_kernel.a
- **runtime/plugin host surfaces**: Dynamic plugin loading where the product runtime needs it
- **Build System**: tools/build_llvm.sh integration complete

---

*Part of Phase 15 Nyash Self-hosting Revolution*
*Documentation: [ChatGPT5 NyRT→NyKernel Design](../../docs/private/roadmap2/phases/phase-15/chatgpt5-nyrt-kernel-design.md)*
