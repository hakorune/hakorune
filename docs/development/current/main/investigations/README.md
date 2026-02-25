# Investigations Folder

This folder contains investigation notes and analysis for debugging sessions.

## Active Investigations

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
