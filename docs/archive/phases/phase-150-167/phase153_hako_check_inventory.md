# Phase 153: hako_check Inventory (Dead Code Detection Mode)

**Created**: 2025-12-04
**Phase**: 153 (hako_check / dead code detection mode revival)

---

## Executive Summary

This document inventories the current state of `hako_check` as of Phase 153, with focus on:
1. Current execution flow and architecture
2. Existing dead code detection capabilities (HC011, HC012)
3. JoinIR integration status
4. Gaps and opportunities for Phase 153 enhancement

**Key Finding**: hako_check already has partial dead code detection through HC011 (dead methods) and HC012 (dead static boxes), but lacks:
- MIR-level unreachable block detection
- CFG-based reachability analysis
- Integration with JoinIR's control flow information
- Unified `--dead-code` flag for comprehensive dead code reporting

---

## 1. Current Architecture

### 1.1 Execution Flow

```
.hako source file
    ↓
tools/hako_check.sh (bash wrapper)
    ↓
tools/hako_check/cli.hako (VM-executed .hako script)
    ↓
HakoAnalysisBuilderBox.build_from_source_flags()
    ├─ HakoParserCoreBox.parse() (AST generation)
    └─ Text-based scanning (fallback)
    ↓
Analysis IR (MapBox) with:
    - methods: Array<String> (qualified: "Box.method/arity")
    - calls: Array<Map{from, to}>
    - boxes: Array<Map{name, is_static, methods}>
    - entrypoints: Array<String>
    - source: String (original text)
    ↓
Rule execution (19 rules, including HC011/HC012)
    ↓
Diagnostics output (text / JSON-LSP / DOT)
```

### 1.2 Key Components

**Entry Points**:
- `tools/hako_check.sh` - Shell script wrapper with environment setup
- `tools/hako_check/cli.hako` - Main .hako script (HakoAnalyzerBox)

**Core Boxes**:
- `HakoAnalyzerBox` (cli.hako) - Main orchestrator
- `HakoAnalysisBuilderBox` (analysis_consumer.hako) - IR builder
- `HakoParserCoreBox` (tools/hako_parser/parser_core.hako) - AST parser

**Dead Code Rules**:
- `RuleDeadMethodsBox` (rule_dead_methods.hako) - HC011
- `RuleDeadStaticBoxBox` (rule_dead_static_box.hako) - HC012

**IR Format** (MapBox):
```javascript
{
  path: String,
  source: String,
  uses: Array<String>,
  boxes: Array<{
    name: String,
    is_static: Boolean,
    span_line: Integer,
    methods: Array<{
      name: String,
      arity: Integer,
      span: Integer
    }>
  }>,
  methods: Array<String>,  // "Box.method/arity"
  calls: Array<{from: String, to: String}>,
  entrypoints: Array<String>
}
```

---

## 2. Existing Dead Code Detection

### 2.1 HC011: Dead Methods (Unreachable Methods)

**File**: `tools/hako_check/rules/rule_dead_methods.hako`

**Algorithm**:
1. Build adjacency graph from `calls` array
2. DFS from entrypoints (Main.main, main, etc.)
3. Mark visited methods
4. Report unvisited methods as dead

**Limitations**:
- Only detects unreachable methods (function-level granularity)
- No unreachable block detection within functions
- Heuristic-based call detection (text scanning fallback)
- No integration with MIR CFG information

**Test Coverage**:
- `HC011_dead_methods/ng.hako` - Contains unreachable method
- `HC011_dead_methods/ok.hako` - All methods reachable

### 2.2 HC012: Dead Static Box (Unused Static Boxes)

**File**: `tools/hako_check/rules/rule_dead_static_box.hako`

**Algorithm**:
1. Collect all static box names from IR
2. Build set of referenced boxes from calls
3. Report boxes with no references (except Main)

**Limitations**:
- Only detects completely unreferenced boxes
- No detection of boxes with unreachable methods only
- AST span_line support for precise line reporting

