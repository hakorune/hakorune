# Phase 161 Task 2: Analyzer Box Design (JoinIrAnalyzerBox / MirAnalyzerBox)

**Status**: 🎯 **DESIGN PHASE** - Defining .hako Analyzer Box structure and responsibilities

**Objective**: Design the foundational .hako Boxes for analyzing Rust JSON MIR/JoinIR data, establishing clear responsibilities and API contracts.

---

## Executive Summary

Phase 161 aims to port JoinIR analysis logic from Rust to .hako. The first step was creating a complete JSON format inventory (Task 1, completed). Now we design the .hako Box architecture that will consume this data.

**Key Design Decision**: Create **TWO specialized Analyzer Boxes** with distinct, non-overlapping responsibilities:
1. **MirAnalyzerBox**: Analyzes MIR JSON v1 (primary)
2. **JoinIrAnalyzerBox**: Analyzes JoinIR JSON v0 (secondary, for compatibility)

Both boxes will share a common **JsonParserBox** utility for low-level JSON parsing operations.

---

## 1. Core Architecture: Box Responsibilities

### 1.1 JsonParserBox (Shared Utility)

**Purpose**: Low-level JSON parsing and traversal (reusable across both analyzers)

**Scope**: Single-minded JSON access without semantic analysis

**Responsibilities**:
- Parse JSON text into MapBox/ArrayBox structure
- Provide recursive accessor methods: `get()`, `getArray()`, `getInt()`, `getString()`
- Handle type conversions safely with nullability
- Provide iteration helpers: `forEach()`, `map()`, `filter()`

**Key Methods**:
```
birth(jsonText)              // Parse JSON from string
get(path: string): any       // Get nested value by dot-notation (e.g., "functions/0/blocks")
getArray(path): ArrayBox     // Get array at path with type safety
getString(path): string      // Get string with default ""
getInt(path): integer        // Get integer with default 0
getBool(path): boolean       // Get boolean with default false
```

**Non-Scope**: Semantic analysis, MIR-specific validation, JoinIR-specific knowledge

---

### 1.2 MirAnalyzerBox (Primary Analyzer)

**Purpose**: Analyze MIR JSON v1 according to Phase 161 specifications

**Scope**: All MIR-specific analysis operations

**Responsibilities**:
1. **Schema Validation**: Verify MIR JSON has required fields (schema_version, functions, cfg)
2. **Instruction Type Detection**: Identify instruction types (14 types in MIR v1)
3. **PHI Detection**: Identify PHI instructions and extract incoming values
4. **Loop Detection**: Identify loops via backward edge analysis (CFG)
5. **If Detection**: Identify conditional branches and PHI merge points
6. **Type Analysis**: Propagate type hints through PHI/BinOp/Compare operations
7. **Reachability Analysis**: Mark unreachable blocks (dead code detection)

**Key Methods** (Single-Function Analysis):
```
birth(mirJsonText)                                   // Parse MIR JSON

// === Schema Validation ===
validateSchema(): boolean                            // Check MIR v1 structure

// === Function-Level Analysis ===
summarize_function(funcIndex: integer): MapBox      // Returns:
                                                    // {
                                                    //   name: string,
                                                    //   params: integer,
                                                    //   blocks: integer,
                                                    //   instructions: integer,
                                                    //   has_loops: boolean,
                                                    //   has_ifs: boolean,
                                                    //   has_phis: boolean
                                                    // }

// === Instruction Detection ===
list_instructions(funcIndex): ArrayBox              // Returns array of:
                                                    // {
                                                    //   block_id: integer,
                                                    //   inst_index: integer,
                                                    //   op: string,
                                                    //   dest: integer (ValueId),
                                                    //   src1, src2: integer (ValueId)
                                                    // }

// === PHI Analysis ===
list_phis(funcIndex): ArrayBox                      // Returns array of PHI instructions:
                                                    // {
                                                    //   block_id: integer,
                                                    //   dest: integer (ValueId),
                                                    //   incoming: ArrayBox of
                                                    //     [value_id, from_block_id]
                                                    // }

// === Loop Detection ===
list_loops(funcIndex): ArrayBox                     // Returns array of loop structures:
                                                    // {
                                                    //   header_block: integer,
                                                    //   exit_block: integer,
                                                    //   back_edge_from: integer,
                                                    //   contains_blocks: ArrayBox
                                                    // }

// === If Detection ===
list_ifs(funcIndex): ArrayBox                       // Returns array of if structures:
                                                    // {
                                                    //   condition_block: integer,
                                                    //   condition_value: integer (ValueId),
                                                    //   true_block: integer,
                                                    //   false_block: integer,
                                                    //   merge_block: integer
                                                    // }

// === Type Analysis ===
propagate_types(funcIndex): MapBox                  // Returns type map:
                                                    // {
                                                    //   value_id: type_string
                                                    //   (e.g., "i64", "void", "boxref")
                                                    // }

// === Control Flow Analysis ===
reachability_analysis(funcIndex): ArrayBox          // Returns:
                                                    // {
                                                    //   reachable_blocks: ArrayBox,
                                                    //   unreachable_blocks: ArrayBox
                                                    // }
```

