# Phase 161 Representative Functions Test Suite

This directory contains 5 representative Nyash functions that exercise all key analyzer patterns for the Phase 161 JoinIR/MIR to .hako migration.

## Overview

These functions are used to validate Phase 161-2+ implementation of MirAnalyzerBox and related analyzer infrastructure. Each function is carefully designed to cover a specific control flow pattern.

## Representative Functions

### 1. rep1_if_simple.hako
**Pattern**: Simple if/else with PHI merge
**Complexity**: ⭐ Simple
**Tests**:
- Branch detection
- If-merge identification
- Single PHI instruction
- PHI incoming values from both branches

**Expected MIR Analysis**:
- 1 PHI instruction
- 1 Branch instruction
- 1 If structure detected

---

### 2. rep2_loop_simple.hako
**Pattern**: Simple loop with back edge
**Complexity**: ⭐ Simple
**Tests**:
- Loop detection via backward edge
- Loop-carried PHI at header
- Back edge identification
- Loop body block identification

**Expected MIR Analysis**:
- 1 Loop detected
- 1 PHI instruction (at loop header)
- Backward edge: Block 2 → Block 1

---

### 3. rep3_if_loop.hako
**Pattern**: Nested if inside loop
**Complexity**: ⭐⭐ Medium
**Tests**:
- Complex nested control flow
- Multiple PHI instructions (loop PHI + if PHI)
- Interaction between loop and if patterns
- Correct block hierarchy

**Expected MIR Analysis**:
- 1 Loop detected (loop header)
- 1 If detected (nested within loop body)
- 3 PHI instructions total:
  - 2 at loop header (for loop carries)
  - 1 at if merge point

---

### 4. rep4_loop_break.hako
**Pattern**: Loop with break statement
**Complexity**: ⭐⭐ Medium
**Tests**:
- Loop with multiple exits
- Break target resolution
- Exit PHI merge with multiple incoming paths
- Complex control flow merging

**Expected MIR Analysis**:
- 1 Loop detected
- Multiple exit paths from loop
- Break condition identified
- Exit merge block identified

---

### 5. rep5_type_prop.hako
**Pattern**: Type propagation through loop
**Complexity**: ⭐⭐ Medium
**Tests**:
- Type inference through PHI chains
- BinOp type preservation
- Type consistency across loop iterations
- Compare operation type hints

**Expected MIR Analysis**:
- Type propagation converges
- All ValueIds have consistent types
- PHI merges compatible types
- 4-iteration propagation completes

---

## Generating MIR JSON

To generate reference MIR JSON for each representative:

```bash
./target/release/nyash --dump-mir --emit-mir-json rep1_if_simple.mir.json rep1_if_simple.hako
./target/release/nyash --dump-mir --emit-mir-json rep2_loop_simple.mir.json rep2_loop_simple.hako
./target/release/nyash --dump-mir --emit-mir-json rep3_if_loop.mir.json rep3_if_loop.hako
./target/release/nyash --dump-mir --emit-mir-json rep4_loop_break.mir.json rep4_loop_break.hako
./target/release/nyash --dump-mir --emit-mir-json rep5_type_prop.mir.json rep5_type_prop.hako
```

This creates the reference `.mir.json` files that MirAnalyzerBox will parse.

---

## Testing MirAnalyzerBox

Phase 161-2+ will implement analyzer methods that should produce these results:

### rep1_if_simple

```
MirAnalyzerBox analyzer("rep1_if_simple.mir.json", text)
analyzer.list_phis()     → [{ block_id: 4, dest: ValueId, incoming: [...] }]
analyzer.list_ifs()      → [{ condition_block: 1, merge_block: 4, ... }]
analyzer.summarize_function(0) → { has_phis: true, has_ifs: true, ... }
```

### rep2_loop_simple

```
analyzer.list_loops()    → [{ header_block: 1, contains_blocks: [1,2], ... }]
analyzer.list_phis()     → [{ block_id: 1, dest: ValueId, incoming: [...] }]
analyzer.summarize_function(0) → { has_loops: true, has_phis: true, ... }
```

### rep3_if_loop

```
analyzer.list_loops()    → 1 loop
analyzer.list_ifs()      → 1 if (nested in loop body)
analyzer.list_phis()     → 3 PHI instructions
```

### rep4_loop_break

```
analyzer.list_loops()    → 1 loop with multiple exits
analyzer.list_ifs()      → 1 if (for break condition)
```

### rep5_type_prop

```
analyzer.propagate_types(0) → { ValueId: "i64", ... }
// All types should be consistent, no conflicts
```

---

## Structure of MIR JSON

Each `rep_N.mir.json` follows the schema defined in [phase161_joinir_analyzer_design.md](../../docs/development/current/main/phase161_joinir_analyzer_design.md):

```json
{
  "schema_version": "1",
  "functions": [
    {
      "name": "main",
      "blocks": [
        {
          "id": 0,
          "instructions": [
            {
              "op": "const",
              "dest": 1,
              "value": 0
            },
            ...
          ]
        },
        ...
      ],
      "cfg": {
        "entry": 0,
        "targets": { "0": [1], "1": [2, 3], ... }
      }
    }
  ]
}
```

---

## Phase 161 Roadmap Usage

These representatives are used in:

1. **Phase 161-2**: Basic MirAnalyzerBox structure
   - Implement on rep1 and rep2 (simple patterns)

2. **Phase 161-3**: PHI/Loop/If detection
   - Full testing on all 5 representatives

3. **Phase 161-4**: Type propagation
   - Validate rep5_type_prop

4. **Phase 161-5**: Full test suite
   - All representatives passing all analyzer methods

---

## References

- [phase161_analyzer_box_design.md](../../docs/development/current/main/phase161_analyzer_box_design.md) - Analyzer Box design
- [phase161_representative_functions.md](../../docs/development/current/main/phase161_representative_functions.md) - Function selection criteria
- [phase161_joinir_analyzer_design.md](../../docs/development/current/main/phase161_joinir_analyzer_design.md) - JSON schema reference
