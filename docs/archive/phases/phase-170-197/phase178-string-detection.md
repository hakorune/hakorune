# Phase 178: LoopUpdateAnalyzer String Detection

## Summary

Phase 178 extends `LoopUpdateAnalyzer` to detect string/complex carrier updates,
enabling Fail-Fast behavior for unsupported patterns.

## Changes

### 1. UpdateRhs Enum Extension (`loop_update_analyzer.rs`)

Added two new variants:
- `StringLiteral(String)` - for `result = result + "x"` patterns
- `Other` - for method calls and complex expressions

### 2. analyze_rhs Extension

Extended to detect:
- String literals: `ASTNode::Literal { value: LiteralValue::String(_) }`
- Method calls: `ASTNode::MethodCall { .. }`
- Other complex expressions: `ASTNode::Call`, `ASTNode::BinaryOp`, etc.

### 3. Pattern 2/4 can_lower Updates

Both `pattern2_with_break.rs` and `pattern4_with_continue.rs` now check for
string/complex updates in `can_lower()` and return `false` if detected.

This triggers a clear error message instead of silent incorrect behavior.

### 4. Legacy Fallback Comment Fixes

Updated misleading comments about "legacy fallback" - LoopBuilder was removed
in Phase 187-2 and all loops must use JoinIR.

## Behavior

When a loop contains string concatenation like:
```nyash
loop(i < limit) {
    result = result + "x"  // String update
    i = i + 1
}
```

Phase 178 now produces a clear error:
```
[pattern2/can_lower] Phase 178: String/complex update detected, rejecting Pattern 2 (unsupported)
[ERROR] MIR compilation error: [joinir/freeze] Loop lowering failed:
JoinIR does not support this pattern, and LoopBuilder has been removed.
```

## Test Results

- P1 (simple while): OK
- P2 (break + int carriers): OK
- P4 (continue + multi-carrier): OK
- String loops: Fail-Fast with clear error

## Known Issues

- **79 global test failures**: Pre-existing issue, NOT caused by Phase 178
  - Confirmed by `git stash` test - failures exist in HEAD~1
  - Tracked separately from Phase 178

## Future Work

To support string loops, either:
1. Add JoinIR instructions for string concatenation (Option A)
2. Process loop body statements in MIR alongside JoinIR control flow (Option B)

Phase 178 provides the detection foundation for future string support.
Status: Historical