**Key Algorithms**:

#### PHI Detection Algorithm
```
For each block in function:
  For each instruction in block:
    If instruction.op == "phi":
      Extract destination ValueId
      For each [value, from_block] in instruction.incoming:
        Record PHI merge point
      Mark block as PHI merge block
```

#### Loop Detection Algorithm (CFG-based)
```
Build adjacency list from CFG (target → [from_blocks])
For each block B:
  For each successor S in B:
    If S's block_id < B's block_id:
      Found backward edge B → S
      S is loop header
      Find all blocks in loop via DFS from S
      Record loop structure
```

#### If Detection Algorithm
```
For each block B with Branch instruction:
  condition = branch.condition (ValueId)
  true_block = branch.targets[0]
  false_block = branch.targets[1]

  For each successor block S of true_block OR false_block:
    If S has PHI instruction with incoming from both true_block AND false_block:
      S is the merge block
      Record if structure
```

#### Type Propagation Algorithm
```
Initialize: type_map[v] = v.hint (from Const/Compare/BinOp)
Iterate 4 times:  // Maximum iterations before convergence
  For each PHI instruction:
    incoming_types = [type_map[v] for each [v, _] in phi.incoming]
    Merge types: take most specific common type
    type_map[phi.dest] = merged_type

  For each BinOp/Compare/etc:
    Propagate operand types to result
```

---

### 1.3 JoinIrAnalyzerBox (Secondary Analyzer)

**Purpose**: Analyze JoinIR JSON v0 (CPS-style format)

**Scope**: JoinIR-specific analysis operations

**Responsibilities**:
1. **Schema Validation**: Verify JoinIR JSON has required fields
2. **Continuation Extraction**: Parse CPS-style continuation structures
3. **Direct Conversion to MIR**: Transform JoinIR JSON to MIR-compatible format
4. **Backward Compatibility**: Support legacy JoinIR analysis workflows

**Key Methods**:
```
birth(joinirJsonText)                               // Parse JoinIR JSON

validateSchema(): boolean                            // Check JoinIR v0 structure

// === JoinIR-Specific Analysis ===
list_continuations(funcIndex): ArrayBox            // Returns continuation structures

// === Conversion ===
convert_to_mir(funcIndex): string                  // Returns MIR JSON equivalent
                                                   // (enables reuse of MirAnalyzerBox)
```

**Note on Design**: JoinIrAnalyzerBox is intentionally minimal - its primary purpose is converting JoinIR to MIR format, then delegating to MirAnalyzerBox for actual analysis. This avoids code duplication.

---

## 2. Shared Infrastructure

### 2.1 AnalyzerCommonBox (Base Utilities)

**Purpose**: Common helper methods used by both analyzers