**Test Coverage**:
- `HC012_dead_static_box/ng.hako` - Unused static box
- `HC012_dead_static_box/ok.hako` - All boxes referenced

### 2.3 HC016: Unused Alias

**File**: `tools/hako_check/rules/rule_unused_alias.hako`

**Note**: While not strictly "dead code", unused aliases are dead declarations.

---

## 3. JoinIR Integration Status

### 3.1 Current Pipeline (Phase 124 Completion)

**Status**: ✅ **JoinIR-only pipeline established**

As of Phase 124 (completed 2025-12-04), hako_check uses JoinIR exclusively:

```
.hako file
    ↓
Tokenize / Parse (Rust Parser)
    ↓
AST Generation
    ↓
MIR Builder (JoinIR lowering for if/loop)
    ├─ cf_if() → lower_if_form() (JoinIR-based PHI)
    └─ cf_loop() → LoopBuilder (JoinIR-based PHI)
    ↓
MIR Generation (with JoinIR PHI)
    ↓
VM Interpreter
    ↓
hako_check Analysis
```

**Key Points**:
- `NYASH_HAKO_CHECK_JOINIR` flag removed (JoinIR is default)
- No legacy PHI fallback in hako_check path
- All If/Loop constructs use JoinIR lowering

### 3.2 MIR Integration Opportunities

**Current Gap**: hako_check does not consume MIR for dead code analysis

**Opportunity**: Integrate MIR CFG for block-level reachability:
- MIR already has `src/mir/passes/dce.rs` (instruction-level DCE)
- MIR has CFG information (`function.blocks`, `block.terminator`)
- JoinIR lowering provides high-quality PHI nodes for control flow

**Potential Input Formats**:
1. **MIR JSON v0** - Existing format from `--emit-mir-json`
2. **JoinIR JSON** - Direct JoinIR representation
3. **Analysis IR** - Current text-based IR (simplest, current approach)

---

## 4. Related Rust Code (Inventory)

### 4.1 MIR Dead Code Elimination

**File**: `src/mir/passes/dce.rs`

**Purpose**: Instruction-level dead code elimination (DCE)

**Scope**:
- Eliminates unused results of pure instructions
- Works at ValueId level (SSA values)
- Does **not** eliminate unreachable blocks or functions

**Relevance**: Could be extended or adapted for hako_check integration

### 4.2 MIR Verification

**File**: `src/mir/verification/cfg.rs`

**Purpose**: CFG consistency verification

**Relevance**: Contains CFG traversal utilities that could be reused for reachability analysis

### 4.3 MIR Optimizer

**File**: `src/mir/optimizer.rs`

**Purpose**: Multi-pass MIR optimization

**Relevance**: Orchestrates DCE and other passes; pattern for DeadCodeAnalyzerBox

---

## 5. Gaps and Phase 153 Scope

### 5.1 Existing Capabilities ✅

- [x] Dead method detection (HC011)
- [x] Dead static box detection (HC012)
- [x] JoinIR-only pipeline (Phase 124)
- [x] Text-based call graph analysis
- [x] Entrypoint-based DFS reachability

### 5.2 Missing Capabilities (Phase 153 Targets)

#### High Priority
- [ ] **Unreachable block detection** (within functions)
  - Example: `if false { ... }` dead branches
  - Example: Code after unconditional `return`

- [ ] **MIR CFG integration**
  - Use MIR's block graph for precise reachability
  - Detect unreachable blocks post-JoinIR lowering

- [ ] **Unified `--dead-code` flag**
  - Aggregate HC011 + HC012 + new block detection
  - Single command for comprehensive dead code audit

#### Medium Priority
- [ ] **JoinIR-specific analysis**
  - Analyze IfMerge/LoopForm for unreachable paths
  - Detect always-true/always-false conditions

- [ ] **CFG visualization integration**
  - Extend DOT output with unreachable block highlighting
  - `--format dot --dead-code` mode

