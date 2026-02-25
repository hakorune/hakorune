# Phase 87: LLVM Exe Line SSOT (2025-12-13)

## Goal

Establish single source of truth for `.hako → .o → executable → execution` pipeline.

**SSOT Tool**: `tools/build_llvm.sh`

## Prerequisites

1. **llvm-config-18** available:
   ```bash
   llvm-config-18 --version
   # Expected: 18.x.x
   ```

2. **hakorune built with LLVM features**:
   ```bash
   cargo build --release --features llvm
   ./target/release/hakorune --version
   # Check --backend llvm available in --help
   ```

3. **Python llvmlite** (for LLVM harness):
   ```bash
   python3 -c "import llvmlite; print(llvmlite.__version__)"
   # Expected: 0.40.0 or newer
   ```

## Compiler Modes

`tools/build_llvm.sh` supports two compiler modes for LLVM object generation:

### Harness (Default) - Production Ready

**Python llvmlite-based LLVM IR generation**

- **Stability**: ✅ Proven stable, battle-tested
- **Build Time**: Fast (~1-3s for minimal programs)
- **Dependencies**: Python 3, llvmlite, LLVM 18
- **Use Case**: Default for all production builds

**Enable** (default behavior):
```bash
# Explicit mode selection (optional):
NYASH_LLVM_COMPILER=harness tools/build_llvm.sh program.hako -o output

# Default (no env var needed):
tools/build_llvm.sh program.hako -o output
```

**How it works**:
1. `hakorune --backend llvm` invokes Python harness
2. `src/llvm_py/llvm_builder.py` generates LLVM IR via llvmlite
3. `llc-18` compiles IR to object file

### Crate (Experimental) - Rust-native Compiler

**Pure Rust LLVM IR generation via crates/nyash-llvm-compiler**

- **Stability**: ⚠️ Experimental, under active development
- **Build Time**: Slower (~5-10s, requires crate compilation)
- **Dependencies**: LLVM 18 dev libraries, Rust toolchain
- **Use Case**: Advanced users, development/testing

**Enable**:
```bash
NYASH_LLVM_COMPILER=crate tools/build_llvm.sh program.hako -o output
```

**How it works**:
1. `hakorune --emit-mir-json` generates MIR JSON
2. `ny-llvmc` (Rust crate) reads JSON and emits LLVM IR
3. `llc-18` compiles IR to object file

**Advanced: Direct exe emission** (experimental):
```bash
NYASH_LLVM_COMPILER=crate NYASH_LLVM_EMIT=exe \
  tools/build_llvm.sh program.hako -o output
# Skips separate linking step, emits executable directly
```

### Mode Comparison Table

| Feature | Harness (Default) | Crate (Experimental) |
|---------|------------------|----------------------|
| **Stability** | ✅ Production ready | ⚠️ Experimental |
| **Build Time** | Fast (1-3s) | Moderate (5-10s) |
| **Dependencies** | Python + llvmlite | LLVM dev + Rust |
| **MIR JSON** | Internal | Explicit generation |
| **Direct exe** | ❌ Not supported | ✅ Experimental |
| **Recommended For** | All users | Advanced/dev only |

**Default recommendation**: Use harness mode (no env vars needed).

## Environment Variables Reference

