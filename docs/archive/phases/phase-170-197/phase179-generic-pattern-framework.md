# Phase 179-B: Generic Pattern Framework Design

**Status**: Implementation in progress
**Related**: Phase 33-22 (CommonPatternInitializer), Phase 171-172 (Builders)

## Objective

Unify the preprocessing pipeline for Patterns 1-4 by creating a `PatternPipelineContext` that consolidates all pattern initialization logic into a single, reusable "解析済みコンテキスト箱" (analyzed context box).

## Current State Analysis

### Pattern Initialization Breakdown (Lines)

| Pattern | File | Total Lines | Preprocessing | Lowering | Target |
|---------|------|-------------|---------------|----------|--------|
| Pattern 1 | pattern1_minimal.rs | 139 | ~61 | ~78 | ~15 |
| Pattern 3 | pattern3_with_if_phi.rs | 169 | ~151 | ~18 | ~30 |
| Pattern 2 | pattern2_with_break.rs | 517 | ~437 | ~80 | ~80 |
| Pattern 4 | pattern4_with_continue.rs | 433 | ~363 | ~70 | ~70 |

**Total Reduction Target**: ~1012 lines → ~195 lines (**81% reduction**)

### Existing Infrastructure

We already have excellent modular components:
- **CommonPatternInitializer**: Loop variable extraction + CarrierInfo initialization
- **LoopScopeShapeBuilder**: LoopScopeShape construction (with/without body_locals)
- **ConditionEnvBuilder**: ConditionEnv + ConditionBinding construction
- **Pattern4CarrierAnalyzer**: Carrier filtering and update analysis

## Design: PatternPipelineContext

### Core Principles

1. **Pure Analysis Container**: Only holds preprocessing results, no JoinIR emission
2. **Analyzer-Only Dependencies**: Only depends on analyzer boxes, never lowering logic
3. **Pattern-Specific Variants**: Use `Option<T>` for pattern-specific data

### Struct Definition

```rust
/// Phase 179-B: Unified Pattern Pipeline Context
///
/// Pure "解析済みコンテキスト箱" - holds only preprocessing results.
/// JoinIR emission and PHI assembly remain in existing lowerers.
///
/// # Design Constraints
///
/// - **Analyzer-only dependencies**: Never depends on lowering logic
/// - **No emission**: No JoinIR/MIR generation in this context
/// - **Pattern variants**: Pattern-specific data stored in Option<T>
///
/// # Usage
///
/// ```rust
/// let ctx = build_pattern_context(
///     builder,
///     condition,
///     body,
///     PatternVariant::Pattern1,
/// )?;
///
/// // Use preprocessed data for lowering
/// let join_module = lower_simple_while_minimal(ctx.loop_scope)?;
/// ```
pub struct PatternPipelineContext {
    // === Common Data (All Patterns) ===

    /// Loop variable name (e.g., "i")
    pub loop_var_name: String,

    /// Loop variable HOST ValueId
    pub loop_var_id: ValueId,

    /// Carrier information (loop variable + carriers)
    pub carrier_info: CarrierInfo,

    /// Loop scope shape (header/body/latch/exit structure)
    pub loop_scope: LoopScopeShape,

    // === Pattern 2/4: Break/Continue Condition ===

    /// Condition environment (variable → JoinIR ValueId mapping)
    /// Used by Pattern 2 (break condition) and Pattern 4 (continue condition)
    pub condition_env: Option<ConditionEnv>,

    /// Condition bindings (HOST↔JoinIR value mappings)
    /// Used by Pattern 2 and Pattern 4
    pub condition_bindings: Option<Vec<ConditionBinding>>,

    /// Carrier update expressions (variable → UpdateExpr)
    /// Used by Pattern 2 (multi-carrier) and Pattern 4 (Select-based updates)
    pub carrier_updates: Option<HashMap<String, UpdateExpr>>,

    // === Pattern 2/4: Trim Pattern Support ===

    /// Trim loop helper (if Trim pattern detected during promotion)
    /// Used by Pattern 2 (string trim) - Pattern 4 support TBD
    pub trim_helper: Option<TrimLoopHelper>,

    // === Pattern 2: Break Condition ===

    /// Effective break condition (may be modified for Trim pattern)
    /// Used only by Pattern 2
    pub break_condition: Option<ASTNode>,
}

