# Phase 129 P1: join_k Materialization Implementation Plan

## Current State Analysis

### Current `lower_if_node` (builder.rs:398-469)

**Problems**:
1. Only processes then branch (line 463)
2. No branching structure generated (sequential processing)
3. `verify_branch_is_return_literal` too restrictive (doesn't allow Assign)
4. No join_k function created
5. No env merging logic

**Current flow**:
```rust
lower_if_node:
  1. Parse condition (Compare instruction)
  2. Verify then/else are Return(literal) ← TOO RESTRICTIVE
  3. Lower then branch only ← INCOMPLETE
  4. Done (no else, no join_k)
```

### Required Changes for Phase 129

**New flow**:
```rust
lower_if_node:
  1. Parse condition (Compare instruction)
  2. Create join_k function with env parameter
  3. Lower then branch:
     - Process Assign statements (update env_then)
     - End with TailCall(join_k, env_then)
  4. Lower else branch:
     - Keep env_else = env (unchanged)
     - End with TailCall(join_k, env_else)
  5. join_k body:
     - Receive env_phi (merged environment)
     - Process post-if statements
     - Return
```

## Implementation Strategy

### Option A: In-Place Modification (Risky)

**Pros**:
- Direct implementation
- No new abstractions

**Cons**:
- High risk of breaking Phase 123-127 support
- Mixes Phase 123-128 and Phase 129 logic
- Hard to test incrementally

### Option B: Phased Approach (Recommended)

**Phase 129-A: Foundation**
1. Extract join_k generation into helper function
2. Add env snapshot/restore helpers
3. Keep existing behavior (no-op join_k)

**Phase 129-B: Branch Lowering**
1. Lower then branch to tail-call join_k
2. Lower else branch to tail-call join_k
3. Test with simple fixture (no post-if)

**Phase 129-C: Post-If Support**
1. Process post-if statements in join_k body
2. Test with Phase 129 fixture (print after if)
3. Full verification

**Pros**:
- Incremental testing
- Clear rollback points
- Preserves existing functionality

**Cons**:
- More commits
- Slightly slower

## Detailed Implementation Plan

### Step 1: Helper Functions (Box-First)

**Create**:  `src/mir/control_tree/normalized_shadow/join_k_builder.rs`

```rust
pub struct JoinKBuilder {
    // join_k function generation
}

impl JoinKBuilder {
    pub fn new() -> Self { ... }

    /// Generate join_k function with env parameter
    pub fn create_join_k_function(
        &mut self,
        env_layout: &EnvLayout,
        next_func_id: &mut u32,
    ) -> JoinFunction { ... }

    /// Snapshot current env (for branch entry)
    pub fn snapshot_env(
        env: &BTreeMap<String, ValueId>,
    ) -> BTreeMap<String, ValueId> { ... }

    /// Generate TailCall to join_k with env
    pub fn generate_tail_call(
        join_k_id: JoinFuncId,
        env: &BTreeMap<String, ValueId>,
    ) -> JoinInst { ... }
}
```

### Step 2: Modify `lower_if_node`

**Replace lines 398-469** with new implementation:

```rust
fn lower_if_node(
    node: &StepNode,
    module: &mut JoinModule,  // ← Need module access to add join_k
    body: &mut Vec<JoinInst>,
    next_value_id: &mut u32,
    next_func_id: &mut u32,  // ← New parameter
    env: &mut BTreeMap<String, ValueId>,
    contract: &StepTreeContract,
) -> Result<(), String> {
    // 1. Parse condition (existing logic)
    let (lhs_var, op, rhs_literal) = Self::parse_minimal_compare(ast)?;

    // 2. Create join_k function
    let join_k_builder = JoinKBuilder::new();
    let join_k_func = join_k_builder.create_join_k_function(
        &env_layout,
        next_func_id
    );
    let join_k_id = join_k_func.id;

    // 3. Lower then branch
    let mut env_then = join_k_builder.snapshot_env(env);
    let mut then_body = Vec::new();
    Self::lower_branch(
        then_branch,
        &mut then_body,
        next_value_id,
        &mut env_then,
        contract
    )?;
    then_body.push(join_k_builder.generate_tail_call(join_k_id, &env_then));

    // 4. Lower else branch
    let env_else = join_k_builder.snapshot_env(env); // unchanged
    let mut else_body = Vec::new();
    if let Some(else_br) = else_branch {
        Self::lower_branch(
            else_br,
            &mut else_body,
            next_value_id,
            &mut env_else,
            contract
        )?;
    }
    else_body.push(join_k_builder.generate_tail_call(join_k_id, &env_else));

    // 5. Generate Branch instruction
    body.push(JoinInst::Branch {
        condition: cond_vid,
        then_block: then_block_id,
        else_block: else_block_id,
    });

    // 6. Add join_k to module
    module.add_function(join_k_func);

    Ok(())
}
```

**Issues with this approach**:
- `lower_if_node` currently takes `body: &mut Vec<JoinInst>`, not module
- Need to refactor to pass module reference
- This changes the signature → affects callers

### Step 3: Refactor Call Chain

**Current call chain**:
```
lower_if_only_to_normalized
  → lower_return_from_tree
    → lower_if_node (takes body)
```

**Need to change to**:
```
lower_if_only_to_normalized
  → lower_return_from_tree_with_module  // ← New
    → lower_if_node_with_module (takes module)
```

**Alternative**: Build join_k in `lower_if_only_to_normalized` and pass as parameter

### Step 4: Verification Function

**Add**: `src/mir/control_tree/normalized_shadow/verification.rs`

```rust
pub fn verify_normalized_structure(module: &JoinModule) -> Result<(), String> {
    // 1. Check phase
    if !module.is_normalized() {
        return Err("Module is not Normalized phase".to_string());
    }

    // 2. If exists → join_k exists
    let has_if = /* detect If/Branch in any function */;
    if has_if {
        let has_join_k = module.functions.values().any(|f|
            f.name.starts_with("join_k") || f.name.starts_with("k_join")
        );
        if !has_join_k {
            return Err("If exists but join_k function not found".to_string());
        }
    }

    // 3. No PHI instructions
    for func in module.functions.values() {
        for inst in &func.body {
            if matches!(inst, JoinInst::Phi { .. }) {
                return Err(format!(
                    "PHI instruction found in Normalized module (function: {})",
                    func.name
                ));
            }
        }
    }

    // 4. then/else end with TailCall (if join_k exists)
    // ... additional checks ...

    Ok(())
}
```

## Testing Strategy

### Phase 129-A: Foundation Tests

```rust
#[test]
fn test_join_k_builder_creates_function() {
    let builder = JoinKBuilder::new();
    let env_layout = /* ... */;
    let mut next_func_id = 10;

    let join_k = builder.create_join_k_function(&env_layout, &mut next_func_id);

    assert_eq!(join_k.id.0, 10);
    assert!(join_k.name.starts_with("join_k"));
    assert_eq!(join_k.params.len(), env_layout.writes.len());
}

#[test]
fn test_env_snapshot() {
    let mut env = BTreeMap::new();
    env.insert("x".to_string(), ValueId(5));

    let snapshot = JoinKBuilder::snapshot_env(&env);

    assert_eq!(snapshot.len(), 1);
    assert_eq!(snapshot["x"], ValueId(5));
}
```

### Phase 129-B: Branch Lowering Tests

```rust
#[test]
fn test_if_generates_join_k() {
    // Create if node with simple then/else
    let tree = /* ... */;

    let (module, _) = lower_if_only_to_normalized(&tree, &BTreeMap::new())?;

    // Verify join_k exists
    let join_k = module.functions.values()
        .find(|f| f.name.starts_with("join_k"))
        .expect("join_k not found");

    assert!(join_k.params.len() > 0);
}
```

### Phase 129-C: Post-If Tests

```rust
#[test]
fn test_post_if_return_var() {
    // Use Phase 129 fixture: x=1; if { x=2 }; print(x)
    let tree = /* ... */;

    let (module, _) = lower_if_only_to_normalized(&tree, &BTreeMap::new())?;

    // Verify join_k has post-if statements
    let join_k = module.functions.values()
        .find(|f| f.name.starts_with("join_k"))
        .expect("join_k not found");

    // Should have print + return in join_k body
    assert!(join_k.body.len() >= 2);
}
```

## Acceptance Criteria

### Code Quality
- [ ] Single responsibility: JoinKBuilder handles join_k generation
- [ ] Fail-Fast: verify_normalized_structure catches malformed modules
- [ ] Box-First: Separate join_k_builder.rs module

### Functionality
- [ ] join_k materialized as JoinFunction
- [ ] then/else tail-call join_k with env
- [ ] post-if statements processed in join_k body
- [ ] No PHI instructions in Normalized

### Testing
- [ ] Unit tests for JoinKBuilder
- [ ] Unit tests for verify_normalized_structure
- [ ] Phase 129 fixture smoke test passes
- [ ] Phase 128 regression passes
- [ ] `cargo test --lib` passes

## Risk Assessment

### High Risk
- Refactoring `lower_if_node` signature (affects call chain)
- Handling post-if statements (requires lookahead in StepTree)
- env merging logic (ValueId mapping complexity)

### Medium Risk
- join_k function ID generation (need unique IDs)
- TailCall instruction generation (correct env parameter passing)

### Low Risk
- JoinKBuilder extraction (pure functions)
- verify_normalized_structure (read-only checks)

## Recommended Next Steps

1. **ChatGPT/User Decision**: Choose Option A or B
2. **If Option B (Phased)**:
   - Commit Phase 129-A (JoinKBuilder + helpers)
   - Commit Phase 129-B (branch lowering)
   - Commit Phase 129-C (post-if support)
3. **If Option A (In-Place)**:
   - Refactor lower_if_node in one commit
   - High risk → thorough testing required

## Open Questions

1. **Function ID allocation**: Where to get next_func_id?
   - Option: Add to `lower_if_only_to_normalized` parameters
   - Option: Module tracks max ID

2. **Post-if statement detection**: How to know where if ends?
   - Option: StepTree structure provides this
   - Option: Parse remaining nodes after if

3. **env parameter passing**: How to represent env_phi?
   - Option: Vec<ValueId> (ordered by env_layout)
   - Option: Tuple unpacking (not supported in JoinInst yet)

4. **join_k naming**: Convention?
   - Option: `join_k_<func_id>`
   - Option: `k_join_<line_number>`
   - Option: Auto-increment `join_k_0`, `join_k_1`, ...
