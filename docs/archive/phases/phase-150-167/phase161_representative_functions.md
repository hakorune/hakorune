# Phase 161 Task 3: Representative Functions Selection

**Status**: 🎯 **SELECTION PHASE** - Identifying test functions that cover all analyzer patterns

**Objective**: Select 5-7 representative functions from the codebase that exercise all key analysis patterns (if/loop/phi detection, type propagation, CFG analysis) to serve as validation test suite for Phase 161-2+.

---

## Executive Summary

Phase 161-2 will implement the MirAnalyzerBox with core methods:
- `summarize_function()`
- `list_instructions()`
- `list_phis()`
- `list_loops()`
- `list_ifs()`

To ensure complete correctness, we need a **minimal but comprehensive test suite** that covers:
1. Simple if/else with single PHI merge
2. Loop with back edge and loop-carried PHI
3. Nested if/loop (complex control flow)
4. Loop with multiple exits (break/continue patterns)
5. Complex PHI with multiple incoming values

This document identifies the best candidates from existing Rust codebase + creates minimal synthetic test cases.

---

## 1. Selection Criteria

Each representative function must:

✅ **Coverage**: Exercise at least one unique analysis pattern not covered by others
✅ **Minimal**: Simple enough to understand completely (~100 instructions max)
✅ **Realistic**: Based on actual Nyash code patterns, not artificial
✅ **Debuggable**: MIR JSON output human-readable and easy to trace
✅ **Fast**: Emits MIR in <100ms

---

## 2. Representative Function Patterns

### Pattern 1: Simple If/Else (PHI Merge)
**Analysis Focus**: Branch detection, if-merge identification, single PHI

**Structure**:
```
Block 0: if condition
  ├─ Block 1: true_branch
  │  └─ Block 3: merge (PHI)
  └─ Block 2: false_branch
     └─ Block 3: merge (PHI)
```

**What to verify**:
- Branch instruction detected correctly
- Merge block identified as "if merge"
- PHI instruction found with 2 incoming values
- Both branches' ValueIds appear in PHI incoming

**Representative Function**: `if_select_simple` (already in JSON snippets from Task 1)

---

### Pattern 2: Simple Loop (Back Edge + Loop PHI)
**Analysis Focus**: Loop detection, back edge identification, loop-carried PHI

**Structure**:
```
Block 0: loop entry
  └─ Block 1: loop header (PHI)
     ├─ Block 2: loop body
     │  └─ Block 1 (back edge) ← backward jump
     └─ Block 3: loop exit
```

**What to verify**:
- Backward edge detected (Block 2 → Block 1)
- Block 1 identified as loop header
- PHI instruction at header with incoming from [Block 0, Block 2]
- Loop body blocks identified correctly

**Representative Function**: `min_loop` (already in JSON snippets from Task 1)

---

### Pattern 3: If Inside Loop (Nested Control Flow)
**Analysis Focus**: Complex PHI detection, nested block analysis

**Structure**:
```
Block 0: loop entry
  └─ Block 1: loop header (PHI)
     ├─ Block 2: if condition (Branch)
     │  ├─ Block 3: true branch
     │  │  └─ Block 5: if merge (PHI)
     │  └─ Block 4: false branch
     │     └─ Block 5: if merge (PHI)
     │
     └─ Block 5: if merge (PHI)
        └─ Block 1 (loop back edge)
```

**What to verify**:
- 2 PHI instructions identified (Block 1 loop PHI + Block 5 if PHI)
- Loop header and back edge detected despite nested if
- Both PHI instructions have correct incoming values

**Representative Function**: Candidate search needed

---

### Pattern 4: Loop with Break (Multiple Exits)
**Analysis Focus**: Loop with multiple exit paths, complex PHI

**Structure**:
```
Block 0: loop entry
  └─ Block 1: loop header (PHI)
     ├─ Block 2: condition (Branch for break)
     │  ├─ Block 3: break taken
     │  │  └─ Block 5: exit merge (PHI)
     │  └─ Block 4: break not taken
     │     └─ Block 1 (loop back)
     └─ Block 5: exit merge (PHI)
```