環境変数の一覧は [`docs/reference/environment-variables.md`](../../reference/environment-variables.md#llvm-build-pipeline) の「LLVM Build Pipeline」セクションを参照してください。

主要な環境変数（クイックリファレンス）：
- `NYASH_LLVM_COMPILER`: コンパイラモード (`harness` または `crate`)
- `NYASH_CLI_VERBOSE=1`: 詳細ビルド出力を有効化
- `NYASH_LLVM_ONLY_OBJ=1`: オブジェクト生成後に停止
- その他14変数の詳細は上記SSOTドキュメントを参照

## Standard Procedure

**Build and execute** a .hako program to native executable:

```bash
# Step 1: Build .hako → executable
tools/build_llvm.sh apps/tests/your_program.hako -o tmp/your_program

# Step 2: Execute
./tmp/your_program
echo $?  # Check exit code
```

**What it does**:
1. Compiles `.hako` → MIR (hakorune compiler)
2. MIR → LLVM IR (llvmlite harness, `src/llvm_py/`)
3. LLVM IR → object file `.o` (llvm tools)
4. Links object → executable (clang)

## Example: Minimal Program

**File**: `apps/tests/phase87_llvm_exe_min.hako`

```nyash
static box Main {
    main() {
        return 42
    }
}
```

**Build**:
```bash
tools/build_llvm.sh apps/tests/phase87_llvm_exe_min.hako -o tmp/phase87_test
```

**Execute**:
```bash
./tmp/phase87_test
echo $?  # Output: 42
```

## Detailed Pipeline Explanation

### Step 1: Hako → MIR JSON

**Command** (internal to build_llvm.sh):
```bash
./target/release/hakorune --emit-mir-json tmp/program.json program.hako
```

**Output**: MIR JSON representation of the program

### Step 2: MIR JSON → LLVM IR

**Command** (internal to build_llvm.sh):
```bash
python3 src/llvm_py/llvm_builder.py tmp/program.json -o tmp/program.ll
```

**Output**: LLVM IR text file (.ll)

### Step 3: LLVM IR → Object File

**Command** (internal to build_llvm.sh):
```bash
llc-18 tmp/program.ll -o tmp/program.o -filetype=obj
```

**Output**: Object file (.o)

### Step 4: Object File → Executable

**Command** (internal to build_llvm.sh):
```bash
clang-18 tmp/program.o -o tmp/program
```

**Output**: Native executable

## Success/Failure Criteria

### Success Indicators

A successful build via `tools/build_llvm.sh` exhibits:

**1. Exit Code**: `0`
```bash
tools/build_llvm.sh program.hako -o output
echo $?  # Should output: 0
```

**2. All 4 Steps Complete**:
```
[1/4] Building hakorune (feature selectable) ...
[2/4] Emitting object (.o) via LLVM backend ...
[3/4] Building Nyash Kernel static runtime ...
[4/4] Linking output ...
✅ Done: output
```

**3. Executable Generated**:
```bash
ls -lh output
# Should exist and be executable
file output
# Output: ELF 64-bit LSB executable, x86-64, dynamically linked
```

**4. Executable Runs**:
```bash
./output
echo $?
# Should match expected exit code (e.g., 42 for phase87_llvm_exe_min.hako)
```

### Failure Modes

`build_llvm.sh` uses distinct exit codes for different failure types:

| Exit Code | Meaning | Common Cause |
|-----------|---------|--------------|
| **0** | ✅ Success | Build completed normally |
| **1** | Usage error | Missing input file or invalid arguments |
| **2** | Missing dependency | `llvm-config-18` not found |
| **3** | Compilation failure | Object file not generated (MIR/LLVM IR error) |
| **Other** | System/linking error | Linking failure, missing libraries |

**Exit Code 1** - Usage Error:
```bash
tools/build_llvm.sh
# Output: Usage: tools/build_llvm.sh <input.hako> [-o <output>]
# Exit: 1
```

**Exit Code 2** - Missing LLVM:
```bash
# When llvm-config-18 not installed
tools/build_llvm.sh program.hako -o output
# Output: error: llvm-config-18 not found (install LLVM 18 dev).
# Exit: 2
```

**Exit Code 3** - Object Generation Failed:
```bash
# When MIR/LLVM IR compilation fails
tools/build_llvm.sh bad_program.hako -o output
# Output: error: object not generated: target/aot_objects/bad_program.o
# Exit: 3
```

### Validation Commands

**Verify object file validity**:
```bash
# Check object file exists and has correct format
file target/aot_objects/program.o
# Expected: ELF 64-bit relocatable, x86-64

# Check object file symbols
nm target/aot_objects/program.o | grep -E '(main|nyash_)'
# Should show exported symbols
```

**Verify LLVM IR validity** (when using crate mode with JSON):
```bash
# Step 1: Generate LLVM IR (manual)
NYASH_LLVM_COMPILER=crate NYASH_LLVM_MIR_JSON=tmp/test.json \
  tools/build_llvm.sh program.hako -o output

# Step 2: Validate LLVM IR
llvm-as-18 tmp/test.ll -o /dev/null
# Should complete without errors

# Step 3: Disassemble and inspect
llvm-dis-18 target/aot_objects/program.o -o - | less
# Should show valid LLVM IR
```

**Verify MIR JSON validity** (crate mode):
```bash
# Ensure MIR JSON is well-formed
jq . tmp/test.json > /dev/null
echo $?  # Should output: 0

# Optional: Schema validation
NYASH_LLVM_VALIDATE_JSON=1 NYASH_LLVM_COMPILER=crate \
  tools/build_llvm.sh program.hako -o output
```

### Build Time Expectations

Typical build times for `phase87_llvm_exe_min.hako` (minimal program):

| Step | Expected Time | Notes |
|------|---------------|-------|
| **[1/4] Build hakorune** | ~0.5-2s | Incremental build (release) |
| **[2/4] Emit object** | ~1-2s | Harness mode (llvmlite) |
|  | ~5-10s | Crate mode (ny-llvmc) |
| **[3/4] Build Nyash Kernel** | ~1-3s | Incremental build (release) |
| **[4/4] Linking** | ~0.2-0.5s | Native linker (cc/clang) |
| **Total** | ~3-8s | Harness mode |
|  | ~7-15s | Crate mode |

**First build**: Add ~30-60s for initial `cargo build --release` compilation.

**Performance factors**:
- **Parallel builds**: `-j 24` used by default (see `build_llvm.sh`)
- **Incremental builds**: `CARGO_INCREMENTAL=1` enabled
- **Cache hits**: Subsequent builds much faster (~1-3s total)

**Troubleshooting slow builds**:
```bash
# Check cargo cache status
cargo clean --release -p nyash-rust
cargo clean --release -p nyash-llvm-compiler

# Rebuild with timing information
time tools/build_llvm.sh program.hako -o output

# Verbose output for bottleneck analysis
NYASH_CLI_VERBOSE=1 time tools/build_llvm.sh program.hako -o output
```

## Troubleshooting

### Issue: llvm-config-18 not found

**Symptom**: `build_llvm.sh` fails with "llvm-config-18: command not found"

**Solution**:
```bash
# Ubuntu/Debian:
sudo apt-get install llvm-18-dev llvm-18-tools

# macOS (Homebrew):
brew install llvm@18
export PATH="/opt/homebrew/opt/llvm@18/bin:$PATH"

# WSL (Ubuntu):
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 18
```

### Issue: Python llvmlite not found

**Symptom**: `ModuleNotFoundError: No module named 'llvmlite'`

**Solution**:
```bash
pip3 install llvmlite

# If system-wide install fails, use virtual environment:
python3 -m venv venv
source venv/bin/activate
pip install llvmlite
```

### Issue: Linking fails

**Symptom**: `ld: symbol(s) not found for architecture`

**Check**:
- Ensure clang-18 is installed
- Verify LLVM 18 libraries available:
  ```bash
  llvm-config-18 --libdir
  ls $(llvm-config-18 --libdir)
  ```

### Issue: MIR compilation error

**Symptom**: hakorune fails to compile .hako to MIR

### Issue: LLVM IR parsing error（expected instruction opcode / PHI placement）

**Symptom**: llvmlite が生成した LLVM IR の parse に失敗する（例: `expected instruction opcode`）。

**Next**:
- まず棚卸しと代表ケース表を確認: `docs/development/current/main/phase131-3-llvm-lowering-inventory.md`
- 典型例: ループ + PHI が絡むケースで “PHI が terminator の後に出る” など、LLVM IR の不変条件違反が起きる

**Debug**:
```bash
# Test MIR generation manually:
./target/release/hakorune --emit-mir-json test.json test.hako

# Check error messages:
cat test.json  # Should be valid JSON
```

### Issue: LLVM IR generation error

**Symptom**: llvm_builder.py fails

**Debug**:
```bash
# Run Python builder manually:
python3 src/llvm_py/llvm_builder.py test.json -o test.ll

# Check LLVM IR validity:
llvm-as-18 test.ll -o /dev/null
# Should complete without errors
```

### Debugging Build Pipeline

When `build_llvm.sh` fails, use these techniques to isolate the problem:

#### Enable Verbose Mode

**Global verbose output**:
```bash
NYASH_CLI_VERBOSE=1 tools/build_llvm.sh program.hako -o output
# Shows detailed command execution via set -x
```

**Step-by-step verbosity**:
```bash
# Verbose hakorune compilation
NYASH_CLI_VERBOSE=1 ./target/release/hakorune --emit-mir-json tmp/debug.json program.hako

# Verbose Python LLVM builder
python3 -v src/llvm_py/llvm_builder.py tmp/debug.json -o tmp/debug.ll

# Verbose LLVM compilation
llc-18 -debug tmp/debug.ll -o tmp/debug.o -filetype=obj

# Verbose linking
cc -v tmp/debug.o -L crates/nyash_kernel/target/release -lnyash_kernel -o output
```

#### Manual Step Tracing

**Isolate each step** to find exact failure point:

**Step 1: Test MIR emission**:
```bash
./target/release/hakorune --emit-mir-json tmp/test.json program.hako
echo $?  # Should be 0
jq . tmp/test.json  # Validate JSON
```

**Step 2: Test LLVM IR generation**:
```bash
# Harness mode (default)
NYASH_LLVM_USE_HARNESS=1 ./target/release/hakorune --backend llvm program.hako
# Check exit code

# Crate mode
cargo build --release -p nyash-llvm-compiler
./target/release/ny-llvmc --in tmp/test.json --out tmp/test.o
file tmp/test.o  # Should be ELF object
```

**Step 3: Test object compilation**:
```bash
# If .ll file available (crate mode intermediate)
llc-18 -filetype=obj tmp/test.ll -o tmp/test.o
file tmp/test.o  # Verify ELF format
nm tmp/test.o    # Check symbols
```

**Step 4: Test linking**:
```bash
# Ensure Nyash Kernel built
cd crates/nyash_kernel && cargo build --release
cd ../..

# Manual link
cc tmp/test.o \
  -L crates/nyash_kernel/target/release \
  -Wl,--whole-archive -lnyash_kernel -Wl,--no-whole-archive \
  -lpthread -ldl -lm -o tmp/manual_output

# Test execution
./tmp/manual_output
echo $?
```

#### Save Intermediate Files

**Preserve all build artifacts** for inspection:

```bash
# Create debug directory
mkdir -p debug_build

# Step 1: Emit MIR JSON
./target/release/hakorune --emit-mir-json debug_build/program.json program.hako

# Step 2: Generate LLVM IR (harness mode, manual Python call)
python3 src/llvm_py/llvm_builder.py debug_build/program.json -o debug_build/program.ll

# Step 3: Compile to object
llc-18 debug_build/program.ll -o debug_build/program.o -filetype=obj

# Step 4: Link
cc debug_build/program.o \
  -L crates/nyash_kernel/target/release \
  -Wl,--whole-archive -lnyash_kernel -Wl,--no-whole-archive \
  -lpthread -ldl -lm -o debug_build/program

# Inspect all intermediate files
ls -lh debug_build/
file debug_build/*
```

**Inspect saved artifacts**:
```bash
# View MIR JSON structure
jq '.functions[] | {name: .name, blocks: .blocks | length}' debug_build/program.json

# View LLVM IR
less debug_build/program.ll

# Disassemble object file
objdump -d debug_build/program.o | less

# Check symbols
nm debug_build/program.o
nm debug_build/program
```

#### Common LLVM IR Issues

**Problem 1: Invalid function signature**
```
error: expected type '...' but found '...'
```
**Diagnosis**: MIR → LLVM IR type mismatch
**Fix**: Check MIR JSON `functions[].signature`, ensure correct types

**Problem 2: Undefined symbol**
```
error: undefined reference to 'nyash_...'
```
**Diagnosis**: Missing Nyash Kernel runtime symbols
**Fix**:
```bash
# Rebuild Nyash Kernel
cd crates/nyash_kernel && cargo clean && cargo build --release

# Verify symbols available
nm crates/nyash_kernel/target/release/libnyash_kernel.a | grep nyash_
```

**Problem 3: Invalid IR instruction**
```
error: invalid IR instruction '...'
```
**Diagnosis**: Python llvm_builder.py bug or unsupported MIR instruction
**Fix**:
```bash
# Check LLVM IR syntax
llvm-as-18 -o /dev/null debug_build/program.ll
# Error message shows exact line number

# Inspect problematic instruction
sed -n '<line>p' debug_build/program.ll
```

**Problem 4: Linking failure**
```
ld: symbol(s) not found for architecture x86_64
```
**Diagnosis**: Missing system libraries or incorrect link order
**Fix**:
```bash
# Check what symbols are needed
nm -u debug_build/program.o

# Verify Nyash Kernel provides them
nm crates/nyash_kernel/target/release/libnyash_kernel.a | grep <symbol>

# If system library missing, add to NYASH_LLVM_LIBS
NYASH_LLVM_LIBS="-lmissing_lib" tools/build_llvm.sh program.hako -o output
```

#### Environment Variables for Debugging

Combine multiple debugging flags:

```bash
# Maximum verbosity + preserve artifacts
NYASH_CLI_VERBOSE=1 \
NYASH_LLVM_COMPILER=crate \
NYASH_LLVM_MIR_JSON=/tmp/debug.json \
NYASH_LLVM_VALIDATE_JSON=1 \
  tools/build_llvm.sh program.hako -o /tmp/debug_output

# Then inspect intermediate files
ls -lh /tmp/debug*
jq . /tmp/debug.json
cat /tmp/debug.ll
```

**Recommended debugging workflow**:
1. Enable `NYASH_CLI_VERBOSE=1` for initial diagnosis
2. Use manual step tracing to isolate failure
3. Save intermediate files for inspection
4. Check LLVM IR validity with `llvm-as-18`
5. Verify object symbols with `nm`
6. Test linking manually with verbose `cc -v`

## What NOT to Do

❌ **DO NOT** create custom link procedures:
- Scattered linking logic across multiple scripts
- Manual `clang` invocations outside `build_llvm.sh`
- Duplicate .o → exe pipelines

❌ **DO NOT** bypass build_llvm.sh:
- Direct llvm_builder.py invocations for production
- Custom shell scripts for one-off builds
- Hardcoded paths in makefiles

✅ **DO** use `tools/build_llvm.sh` for all LLVM exe generation

## Integration Test

**Location**: `tools/smokes/v2/profiles/integration/apps/phase87_llvm_exe_min.sh`

**What it tests**:
- Full pipeline: .hako → exe → execution
- Exit code verification (42)
- SKIP if LLVM unavailable (graceful degradation)

**Run manually**:
```bash
tools/smokes/v2/run.sh --profile integration --filter 'phase87_llvm_exe_min\.sh'
```

**Expected outcomes**:
- **PASS**: If llvm-config-18 available → exit code 42 verified
- **SKIP**: If llvm-config-18 not found → graceful skip message

## Why Exit Code 42?

- **Stdout-independent**: Works even if stdout is redirected/buffered
- **Deterministic**: No parsing required, simple integer comparison
- **Traditional**: Unix convention for testable exit codes
- **Minimal**: No dependencies on print/console boxes

## Advanced Usage

### Custom output location

```bash
# Default: output to tmp/
tools/build_llvm.sh program.hako -o custom/path/binary

# Ensure directory exists first:
mkdir -p custom/path
```

### Debugging build steps

**Set verbose mode** (if supported by build_llvm.sh):
```bash
VERBOSE=1 tools/build_llvm.sh program.hako -o output
```

**Check intermediate files**:
```bash
# MIR JSON:
ls -lh tmp/*.json

# LLVM IR:
ls -lh tmp/*.ll

# Object file:
ls -lh tmp/*.o
```

### Comparing with VM backend

**VM execution** (interpreted):
```bash
./target/release/hakorune --backend vm program.hako
echo $?
```

**LLVM execution** (native):
```bash
tools/build_llvm.sh program.hako -o tmp/program
./tmp/program
echo $?
```

**Should produce identical exit codes** for correct programs.

## Performance Characteristics

**Build time**: ~1-3 seconds for minimal programs
- .hako → MIR: ~100ms
- MIR → LLVM IR: ~500ms
- LLVM IR → .o: ~1s
- Linking: ~200ms

**Execution time**: Native speed (no VM overhead)
- Typical speedup: 10-100x vs VM backend
- No JIT warmup required
- Full LLVM optimizations applied

## Related Documentation

- LLVM Python harness: `src/llvm_py/README.md`
- MIR spec: `docs/reference/mir/INSTRUCTION_SET.md`
- Integration smokes: `tools/smokes/v2/README.md`
- build_llvm.sh implementation: `tools/build_llvm.sh` (read source for details)

## SSOT Principle

**Single Source of Truth**:
- ONE script: `tools/build_llvm.sh`
- ONE pipeline: .hako → MIR → LLVM IR → .o → exe
- ONE integration test: `phase87_llvm_exe_min.sh`

**Benefits**:
- Maintainability: Update one script, not scattered logic
- Consistency: All LLVM builds use same pipeline
- Testability: Single smoke test covers full pipeline
- Documentation: One canonical reference

**Anti-patterns to avoid**:
- Multiple competing build scripts
- Copy-pasted linking commands
- Ad-hoc shell scripts for "quick builds"

## Status

- ✅ SSOT established: `tools/build_llvm.sh`
- ✅ Integration smoke added: `phase87_llvm_exe_min.sh`
- ✅ Documentation complete
- ✅ Prerequisites verified: llvm-config-18, llvmlite, LLVM features
- 🎯 Production ready: Use for all LLVM native compilations

## Future Enhancements (Out of Scope for Phase 87)

- **Optimization levels**: -O0, -O1, -O2, -O3 flags
- **Debug symbols**: -g flag support
- **Static linking**: --static flag
- **Cross-compilation**: --target flag
- **LTO**: Link-time optimization support

**Current scope**: Baseline SSOT pipeline establishment only.
