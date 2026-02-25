# Phase 173-2 Completion Summary

**Date**: 2025-12-04
**Status**: Investigation Complete, Strategy Revision Required

## Summary

Phase 173-2 investigation has been completed successfully. Through detailed VM tracing and code analysis, I identified the root cause of the static box method resolution issue and proposed two alternative implementation strategies.

## What Was Accomplished ✅

### 1. Deep Investigation
- **VM Execution Tracing**: Used `NYASH_CALLEE_RESOLVE_TRACE=1` and `NYASH_DEBUG_FUNCTION_LOOKUP=1` to trace the complete execution flow
- **MIR Lowering Analysis**: Verified that static box internal calls (`me.method()`) correctly lower to `Callee::Global`
- **VM Function Lookup**: Confirmed all JsonParserBox methods are properly registered in the function table
- **Error Point Identification**: Pinpointed that the error occurs at the call site in Main, not in the VM execution

### 2. Root Cause Identified
**Problem**: Parser treats `JsonParserBox` as a **variable** (VarRef) instead of a **type** (TypeRef)

**Evidence**:
```
Main.main():
  JsonParserBox.parse("{\"x\":1}")
    ↓ Parser treats "JsonParserBox" as variable (VarRef)
    ↓ MIR generates Callee::Method with undefined receiver
    ↓ Receiver defaults to "InstanceBox"
    ↓ ERROR: "Unknown method 'parse' on InstanceBox"
```

**Confirmed Working**:
- Internal static box calls: `me.method()` within JsonParserBox ✅
- Function registration: All methods in VM function table ✅
- Function lookup: VM successfully finds functions ✅

**Not Working**:
- External static box calls: `JsonParserBox.parse()` from Main ❌

### 3. Strategy Revision
**Original Plan Issues**:
- Violated "Rust VM不変" (Rust VM unchanged) principle
- Required complex .hako compiler modifications
- Introduced scope creep (essentially building a type system)

**Revised Recommendations**:

#### Option 1: Minimal Parser Fix (Recommended)
**Approach**: Detect `UsingAlias.method()` pattern in parser, emit StaticBoxCall AST node

**Changes required**:
1. **Parser** (.hako): Add using alias table lookup in call expression parsing (~30 lines)
2. **AST**: Add StaticBoxCall node type or flag to existing MethodCall
3. **MIR lowering** (Rust): Handle StaticBoxCall → `Callee::Global` (~20 lines)

**Effort**: 2-3 hours
**Risk**: Low (isolated, additive changes)
**Benefit**: Clean solution, proper syntax support

#### Option 2: Workaround Documentation (Quick Exit)
**Approach**: Document workaround pattern, defer to Phase 174+ type system work

**Pattern**:
```hako
// Workaround: Create dummy instance
local parser = new JsonParserBox()
parser.parse("{}")  // Works via Global call lowering
```

**Effort**: 30 minutes
**Risk**: None
**Benefit**: Unblocks other work, defers complexity

## Documentation Created

1. **phase173-2_investigation_findings.md** (330+ lines)
   - Complete technical analysis
   - Root cause explanation with call flow diagrams
   - Two implementation options with trade-offs
   - Test case status and diagnosis

2. **CURRENT_TASK.md** (updated)
   - Task 1-3 marked complete (investigation + JsonParserBox bugfix)
   - Task 4-6 revised with new strategy recommendations
   - Root cause summary added
   - Files created/modified list updated

3. **phase173-2_completion_summary.md** (this document)
   - High-level overview for stakeholders
   - Clear recommendations
   - Next steps

## Technical Insights

### Architecture Principle Adherence
✅ **箱化モジュール化** (Modular Boxing): Investigation maintained the principle of isolated, incremental changes
✅ **Rust VM不変** (Rust VM Unchanged): Proposed solutions minimize Rust VM changes
✅ **段階的確認** (Staged Verification): Traced AST → MIR → VM flow systematically

### Code Quality
- All investigation code is read-only (no modifications during investigation)
- Comprehensive tracing and logging used for debugging
- Clear separation of concerns maintained

### Knowledge Gained
1. **Static box internal calls work correctly**: The MIR lowering already handles `me.method()` properly in static box context
2. **VM infrastructure is sound**: Function registration, lookup, and execution all work as expected
3. **Parser is the bottleneck**: The issue is purely at the parser level, not in VM or MIR lowering

## Recommendation

**Proceed with Option 1** (Minimal Parser Fix) because:
1. **Well-scoped**: Clear boundaries, minimal changes
2. **Architecturally sound**: Aligns with existing design principles
3. **User-friendly**: Provides the expected syntax (`JsonParserBox.parse()`)
4. **Low risk**: Changes are additive and testable
5. **Immediate value**: Unblocks Phase 171-2 (hako_check integration)

**Alternative**: If time-constrained or if there are other higher-priority tasks, use Option 2 as an interim solution.

## Next Steps

### Immediate (Decision Required)
User/stakeholder should decide:
- [ ] Option 1: Implement minimal parser fix (2-3 hours)
- [ ] Option 2: Document workaround, defer to Phase 174+ (30 minutes)

### After Decision
**If Option 1**:
1. Implement parser enhancement for `UsingAlias.method()` detection
2. Add StaticBoxCall AST node or flag
3. Modify MIR lowering to handle StaticBoxCall
4. Test with json_parser_min.hako (expect RC 0)
5. Run hako_check smoke tests (HC019/HC020)
6. Update documentation and commit

**If Option 2**:
1. Update using.md with workaround pattern and examples
2. Add note to LANGUAGE_REFERENCE_2025.md
3. Mark Phase 173 as "interim complete" with workaround
4. Schedule Phase 174 for comprehensive type system work
5. Update documentation and commit

## Impact Assessment

### Blocked/Unblocked
**Currently Blocked**:
- Phase 171-2: hako_check JsonParserBox integration
- Any code using `using` for static box libraries

**Will Unblock** (with Option 1):
- Phase 171-2: Can complete hako_check integration
- JsonParserBox as official standard library
- Future static box libraries (e.g., ProgramJSONBox usage)

### Technical Debt
**Option 1**: Minimal debt (proper solution)
**Option 2**: Moderate debt (workaround until Phase 174+)

## Files for Review

### Investigation Documents
- `phase173-2_investigation_findings.md` (detailed analysis, read first)
- `phase173-2_completion_summary.md` (this document, executive summary)
- `phase173_task1-2_completion_report.md` (Task 1-2 details)
- `mir-nested-if-loop-bug.md` (related bug found during investigation)

### Test Cases
- `apps/tests/json_parser_min.hako` (currently fails, will pass after fix)

### Reference
- `docs/reference/language/using.md` (Phase 173 static box section)
- `docs/reference/language/LANGUAGE_REFERENCE_2025.md` (static box usage)

---

**Created**: 2025-12-04
**Phase**: 173-2 (using resolver + MIR lowering)
**Outcome**: Investigation complete, two implementation options proposed
**Recommendation**: Option 1 (minimal parser fix)
**Estimated Completion**: 2-3 hours (Option 1) or 30 minutes (Option 2)
Status: Historical
