Status: SSOT, Active

# Self‑Hosting Quickstart (Phase 15 — Resume)

This note shows how to run the Hakorune self‑host compiler MVP. Read the current lanes as:
- product main: `llvm/exe`
- compat/proof keep: `rust-vm`
- reference/conformance: `vm-hako`

The flow below keeps raw defaults unchanged and uses small, opt‑in flags for development.

## Layout (migrated)
- Compiler entry (Stage‑B): `lang/src/compiler/entry/compiler_stageb.hako`
- Compiler entry (compat, Stage‑A/AOT): `lang/src/compiler/entry/compiler.hako`
- Shared helpers live under `lang/src/shared/`
- VM/Core live under `lang/src/vm/`

## Run the self‑host compiler
Compile a minimal program (string embedded in the compiler) and print JSON v0:

```
./target/release/nyash lang/src/compiler/entry/compiler_stageb.hako -- --stage3 --source 'static box Main { main() { return 7 } }'
```

ENV → child args (透過):
- `NYASH_NY_COMPILER_MIN_JSON=1` → `-- --min-json`
- `NYASH_SELFHOST_READ_TMP=1`    → `-- --read-tmp` (reads `tmp/ny_parser_input.ny`)
- `NYASH_NY_COMPILER_STAGE3=1`   → `-- --stage3` (Stage‑3 surface enable)
- `NYASH_NY_COMPILER_CHILD_ARGS="..."` → passes extra args verbatim

Examples:
```
NYASH_NY_COMPILER_MIN_JSON=1 ./target/release/nyash lang/src/compiler/entry/compiler_stageb.hako -- --stage3 --source 'static box Main { main() { return 1+2 } }' > /tmp/out.json
NYASH_SELFHOST_READ_TMP=1 ./target/release/nyash lang/src/compiler/entry/compiler_stageb.hako -- --min-json --stage3
```

## Execute MIR(JSON v0)
Use Rust VM only as the compat/proof keep lane. Product native output lives on the LLVM/EXE line. Historical PyVM checks are direct-only scripts.

Rust VM (compat/proof keep):
```
./target/release/nyash --backend vm apps/examples/json_query/main.hako
```

Historical PyVM parity (direct route):
```
bash tools/historical/pyvm/pyvm_vs_llvmlite.sh apps/examples/json_query/main.hako
```

LLVM harness (llvmlite):
```
NYASH_LLVM_USE_HARNESS=1 ./target/release/nyash --backend llvm apps/examples/json_query/main.hako
```

Product EXE line:
```
./target/release/hakorune --backend llvm --emit-exe /tmp/app apps/examples/json_query/main.hako
/tmp/app
```

Notes:
- For self‑host emitted JSON, route the file to your runner pipeline or a small loader script (dev only). Keep defaults unchanged in CI (no new jobs required).
- `--backend vm` remains the raw legacy compat/proof ingress for now; do not read it as product ownership.

## One‑shot dev smoke
Run a minimal engineering smoke that tries to emit JSON (best‑effort) and verifies VM outputs match with Known rewrite ON/OFF:

```
tools/selfhost/selfhost_smoke.sh
```

It does not modify defaults and is safe to run locally.

## Flags (dev)
- Known rewrite default ON (userbox only, strict guards): `NYASH_REWRITE_KNOWN_DEFAULT=0|1`
- Router trace: `NYASH_ROUTER_TRACE=1`
- KPI sampling: `NYASH_DEBUG_KPI_KNOWN=1` (+ `NYASH_DEBUG_SAMPLE_EVERY=N`)
- Local SSA trace: `NYASH_LOCAL_SSA_TRACE=1`

## Acceptance (P6 resume)
- quick/integration remain green.
- Minimal self‑host emit→execute path PASS in a dev job (no CI change).
- No default behavior changes; all diagnostics under env flags.