**Key Methods**:
```
// === Utility Methods ===
extract_function(funcIndex: integer): MapBox       // Extract single function data
extract_cfg(funcIndex: integer): MapBox             // Extract CFG for block analysis
build_adjacency_list(cfg): MapBox                  // Build block→blocks adjacency

// === Debugging/Tracing ===
set_verbose(enabled: boolean)                      // Enable detailed output
dump_function(funcIndex): string                   // Pretty-print function data
dump_cfg(funcIndex): string                        // Pretty-print CFG
```

---

## 3. Data Flow Architecture

```
JSON Input (MIR or JoinIR)
    ↓
JsonParserBox (Parse to MapBox/ArrayBox)
    ↓
    ├─→ MirAnalyzerBox → Semantic Analysis
    │       ↓
    │   (PHI detection, loop detection, etc.)
    │       ↓
    │   Analysis Results (ArrayBox/MapBox)
    │
    └─→ JoinIrAnalyzerBox → Convert to MIR
            ↓
        (Transform JoinIR → MIR)
            ↓
        MirAnalyzerBox (reuse)
            ↓
        Analysis Results
```

---

## 4. API Contract: Method Signatures (Finalized)

### MirAnalyzerBox

```hako
static box MirAnalyzerBox {
    // Parser state
    parsed_mir: MapBox
    json_parser: JsonParserBox

    // Analysis cache
    func_cache: MapBox          // Memoization for expensive operations
    verbose_mode: BoolBox

    // Constructor
    birth(mir_json_text: string) {
        me.parsed_mir = JsonParserBox.parse(mir_json_text)
        me.json_parser = new JsonParserBox()
        me.func_cache = new MapBox()
        me.verbose_mode = false
    }

    // === Validation ===
    validateSchema(): BoolBox {
        // Returns true if MIR v1 schema valid
    }

    // === Analysis Methods ===
    summarize_function(funcIndex: IntegerBox): MapBox {
        // Returns { name, params, blocks, instructions, has_loops, has_ifs, has_phis }
    }

    list_instructions(funcIndex: IntegerBox): ArrayBox {
        // Returns array of { block_id, inst_index, op, dest, src1, src2 }
    }

    list_phis(funcIndex: IntegerBox): ArrayBox {
        // Returns array of { block_id, dest, incoming }
    }

    list_loops(funcIndex: IntegerBox): ArrayBox {
        // Returns array of { header_block, exit_block, back_edge_from, contains_blocks }
    }

    list_ifs(funcIndex: IntegerBox): ArrayBox {
        // Returns array of { condition_block, condition_value, true_block, false_block, merge_block }
    }

    propagate_types(funcIndex: IntegerBox): MapBox {
        // Returns { value_id: type_string }
    }

    reachability_analysis(funcIndex: IntegerBox): ArrayBox {
        // Returns { reachable_blocks, unreachable_blocks }
    }

    // === Debugging ===
    set_verbose(enabled: BoolBox) { }
    dump_function(funcIndex: IntegerBox): StringBox { }
    dump_cfg(funcIndex: IntegerBox): StringBox { }
}
```

### JsonParserBox

```hako
static box JsonParserBox {
    root: MapBox

    birth(json_text: string) {
        // Parse JSON text into MapBox/ArrayBox structure
    }

    get(path: string): any {
        // Get value by dot-notation path
    }

    getArray(path: string): ArrayBox { }
    getString(path: string): string { }
    getInt(path: string): integer { }
    getBool(path: string): boolean { }
}
```

---

## 5. Implementation Strategy

### Phase 161-2: Basic MirAnalyzerBox Structure (First Iteration)

**Scope**: Get basic structure working, focus on `summarize_function()` and `list_instructions()`

1. Implement JsonParserBox (simple recursive MapBox builder)
2. Implement MirAnalyzerBox.birth() to parse MIR JSON
3. Implement validateSchema() to verify structure
4. Implement summarize_function() (basic field extraction)
5. Implement list_instructions() (iterate blocks, extract instructions)

**Success Criteria**:
- Can parse MIR JSON test files
- Can extract function metadata
- Can list all instructions in order

---

### Phase 161-3: PHI/Loop/If Detection

**Scope**: Advanced control flow analysis

1. Implement list_phis() using pattern matching
2. Implement list_loops() using CFG and backward edge detection
3. Implement list_ifs() using condition and merge detection
4. Test on representative functions

