# CollectionsHot Fix for matmul Pattern

## Problem Summary

**Issue**: CollectionsHot pass was not rewriting ArrayBox get/set/push operations to externcalls in matmul benchmark, causing 200,000% performance degradation vs C.

**Root Cause**: PHI propagation in `build_type_table()` failed to resolve copy chains before checking if incoming PHI values were typed, causing type propagation to stop prematurely in deeply nested loop structures.

## Analysis Results (Task 1)

### Box→SSA Mapping for matmul

**Test case**: `/tmp/matmul_min.hako` with n=512, reps=8, A/B/C as `new ArrayBox()`

#### ArrayBox A (newbox dst=2):
- Value flow: `[2, 10, 11, 39, 40, 55, 56, 70, 71, 100, 101]`
- Boxcalls using A: 2
  - Block 2: `box=11.push([17])`
  - Block 11: `box=71.get([86])`

#### ArrayBox B (newbox dst=3):
- Value flow: `[3, 12, 13, 41, 42, 57, 58, 72, 73, 102, 103]`
- Boxcalls using B: 2
  - Block 2: `box=13.push([25])`
  - Block 11: `box=73.get([89])`

#### ArrayBox C (newbox dst=4):
- Value flow: `[4, 14, 15, 43, 44, 59, 60, 74, 75, 104, 105]`
- Boxcalls using C: 3
  - Block 2: `box=15.push([30])`
  - Block 6: `box=44.get([135])`
  - Block 15: `box=105.set([124, 107])`

### Type Table Propagation Issue

**Before fix**: type_table stopped at SSA 43
- Final size: 12 entries
- Entries: `{2: 'arr', 3: 'arr', 4: 'arr', 10: 'arr', 11: 'arr', 12: 'arr', 13: 'arr', 14: 'arr', 15: 'arr', 39: 'arr', 41: 'arr', 43: 'arr'}`

**Problem**: Boxcalls used SSA 71, 73, 105 which were NOT in type_table

**SSA 71 trace back to origin**:
```
71 -> 70 -> 56 -> 55 -> 40 -> 39 -> 11 -> 10 -> 2 (newbox ArrayBox)
```

**Blocking PHI**: SSA 40
```json
{
  "dst": 40,
  "incoming": [[39, 3], [56, 9]],
  "op": "phi"
}
```

- SSA 39: in type_table ✓
- SSA 56: NOT in type_table ✗ (but resolves via copy to SSA 11 which IS typed)

## Solution (Task 2)

### Changes to `collections_hot.hako`

#### 1. Updated `build_type_table` signature (line 23):
```hako
// OLD:
build_type_table(text) {

// NEW:
build_type_table(text, copy_src_map) {
```

#### 2. Added copy resolution in PHI propagation (lines 62-66):
```hako
// NEW CODE:
local vid = StringHelpers.read_digits(body, posb+1)
// Resolve copy chains before checking type table
if vid != "" && copy_src_map != null {
  vid = AotPrepCollectionsHotBox.resolve_copy(copy_src_map, vid)
}
if vid != "" && tmap.has(vid) {
  // ... rest of logic
```

#### 3. Updated call site to pass copy_src (line 485):
```hako
// OLD:
local type_table = AotPrepCollectionsHotBox.build_type_table(out)

// NEW:
// Pass copy_src to enable PHI propagation through copy chains (matmul fix)
local type_table = AotPrepCollectionsHotBox.build_type_table(out, copy_src)
```

### Why This Works

1. **copy_src map** is already built before calling `build_type_table` (lines 448-460)
2. **PHI propagation** now resolves copy chains: when checking PHI incoming value 56, it resolves to 11, which IS in type_table
3. **Cascading effect**: Once SSA 40 gets typed, subsequent PHIs (56, 70, 71) get typed in later passes
4. **Fixpoint convergence**: Multi-pass algorithm (up to 12 passes) ensures deep nested loops converge

## Expected Results (Task 3-5)

### Task 3: Verification
- PREP MIR should show `"op":"externcall"` with `nyash.array.get_h`, `nyash.array.set_h`, `nyash.array.push_h`
- Original PREP had 0 externcalls, 1+ boxcalls
- Fixed PREP should have many externcalls, fewer boxcalls

### Task 4: Performance Target
- **Current**: C ≈ 12ms, Hakorune EXE ≈ 25s, ratio ≈ 200,000%
- **Target**: ratio < 1000% (ideally < 500% after subsequent hoist/CSE passes)
- **Stretch goal**: ratio ≈ 125% (matching maplin's C-equivalent performance)

### Task 5: Regression Checks
Must verify no degradation in:
- `maplin`: Currently at ~100% ratio (C-equivalent)
- `arraymap`: Existing optimizations preserved
- `linidx`: Existing optimizations preserved

## Testing Commands

```bash
# Generate ORIG MIR (no AOT prep)
NYASH_SKIP_TOML_ENV=1 NYASH_DISABLE_PLUGINS=1 NYASH_PARSER_STAGE3=1 \
./target/release/hakorune --emit-mir-json tmp/matmul_orig.json /tmp/matmul_min.hako

# Generate PREP MIR (with CollectionsHot fix)
# Note: Need to use tools/hakorune_emit_mir.sh or manually apply AotPrep.run_json

# Count externcalls vs boxcalls
rg '"op":"externcall"' tmp/matmul_prep.json | wc -l
rg '"op":"boxcall"' tmp/matmul_prep.json | wc -l

# Run benchmark
NYASH_SKIP_TOML_ENV=1 NYASH_DISABLE_PLUGINS=1 \
tools/perf/microbench.sh --case matmul --backend llvm --exe --runs 3

# Regression checks
tools/perf/microbench.sh --case maplin --backend llvm --exe --runs 3
tools/perf/microbench.sh --case arraymap --backend llvm --exe --runs 3 --budget-ms 10000
tools/perf/microbench.sh --case linidx --backend llvm --exe --runs 3
```

## Implementation Status

- ✅ Task 1: Analysis complete
- ✅ Task 2: Code changes complete (collections_hot.hako modified)
- ⏳ Task 3: Testing pending (VM step budget issues with large MIR)
- ⏳ Task 4: Performance benchmark pending
- ⏳ Task 5: Regression checks pending

## Notes

- **Phase 15.5 compliance**: Fix follows "既定OFF" principle - only affects when `NYASH_AOT_COLLECTIONS_HOT=1` is set
- **Backwards compatibility**: `copy_src_map` parameter is optional (can be null), so existing callers still work
- **No behavior change**: Existing maplin/arraymap/linidx should be unaffected as they don't have deep PHI chains requiring copy resolution
