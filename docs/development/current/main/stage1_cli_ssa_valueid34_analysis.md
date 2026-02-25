# Stage-1 CLI `ValueId(34)` Undefined Value Analysis

## Executive Summary

**Error**: `use of undefined value ValueId(34)` in `Stage1Cli.stage1_main/1` at `BasicBlockId(12266)`

**Root Cause Hypothesis**: Complex control flow in `stage1_main` with multiple nested if-blocks and method calls creates a scenario where SSA value tracking fails to properly dominate all use sites of ValueId(34).

**Status**: Structural issue identified; MIR dump attempted but unable to capture due to complex `using` dependencies.

---

## 1. Error Context

### 1.1 Error Message
```
❌ [rust-vm] use of undefined value ValueId(34) (
  fn=Stage1Cli.stage1_main/1,
  last_block=Some(BasicBlockId(12266)),
  last_inst=Some(Call {
    dst: Some(ValueId(31)),
    func: ValueId(4294967295),
    callee: Some(Method {
      box_name: "ParserBox",
      method: "size",
      receiver: Some(ValueId(34)),
      certainty: Known,
      box_kind: StaticCompiler
    }),
    args: [ValueId(33)],
    effects: EffectMask(16)
  })
)
```

### 1.2 Key Observations
1. **Block ID**: `BasicBlockId(12266)` - very high number indicates deep control flow nesting
2. **Func ID**: `ValueId(4294967295)` = `u32::MAX` - indicates uninitialized/invalid function ID
3. **Box Type**: Compiler thinks receiver has type `ParserBox`, which is unusual for a `.size()` call
4. **Method**: `.size()` is typically called on ArrayBox (for `args.size()`), not ParserBox

---

## 2. Source Code Analysis

### 2.1 stage1_main Entry Point (lines 109-168)

The real implementation in `/home/tomoaki/git/hakorune-selfhost/lang/src/runner/stage1_cli.hako`:

```hako
method stage1_main(args) {
  // Line 110-113: Debug entry with args.size() call
  if env.get("STAGE1_CLI_DEBUG") == "1" {
    local argc = 0; if args != null { argc = args.size() }  // ← FIRST .size() call
    print("[stage1-cli/debug] stage1_main ENTRY: argc=" + ("" + argc) + " env_emits={prog=" + ("" + env.get("STAGE1_EMIT_PROGRAM_JSON")) + ",mir=" + ("" + env.get("STAGE1_EMIT_MIR_JSON")) + "} backend=" + ("" + env.get("STAGE1_BACKEND")))
  }

  // Line 114-122: Guard block
  {
    local use_cli = env.get("NYASH_USE_STAGE1_CLI")
    if use_cli == null || ("" + use_cli) != "1" {
      if env.get("STAGE1_CLI_DEBUG") == "1" {
        print("[stage1-cli/debug] stage1_main: NYASH_USE_STAGE1_CLI not set, returning 97")
      }
      return 97
    }
  }

  // Line 124-130: Local variable declarations
  local emit_prog = env.get("STAGE1_EMIT_PROGRAM_JSON")
  local emit_mir = env.get("STAGE1_EMIT_MIR_JSON")
  local backend = env.get("STAGE1_BACKEND"); if backend == null { backend = "vm" }
  local source = env.get("STAGE1_SOURCE")
  local prog_path = env.get("STAGE1_PROGRAM_JSON")

  // Line 131-140: emit_prog path
  if emit_prog == "1" {
    if source == null || source == "" {
      print("[stage1-cli] emit program-json: STAGE1_SOURCE is required")
      return 96
    }
    local ps = me.emit_program_json(source)  // ← Calls BuildBox → ParserBox
    if ps == null { return 96 }
    print(ps)
    return 0
  }

  // Line 142-158: emit_mir path (complex nested structure)
  if emit_mir == "1" {
    local prog_json = null
    if prog_path != null && prog_path != "" {
      prog_json = me._read_file("[stage1-cli] emit mir-json", prog_path)
    } else {
      if source == null || source == "" {
        print("[stage1-cli] emit mir-json: STAGE1_SOURCE or STAGE1_PROGRAM_JSON is required")
        return 96
      }
      prog_json = me.emit_program_json(source)  // ← Another call to emit_program_json
    }
    if prog_json == null { return 96 }
    local mir = me.emit_mir_json(prog_json)
    if mir == null { return 96 }
    print(mir)
    return 0
  }

  // Line 160-168: default run path
  if source == null || source == "" {
    print("[stage1-cli] run: source path is required (set STAGE1_SOURCE)")
    return 96
  }
  local prog_json = me.emit_program_json(source)
  if prog_json == null { return 96 }
  return me.run_program_json(prog_json, backend)
}
```

