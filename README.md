# 🐱 Hakorune Programming Language (formerly Nyash)
> Note: the project binary and user‑visible brand have been renamed to “Hakorune”.
> The legacy `nyash` binary is deprecated (use `hakorune`). Config prefers `hako.toml` (fallback: `nyash.toml`). In scripts and docs, prefer `$NYASH_BIN` which points to `target/release/hakorune` when available.
**A Seriously-Crafted Hobby Language**  
**From Zero to Native Binary in 20 Days - The AI-Powered Language Revolution**

Quick — Emit MIR (Hako‑first helper)
- Generate MIR(JSON) from a Hako file using the Stage‑B parser + MirBuilder (wrapper falls back to the Rust CLI builder on failure to keep runs green):

```
tools/hakorune_emit_mir.sh path/to/program.hako /tmp/program.mir.json
target/release/hakorune --mir-json-file /tmp/program.mir.json
```

Notes
- The wrapper runs Stage‑B with `NYASH_JSON_ONLY=1` to keep the output clean (no `RC:` lines).
- When the Hako MirBuilder fails (e.g., under development), it automatically falls back to the explicit compat bridge `--program-json-to-mir` (no behavior change by default).

[Performance quickstart]
- MIR emit bench (Stage‑B → MIR(JSON))
  - `tools/perf/bench_hakorune_emit_mir.sh apps/examples/json_query/main.hako 5`
  - Env toggles: `HAKO_USING_RESOLVER_FIRST=1`, `HAKO_SELFHOST_BUILDER_FIRST=1`
- MIR(JSON) → obj/exe bench（ny-llvmc/crate）
  - `NYASH_LLVM_BACKEND=crate tools/perf/bench_ny_mir_builder.sh /tmp/program.mir.json 3`
  - Optional: `HAKO_LLVM_OPT_LEVEL=1`（default O0）
- MIR(JSON) structural compare
  - `tools/perf/compare_mir_json.sh out_before.json out_after.json`

See also: docs/guides/perf/benchmarks.md
*[🇯🇵 日本語版はこちら / Japanese Version](README.ja.md)*