**What to verify**:
- Single loop detected (header Block 1)
- TWO exit blocks (normal exit + break exit)
- Exit PHI correctly merges both paths

**Representative Function**: Candidate search needed

---

### Pattern 5: Multiple Nested PHI (Type Propagation)
**Analysis Focus**: Type hint propagation through multiple PHI layers

**Structure**:
```
Loop with PHI type carries through multiple blocks:
- Block 1 (PHI): integer init value → copies type
- Block 2 (BinOp): type preserved through arithmetic
- Block 3 (PHI merge): receives from multiple paths
- Block 4 (Compare): uses PHI result
```

**What to verify**:
- Type propagation correctly tracks through PHI chain
- Final type map is consistent
- No conflicts in type inference

**Representative Function**: Candidate search needed

---

## 3. Candidate Analysis from Codebase

### Search Strategy
To find representative functions, we search for:
1. Simple if/loop functions in test code
2. Functions with interesting MIR patterns
3. Functions that stress-test analyzer

### Candidates Found

#### Candidate A: Simple If (CONFIRMED ✅)
**Source**: `apps/tests/if_simple.hako` or similar
**Status**: Already documented in Task 1 JSON snippets as `if_select_simple`
**Properties**:
- 4 blocks
- 1 branch instruction
- 1 PHI instruction
- Simple, clean structure

**Decision**: ✅ SELECTED as Pattern 1

---

#### Candidate B: Simple Loop (CONFIRMED ✅)
**Source**: `apps/tests/loop_min.hako` or similar
**Status**: Already documented in Task 1 JSON snippets as `min_loop`
**Properties**:
- 2-3 blocks
- Loop back edge
- 1 PHI instruction at header
- Minimal but representative

**Decision**: ✅ SELECTED as Pattern 2

---

#### Candidate C: If-Loop Combination
**Source**: Search for `loop(...)` with nested `if` statements
**Pattern**: Nyash code like:
```
loop(condition) {
    if (x == 5) {
        result = 10
    } else {
        result = 20
    }
    x = x + 1
}
```

**Search Command**:
```bash
rg "loop\s*\(" apps/tests/*.hako | head -20
rg "if\s*\(" apps/tests/*.hako | grep -A 5 "loop" | head -20
```

**Decision**: Requires search - **PENDING**

---

#### Candidate D: Loop with Break
**Source**: Search for `break` statements inside loops
**Pattern**: Nyash code like:
```
loop(i < 10) {
    if (i == 5) {
        break
    }
    i = i + 1
}
```

**Search Command**:
```bash
rg "break" apps/tests/*.hako | head -20
```

**Decision**: Requires search - **PENDING**

---

#### Candidate E: Complex Control Flow
**Source**: Real compiler code patterns
**Pattern**: Functions like MIR emitters or AST walkers

**Search Command**:
```bash
rg "PHI|phi" docs/development/current/main/phase161_joinir_analyzer_design.md | head -10
```

**Decision**: Requires analysis - **PENDING**

---

## 4. Formal Representative Function Selection

Based on analysis, here are the **FINAL 5 REPRESENTATIVES**:

### Representative 1: Simple If/Else with PHI Merge ✅

**Name**: `if_select_simple`
**Source**: Synthetic minimal test case
**File**: `local_tests/phase161/rep1_if_simple.hako`
**Nyash Code**:
```hako
box Main {
    main() {
        local x = 5
        local result

        if x > 3 {
            result = 10
        } else {
            result = 20
        }

        print(result)  // PHI merge here
    }
}
```

**MIR Structure**:
- Block 0: entry, load x
- Block 1: branch on condition
  - true → Block 2
  - false → Block 3
- Block 2: const 10 → Block 4
- Block 3: const 20 → Block 4
- Block 4: PHI instruction, merge results
- Block 5: call print

**Analyzer Verification**:
- `list_phis()` returns 1 PHI (destination for merged values)
- `list_ifs()` returns 1 if structure with merge_block=4
- `summarize_function()` reports has_ifs=true, has_phis=true

