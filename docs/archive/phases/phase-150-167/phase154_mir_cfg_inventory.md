# Phase 154: MIR/CFG Information Inventory

## Task 1 Results: MIR/CFG Information Investigation

### MIR BasicBlock Structure (from `src/mir/basic_block.rs`)

The MIR already contains rich CFG information:

```rust
pub struct BasicBlock {
    pub id: BasicBlockId,
    pub instructions: Vec<MirInstruction>,
    pub terminator: Option<MirInstruction>,
    pub predecessors: BTreeSet<BasicBlockId>,
    pub successors: BTreeSet<BasicBlockId>,
    pub effects: EffectMask,
    pub reachable: bool,  // Already computed!
    pub sealed: bool,
}
```

**Key findings:**
- CFG edges already tracked via `predecessors` and `successors`
- Block reachability already computed during MIR construction
- Terminators (Branch/Jump/Return) determine control flow

### Terminator Types (from `src/mir/instruction.rs`)

```rust
// Control flow terminators
Branch { condition, then_bb, else_bb }  // Conditional
Jump { target }                          // Unconditional
Return { value }                         // Function exit
```

### Current Analysis IR Structure (from `tools/hako_check/analysis_consumer.hako`)

```javascript
{
  "path": String,
  "uses": Array<String>,
  "boxes": Array<BoxInfo>,
  "methods": Array<String>,
  "calls": Array<CallEdge>,
  "entrypoints": Array<String>,
  "source": String
}
```

**Missing:** CFG/block-level information

## Proposed Analysis IR Extension

### Option A: Add CFG field (Recommended)

```javascript
{
  // ... existing fields ...
  "cfg": {
    "functions": [
      {
        "name": "Main.main/0",
        "entry_block": 0,
        "blocks": [
          {
            "id": 0,
            "reachable": true,
            "successors": [1, 2],
            "terminator": "Branch"
          },
          {
            "id": 1,
            "reachable": true,
            "successors": [3],
            "terminator": "Jump"
          },
          {
            "id": 2,
            "reachable": false,  // <-- Dead block!
            "successors": [3],
            "terminator": "Jump"
          }
        ]
      }
    ]
  }
}
```

**Advantages:**
- Minimal: Only essential CFG data
- Extensible: Can add more fields later
- Backward compatible: Optional field

### Option B: Embed in methods array

```javascript
{
  "methods": [
    {
      "name": "Main.main/0",
      "arity": 0,
      "cfg": { /* ... */ }
    }
  ]
}
```

**Disadvantages:**
- Breaks existing method array format (Array<String>)
- More complex migration

**Decision: Choose Option A**

## CFG Information Sources

### Source 1: MIR Module (Preferred)

**File:** `src/mir/mod.rs`

```rust
pub struct MirModule {
    pub functions: BTreeMap<String, MirFunction>,
    // ...
}

pub struct MirFunction {
    pub blocks: BTreeMap<BasicBlockId, BasicBlock>,
    // ...
}
```

**Access Pattern:**
```rust
for (func_name, function) in &module.functions {
    for (block_id, block) in &function.blocks {
        println!("Block {}: reachable={}", block_id, block.reachable);
        println!("  Successors: {:?}", block.successors);
        println!("  Terminator: {:?}", block.terminator);
    }
}
```

### Source 2: MIR Printer

**File:** `src/mir/printer.rs`

Already has logic to traverse and format CFG:
```rust
pub fn print_function(&self, function: &MirFunction) -> String {
    // Iterates over blocks and prints successors/predecessors
}
```

## Implementation Strategy

### Step 1: Extract CFG during MIR compilation

**Where:** `src/mir/mod.rs` or new `src/mir/cfg_extractor.rs`

```rust
pub fn extract_cfg_info(module: &MirModule) -> serde_json::Value {
    let mut functions = Vec::new();

    for (func_name, function) in &module.functions {
        let mut blocks = Vec::new();

        for (block_id, block) in &function.blocks {
            blocks.push(json!({
                "id": block_id.0,
                "reachable": block.reachable,
                "successors": block.successors.iter()
                    .map(|id| id.0).collect::<Vec<_>>(),
                "terminator": terminator_name(&block.terminator)
            }));
        }

        functions.push(json!({
            "name": func_name,
            "entry_block": function.entry_block.0,
            "blocks": blocks
        }));
    }

    json!({ "functions": functions })
}
```

### Step 2: Integrate into Analysis IR

**File:** `tools/hako_check/analysis_consumer.hako`

Add CFG extraction call:
```hako
// After existing IR building...
if needs_cfg {
    local cfg_info = extract_cfg_from_mir(module)
    ir.set("cfg", cfg_info)
}
```

### Step 3: DeadBlockAnalyzerBox consumes CFG

**File:** `tools/hako_check/rules/rule_dead_blocks.hako`

```hako
static box DeadBlockAnalyzerBox {
    method apply_ir(ir, path, out) {
        local cfg = ir.get("cfg")
        if cfg == null { return }

        local functions = cfg.get("functions")
        local i = 0
        while i < functions.size() {
            local func = functions.get(i)
            me._analyze_function_blocks(func, path, out)
            i = i + 1
        }
    }

    _analyze_function_blocks(func, path, out) {
        local blocks = func.get("blocks")
        local func_name = func.get("name")

        local bi = 0
        while bi < blocks.size() {
            local block = blocks.get(bi)
            local reachable = block.get("reachable")

            if reachable == 0 {
                local msg = "[HC020] Unreachable block: fn=" + func_name
                          + " bb=" + me._itoa(block.get("id"))
                out.push(msg + " :: " + path)
            }

            bi = bi + 1
        }
    }
}
```

## JoinIR Strict Mode Compatibility

**Question:** Does `NYASH_JOINIR_STRICT=1` affect CFG structure?

**Answer:** No. CFG is computed **after** JoinIR lowering in `MirBuilder`:
1. JoinIR → MIR lowering (produces blocks with terminators)
2. CFG computation (fills predecessors/successors from terminators)
3. Reachability analysis (marks unreachable blocks)

**Verification needed:** Test with Phase 150 representative cases:
- `peek_expr_block.hako` - Match expressions
- `loop_min_while.hako` - Loop with PHI
- `joinir_min_loop.hako` - Break control
- `joinir_if_select_simple.hako` - Early return

## Next Steps

1. ✅ Create `src/mir/cfg_extractor.rs` - Extract CFG to JSON
2. ⏳ Modify `analysis_consumer.hako` - Add CFG field
3. ⏳ Implement `rule_dead_blocks.hako` - DeadBlockAnalyzerBox
4. ⏳ Create test cases - 4 dead block patterns
5. ⏳ Update CLI - Add `--dead-blocks` flag

---

**Created:** 2025-12-04
**Phase:** 154 (MIR CFG Integration & Dead Block Detection)
**Status:** Task 1 Complete
Status: Historical