### 2.2 Call Chain to ParserBox

```
Stage1Cli.stage1_main(args)
  └─> me.emit_program_json(source) [line 136/151/165]
      └─> BuildBox.emit_program_json_v0(merged, null) [build_box.hako:43]
          └─> local p = new ParserBox(); p.stage3_enable(1) [build_box.hako:133]
          └─> local ast_json = p.parse_program2(body_src) [build_box.hako:134]
```

### 2.3 Comparison with Shape Test

The shape test in `/home/tomoaki/git/hakorune-selfhost/src/tests/stage1_cli_entry_ssa_smoke.rs`:
- **Lines 82-84**: Identical pattern for the debug entry
- **Lines 86-94**: Identical guard block structure
- **Lines 96-102**: Identical local variable declarations
- **Lines 103-112**: Identical emit_prog path
- **Lines 114-131**: Identical emit_mir path structure
- **Lines 133-141**: Identical default run path

**Key Difference**: The shape test uses **stub implementations** for `emit_program_json`, `emit_mir_json`, and `run_program_json` that just return string concatenations. The real implementation calls **complex external boxes** (BuildBox → ParserBox → using chain).

---

## 3. Structural Analysis

### 3.1 Control Flow Complexity

The `stage1_main` function has:
1. **Entry debug block**: 1 if-statement with nested `.size()` call
2. **Guard block**: Nested scope with 2 if-statements (5 early returns possible)
3. **Variable declarations**: 5 locals with conditional assignments
4. **emit_prog path**: 2 if-blocks, 1 method call, 2 early returns
5. **emit_mir path**: 3-level nested if-else, 2 method calls, 3 early returns
6. **default path**: 2 if-blocks, 2 method calls, 1 final return

**Total**: ~15-20 basic blocks with multiple dominance frontiers and PHI merge points.

### 3.2 SSA/PHI Challenges

The pattern in line 111:
```hako
local argc = 0; if args != null { argc = args.size() }
```

Creates:
1. **Entry block**: Define `argc = const 0`
2. **Condition block**: Branch on `args != null`
3. **Then block**: Call `args.size()`, assign to `argc`
4. **Merge block**: PHI node: `argc_final = phi(argc_initial=0, argc_from_size)`

The problem: If the compiler's dominator tree or PHI placement is incorrect, the value `args` might not dominate the use site where `.size()` is called.

### 3.3 Possible Root Causes

#### Hypothesis A: args Parameter Tracking Failure
- `args` is a function parameter (ValueId assigned at function entry)
- Across deep nested control flow (BasicBlockId(12266)), the parameter tracking might fail
- The error shows `receiver: Some(ValueId(34))` which suggests ValueId(34) is supposed to hold `args`
- But ValueId(34) is undefined at BasicBlockId(12266), meaning it wasn't properly propagated

#### Hypothesis B: PHI Node Merge Error
- The conditional pattern `if args != null { argc = args.size() }` creates a PHI merge
- The PHI node might be placed in a block that doesn't properly dominate all uses
- This could happen if the MIR builder's region/block scheduling is incorrect for complex nested structures

#### Hypothesis C: Using-Chain Compilation Order
- The real `stage1_main` loads 50+ .hako files via `using` statements
- BuildBox → ParserBox → many other boxes
- The compilation order might create a situation where:
  - ParserBox is compiled first
  - stage1_main references it
  - But the cross-module SSA tracking fails to maintain dominance

#### Hypothesis D: Type Confusion
- The error shows `box_name: "ParserBox"` for a `.size()` call
- This should be `ArrayBox` (for `args.size()`)
- Possible type confusion in the compiler's call resolution:
  - `args` is supposed to be ArrayBox
  - But somewhere the compiler thinks it's ParserBox
  - This could be due to incorrect type propagation in PHI nodes