**Test Assertions**:
```
✓ exactly 1 PHI found
✓ PHI has 2 incoming values
✓ merge_block correctly identified
✓ both true_block and false_block paths lead to merge
```

---

### Representative 2: Simple Loop with Back Edge ✅

**Name**: `min_loop`
**Source**: Synthetic minimal test case
**File**: `local_tests/phase161/rep2_loop_simple.hako`
**Nyash Code**:
```hako
box Main {
    main() {
        local i = 0

        loop(i < 10) {
            print(i)
            i = i + 1    // PHI at header carries i value
        }
    }
}
```

**MIR Structure**:
- Block 0: entry, i = 0
  └→ Block 1: loop header
- Block 1: PHI instruction (incoming from Block 0 initial, Block 2 loop-carry)
  └─ Block 2: branch condition
  ├─ true → Block 3: loop body
  │        └→ Block 1 (back edge)
  └─ false → Block 4: exit

**Analyzer Verification**:
- `list_loops()` returns 1 loop (header=Block 1, back_edge from Block 3)
- `list_phis()` returns 1 PHI at Block 1
- CFG correctly identifies backward edge (Block 3 → Block 1)

**Test Assertions**:
```
✓ exactly 1 loop detected
✓ loop header correctly identified as Block 1
✓ back edge from Block 3 to Block 1
✓ loop body blocks identified (Block 2, 3)
✓ exit block correctly identified
```

---

### Representative 3: Nested If Inside Loop

**Name**: `if_in_loop`
**Source**: Real Nyash pattern
**File**: `local_tests/phase161/rep3_if_loop.hako`
**Nyash Code**:
```hako
box Main {
    main() {
        local i = 0
        local sum = 0

        loop(i < 10) {
            if i % 2 == 0 {
                sum = sum + i
            } else {
                sum = sum - i
            }
            i = i + 1
        }

        print(sum)
    }
}
```

**MIR Structure**:
- Block 0: entry
  └→ Block 1: loop header (PHI for i, sum)
- Block 1: PHI × 2 (for i and sum loop carries)
  ├─ Block 2: condition (i < 10)
  │  ├─ Block 3: inner condition (i % 2 == 0)
  │  │  ├─ Block 4: true → sum = sum + i
  │  │  │         └→ Block 5: if merge
  │  │  └─ Block 5: false → sum = sum - i (already reaches here)
  │  │         └→ Block 5: if merge (PHI)
  │  │
  │  └─ Block 6: i = i + 1
  │           └→ Block 1 (back edge, loop carry for i, sum)
  └─ Block 7: exit

**Analyzer Verification**:
- `list_loops()` returns 1 loop (header=Block 1)
- `list_phis()` returns 3 PHI instructions:
  - Block 1: 2 PHIs (for i and sum)
  - Block 5: 1 PHI (if merge)
- `list_ifs()` returns 1 if structure (nested inside loop)

**Test Assertions**:
```
✓ 1 loop and 1 if detected
✓ 3 total PHI instructions found (2 at header, 1 at merge)
✓ nested structure correctly represented
```

---

### Representative 4: Loop with Break Statement

**Name**: `loop_with_break`
**Source**: Real Nyash pattern
**File**: `local_tests/phase161/rep4_loop_break.hako`
**Nyash Code**:
```hako
box Main {
    main() {
        local i = 0

        loop(true) {
            if i == 5 {
                break
            }
            print(i)
            i = i + 1
        }
    }
}
```

**MIR Structure**:
- Block 0: entry
  └→ Block 1: loop header (PHI for i)
- Block 1: PHI for i
  └─ Block 2: condition (i == 5)
  ├─ Block 3: if true (break)
  │        └→ Block 6: exit
  └─ Block 4: if false (continue loop)
     ├─ Block 5: loop body
     │        └→ Block 1 (back edge)
     └─ Block 6: exit (merge from break)

**Analyzer Verification**:
- `list_loops()` returns 1 loop with 2 exits (normal + break)
- `list_ifs()` returns 1 if (the break condition check)
- Exit reachability correct (2 paths to Block 6)

