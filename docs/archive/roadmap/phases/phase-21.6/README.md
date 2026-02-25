# Phase 21.6 — Dual‑Emit Parity & C‑line Readiness

Goal: Produce identical MIR(JSON) from both provider (Rust) and selfhost (Hako) builder paths, measure generation cost, and keep AOT (ny‑llvmc) fast/green. All work is semantics‑preserving; defaults remain unchanged.

## Checklists

- [ ] Dual‑emit parity on representative apps (MIR(JSON) normalized SHA1 equal)
- [ ] Resolver‑first ON passes quick/integration
- [ ] Selfhost‑first fallback ok (provider/legacy on failure)
- [ ] AOT obj/exe via ny‑llvmc (crate backend) green
- [ ] Docs updated (bench guides, env vars, quick recipes)

## Scripts

- Dual emit + compare + bench: `tools/perf/dual_emit_compare.sh <input.hako> [rounds]`
- MIR emit bench: `tools/perf/bench_hakorune_emit_mir.sh <input.hako> [rounds]`
- AOT bench: `tools/perf/bench_ny_mir_builder.sh <mir.json> [rounds]`
- MIR diff: `tools/perf/compare_mir_json.sh <a.json> <b.json>`

## Env Knobs

- `HAKO_USING_RESOLVER_FIRST=1` (resolver‑first)
- `HAKO_SELFHOST_BUILDER_FIRST=1` (selfhost→provider→legacy)
- `HAKO_MIR_BUILDER_BOX=hako.mir.builder|min`
- `NYASH_LLVM_BACKEND=crate`（ny‑llvmc）
- `HAKO_LLVM_OPT_LEVEL=0|1`（AOT O0 既定）

## Benchmarks — Tracking

Record normalized parity and generation times here (edit in place).

Legend: SHA1 = normalized JSON digest; Parity=Yes when SHA1 equal; Times are medians unless noted.

| Benchmark (Hako)                         | Resolver | Parity | Provider p50 (ms) | Selfhost p50 (ms) | Notes |
|------------------------------------------|----------|--------|-------------------|-------------------|-------|
| apps/examples/json_query/main.hako       | off/on   |        |                   |                   |       |
| apps/examples/json_pp/main.hako          | off/on   |        |                   |                   |       |
| apps/examples/json_lint/main.hako        | off/on   |        |                   |                   |       |
| apps/examples/json_query_min/main.hako   | off/on   |        |                   |                   |       |

How to fill:
1) Run `tools/perf/dual_emit_compare.sh <file> 5`
2) Copy p50s from the summary lines and mark Parity based on `compare_mir_json.sh` output.
3) Note any diffs (callee kinds/order/phi/meta) in Notes.

## Next Steps

- [ ] If parity holds on the above set, extend to apps/tests subset
- [ ] If diffs remain, categorize and align either provider or selfhost output
- [ ] Keep AOT line green under `HAKO_LLVM_OPT_LEVEL=0` and optional `=1` spot checks