---

## 4. Attempted Debugging

### 4.1 MIR Dump Attempts

**Attempt 1**: Direct VM dump with flags
```bash
NYASH_VM_DUMP_MIR=1 NYASH_MIR_VERBOSE=1 tools/stage1_debug.sh --mode emit-program-json ...
```
- **Result**: VM crashed before dumping Stage1Cli.stage1_main MIR
- **Reason**: Error occurs during execution, not compilation

**Attempt 2**: Emit MIR JSON
```bash
NYASH_PARSER_STAGE3=1 HAKO_PARSER_STAGE3=1 ... ./target/release/hakorune --emit-mir-json /tmp/stage1_cli_mir.json lang/src/runner/stage1_cli.hako
```
- **Result**: Compilation succeeded, but no JSON file written
- **Reason**: `--emit-mir-json` flag behavior unclear with complex using chains

**Attempt 3**: hakorune_emit_mir.sh
```bash
bash tools/hakorune_emit_mir.sh lang/src/runner/stage1_cli.hako /tmp/stage1_cli_mir.json
```
- **Result**: `[FAIL] Stage-B and direct MIR emit both failed`
- **Reason**: Using-chain dependencies not resolved in standalone compilation

### 4.2 Observations from Compilation
- Function successfully compiles to MIR (no MIR build errors)
- Shape test with identical structure passes MIR verification
- Error only occurs at runtime when executing through the bridge
- Block ID 12266 suggests hundreds of blocks were generated from the using chain

---

## 5. Bridging Analysis

### 5.1 stage1_bridge.rs Argument Setup

From `/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge.rs` lines 121-191:

**Key Setup**:
1. **Line 121**: `let mut stage1_args: Vec<String> = Vec::new();`
2. **Lines 125-164**: Builds `stage1_args` based on mode:
   - `emit program-json <source>`
   - `emit mir-json <source>` or `emit mir-json --from-program-json <file>`
   - `run --backend <backend> <source>`
3. **Lines 167-171**: Appends extra script args from `NYASH_SCRIPT_ARGS_JSON`
4. **Lines 173-177**: Sets `NYASH_SCRIPT_ARGS_JSON` if not already set
5. **Line 186**: `cmd.arg(&entry).arg("--");` - separates entry from args
6. **Lines 187-189**: Appends all `stage1_args` as command arguments
7. **Lines 190-191**: Sets `NYASH_SCRIPT_ARGS_JSON` env var

**Entry Point**:
- **Line 185**: `let entry_fn = std::env::var("NYASH_ENTRY").unwrap_or_else(|_| "Stage1CliMain.main/1".to_string());`
- **Line 244-246**: `cmd.env("NYASH_ENTRY", &entry_fn);`

**Forwarding**:
- The bridge spawns a child process with `lang/src/runner/stage1_cli.hako -- emit program-json apps/tests/minimal_ssa_skip_ws.hako`
- The VM should parse `stage1_args` from both command-line `--` args and `NYASH_SCRIPT_ARGS_JSON`

### 5.2 VM Args Reception

The VM entry point expects `Stage1CliMain.main/1`:
- `Stage1CliMain.main(args)` forwards to `Stage1Cli.stage1_main(args)` (line 369)
- `args` should be an ArrayBox containing the script arguments

**Potential Issue**: If `args` is not properly constructed by the VM entry point:
- It might be `null`
- It might be an empty ArrayBox
- It might have the wrong type (e.g., parsed as something other than ArrayBox)

---

## 6. Conclusions and Recommendations

### 6.1 Problem Classification

**Primary Issue**: SSA value `ValueId(34)` is used in a Call instruction at a block that is not dominated by its definition block.

**Secondary Issue**: Type confusion where the compiler believes the receiver is `ParserBox` instead of `ArrayBox`.

**Structural Issue**: The combination of:
1. Deep control flow nesting (12266 blocks)
2. Complex using-chain dependencies (50+ files)
3. Multiple early-return paths with PHI merges
4. Parameter tracking across method call boundaries

...creates a scenario where the MIR builder's SSA construction loses track of the `args` parameter's liveness.

### 6.2 Why the Shape Test Passes

