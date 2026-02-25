# Loop Testing Guide - Two Paths

## Quick Reference

### ✅ FORCE Path (RECOMMENDED - Works Correctly)

Use this for all loop development and testing:

```bash
# Generate MIR with FORCE path
HAKO_SELFHOST_BUILDER_FIRST=1 \
HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG=1 \
HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE=1 \
bash tools/hakorune_emit_mir.sh input.hako output.json

# Build EXE
NYASH_LLVM_BACKEND=crate \
bash tools/ny_mir_builder.sh --in output.json --emit exe -o output.exe

# Run
./output.exe
echo "Exit code: $?"
```

### ❌ Delegate Path (BROKEN - DO NOT USE)

**Known Issue**: Stage-B parser produces malformed Program JSON for loops.

**Symptoms**:
- Empty loop bodies
- Wrong loop conditions (comparing with 0 instead of variables)
- Incorrect exit codes

**Status**: Under investigation. See [delegate_loop_lowering_analysis.md](../analysis/delegate_loop_lowering_analysis.md)

## Test Matrix

| Test Case | FORCE Path | Delegate Path | Notes |
|-----------|------------|---------------|-------|
| Simple while loop | ✅ Pass | ❌ Fail | Returns 0 instead of expected value |
| Loop with break | ✅ Pass | ❌ Fail | Body not executed |
| Loop with continue | ✅ Pass | ❌ Fail | Increment not applied |
| Nested loops | ✅ Pass | ❌ Fail | Inner loop empty |

## Environment Variables

### FORCE Path Variables

```bash
# Primary control
HAKO_SELFHOST_BUILDER_FIRST=1      # Use selfhost builder first

# Loop-specific
HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG=1  # Force JsonFrag for loops
HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE=1    # Enable normalization
HAKO_MIR_BUILDER_JSONFRAG_PURIFY=1       # Enable purification (optional)

# Backend
NYASH_LLVM_BACKEND=crate           # Use crate backend for EXE
NYASH_LLVM_VERIFY=1                # Enable LLVM IR verification
NYASH_LLVM_DUMP_IR=path.ll         # Dump LLVM IR for inspection
```

### Delegate Path Variables (For Debugging Only)

```bash
# Basic delegate (DO NOT USE for loops)
NYASH_JSON_ONLY=1                  # Generate MIR via delegate
# This will produce broken MIR for loops!
```

## Debugging Loop Issues

### 1. Verify MIR Structure

```bash
# Check FORCE MIR
jq '.functions[0].blocks' output_force.json

# Expected structure:
# - Block 0: preheader with const declarations
# - Block 1: header with PHI nodes, compare, and branch
# - Block 2: body with loop operations (e.g., i+1)
# - Block 3: exit
```

### 2. Check Loop Header PHIs

```bash
# Inspect header block PHIs
jq '.functions[0].blocks[1].instructions[] | select(.op == "phi")' output.json

# Expected:
# - PHI for loop variable: incoming from [preheader, latch]
# - PHI for condition variable: incoming from [preheader, latch]
# - Different value IDs for latch (updated values)
```

### 3. Verify Loop Body

```bash
# Check body block
jq '.functions[0].blocks[2]' output.json

# Expected:
# - At least one operation (e.g., binop for i+1)
# - Jump back to header
# - NOT empty!
```

### 4. Compare LLVM IR

```bash
# Generate IR from both paths
HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG=1 \
  NYASH_LLVM_DUMP_IR=/tmp/force.ll \
  bash tools/hakorune_emit_mir.sh test.hako force.json && \
  NYASH_LLVM_BACKEND=crate bash tools/ny_mir_builder.sh --in force.json --emit exe -o force.exe

# FORCE IR will show:
# - Proper PHI nodes in loop header
# - Loop body with operations
# - Correct comparison (e.g., %i < %n)
```

## Common Pitfalls

### ❌ Forgetting FORCE Variables

```bash
# This will use delegate path and FAIL for loops:
bash tools/hakorune_emit_mir.sh test.hako output.json

# Always use:
HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG=1 \
  bash tools/hakorune_emit_mir.sh test.hako output.json
```

### ❌ Testing Delegate Path for Loops

```bash
# DO NOT test loops with delegate path:
NYASH_JSON_ONLY=1 bash tools/hakorune_emit_mir.sh loop_test.hako output.json
# This WILL produce broken MIR!
```

### ✅ Correct Workflow

```bash
# 1. Set up environment
export HAKO_SELFHOST_BUILDER_FIRST=1
export HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG=1
export HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE=1

# 2. Generate MIR
bash tools/hakorune_emit_mir.sh test.hako output.json

# 3. Build and test
NYASH_LLVM_BACKEND=crate bash tools/ny_mir_builder.sh --in output.json --emit exe -o test.exe
./test.exe
echo "Exit code: $?"
```

## Canary Tests

Use these existing canaries to verify loop functionality:

```bash
# Stage-B loop canary (uses FORCE path)
bash tools/smokes/v2/profiles/quick/core/phase2100/stageb_loop_jsonfrag_crate_exe_canary_vm.sh

# This test verifies:
# - Loop MIR generation via FORCE path
# - Correct PHI structure
# - Proper loop body execution
# - Expected exit code
```

## When to Use Each Path

| Scenario | Use FORCE | Use Delegate | Notes |
|----------|-----------|--------------|-------|
| Loop development | ✅ Yes | ❌ No | Delegate broken for loops |
| Loop testing | ✅ Yes | ❌ No | FORCE path verified |
| If/else | ✅ Yes | ✅ Yes | Both work |
| Simple expressions | ✅ Yes | ✅ Yes | Both work |
| Production builds | ✅ Yes | ❌ No | FORCE path reliable |

## Related Documentation

- [Delegate Loop Lowering Analysis](../analysis/delegate_loop_lowering_analysis.md) - Root cause analysis
- [Phase 21.5 Optimization Readiness](../../roadmap/phases/phase-21.5/) - Current phase docs
- [MIR Builder Configuration](../reference/mir_builder_config.md) - All configuration options

---

**Last Updated**: 2025-11-11
**Status**: FORCE path stable, delegate path broken for loops
**Recommendation**: Always use FORCE path for loop development until Stage-B parser is fixed
