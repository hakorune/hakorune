# `nyash --help` Snapshot (historical capture)

Captured: 2025-08-23
Source: Built-in clap help from the `nyash` binary

Current reading (phase-30x):
- `llvm/exe` = product main
- `vm` = engineering/bootstrap keep
- `vm-hako` = reference/conformance
- `wasm` = experimental / monitor-only

Known drift from current source:
- this snapshot still says `interpreter` is the default backend
- current source truth in `src/cli/args.rs` defaults to `vm` and also exposes `vm-hako`
- treat the block below as a historical capture, not the current ownership story

```
🦀 Nyash Programming Language - Everything is Box in Rust! 🦀

Usage: nyash [OPTIONS] [FILE]

Arguments:
  [FILE]  Nyash file to execute

Options:
      --debug-fuel <ITERATIONS>  Set parser debug fuel limit (default: 100000, 'unlimited' for no limit) [default: 100000]
      --dump-mir                 Dump MIR (Mid-level Intermediate Representation) instead of executing
      --verify                   Verify MIR integrity and exit
      --mir-verbose              Show verbose MIR output with statistics
      --backend <BACKEND>        Choose execution backend: 'interpreter' (default), 'vm', or 'llvm' [default: interpreter]
      --compile-wasm             Compile to WebAssembly binary (.wasm) instead of executing
      --compile-native           Compile to native AOT executable using wasmtime precompilation
      --aot                      Short form of --compile-native
  -o, --output <FILE>            Output file (for WASM compilation or AOT executable)
      --benchmark                Run performance benchmarks across all backends
      --iterations <COUNT>       Number of iterations for benchmarks (default: 10) [default: 10]
      --vm-stats                 Enable VM instruction statistics (equivalent to NYASH_VM_STATS=1)
      --vm-stats-json            Output VM statistics in JSON format
  -h, --help                     Print help
  -V, --version                  Print version
```

関連: CLIオプション早見表は `docs/tools/cli-options.md`