[![Selfhost Minimal](https://github.com/moe-charm/nyash/actions/workflows/selfhost-minimal.yml/badge.svg?branch=selfhosting-dev)](https://github.com/moe-charm/nyash/actions/workflows/selfhost-minimal.yml)
[![Core Smoke](https://github.com/moe-charm/nyash/actions/workflows/smoke.yml/badge.svg)](https://github.com/moe-charm/nyash/actions/workflows/smoke.yml)
[![Everything is Box](https://img.shields.io/badge/Philosophy-Everything%20is%20Box-blue.svg)](#philosophy)
[![Performance](https://img.shields.io/badge/Performance-13.5x%20Faster-ff6b6b.svg)](#performance)
[![JIT Ready](https://img.shields.io/badge/JIT-Cranelift%20Powered%20(runtime%20disabled)-orange.svg)](#execution-modes)
[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](#license)

---

Architecture notes
- Runtime rings (ring0/ring1/ring2) and provider policy: see `docs/architecture/RINGS.md`.

JoinIR system (structure + PHI + type hint SSOT)
- JoinIR is the Single Source of Truth for:
  - **Structure**: Loop forms (LoopForm lowering), control flow shapes
  - **PHI generation**: SSA merge nodes (If/Loop/Select/Merge patterns)
  - **Type hints**: Type propagation from JoinIR → PHI → lifecycle.rs
- Dev flags:
  - `NYASH_JOINIR_DEBUG=1` - JoinIR lowering traces
  - `NYASH_P3C_DEBUG=1` - Generic type inference traces (Phase 66-67)
  - `NYASH_LOOPFORM_DEBUG=1` - LoopForm construction traces
- Phase documentation:
  - [Phase 30: JoinIR Architecture](docs/private/roadmap2/phases/phase-30-final-joinir-world/README.md)
  - [Phase 33: If Lowering](docs/private/roadmap2/phases/phase-33/)
  - [Phase 34: LoopForm](docs/private/roadmap2/phases/phase-34/)
  - [Phase 48: Loop Lowering](docs/private/roadmap2/phases/phase-48/)
  - [Phase 63-65: Type Hints P1-P3-B](docs/private/roadmap2/phases/phase-65-p3-type-hints/README.md)
  - [Phase 66-67: Generic Type Inference P3-C](docs/private/roadmap2/phases/phase-65-p3-type-hints/README.md#phase-66-67-p3-c-ジェネリック型推論-箱化--実利用2025-11-30)
- Current state: **JoinIR First Chapter Complete** (Phase 27-67)
  - Four pillars established: Structure / Scope / JoinIR / Type Hints
  - See [PHI_BOX_INVENTORY.md](docs/private/roadmap2/phases/phase-30-final-joinir-world/PHI_BOX_INVENTORY.md) for detailed status

Call system (unified by default)
- Builder emits `Call { callee: Callee }` whenever possible; the VM routes by callee kind (Global/Method/Extern/... ).
- Legacy by‑name calls（callee なし）は廃止。必ず Builder が `Callee` を付与する（付与されない場合は Fail‑Fast）。
- Extern dispatch SSOT lives in `src/backend/mir_interpreter/handlers/calls/externs.rs`. Global extern‑like names should delegate there (e.g., `env.get`).
- Extern arity normalization: names with arity suffix (e.g., `env.get/1`) are accepted and normalized to their base form (`env.get`).
  See environment knobs and policies in `docs/ENV_VARS.md`.

Execution Status (Feature Additions Pause)
- Active
  - `--backend llvm` (ny-llvmc crate backend; product main for AOT object/EXE)
  - `--backend vm` (engineering/bootstrap lane; keep for selfhost, recovery, and tooling)
- Inactive/Sealed
  - `--backend cranelift`, `--jit-direct` (sealed; use LLVM harness)
  - AST interpreter (legacy) is gated by feature `interpreter-legacy` and excluded from default builds (LLVM is product main; Rust VM is engineering/bootstrap)

Quick pointers
- Emit object/EXE with crate backend:
  - `tools/ny_mir_builder.sh --in /path/mir.json --emit obj -o a.o`
  - `tools/ny_mir_builder.sh --in /path/mir.json --emit exe -o a.out`
  - auto-selects `ny-llvmc` when present（`NYASH_LLVM_BACKEND=crate` 明示でも可）
- Historical / engineering PyVM parity check: `bash tools/historical/pyvm/pyvm_stage2_smoke.sh`.

Program(JSON v0) → MIR(JSON)
- Compat-only bridge route: convert a Stage‑B Program(JSON v0) file to MIR(JSON):
  - `$NYASH_BIN --json-file /path/program.json --program-json-to-mir /tmp/mir.json`

Dev shortcuts (Operator Boxes & JSON smokes)
- One‑shot JSON verification (dev, Operator Boxes ON): `./tools/opbox-json.sh`
- Run quick profile with Operator Boxes: `./tools/opbox-quick.sh`
- Details: `docs/guides/operator-boxes.md`

Dev mode and defaults
- `nyash --dev script.hako` turns on safe development defaults (AST using ON, Operator Boxes observe, diagnostics minimal) while `nyash script.hako` stays production‑like and quiet.
- You can still use the dev shortcuts for a one‑command setup: `./tools/opbox-json.sh`, `./tools/opbox-quick.sh`.
- Using guard: duplicate `using` of the same file (or alias rebind to a different file) now errors with a line number hint to avoid ambiguous resolution.
  - Example error: `using: duplicate import of '<canon_path>' at file.hako:12 (previous alias 'X' first seen at line 5)`
  - Fix by removing the duplicate or consolidating aliases.

Phase‑15 (2025‑09) update
- Parser newline/TokenCursor 統一は env ゲート下で進行中（`NYASH_PARSER_TOKEN_CURSOR=1`）。
- if/else の PHI incoming は実際の遷移元（exit）へ修正済み（VM/LLVM パリティ緑）。
- 自己ホスト準備として Nyash 製 JSON ライブラリと Ny Executor（最小命令）を既定OFFのトグルで追加予定。
- 推奨トグル: `NYASH_LLVM_USE_HARNESS=1`, `NYASH_PARSER_TOKEN_CURSOR=1`, `NYASH_JSON_PROVIDER=ny`, `NYASH_SELFHOST_EXEC=1`。

Developer quickstart: see `docs/guides/getting-started.md`. Changelog highlights: `CHANGELOG.md`.
ny‑llvm line quickstart: `docs/development/testing/selfhost_exe_stageb_quick_guide.md`（Stage‑B → MirBuilder → ny‑llvmc → EXE）
User Macros (Phase 2): `docs/guides/user-macros.md`
Exceptions (postfix catch/cleanup): `docs/guides/exception-handling.md`
ScopeBox & MIR hints: `docs/guides/scopebox.md`
AST JSON v0 (macro/bridge): `docs/reference/ir/ast-json-v0.md`
MIR mode note: Default PHI behavior
- Phase‑15 ships PHI‑ON by default. Builders emit SSA `Phi` nodes at merges for loops, break/continue, and structured control flow.
- Legacy PHI‑off fallback: set `NYASH_MIR_NO_PHI=1` (pair with `NYASH_VERIFY_ALLOW_NO_PHI=1` if you need relaxed verification).
- See `docs/reference/mir/phi_policy.md` for rationale and troubleshooting.
Self‑hosting one‑pager: `docs/how-to/self-hosting.md`.
ExternCall (env.*) and println normalization: `docs/reference/runtime/externcall.md`.

### Minimal ENV (VM vs LLVM harness)
- VM: no extra environment needed for typical runs.
  - Example: `$NYASH_BIN --backend vm apps/tests/ternary_basic.hako`
- LLVM harness: set three variables so the runner finds the harness and runtime.
  - `NYASH_LLVM_USE_HARNESS=1`
  - `NYASH_NY_LLVM_COMPILER=$NYASH_ROOT/target/release/ny-llvmc`
  - `NYASH_EMIT_EXE_NYRT=$NYASH_ROOT/target/release`
  - Example: `NYASH_LLVM_USE_HARNESS=1 NYASH_NY_LLVM_COMPILER=target/release/ny-llvmc NYASH_EMIT_EXE_NYRT=target/release $NYASH_BIN --backend llvm apps/ny-llvm-smoke/main.hako`

### DebugHub Quick Guide
- Enable: `NYASH_DEBUG_ENABLE=1`
- Select kinds: `NYASH_DEBUG_KINDS=resolve,ssa`
- Output file: `NYASH_DEBUG_SINK=/tmp/nyash_debug.jsonl`
- Use with smokes: `NYASH_DEBUG_ENABLE=1 NYASH_DEBUG_KINDS=resolve,ssa NYASH_DEBUG_SINK=/tmp/nyash.jsonl tools/smokes/v2/run.sh --profile quick --filter "userbox_*"`

### MIR Unified Call（default ON）
- Centralized call emission is enabled by default in development builds.
  - Env toggle: `NYASH_MIR_UNIFIED_CALL` — default ON unless set to `0|false|off`.
  - Instance method calls are normalized in one place (`emit_unified_call`):
    - Early mapping: `toString/stringify → str`
    - `equals/1`: Known first → unique-suffix fallback (user boxes only)
    - Known→function rewrite: `obj.m(a) → Class.m(me,obj,a)`
- Disable legacy rewrite path (dev-only) to avoid duplicate behavior during migration:
  - `NYASH_DEV_DISABLE_LEGACY_METHOD_REWRITE=1`
- JSON emit follows unified format (v1) when unified is ON; legacy v0 otherwise.

Dev metrics (opt-in)
- Known-rate KPI for `resolve.choose`:
  - `NYASH_DEBUG_KPI_KNOWN=1` (enable)
  - `NYASH_DEBUG_SAMPLE_EVERY=<N>` (print every N events)

Layer guard (one-way deps: origin→observe→rewrite)
- Check script: `tools/dev/check_builder_layers.sh`
- Ensures builder layering hygiene during refactors.

### Dev Safety Guards (VM)
- stringify(Void) → "null" for JSON-friendly printing (dev safety; prod behavior unchanged).
- JsonScanner defaults (guarded by `NYASH_VM_SCANNER_DEFAULTS=1`) for missing numeric/text fields inside `is_eof/current/advance` contexts only.
- VoidBox common methods (length/size/get/push) are neutral no-ops in guarded paths to avoid dev-time hard stops.

Profiles (quick)
- `--profile dev` → Macros ON (strict), PyVM dev向け設定を適用（必要に応じて環境で上書き可）
- `--profile lite` → Macros OFF の軽量実行
  - 例: `$NYASH_BIN --profile dev --backend vm apps/tests/ternary_basic.hako`

Specs & Constraints
- Invariants (must-hold): `docs/reference/invariants.md`
- Constraints (known/temporary/resolved): `docs/reference/constraints.md`
- PHI & SSA design: `docs/reference/architecture/phi-and-ssa.md`
- Testing matrix (spec → tests): `docs/guides/testing-matrix.md`
- Comparison with other languages: `docs/guides/comparison/nyash-vs-others.md`

## Table of Contents
- [Self-Hosting (Engineering Bootstrap)](#self-hosting)
- [🌟 Property System Revolution](#-property-system-revolution-september-18-2025)
- [Language Features](#-language-features)
- [Plugin System](#-revolutionary-plugin-system-typebox-architecture)

<a id="self-hosting"></a>
## 🧪 Self‑Hosting (Engineering Bootstrap)
- Guide: `docs/how-to/self-hosting.md`
- Engineering bootstrap E2E: `$NYASH_BIN --backend vm apps/selfhost-minimal/main.hako`
- Engineering smokes: `bash tools/jit_smoke.sh` / `bash tools/selfhost_vm_smoke.sh`
- JSON (Operator Boxes, dev): `./tools/opbox-json.sh` / `./tools/opbox-quick.sh`
- Makefile: `make run-minimal`, `make smoke-selfhost`

MIR note: Core‑13 minimal kernel is enforced by default (NYASH_MIR_CORE13=1). Legacy ops are normalized (Array/Ref→BoxCall; TypeCheck/Cast/Barrier/WeakRef unified).

Pure mode: set `NYASH_MIR_CORE13_PURE=1` to enable strict Core‑13. The optimizer rewrites a few ops (Load/Store/NewBox/Unary) to Core‑13 forms, and the compiler rejects any remaining non‑Core‑13 ops. This may break execution temporarily by design to surface MIR violations early.

Note: JIT runtime execution is currently disabled to reduce debugging overhead. Use Interpreter/VM for running and AOT (Cranelift/LLVM) for distribution.

## 🚀 **Breaking News: Self-Hosting Revolution!**

**September 2, 2025** - 🔥 **ABI as a Box!** Nyash ABI itself implemented as TypeBox (C language) - path to self-hosting clear!
**September 1, 2025** - Revolutionary TypeBox ABI unification achieved! C ABI + Nyash ABI seamlessly integrated.
**August 29, 2025** - Just 20 days after inception, Nyash can now compile to native executables!

```bash
# From Nyash source to native binary (Cranelift required)
cargo build --release --features cranelift-jit
./tools/build_aot.sh program.hako -o app         # Native EXE
./app                                             # Standalone execution!
```

**What we achieved in 24 days:**
- ✅ Full programming language with interpreter
- ✅ VM with 13.5x performance boost  
- ✅ JIT compiler (Cranelift integration)
- ✅ WebAssembly support
- ✅ Plugin system (C ABI + Nyash ABI unified)
- ✅ Native binary generation
- ✅ Python integration via plugins
- ✅ TypeBox ABI bridge (revolutionary plugin unification)
- ✅ **Self-hosting path clear** (Nyash ABI in C, no Rust dependency!)

---

## ✨ **Why Nyash?**

### 🎯 **Everything is Box Philosophy**
```nyash
// Traditional languages have complex type systems
// Nyash: One concept rules them all - Box

static box Main {
    main() {
        // Every value is a Box - unified, safe, simple
        local name = new StringBox("Nyash")
        local count = new IntegerBox(42)
        local data = new MapBox()
        
        // Even Python objects are Boxes!
        local py = new PyRuntimeBox()
        local math = py.import("math")
        print("sqrt(9) = " + math.getattr("sqrt").call(9).str())
        
        return 0
    }
}
```

### ⚡ **Unprecedented Development Speed**
- **Day 1**: Basic interpreter working
- **Day 4**: Already planning JIT
- **Day 13**: VM achieving 13.5x speedup
- **Day 20**: Native executable generation!

### 🔌 **Plugin-First Architecture**
```nyash
// Any functionality can be a plugin Box
local file = new FileBox()          // File I/O plugin
local http = new HttpClientBox()    // Network plugin  
local py = new PyRuntimeBox()       // Python plugin

// Plugins compile to native code too!
```

---

## 🏗️ **Multiple Execution Modes**

Important: JIT runtime execution is sealed for now. Use Rust VM for running, and Cranelift AOT/LLVM AOT for native executables.

Phase‑15 (Self‑Hosting): Legacy VM/Interpreter are feature‑gated
- Default build runs Rust VM for `--backend vm` as the engineering/bootstrap lane
- PyVM route is historical/direct-only and delegates to `tools/historical/pyvm/pyvm_runner.py`
- To enable legacy Rust VM/Interpreter, build with:
  ```bash
  cargo build --release --features vm-legacy,interpreter-legacy
  ```
  Then `--backend vm`/`--backend interpreter` use the legacy paths.
 - Note: `--benchmark` requires the legacy VM. Build with `--features vm-legacy` before running benchmarks.

### 1. **Interpreter Mode** (Development)
```bash
$NYASH_BIN program.hako
```
- Instant execution
- Full debug information
- Perfect for development

### 2. **VM Mode (engineering/bootstrap lane)**
```bash
# Engineering/bootstrap default: Rust VM
$NYASH_BIN --backend vm program.hako

# Historical PyVM parity route
bash tools/historical/pyvm/pyvm_vs_llvmlite.sh program.hako
```
- Default: Rust VM executes MIR directly for engineering/bootstrap
- Legacy PyVM: executes MIR(JSON) via `tools/historical/pyvm/pyvm_runner.py`
- Legacy VM: 13.5x over interpreter (historical); kept for comparison and plugin tests

### 3. **Native Binary (Cranelift AOT)** (Distribution / non-primary native path)
```bash
# Build once (Cranelift)
cargo build --release --features cranelift-jit

./tools/build_aot.sh program.hako -o myapp
./myapp  # Standalone executable!
```
- Zero dependencies
- Maximum performance
- Easy distribution

### 4. **Native Binary (LLVM AOT, ny-llvmc crate backend, product main)**
```bash
# Build harness + CLI (no LLVM_SYS_180_PREFIX needed)
cargo build --release -p nyash-llvm-compiler && cargo build --release --features llvm

# Emit and run native executable via crate backend (ny-llvmc)
NYASH_LLVM_USE_HARNESS=1 \
NYASH_NY_LLVM_COMPILER=target/release/ny-llvmc \
NYASH_EMIT_EXE_NYRT=target/release \
  $NYASH_BIN --backend llvm --emit-exe myapp program.hako
./myapp

# Alternatively, emit an object file then link manually
NYASH_LLVM_USE_HARNESS=1 \
NYASH_NY_LLVM_COMPILER=target/release/ny-llvmc \
  $NYASH_BIN --backend llvm program.hako \
  -D NYASH_LLVM_OBJ_OUT=$PWD/nyash_llvm_temp.o
cc nyash_llvm_temp.o -L crates/nyrt/target/release -Wl,--whole-archive -lnyrt -Wl,--no-whole-archive -lpthread -ldl -lm -o myapp
./myapp
```

Quick comparison smoke (engineering parity, VM vs EXE):
```bash
tools/smoke_aot_vs_vm.sh examples/aot_min_string_len.hako
```

### LLVM Backend Notes
- `NYASH_LLVM_OBJ_OUT`: Path to emit `.o` when running `--backend llvm` (product main).
  - Example: `NYASH_LLVM_OBJ_OUT=$PWD/nyash_llvm_temp.o $NYASH_BIN --backend llvm apps/ny-llvm-smoke/main.hako`
- Previously available `NYASH_LLVM_ALLOW_BY_NAME=1`: Removed - all plugin calls now use method_id by default.
  - The LLVM backend only supports method_id-based plugin calls for better performance and type safety.


### 5. **WebAssembly (Browser)** — Status: Experimental / monitor-only
The WASM/browser path is experimental and not part of the product mainline or default CI. The older playground and guides are kept for historical reference only.

- Source (archived): `projects/nyash-wasm/` (build not guaranteed)
- Current role: experimental / monitor-only, separate from the product main and engineering bootstrap lanes
- If you experiment locally, see the project README and `projects/nyash-wasm/build.sh` (wasm-pack required). No support guarantees.

---

## 🧰 One‑Command Build (MVP): `nyash --build`

Reads `hako.toml` (compat: `nyash.toml`), builds plugins → core → emits AOT object → links an executable in one shot.

Basic (Cranelift AOT)
```bash
$NYASH_BIN --build hako.toml \
  --app apps/egui-hello-plugin/main.hako \
  --out app_egui
```

Key options (minimal)
- `--build <path>`: path to `hako.toml` (compat: `nyash.toml`)
- `--app <file>`: entry `.hako`
- `--out <name>`: output executable (default: `app`/`app.exe`)
- `--build-aot cranelift|llvm` (default: cranelift)
- `--profile release|debug` (default: release)
- `--target <triple>` (only when needed)

Notes
- LLVM AOT main line is the ny-llvmc crate backend and the product main. The default caller route stays on ny-llvmc's boundary route; llvmlite is only reached through explicit keep/replay lanes (for example `--driver harness` or `NYASH_LLVM_USE_HARNESS=1`). End users should invoke ny-llvmc (or tools/ny_mir_builder.sh) rather than calling the Python harness directly.
- Ensure `ny-llvmc` is built (`cargo build -p nyash-llvm-compiler`) and Python3 is available for the internal harness. No `LLVM_SYS_180_PREFIX` required.
- Apps that open a GUI may show a window during AOT emission; close it to continue.
- On WSL if the window doesn’t show, see `docs/guides/cranelift_aot_egui_hello.md` (Wayland→X11).


## 📊 **Performance Benchmarks**

Real-world benchmark results (ny_bench.hako):

```
Mode           | Time      | Relative Speed
---------------|-----------|---------------
Interpreter    | 110.10ms  | 1.0x (baseline)
VM             | 8.14ms    | 13.5x faster
Cranelift AOT  | ~4–6ms    | ~20–27x faster  
Native (LLVM)  | ~4ms      | ~27x faster
```

---

## 🎮 **Language Features**

### Clean Syntax
```nyash
box GameCharacter {
    private { name, health, skills }
    
    // Birth constructor - giving life to Boxes!
    birth(characterName) {
        me.name = characterName
        me.health = 100
        me.skills = new ArrayBox()
        print("🌟 " + characterName + " has been born!")
    }
    
    learnSkill(skill) {
        me.skills.push(skill)
        return me  // Method chaining
    }
}

// Usage
local hero = new GameCharacter("Neko")
hero.learnSkill("Fire Magic").learnSkill("Healing")
```

### Modern Async/Await
```nyash
// Concurrent operations made simple
nowait task1 = fetchDataFromAPI()
nowait task2 = processLocalFiles()

// Do other work while waiting
updateUI()

// Collect results
local apiData = await task1
local files = await task2
```

### Delegation Pattern
```nyash
// Composition over inheritance
box EnhancedArray from ArrayBox {
    private { logger }
    
    override push(item) {
        me.logger.log("Adding: " + item)
        from ArrayBox.push(item)  // Delegate to parent
    }
}
```

---

## 🌟 **Property System Revolution (September 18, 2025)**

### The 4-Category Property Breakthrough
**Just completed: Revolutionary unification of all property types into one elegant syntax!**

```nyash
box RevolutionaryBox {
    // 🔵 stored: Traditional field storage
    name: StringBox
    
    // 🟢 computed: Calculated every access  
    size: IntegerBox { me.items.count() }
    
    // 🟡 once: Lazy evaluation with caching
    once cache: CacheBox { buildExpensiveCache() }
    
    // 🔴 birth_once: Eager evaluation at object creation
    birth_once config: ConfigBox { loadConfiguration() }
    
    birth() {
        me.name = "Example"
        // birth_once properties already initialized!
    }
}
```

### Python Integration Breakthrough  
**The Property System enables revolutionary Python → Nyash transpilation:**

```python
# Python side
class DataProcessor:
    @property
    def computed_result(self):
        return self.value * 2
    
    @functools.cached_property
    def expensive_data(self):
        return heavy_computation()
```

```nyash
// Auto-generated Nyash (1:1 mapping!)
box DataProcessor {
    computed_result: IntegerBox { me.value * 2 }      // computed
    once expensive_data: ResultBox { heavy_computation() }  // once
}
```

**Result**: Python code runs 10-50x faster as native Nyash binaries!

### Documentation
- **[Property System Specification](docs/development/proposals/unified-members.md)** - Complete syntax reference
- **[Python Integration Guide](docs/private/roadmap2/phases/phase-10.7/)** - Python → Nyash transpilation
- **[Implementation Strategy](docs/private/papers/paper-m-method-postfix-catch/implementation-strategy.md)** - Technical details

---

## 🔌 **Revolutionary Plugin System (TypeBox Architecture)**

### TypeBox: The Universal Plugin Bridge (September 2025)
**"Everything is Box" Philosophy - Even ABI is a Box!**

```c
// TypeBox - Type information as a Box (enables cross-plugin creation)
typedef struct {
    uint32_t abi_tag;           // 'TYBX'
    const char* name;           // "ArrayBox"
    void* (*create)(void);      // Box creation function
} NyrtTypeBox;

// NEW: Nyash ABI itself as a TypeBox! (C implementation, no Rust)
typedef struct {
    uint32_t abi_tag;           // 'NABI'
    const char* name;           // "NyashABIProvider"
    void* (*create)(void);      // ABI provider creation
    // ... Nyash operations (call, retain, release)
} NyashABITypeBox;
```

**Revolutionary Achievement**: ABI implementation in pure C enables self-hosting!

### Plugin Configuration
```toml
# hako.toml v3.0 (compat: nyash.toml) - Unified plugin support
[plugins.map]
path = "plugins/map.so"
abi = "c"              # Traditional C ABI

[plugins.advanced_map]
path = "plugins/adv_map.so"
abi = "nyash"          # Type-safe Nyash ABI

[plugins.hybrid]
path = "plugins/hybrid.so"
abi = "unified"        # Both ABIs supported!
```

**Key Innovation**: TypeBox enables cross-plugin Box creation without circular dependencies. MapBox can now return ArrayBox seamlessly!

📚 **[Full TypeBox Documentation](docs/private/roadmap2/phases/phase-12/)**

---

## 🛠️ **Getting Started**

### Quick Install (Linux/Mac/WSL)
```bash
# Clone and build
git clone https://github.com/moe-charm/nyash.git
cd nyash
cargo build --release --features cranelift-jit

# Run your first program
echo 'print("Hello Nyash!")' > hello.hako
$NYASH_BIN hello.hako
```

### Windows
```bash
# Cross-compile for Windows
cargo install cargo-xwin
cargo xwin build --target x86_64-pc-windows-msvc --release
# Use target/x86_64-pc-windows-msvc/release/nyash.exe

# Native EXE (AOT) on Windows (requires Cranelift and MSYS2/WSL toolchain for linking)
cargo build --release --features cranelift-jit
powershell -ExecutionPolicy Bypass -File tools\build_aot.ps1 -Input examples\aot_min_string_len.hako -Out app.exe
./app.exe
```

---

## 🌟 **Unique Innovations**

### 1. **AI-Driven Development**
- Developed with Claude, ChatGPT, and Codex collaboration
- 20-day journey from concept to native compilation
- Proves AI can accelerate language development by 30x

### 2. **Box-First Architecture**
- Every optimization preserves the Box abstraction
- Plugins are Boxes, JIT preserves Boxes, even native code respects Boxes
- TypeBox: Even type information is a Box!
- Unprecedented consistency across all execution modes

### 3. **Observable by Design**
- Built-in debugging and profiling
- JSON event streams for JIT compilation
- DOT graph visualization of optimizations

---

## 📚 **Examples**

### Python Integration
```nyash
// Use Python libraries from Nyash!
local py = new PyRuntimeBox()
local np = py.import("numpy")
local array = np.getattr("array").call([1, 2, 3])
print("NumPy array: " + array.str())
```

### Web Server
```nyash
local server = new HttpServerBox()
server.start(8080)

loop(true) {
    local request = server.accept()
    local response = new HttpResponseBox()
    response.setStatus(200)
    response.write("Hello from Nyash!")
    request.respond(response)
}
```

### Game Development
```nyash
box GameObject {
    public { x, y, sprite }
    
    update(deltaTime) {
        // Physics simulation
        me.y = me.y + gravity * deltaTime
    }
    
    render(canvas) {
        canvas.drawImage(me.sprite, me.x, me.y)
    }
}
```

---

## 🤝 **Contributing**

Join the revolution! We welcome:
- 🐛 Bug reports and fixes
- ✨ New Box types via plugins
- 📚 Documentation improvements
- 🎮 Cool example programs

See also: Contributor guide in `AGENTS.md` (Repository Guidelines) for project layout, build/test commands, and PR expectations.

## 📄 **License**

MIT License - Use freely in your projects!

---

## 👨‍💻 **Creator**

**charmpic** - Hobby Language Developer
- 🐱 GitHub: [@moe-charm](https://github.com/moe-charm)
- 🌟 Created with: Claude, ChatGPT, Codex collaboration

---

## 🎉 **Historical Timeline**

- **August 9, 2025**: First commit - "Hello Nyash!"
- **August 13**: JIT planning begins (day 4!)
- **August 20**: VM achieves 13.5x performance
- **August 29**: Native EXE compilation achieved!
- **September 1**: TypeBox ABI unification - C ABI + Nyash ABI seamless integration
- **September 2**: 🔥 Self-hosting path clear - Nyash ABI in C (no Rust dependency!)
- **September 4**: 🪟 Windows GUI displayed via JIT/native EXE (OS-native window)

*24 days from zero to self-hosting capability - a new record in language development!*

---

**🚀 Nyash - Where Everything is a Box, and Boxes Compile to Native Code!**

*Built with ❤️, 🤖 AI collaboration, and the belief that programming languages can be created at the speed of thought*