/// Pattern variant selector
pub enum PatternVariant {
    Pattern1,  // Simple while loop
    Pattern2,  // Loop with break
    Pattern3,  // Loop with if-else PHI
    Pattern4,  // Loop with continue
}
```

### Pipeline Function

```rust
/// Build pattern preprocessing context
///
/// This consolidates all preprocessing steps shared by Patterns 1-4:
/// 1. Loop variable extraction (CommonPatternInitializer)
/// 2. LoopScopeShape construction (LoopScopeShapeBuilder)
/// 3. Pattern-specific analysis (ConditionEnv, carrier updates, etc.)
/// 4. Trim pattern promotion (if applicable)
///
/// # Arguments
///
/// * `builder` - MirBuilder instance
/// * `condition` - Loop condition AST node
/// * `body` - Loop body AST nodes
/// * `variant` - Pattern variant selector
///
/// # Returns
///
/// PatternPipelineContext with all preprocessing results
///
/// # Errors
///
/// Returns error if:
/// - Loop variable not found in variable_map
/// - Condition variable not found (Pattern 2/4)
/// - Trim pattern promotion fails (Pattern 2/4)
pub fn build_pattern_context(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    variant: PatternVariant,
) -> Result<PatternPipelineContext, String> {
    // Step 1: Common initialization
    let (loop_var_name, loop_var_id, mut carrier_info) =
        CommonPatternInitializer::initialize_pattern(
            builder,
            condition,
            &builder.variable_map,
            None,
        )?;

    // Step 2: Build LoopScopeShape
    let loop_scope = match variant {
        PatternVariant::Pattern1 | PatternVariant::Pattern3 => {
            // Pattern 1, 3: No body_locals needed
            LoopScopeShapeBuilder::empty_body_locals(
                BasicBlockId(0),
                BasicBlockId(0),
                BasicBlockId(0),
                BasicBlockId(0),
                BTreeSet::new(),
            )
        }
        PatternVariant::Pattern2 | PatternVariant::Pattern4 => {
            // Pattern 2, 4: Extract body_locals for Trim support
            LoopScopeShapeBuilder::with_body_locals(
                BasicBlockId(0),
                BasicBlockId(0),
                BasicBlockId(0),
                BasicBlockId(0),
                BTreeSet::new(),
                body,
            )
        }
    };

    // Step 3: Pattern-specific preprocessing
    let (condition_env, condition_bindings, carrier_updates, trim_helper, break_condition) =
        match variant {
            PatternVariant::Pattern1 => {
                // Pattern 1: No additional preprocessing
                (None, None, None, None, None)
            }
            PatternVariant::Pattern3 => {
                // Pattern 3: No condition env, but has carrier updates (for if-else PHI)
                // TODO: Pattern 3 analyzer integration
                (None, None, None, None, None)
            }
            PatternVariant::Pattern2 => {
                // Pattern 2: Full preprocessing (break condition, carriers, Trim)
                build_pattern2_context(builder, condition, body, &loop_var_name, loop_var_id, &mut carrier_info, &loop_scope)?
            }
            PatternVariant::Pattern4 => {
                // Pattern 4: Similar to Pattern 2 but with continue semantics
                build_pattern4_context(builder, condition, body, &loop_var_name, loop_var_id, &mut carrier_info, &loop_scope)?
            }
        };

    Ok(PatternPipelineContext {
        loop_var_name,
        loop_var_id,
        carrier_info,
        loop_scope,
        condition_env,
        condition_bindings,
        carrier_updates,
        trim_helper,
        break_condition,
    })
}
```

## Integration Strategy

### Migration Order: P1 → P3 → P2 → P4

1. **Pattern 1** (Minimal complexity)
   - Zero dependencies on ConditionEnv
   - Only needs: loop_var_name, loop_var_id, loop_scope
   - Best for testing the framework

2. **Pattern 3** (PHI without break/continue)
   - Adds carrier handling for if-else PHI
   - No break/continue complexity
   - Tests carrier_info flow

3. **Pattern 2** (Most complex)
   - Full feature set: break + Trim + multi-carrier
   - Tests all context fields
   - Validates Trim pattern integration

4. **Pattern 4** (Continue semantics)
   - Similar to Pattern 2 but Select-based
   - Final validation of framework completeness

### Pattern 1 Migration Example

**Before** (61 lines of preprocessing):
```rust
pub(in crate::mir::builder) fn cf_loop_pattern1_minimal(
    &mut self,
    condition: &ASTNode,
    _body: &[ASTNode],
    _func_name: &str,
    debug: bool,
) -> Result<Option<ValueId>, String> {
    // ... 61 lines of initialization ...

    let join_module = lower_simple_while_minimal(scope)?;

    // ... merge and return ...
}
```

**After** (~15 lines):
```rust
pub(in crate::mir::builder) fn cf_loop_pattern1_minimal(
    &mut self,
    condition: &ASTNode,
    body: &[ASTNode],
    _func_name: &str,
    debug: bool,
) -> Result<Option<ValueId>, String> {
    // Step 1: Build preprocessing context
    let ctx = build_pattern_context(
        self,
        condition,
        body,
        PatternVariant::Pattern1,
    )?;

    // Step 2: Call lowerer with preprocessed data
    let join_module = lower_simple_while_minimal(ctx.loop_scope)?;

    // Step 3: Create boundary from context
    let boundary = JoinInlineBoundaryBuilder::new()
        .with_inputs(vec![ValueId(0)], vec![ctx.loop_var_id])
        .with_loop_var_name(Some(ctx.loop_var_name.clone()))
        .build();

    // Step 4: Merge and return
    JoinIRConversionPipeline::execute(self, join_module, Some(&boundary), "pattern1", debug)?;
    Ok(Some(emit_void(self)))
}
```

## Trim/P5 Special Case Handling

### Current Situation

Pattern 2 has complex Trim pattern logic (~100 lines) that:
1. Detects Trim pattern via LoopBodyCarrierPromoter
2. Generates initial whitespace check
3. Modifies break condition
4. Adds carrier to ConditionEnv

### Phase 179-B Strategy

**Include in PatternPipelineContext**:
- Trim detection and validation (LoopBodyCarrierPromoter)
- TrimLoopHelper storage
- Modified break condition

**Keep in Pattern 2 lowerer**:
- Initial whitespace check emission (MIR instruction generation)
- This is "lowering" not "analysis"

**Future Phase 180+ (Trim-specific refactor)**:
- Move Trim lowering logic to `trim_loop_lowering.rs`
- Create `TrimPatternLowerer` box
- PatternPipelineContext just provides the analysis result

## Benefits

### Code Reduction
- **Pattern 1**: 61 → ~15 lines (75% reduction)
- **Pattern 3**: 151 → ~30 lines (80% reduction)
- **Pattern 2**: 517 → ~80 lines (85% reduction)
- **Pattern 4**: 433 → ~70 lines (84% reduction)
- **Total**: 1162 → ~195 lines (**83% reduction**)

### Maintainability
- Single source of truth for preprocessing
- Easier to add new patterns
- Clear separation: analysis vs lowering

### Testability
- Can test preprocessing independently
- Pattern-specific logic isolated
- Easier to mock/stub for unit tests

### Consistency
- All patterns use same initialization flow
- Consistent error messages
- Uniform trace/debug output

## Non-Goals (Out of Scope)

❌ **Not included in PatternPipelineContext**:
- JoinIR emission (remains in existing lowerers)
- PHI assembly (remains in existing lowerers)
- MIR instruction generation (remains in existing lowerers)
- Block merging (remains in JoinIRConversionPipeline)

✅ **Only preprocessing**:
- Variable extraction
- Carrier analysis
- Condition environment setup
- Trim pattern detection

## Implementation Checklist

- [ ] Task 179-B-1: Design document (this file)
- [ ] Task 179-B-2: PatternPipelineContext implementation
- [ ] Task 179-B-3: Pattern 1 migration
- [ ] Task 179-B-4: Pattern 3 migration
- [ ] Task 179-B-5: Pattern 2 migration
- [ ] Task 179-B-6: Pattern 4 migration
- [ ] Task 179-B-7: Tests and documentation update

## References

- **CommonPatternInitializer**: `src/mir/builder/control_flow/joinir/patterns/common_init.rs`
- **LoopScopeShapeBuilder**: `src/mir/builder/control_flow/joinir/patterns/loop_scope_shape_builder.rs`
- **ConditionEnvBuilder**: `src/mir/builder/control_flow/joinir/patterns/condition_env_builder.rs`
- **JoinIRConversionPipeline**: `src/mir/builder/control_flow/joinir/patterns/conversion_pipeline.rs`
Status: Historical
