# Investigations Folder

This folder contains investigation notes and analysis for debugging sessions.

## Active Investigations

### Perf Kilo String Birth Hot Path (2026-03-28)

**Status**: 🟢 active summary
**Problem**: `perf-kilo` current wave の string birth hot path について、accepted / rejected / stop-line を 1 枚で読めるようにする

**Key Document**:
1. [perf-kilo-string-birth-hotpath-summary-2026-03-28.md](perf-kilo-string-birth-hotpath-summary-2026-03-28.md)
2. [perf-kilo-string-leaf-rejected-followups-2026-03-28.md](perf-kilo-string-leaf-rejected-followups-2026-03-28.md)

**Current Read**:
- accepted slices are already landed for `freeze.str`, `BorrowedSubstringPlan`, `array_set`, `Registry::alloc`, and `Registry::get`
- rejected slices are stable and must stay out of the active lane
- current stop-line is `BoxBase::new`; the next move, if any, must come from upstream birth-density / placement rather than more sink-local tuning

### Phase29ck: Array Substrate Rejected Optimizations (2026-03-27)

**Status**: 🟢 active ledger
**Problem**: `phase-29ck` array substrate perf wave の rejected attempts を散逸させず、next exact front を proof-vocabulary first + live-route debug bundle に固定する

**Key Document**:
1. [phase29ck-array-substrate-rejected-optimizations-2026-03-27.md](phase29ck-array-substrate-rejected-optimizations-2026-03-27.md)

**Current Read**:
- current exact front is docs-first `P18-LIVE-ROUTE-DEBUG-BUNDLE-LOCK.md`
- adjacent fused-leaf reject is now explained by live no-replay route evidence (`get -> copy* -> const 1 -> add -> set`)
- `RwLock -> Mutex` and raw borrowed-cache variants were both rejected and reverted under `WSL warmup=1 repeat=3`

### Phase29x: Direct Route Probe (2026-03-02)

**Status**: 🟢 monitor active  
**Problem**: `--emit-mir-json` direct route の `emit:other`（plan freeze系）残件を段階削減中

**Key Document**:
1. [phase29x-direct-route-probe-2026-03-02.md](phase29x-direct-route-probe-2026-03-02.md)

**Latest Snapshot**:
- `emit_fail=27`, `run_nonzero=7`, `run_ok=84`
- class: `emit:direct-verify=0`, `emit:joinir-reject=0`, `emit:other=27`, `emit:freeze-contract=0`

**Current Head Blocker**:
- `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_min.hako`

### Phase 131-12: LLVM Wrong Result (Case C)

**Status**: ✅ Root cause identified
**Problem**: LLVM backend returns wrong results for loop exit values
**Root Cause**: vmap object identity mismatch between Pass A and Pass C

**Key Documents**:
1. [phase131-12-case-c-llvm-wrong-result.md](phase131-12-case-c-llvm-wrong-result.md) - Initial investigation scope
2. [phase131-12-p1-vmap-identity-analysis.md](phase131-12-p1-vmap-identity-analysis.md) - Detailed trace analysis
3. [phase131-12-p1-trace-summary.md](phase131-12-p1-trace-summary.md) - Executive summary with fix recommendations

**Quick Summary**:
- **Bug**: Pass A deletes `_current_vmap` before Pass C runs
- **Impact**: Terminators use wrong vmap object, missing all Pass A writes
- **Fix**: Store vmap_cur in deferred_terminators tuple (Option 3)

**Next Steps**:
1. Implement Option 3 fix in block_lower.py
2. Add Fail-Fast check in instruction_lower.py
3. Verify with NYASH_LLVM_VMAP_TRACE=1
4. Run full test suite

## Trace Environment Variables

### Phase 131-12-P1 Traces
```bash
NYASH_LLVM_VMAP_TRACE=1    # Object identity and vmap keys tracing
NYASH_LLVM_USE_HARNESS=1   # Enable llvmlite harness
NYASH_LLVM_DUMP_IR=<path>  # Save LLVM IR to file
```

## Investigation Workflow

1. **Scope** - Define problem and test case (phase131-12-case-c-*.md)
2. **Trace** - Add instrumentation and collect data (phase131-12-p1-vmap-identity-*.md)
3. **Analysis** - Identify root cause with evidence (phase131-12-p1-trace-summary.md)
4. **Fix** - Implement solution with validation
5. **Document** - Update investigation notes with results

## Archive

Completed investigations are kept for reference and pattern recognition.

### JoinIR Generalization Study (Historical)

- `joinir-generalization-study.md`（Phase 131–138 の状況と一般化案の相談用コンテキスト。SSOT は design/ を参照）