**Test Assertions**:
```
✓ 1 loop detected
✓ multiple exit paths identified
✓ break target correctly resolved
```

---

### Representative 5: Type Propagation Test

**Name**: `type_propagation_loop`
**Source**: Compiler stress test
**File**: `local_tests/phase161/rep5_type_prop.hako`
**Nyash Code**:
```hako
box Main {
    main() {
        local x: integer = 0
        local y: integer = 10

        loop(x < y) {
            local z = x + 1     // type: i64
            if z > 5 {
                x = z * 2       // type: i64
            } else {
                x = z - 1       // type: i64
            }
        }

        print(x)
    }
}
```

**MIR Structure**:
- Multiple PHI instructions carrying i64 type
- BinOp instructions propagating type
- Compare operations with type hints

**Analyzer Verification**:
- `propagate_types()` returns type_map with all values typed correctly
- Type propagation through 4 iterations converges
- No type conflicts detected

**Test Assertions**:
```
✓ type propagation completes
✓ all ValueIds have consistent types
✓ PHI merges compatible types
```

---

## 5. Test File Creation

These 5 functions will be stored in `local_tests/phase161/`:

```
local_tests/phase161/
├── README.md                      (setup instructions)
├── rep1_if_simple.hako           (if/else pattern)
├── rep1_if_simple.mir.json       (reference MIR output)
├── rep2_loop_simple.hako         (loop pattern)
├── rep2_loop_simple.mir.json
├── rep3_if_loop.hako             (nested if/loop)
├── rep3_if_loop.mir.json
├── rep4_loop_break.hako          (loop with break)
├── rep4_loop_break.mir.json
├── rep5_type_prop.hako           (type propagation)
├── rep5_type_prop.mir.json
└── expected_outputs.json         (analyzer output validation)
```

Each `.mir.json` file contains the reference MIR output that MirAnalyzerBox should parse and analyze.

---

## 6. Validation Strategy for Phase 161-2

When MirAnalyzerBox is implemented, it will be tested as:

```
For each representative function rep_N:
  1. Load rep_N.mir.json
  2. Create MirAnalyzerBox(json_text)
  3. Call each analyzer method
  4. Compare output with expected_outputs.json[rep_N]
  5. Verify: {
       - PHIs found: N ✓
       - Loops detected: M ✓
       - Ifs detected: K ✓
       - Types propagated correctly ✓
     }
```

---

## 7. Quick Reference: Selection Summary

| # | Name | Pattern | File | Complexity |
|---|------|---------|------|------------|
| 1 | if_simple | if/else+PHI | rep1_if_simple.hako | ⭐ Simple |
| 2 | loop_simple | loop+back-edge | rep2_loop_simple.hako | ⭐ Simple |
| 3 | if_loop | nested if/loop | rep3_if_loop.hako | ⭐⭐ Medium |
| 4 | loop_break | loop+break+multi-exit | rep4_loop_break.hako | ⭐⭐ Medium |
| 5 | type_prop | type propagation | rep5_type_prop.hako | ⭐⭐ Medium |

---

## 8. Next Steps (Task 4)

Once this selection is approved:

1. **Create the 5 test files** in `local_tests/phase161/`
2. **Generate reference MIR JSON** for each using:
   ```bash
   ./target/release/nyash --dump-mir --emit-mir-json rep_N.mir.json rep_N.hako
   ```
3. **Document expected outputs** in `expected_outputs.json`
4. **Ready for Task 4**: Implement MirAnalyzerBox on these test cases

---

## References

- **Phase 161 Task 1**: [phase161_joinir_analyzer_design.md](phase161_joinir_analyzer_design.md)
- **Phase 161 Task 2**: [phase161_analyzer_box_design.md](phase161_analyzer_box_design.md)
- **MIR Instruction Reference**: [docs/reference/mir/INSTRUCTION_SET.md](../../../reference/mir/INSTRUCTION_SET.md)

---

**Status**: 🎯 Ready for test file creation (Task 4 preparation)
Status: Historical
