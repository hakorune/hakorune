Status: Active  
Scope: `tools/selfhost_exe_stageb.sh` の具体的な使い方と代表パターン（EXE ビルド/VM とのパリティ検証）をまとめたガイド。  
See also: `docs/development/selfhosting/quickstart.md`（Self‑Host 全体フローの SSOT）。

# selfhost_exe_stageb.sh - Quick Usage Guide

## TL;DR

```bash
# Build and run native EXE from Nyash source (Stage-B pipeline)
bash tools/selfhost_exe_stageb.sh program.hako -o program.exe --run
```

## What It Does

Converts Nyash `.hako` source → MIR JSON → Native executable using:
- **Stage-B** selfhost parser
- **MirBuilder** with JsonFrag optimization
- **ny-llvmc** (Rust LLVM compiler)
- **nyash_kernel** runtime library

## Quick Start

### 1. Build Prerequisites
```bash
# One-time setup
cargo build --release -p nyash-llvm-compiler
cd crates/nyash_kernel && cargo build --release && cd ../..
cargo build --release
```

### 2. Create a Simple Program
```bash
cat > hello.hako <<'EOF'
static box Main {
  method main(args) {
    return 42
  }
}
EOF
```

### 3. Build and Run
```bash
# Build EXE
bash tools/selfhost_exe_stageb.sh hello.hako -o hello.exe

# Run it
./hello.exe
echo "Exit code: $?"  # Should be 42
```

Or in one step:
```bash
bash tools/selfhost_exe_stageb.sh hello.hako -o hello.exe --run
```

## Common Use Cases

### Test VM vs EXE Parity
```bash
# Run VM
./target/release/hakorune --backend vm program.hako
vm_rc=$?

# Build and run EXE
bash tools/selfhost_exe_stageb.sh program.hako -o test.exe
./test.exe
exe_rc=$?

# Compare
if [[ $vm_rc -eq $exe_rc ]]; then
  echo "✅ PARITY OK: both return $vm_rc"
else
  echo "❌ PARITY FAIL: VM=$vm_rc EXE=$exe_rc"
fi
```

### Debug MIR Generation
```bash
# Check what MIR is being generated
TMP_JSON=$(mktemp --suffix .json)
HAKO_SELFHOST_BUILDER_FIRST=1 \
NYASH_JSON_ONLY=1 \
  bash tools/hakorune_emit_mir.sh program.hako "$TMP_JSON"

# Pretty-print MIR
cat "$TMP_JSON" | jq .

# Clean up
rm "$TMP_JSON"
```

### Custom Paths
```bash
# Use custom compiler
export NYASH_NY_LLVM_COMPILER=/path/to/custom/ny-llvmc

# Use custom runtime
export NYASH_EMIT_EXE_NYRT=/path/to/custom/runtime

# Build
bash tools/selfhost_exe_stageb.sh program.hako -o output.exe
```

## Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `NYASH_NY_LLVM_COMPILER` | Path to ny-llvmc | `target/release/ny-llvmc` |
| `NYASH_EMIT_EXE_NYRT` | Runtime library path | `target/release` |

## Integration with Smoke Tests

### Run All EXE Tests
```bash
tools/smokes/v2/run.sh --profile quick --filter "*exe*"
```

### Run Specific Test
```bash
bash tools/smokes/v2/profiles/quick/core/phase2100/s3_backend_selector_crate_exe_return42_canary_vm.sh
```

### Run Parity Tests
```bash
tools/smokes/v2/run.sh --profile quick --filter "*parity*"
```

## Output Files

- **MIR JSON**: `/tmp/tmp.*.json` (temporary, cleaned up automatically)
- **EXE**: Specified by `-o` flag (default: `a.out`)

## Exit Codes

- `0`: Success
- `2`: Usage error (missing input file)
- Non-zero: Build or runtime error

## Performance

