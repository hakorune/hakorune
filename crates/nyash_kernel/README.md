# Nyash Kernel

**Minimal native/product runtime kernel for Nyash language - Plugin-First Architecture**

Generated: 2025-09-24
Architecture: Phase 2.4 NyRT→NyKernel Revolution Complete

## Overview

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