**Success Criteria**:
- Correctly identifies all PHI instructions
- Correctly detects loop header and back edges
- Correctly identifies if/merge structures

---

### Phase 161-4: Type Propagation

**Scope**: Type hint system

1. Implement type extraction from Const/Compare/BinOp
2. Implement 4-iteration propagation algorithm
3. Build type map for ValueId

**Success Criteria**:
- Type map captures all reachable types
- No type conflicts or inconsistencies

---

### Phase 161-5: Analysis Features

**Scope**: Extended functionality

1. Implement reachability analysis (mark unreachable blocks)
2. Implement dump methods for debugging
3. Add caching to expensive operations

---

## 6. Representative Functions for Testing

Per Task 3 selection criteria, these functions will be used for Phase 161-2+ validation:

1. **if_select_simple** (Simple if/else with PHI)
   - 4 BasicBlocks
   - 1 Branch instruction
   - 1 PHI instruction at merge
   - Type: Simple if pattern

2. **min_loop** (Minimal loop with PHI)
   - 2 BasicBlocks (header + body)
   - Loop back edge
   - PHI instruction at header
   - Type: Loop pattern

3. **skip_ws** (From JoinIR, more complex)
   - 6+ BasicBlocks
   - Nested control flow
   - Multiple PHI instructions
   - Type: Complex pattern

**Usage**: Each will be analyzed by MirAnalyzerBox to verify correctness of detection algorithms.

---

## 7. Design Principles Applied

### 🏗️ 箱にする (Boxification)
- Each analyzer box has single responsibility
- Clear API boundary (methods) with defined input/output contracts
- No shared mutable state between boxes

### 🌳 境界を作る (Clear Boundaries)
- JsonParserBox: Low-level JSON only
- MirAnalyzerBox: MIR semantics only
- JoinIrAnalyzerBox: JoinIR conversion only
- No intermingling of concerns

### ⚡ Fail-Fast
- validateSchema() must pass or error (no silent failures)
- Invalid instruction types cause immediate error
- Type propagation inconsistencies detected and reported

### 🔄 遅延シングルトン (Lazy Evaluation)
- Each method computes its result on-demand
- Results are cached in func_cache to avoid recomputation
- No pre-computation of unnecessary analysis

---

## 8. Questions Answered by This Design

**Q: Why two separate analyzer boxes?**
A: MIR and JoinIR have fundamentally different schemas. Separate boxes with clear single responsibilities are easier to test, maintain, and extend.

**Q: Why separate JsonParserBox?**
A: JSON parsing is orthogonal to semantic analysis. Extracting it enables reuse and makes testing easier.

**Q: Why caching?**
A: Control flow analysis is expensive (CFG traversal, reachability). Caching prevents redundant computation when multiple methods query the same data.

**Q: Why 4 iterations for type propagation?**
A: Based on Phase 25 experience - 4 iterations handles most practical programs. Documented in phase161_joinir_analyzer_design.md.

---

## 9. Next Steps (Task 3)

Once this design is approved:

1. **Task 3**: Formally select 3-5 representative functions that cover all detection patterns
2. **Task 4**: Implement basic .hako JsonParserBox and MirAnalyzerBox
3. **Task 5**: Create joinir_analyze.sh CLI entry point
4. **Task 6**: Test on representative functions
5. **Task 7**: Update CURRENT_TASK.md and roadmap

---

## 10. References

- **Phase 161 Task 1**: [phase161_joinir_analyzer_design.md](phase161_joinir_analyzer_design.md) - JSON schema inventory
- **Phase 173-B**: [phase173b-boxification-assessment.md](phase173b-boxification-assessment.md) - Boxification design principles
- **MIR INSTRUCTION_SET**: [docs/reference/mir/INSTRUCTION_SET.md](../../../reference/mir/INSTRUCTION_SET.md)
- **Box System**: [docs/reference/boxes-system/](../../../reference/boxes-system/)

---

**Status**: 🎯 Ready for Task 3 approval and representative function selection
Status: Historical
