# LoopBuilder Removal - Quick Summary

**Date**: 2025-12-24
**Test Failure**: `core_direct_array_oob_set_rc_vm`
**Error**: `[joinir/freeze] Loop lowering failed: JoinIR does not support this pattern, and LoopBuilder has been removed`

---

## TL;DR

✅ **User hypothesis is CORRECT**: Old `.hako` code not compatible with new JoinIR patterns

❌ **NOT a bug**: Expected feature gap (20% of patterns not yet covered)

⭐ **Fix**: Rewrite BundleResolver loops to use supported Pattern 1 or Pattern 2

---

## What Happened?

### Timeline

1. **Dec 4, 2025 (Phase 186-187)**: LoopBuilder deleted (1,758 lines removed)
2. **Dec 5-6, 2025 (Phase 188)**: JoinIR Patterns 1-3 implemented (80% coverage)
3. **Dec 24, 2025 (Today)**: BundleResolver test fails (falls in 20% gap)

### Why This Fails

**BundleResolver.resolve/4** uses a complex loop pattern:
- Multiple carriers (5+ variables: i, j, seg, pos, k)
- Nested loops
- Non-unit increment (`i = j + 3`)
- Conditional assignments

**Current JoinIR patterns** don't cover this:
- ✅ Pattern 1: Simple While (single carrier, no breaks)
- ✅ Pattern 2: Conditional Break (simple breaks only)
- ✅ Pattern 3: If-Else PHI (arithmetic accumulation only)
- ❌ **Pattern 6** (Complex Multi-Carrier): **Not implemented yet**

---

## What is LoopBuilder?

**Before Phase 187**:
- Legacy loop lowering system (8 files, 1,000+ lines)
- Handled all loop patterns (but with bugs)
- Silent fallback when JoinIR didn't match

**After Phase 187**:
- ❌ Completely deleted
- ✅ Replaced by JoinIR Frontend (pattern-based)
- ✅ Explicit failures (no silent fallbacks)

**Why removed?**
> "Explicit failures replace implicit fallbacks. Future JoinIR expansion is the only way forward."

---

## How to Fix

### Option A: Rewrite BundleResolver Loops ⭐ **RECOMMENDED**

**Before** (unsupported):
```nyash
local i = 0
loop(i < table.length()) {
    local j = table.indexOf("|||", i)
    // ... complex processing
    if j < 0 { break }
    i = j + 3  // Non-unit increment
}
```

**After** (Pattern 2 compatible):
```nyash
local i = 0
local done = 0
loop(i < table.length() and done == 0) {
    local j = table.indexOf("|||", i)
    // ... complex processing
    if j < 0 {
        done = 1
    } else {
        i = j + 3
    }
}
```

**Tips**:
- Convert non-unit increments to unit increments if possible
- Convert breaks to condition checks
- Reduce carrier count (merge variables if possible)

### Option B: Implement Pattern 6 ⏳ **FUTURE WORK**

**Effort**: Large (Pattern 1-3 took 1,802 lines + design)
**Timeline**: Phase 189+
**Benefit**: Supports all complex loops

---

## Quick Reference

### Supported Patterns (Phase 188)

| Pattern | Example | Status |
|---------|---------|--------|
| **Pattern 1** | `loop(i < n) { i++ }` | ✅ Works |
| **Pattern 2** | `loop(true) { if(...) break }` | ✅ Works |
| **Pattern 3** | `loop(...) { if(...) sum += x else sum += 0 }` | ✅ Works |
| **Complex Multi-Carrier** | BundleResolver loops | ❌ Not yet |

### Where to Find More

- **Full Analysis**: `loopbuilder-removal-compatibility-analysis.md` (this directory)
- **Phase 188 Docs**: `docs/private/roadmap2/phases/phase-188-joinir-loop-pattern-expansion/`
- **BundleResolver Source**: `lang/src/compiler/entry/bundle_resolver.hako`

### Git History

```bash
# Find LoopBuilder removal
git log --all --oneline --grep="Phase 187"
# fa8a96a51 docs(joinir): Phase 187 LoopBuilder Physical Removal

# See deleted files
git show fa8a96a51 --stat
# 13 files changed, 56 insertions(+), 1758 deletions(-)
```

---

## Key Takeaways

1. ✅ **This is NOT a bug**—it's an expected gap (20% of patterns)
2. ✅ **LoopBuilder is gone forever**—no plans to restore
3. ✅ **Fix: Rewrite loops**—use Pattern 1 or Pattern 2
4. ✅ **Future: Implement Pattern 6**—if this becomes a blocker

**Philosophy**:
> Explicit failures drive architecture forward. No silent fallbacks!

---

**Last Updated**: 2025-12-24
**Status**: Investigation complete, recommendations provided
