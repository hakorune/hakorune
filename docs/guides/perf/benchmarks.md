# Benchmarking MIR Generation & AOT (Quickstart)

This guide shows how to measure Hakorune's MIR emit (Stage‑B → MIR(JSON)) and AOT build (MIR(JSON) → obj/exe) without llvmlite. All commands are semantics‑preserving and keep defaults conservative (fail‑fast, O0).

## Prerequisites

- Build binaries once (release):
  - `cargo build --release`
  - `cargo build --release -p nyash-llvm-compiler` (ny-llvmc)
  - `cargo build --release -p nyash_kernel` (NyRT static runtime)

## Scripts

### 1) MIR emit bench (Stage‑B → MIR(JSON))

- Script: `tools/perf/bench_hakorune_emit_mir.sh`
- Usage: `tools/perf/bench_hakorune_emit_mir.sh <input.hako> [rounds]`
- Output CSV: `round,ms,size,sha1` (sha1 is normalized JSON digest; identical = structure equal)
- Useful env toggles:
  - `HAKO_USING_RESOLVER_FIRST=1` (resolver‑first)
  - `HAKO_SELFHOST_BUILDER_FIRST=1` (selfhost builder → provider fallback)
  - `HAKO_MIR_BUILDER_BOX=hako.mir.builder|min` (builder selector)
  - `HAKO_SELFHOST_TRACE=1` (stderr trace)

Example:

```
tools/perf/bench_hakorune_emit_mir.sh apps/examples/json_query/main.hako 5
```

### 2) MIR(JSON) → obj/exe bench（ny-llvmc / crate backend）

- Script: `tools/perf/bench_ny_mir_builder.sh`
- Usage: `tools/perf/bench_ny_mir_builder.sh <mir.json> [rounds]`
- Output CSV: `kind,round,ms`（kind = obj|exe）
- Useful env toggles:
  - `NYASH_LLVM_BACKEND=crate`（既定。ny-llvmc を使う）
  - `HAKO_LLVM_OPT_LEVEL=0|1`（既定は 0／O0）

Example:

```
tools/perf/bench_ny_mir_builder.sh /path/to/out.json 3
```

### 3) MIR(JSON) 構造比較

- Script: `tools/perf/compare_mir_json.sh`
- Usage: `tools/perf/compare_mir_json.sh <a.json> <b.json>`
- 出力: サイズと正規化 SHA1、差分（jq -S 利用時は整形差分）。

## Typical Workflow

1) Emit MIR(JSON)
   - `tools/hakorune_emit_mir.sh apps/APP/main.hako out.json`
2) Measure MIR emit time
   - `HAKO_USING_RESOLVER_FIRST=1 tools/perf/bench_hakorune_emit_mir.sh apps/APP/main.hako 5`
3) Measure AOT（obj/exe）
   - `NYASH_LLVM_BACKEND=crate tools/perf/bench_ny_mir_builder.sh out.json 3`
4) Compare MIR outputs across toggles/branches
   - `tools/perf/compare_mir_json.sh out_before.json out_after.json`

## Notes

- All benches are best‑effort micro‑measurements; run multiple rounds and compare medians.
- Keep defaults strict: resolver/selfhost togglesは明示時のみON。AOTは O0 既定（`HAKO_LLVM_OPT_LEVEL` で上げられます）。

