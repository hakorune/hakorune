# Phase 173-2 Investigation Findings

**Date**: 2025-12-04
**Status**: Investigation Complete, Implementation Strategy Needs Revision

## Executive Summary

Completed investigation of Phase 173-2 requirements. Found that the original instruction document's approach needs revision based on actual system behavior and architecture principles.

## Investigation Results

### What Works ✅
1. **Static box file loading**: using statement correctly loads JsonParserBox source
2. **Function compilation**: All JsonParserBox methods compile to MIR correctly
3. **Function registration**: All static methods registered in VM function table with correct names (`JsonParserBox.method/arity`)
4. **Internal static calls**: `me.method()` calls within static box work correctly
5. **Function lookup**: VM successfully finds all static box functions

### What Doesn't Work ❌
1. **External static box calls**: `JsonParserBox.parse()` from Main doesn't work
2. **Root cause**: Parser treats `JsonParserBox` as a **variable** (VarRef), not a **type** (TypeRef)
3. **Result**: MIR lowering generates `Callee::Method` with an undefined receiver, leading to "Unknown method on InstanceBox" error

## Technical Analysis

### Current Call Flow (Broken)

```
Main.main():
  JsonParserBox.parse("{\"x\":1}")
    ↓ Parser: treats "JsonParserBox" as variable
    ↓ AST: CallTarget::Method { receiver: VarRef("JsonParserBox"), method: "parse" }
    ↓ MIR: Callee::Method { receiver: ValueId(?), method: "parse", box_name: "InstanceBox" }
    ↓ VM: ERROR - receiver has no type, defaults to InstanceBox
```

### Expected Call Flow (Target)

```
Main.main():
  JsonParserBox.parse("{\"x\":1}")
    ↓ Parser: recognizes "JsonParserBox" as type
    ↓ AST: CallTarget::StaticMethod { box_type: "JsonParserBox", method: "parse" }
    ↓ MIR: Callee::Global("JsonParserBox.parse/1")
    ↓ VM: SUCCESS - function table lookup, execute
```

## Original Implementation Plan Issues

### Instruction Document Approach
The instruction document (phase173-2_using_resolver_mir_lowering.md) proposes:
1. Modify using resolver (.hako) to register static boxes as types
2. Modify parser (.hako) to recognize `Alias.method()` as type references
3. Modify MIR lowering (Rust) to detect static box calls

### Problems with This Approach
1. **Violates "Rust VM不変" principle**: Would require changes to Rust MIR lowering
2. **Complex .hako modifications**: Requires symbol table/type system in .hako compiler
3. **Scope creep**: Essentially implementing a type system in Stage-1 parser
4. **Maintenance burden**: Two-language coordination (.hako parser + Rust MIR)

## Recommended Alternative Approach

### Strategy: AST-level Static Call Recognition

**Principle**: Minimize changes, leverage existing infrastructure

### Phase A: Parser Enhancement (Minimal)
**File**: `lang/src/compiler/parser/parser_calls.hako` (or similar)
**Change**: Add special handling for `Alias.method()` pattern where Alias is from using

```hako
// When parsing method call expression:
if receiver_is_identifier(receiver) {
    local name = receiver_name
    if is_using_alias(name) {  // Check against using table
        // Emit StaticBoxCall instead of MethodCall
        return make_static_box_call_ast(name, method, args)
    }
}
```

### Phase B: AST Representation
**File**: Extend AST to support static box calls explicitly
**Options**:
1. Add `StaticBoxCall` AST node type
2. Or: Add flag to existing MethodCall: `is_static_box_call: true`

### Phase C: MIR Lowering (Minimal Rust Change)
**File**: `src/mir/builder/calls/builder_calls.rs`
**Change**: Detect StaticBoxCall AST node and emit `Callee::Global`

```rust
match ast_call {
    AstCallType::StaticBoxCall { box_name, method, args } => {
        let func_name = format!("{}.{}", box_name, method);
        // Emit Callee::Global(func_name) with args
    }
    // ... existing cases
}
```

## Alternative: Quick Fix Approach

### If Full Implementation is Too Complex

**Workaround**: Document that static box methods must be called with explicit constructor pattern:

```hako
// Instead of:
JsonParserBox.parse("{}")

// Use:
local parser = new JsonParserBox()  // dummy instance
parser.parse("{}")  // works because lowered to Global

// Or use direct function call syntax (if supported):
JsonParserBox.parse/1("{}")
```

**Pros**: No code changes required
**Cons**: Poor user experience, not the desired syntax

## Impact Assessment

### Affected Components
1. **Parser** (.hako): Minimal changes to call expression parsing
2. **AST** (JSON v0): Add static box call representation
3. **MIR lowering** (Rust): Add static box call handling (~20 lines)
4. **VM**: No changes required ✅

### Risk Level
- **Low**: Changes are isolated and additive
- **No breaking changes**: Existing code continues to work
- **Testable**: Can verify with json_parser_min.hako immediately

## Test Case Status

### json_parser_min.hako
**Current**: ❌ Fails with "Unknown method '_skip_whitespace' on InstanceBox"
**Expected after fix**: ✅ RC 0, no errors

**Current output**:
```
[DEBUG/vm] Looking up function: 'JsonParserBox._skip_whitespace'
[DEBUG/vm]   ✅ 'JsonParserBox._skip_whitespace/2' found
[ERROR] ❌ [rust-vm] VM error: Invalid instruction: Unknown method '_skip_whitespace' on InstanceBox
```

### Diagnosis
- Function **is** found in function table
- Error happens when trying to execute because receiver is InstanceBox
- Root cause: Call site in Main generates Method call, not Global call

## Next Steps (Revised)

### Option 1: Implement Minimal Parser Fix (Recommended)
1. Add using alias table to parser context
2. Detect `UsingAlias.method()` pattern in call parsing
3. Emit StaticBoxCall AST node
4. Handle in MIR lowering to emit Callee::Global
5. Test with json_parser_min.hako

**Estimated effort**: 2-3 hours
**Risk**: Low
**Benefit**: Clean solution, proper syntax support

### Option 2: Document Workaround (Quick Exit)
1. Update using.md with workaround pattern
2. Mark Phase 173 as "deferred" pending type system work
3. Move to Phase 174+ with comprehensive type system

**Estimated effort**: 30 minutes
**Risk**: None
**Benefit**: Unblocks other work, defers complexity

## Recommendation

**Proceed with Option 1**: The minimal parser fix is well-scoped, aligns with architecture principles, and provides immediate value without introducing technical debt.

**Alternative**: If time-constrained, use Option 2 as interim solution and schedule Option 1 for Phase 174.

## Files to Review

### Investigation Evidence
- Trace output: See "Test current behavior" section above
- MIR lowering code: `src/mir/builder/calls/resolver.rs` (lines 80-120)
- VM function lookup: `src/backend/mir_interpreter/handlers/calls/global.rs` (lines 5-60)

### Implementation Targets
- Parser: `lang/src/compiler/parser/parser_calls.hako` or similar
- AST: JSON v0 schema (or extend existing MethodCall node)
- MIR lowering: `src/mir/builder/calls/builder_calls.rs`

---

**Created**: 2025-12-04
**Phase**: 173-2 Investigation
**Outcome**: Strategy revision required
**Recommendation**: Minimal parser fix (Option 1)
Status: Historical