| Metric | Typical Value |
|--------|---------------|
| MIR JSON | ~500 bytes (simple program) |
| EXE Size | ~13MB (with symbols) |
| Build Time | ~30s (clean), ~1s (incremental) |
| Runtime | <100ms (simple program) |

## Troubleshooting

### MIR Generation Fails
```bash
# Check input file
test -f program.hako && echo "File exists" || echo "File not found"

# Try manual MIR generation
bash tools/hakorune_emit_mir.sh program.hako debug.json
cat debug.json | jq . | less
```

### EXE Build Fails
```bash
# Verify compiler exists
test -f target/release/ny-llvmc && echo "Compiler found" || echo "Run: cargo build --release -p nyash-llvm-compiler"

# Verify runtime exists
test -f target/release/libnyash_kernel.a && echo "Runtime found" || echo "Run: cd crates/nyash_kernel && cargo build --release"

# Check for LLVM errors
NYASH_LLVM_VERIFY=1 NYASH_CLI_VERBOSE=1 \
  bash tools/selfhost_exe_stageb.sh program.hako -o test.exe
```

### EXE Runtime Error
```bash
# Run with verbose output
NYASH_CLI_VERBOSE=1 ./test.exe

# Compare with VM
NYASH_CLI_VERBOSE=1 ./target/release/hakorune --backend vm program.hako
```

## Tips & Best Practices

### Timeout Settings for Quick Profile Tests

When running smoke tests with the quick profile, EXE-based tests may take longer due to compilation overhead. Use increased timeouts for reliability:

```bash
# Recommended timeout for EXE tests
tools/smokes/v2/run.sh --profile quick --timeout 120

# For individual EXE tests
tools/smokes/v2/run.sh --profile quick --filter "*exe*" --timeout 120
```

**Note**: VM-based tests typically complete within the default 30-second timeout. The extended timeout is primarily needed for tests that build native executables via ny-llvmc.

### Crate Build Prerequisites

Before running `selfhost_exe_stageb.sh` or EXE-based smoke tests, ensure the compiler and runtime libraries are built:

```bash
# Build ny-llvmc compiler
cargo build --release -p nyash-llvm-compiler

# Build nyash_kernel runtime
cd crates/nyash_kernel && cargo build --release && cd ../..

# Build main binary
cargo build --release
```

These builds are required for the crate backend to function. Without them, EXE generation will fail silently or produce incorrect results.

### Command-Line Arguments (argv)

By default, the `main(args)` method receives an empty array. To enable actual command-line argument passing:

```bash
# Enable argv support
NYASH_EXE_ARGV=1 bash tools/selfhost_exe_stageb.sh program.hako -o program.exe

# Run with arguments
./program.exe arg1 arg2 arg3
```

**How it works**:
- When `NYASH_EXE_ARGV=1` is set, the generated EXE calls `ensure_ny_main` which invokes `argv_get`
- Without this flag (default), `main(args)` receives an empty `ArrayBox`
- This allows programs to be portable between VM and EXE backends

**Example program**:
```nyash
static box Main {
  method main(args) {
    // args is ArrayBox containing command-line arguments
    local count = args.length()
    return count  // Returns number of arguments
  }
}
```

## Known Limitations

1. **MapBox Issue**: Advanced loop scenarios with MapBox may fail (non-critical)
2. **Size**: EXE size is large (~13MB) - optimization pending
3. **Dependencies**: Requires full Rust toolchain + LLVM

## Next Steps

- See full verification report: `docs/development/testing/selfhost_exe_stageb_verification_report.md`
- Run smoke tests: `tools/smokes/v2/run.sh --profile quick`
- Report issues: Check existing tests first, then file bug report

---

**Quick Reference**:
```bash
# Build
tools/selfhost_exe_stageb.sh in.hako -o out.exe

# Build + Run
tools/selfhost_exe_stageb.sh in.hako -o out.exe --run

# Test parity
tools/smokes/v2/run.sh --profile quick --filter "*parity*"
```