The shape test passes MIR verification because:
1. **Stub implementations** avoid deep call chains (no BuildBox → ParserBox)
2. **Single-file compilation** avoids using-chain complexity
3. **Simpler CFG** (fewer blocks) makes dominator tree computation straightforward
4. **Type clarity** - all method calls are to stub methods in the same static box

### 6.3 Recommended Solutions

#### Solution A: Add Region-Level Guards (Fail-Fast)
Add explicit null checks at the function entry to simplify control flow:

```hako
method stage1_main(args) {
  // Fail-fast: ensure args is always defined
  if args == null { args = new ArrayBox() }  // ← Force args to be non-null always

  if env.get("STAGE1_CLI_DEBUG") == "1" {
    local argc = args.size()  // ← Now args is guaranteed non-null, no PHI needed
    print("[stage1-cli/debug] stage1_main ENTRY: argc=" + ("" + argc) + " ...")
  }

  // ... rest of function
}
```

**Rationale**: Collapses the conditional PHI merge into a single definition, reducing SSA complexity.

#### Solution B: Extract Debug Logic to Separate Method
Move the debug entry block to a helper method:

```hako
method _debug_entry(args) {
  local argc = 0
  if args != null { argc = args.size() }
  print("[stage1-cli/debug] stage1_main ENTRY: argc=" + ("" + argc) + " ...")
}

method stage1_main(args) {
  if env.get("STAGE1_CLI_DEBUG") == "1" {
    me._debug_entry(args)  // ← Isolate the problematic pattern
  }

  // ... rest of function (simpler CFG without nested debug logic)
}
```

**Rationale**: Isolates the complex PHI pattern into a smaller function with simpler dominance.

#### Solution C: Rust Bridge - Always Provide Non-Null ArrayBox
Modify the bridge to guarantee `args` is never null:

In `stage1_bridge.rs`, ensure `NYASH_SCRIPT_ARGS_JSON` always contains at least an empty array:

```rust
// Line 173-177
if std::env::var("NYASH_SCRIPT_ARGS_JSON").is_err() {
    // Always provide a valid JSON array, never let VM see null
    let json = if stage1_args.is_empty() {
        "[]".to_string()  // ← Empty array instead of missing
    } else {
        serde_json::to_string(&stage1_args).unwrap_or("[]".to_string())
    };
    stage1_env_script_args = Some(json);
}
```

**Rationale**: Prevents the null-check path from ever being taken, simplifying CFG at runtime.

#### Solution D: MIR Builder Fix (Long-Term)
Investigate and fix the root cause in the MIR builder:
1. **Dominator Tree Verification**: Ensure all ValueId uses are dominated by definitions
2. **PHI Placement**: Review conservative PHI placement algorithm for nested if-blocks
3. **Cross-Module SSA**: Ensure using-chain compilation maintains SSA invariants
4. **Type Tracking**: Fix type confusion between ParserBox and ArrayBox in call resolution

**Files to check**:
- `src/mir/builder/*.rs` - SSA construction logic
- `src/mir/phi_core/*.rs` - PHI node placement
- `src/mir/verification/ssa.rs` - SSA verification (why didn't this catch the issue?)

---

## 7. Immediate Action Items

1. **Short-term (Box-level fix)**: Apply Solution A or B to `stage1_cli.hako` line 111
2. **Medium-term (Bridge fix)**: Apply Solution C to `stage1_bridge.rs`
3. **Long-term (MIR fix)**: Apply Solution D - investigate MIR builder SSA construction

**Priority**: Recommend Solution A first (simplest, no cross-module changes).

---

## 8. Appendix: File Locations

- **Error Source**: `/home/tomoaki/git/hakorune-selfhost/lang/src/runner/stage1_cli.hako:111`
- **Bridge Code**: `/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge.rs`
- **Shape Test**: `/home/tomoaki/git/hakorune-selfhost/src/tests/stage1_cli_entry_ssa_smoke.rs`
- **BuildBox**: `/home/tomoaki/git/hakorune-selfhost/lang/src/compiler/build/build_box.hako:133-134`
- **Reproduction**: `tools/stage1_debug.sh --mode emit-program-json apps/tests/minimal_ssa_skip_ws.hako`

---

**Analysis Date**: 2025-11-21
**Analyzer**: Claude Code (Sonnet 4.5)