#### Low Priority (Future Phases)
- [ ] Call graph visualization with dead paths
- [ ] Dataflow-based dead code detection
- [ ] Integration with Phase 160+ .hako JoinIR/MIR migration

---

## 6. Phase 153 Implementation Plan

### 6.1 Minimal Viable Product (MVP)

**Goal**: Revive dead code detection with MIR block-level reachability

**Scope**:
1. Create `DeadCodeAnalyzerBox` (.hako implementation)
2. Input: Analysis IR (current format) + optional MIR JSON
3. Output: Unreachable block reports
4. CLI: Add `--dead-code` flag to aggregate all dead code diagnostics

**Non-Goals** (Phase 153):
- No new environment variables
- No changes to JoinIR/MIR semantics
- No complex dataflow analysis (pure reachability only)

### 6.2 Box-Based Architecture (Phase 133/134 Pattern)

**Pattern**: Modular analyzer box following if_dry_runner.rs precedent

```
DeadCodeAnalyzerBox
├─ analyze_reachability(ir) → UnreachableBlocks
├─ analyze_call_graph(ir) → DeadFunctions
└─ aggregate_report() → Array<Diagnostic>
```

**Characteristics**:
- Self-contained .hako implementation
- No modification to existing rules (HC011/HC012 unchanged)
- Additive enhancement (no breaking changes)

### 6.3 Input Format Decision

**Recommendation**: Start with **Analysis IR** (current format)

**Rationale**:
- Minimally invasive (no new serialization)
- Works with existing hako_check pipeline
- Can extend to MIR JSON in Phase 154+ if needed

**Analysis IR Extensions** (if needed):
```javascript
{
  // ... existing fields ...
  blocks: Array<{
    id: Integer,
    function: String,
    reachable: Boolean,
    terminator: String
  }>
}
```

---

## 7. Test Inventory

### 7.1 Existing Dead Code Tests

**HC011 Tests** (Dead Methods):
- `tools/hako_check/tests/HC011_dead_methods/`
  - `ng.hako` - Method never called
  - `ok.hako` - All methods reachable
  - `expected.json` - Diagnostic expectations

**HC012 Tests** (Dead Static Box):
- `tools/hako_check/tests/HC012_dead_static_box/`
  - `ng.hako` - Box never instantiated
  - `ok.hako` - All boxes used
  - `expected.json` - Diagnostic expectations

### 7.2 Planned Phase 153 Tests

**HC011-B** (Unreachable Block):
```hako
static box Test {
  method demo() {
    if false {
      // This block is unreachable
      print("dead code")
    }
    return 0
  }
}
```

**HC011-C** (Code After Return):
```hako
static box Test {
  method demo() {
    return 0
    print("unreachable")  // Dead code
  }
}
```

**Integration Test** (Comprehensive):
- Combines dead methods, dead boxes, and dead blocks
- Verifies `--dead-code` flag aggregates all findings

---

## 8. CLI Design (Phase 153)

### 8.1 Current CLI

```bash
# Basic analysis
./tools/hako_check.sh target.hako

# Rule filtering
./tools/hako_check.sh --rules dead_methods,dead_static_box target.hako

# JSON-LSP output
./tools/hako_check.sh --format json-lsp target.hako
```

### 8.2 Proposed Phase 153 CLI

```bash
# Enable comprehensive dead code detection
./tools/hako_check.sh --dead-code target.hako

# Dead code only (skip other rules)
./tools/hako_check.sh --rules dead_code target.hako

# Combine with visualization
./tools/hako_check.sh --dead-code --format dot target.hako > cfg.dot
```

**Behavior**:
- `--dead-code`: Enables HC011 + HC012 + new block analysis
- Exit code: Number of dead code findings (0 = clean)
- Compatible with existing `--format` options

---

## 9. Environment Variables (No New Additions)

**Phase 153 Constraint**: No new environment variables

**Existing Variables** (hako_check uses):
- `NYASH_DISABLE_PLUGINS=1` - Required for stability
- `NYASH_BOX_FACTORY_POLICY=builtin_first` - Box resolution
- `NYASH_FEATURES=stage3` - Parser stage
- `NYASH_JSON_ONLY=1` - Pure JSON output (json-lsp mode)

**Decision**: All Phase 153 control via CLI flags only

---

## 10. JoinIR Design Principles Compliance

### 10.1 Read-Only Analysis

✅ **Compliant**: DeadCodeAnalyzerBox only reads IR, does not modify it

### 10.2 No Semantic Changes

✅ **Compliant**: Analysis is post-compilation, no effect on MIR generation

### 10.3 Box-First Modularity

✅ **Compliant**: DeadCodeAnalyzerBox follows established pattern

### 10.4 Fail-Fast (Not Applicable)

N/A: Analysis cannot fail (always produces some result, possibly empty)

---

## 11. Integration with Phase 160+

**Context**: Phase 160+ will migrate .hako sources to JoinIR/MIR

**hako_check Role**: Safety net for migration

**Benefits**:
- Detect dead code introduced during migration
- Verify call graph integrity
- Catch unreachable blocks from refactoring

**Preparation** (Phase 153):
- Establish solid baseline for dead code detection
- Prove DeadCodeAnalyzerBox on current codebase
- Document false positive patterns

---

## 12. Known Limitations (Phase 153)

### 12.1 Dynamic Call Detection

**Limitation**: Text-based call scanning cannot detect dynamic calls

**Example**:
```hako
local method_name = "compute"
// Call via reflection (not detectable by hako_check)
```

**Mitigation**: Document as known limitation

### 12.2 False Positives

**Pattern**: Intentionally unused utility methods

**Example**:
```hako
static box Utils {
  // Future use, not yet called
  method reserved_for_later() { }
}
```

**Mitigation**: Allow suppression comments (future phase)

### 12.3 Cross-Module Analysis

**Limitation**: hako_check analyzes single files only

**Consequence**: Cannot detect dead code exported but unused elsewhere

**Mitigation**: Document as boundary condition

---

## 13. Success Criteria (Phase 153)

### 13.1 Functional Requirements

- [ ] DeadCodeAnalyzerBox implemented in .hako
- [ ] `--dead-code` flag functional
- [ ] HC011 + HC012 + block detection working
- [ ] 2-3 test cases passing
- [ ] Smoke script created

### 13.2 Quality Requirements

- [ ] No regression in existing hako_check tests
- [ ] Box-based modular architecture
- [ ] Documentation updated (this file + hako_check_design.md)
- [ ] Git commit with clear message

### 13.3 Non-Functional Requirements

- [ ] Performance: <5% overhead on existing hako_check
- [ ] Compatibility: Works with all existing CLI options
- [ ] Maintainability: <200 lines for DeadCodeAnalyzerBox

---

## 14. Next Steps (Post-Inventory)

1. **Task 2**: Verify JoinIR-only pipeline (confirm no legacy fallback)
2. **Task 3**: Design DeadCodeAnalyzerBox API and format
3. **Task 4**: Implement DeadCodeAnalyzerBox
4. **Task 5**: Create test cases and smoke script
5. **Task 6**: Update documentation and commit

---

## 15. References

**Related Documents**:
- `phase153_hako_check_deadcode.md` - Phase 153 specification
- `hako_check_design.md` - Current hako_check architecture
- `phase121_integration_roadmap.md` - JoinIR integration history
- `phase124_hako_check_joinir_finalization.md` - JoinIR-only completion

**Related Code**:
- `tools/hako_check/` - Current implementation
- `src/mir/passes/dce.rs` - Rust DCE reference
- `src/mir/verification/cfg.rs` - CFG verification utilities

**Test Fixtures**:
- `tools/hako_check/tests/HC011_dead_methods/`
- `tools/hako_check/tests/HC012_dead_static_box/`

---

**Status**: Inventory Complete ✅
**Next**: Task 2 (JoinIR Pipeline Verification)
Status: Historical
