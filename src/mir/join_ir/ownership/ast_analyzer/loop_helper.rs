use crate::ast::ASTNode;
use crate::mir::join_ir::ownership::{
    CapturedVar, OwnershipPlan, RelayVar, ScopeId, ScopeKind, ScopeOwnedVar,
};

use super::AstOwnershipAnalyzer;

/// Phase 64: Analyze a single loop (condition + body) with parent context.
///
/// This helper is designed for P3 production integration. It creates a temporary
/// function scope for parent-defined variables, then analyzes the loop scope.
///
/// # Arguments
///
/// * `condition` - Loop condition AST node
/// * `body` - Loop body statements
/// * `parent_defined` - Variables defined in parent scope (function params/locals)
///
/// # Returns
///
/// OwnershipPlan for the loop scope only (not the temporary function scope).
///
/// # Example
///
/// ```ignore
/// // loop(i < 10) { local sum=0; sum=sum+1; i=i+1; }
/// let condition = /* i < 10 */;
/// let body = vec![/* local sum=0; sum=sum+1; i=i+1; */];
/// let parent_defined = vec![];
/// let plan = analyze_loop(&condition, &body, &parent_defined)?;
/// // plan.owned_vars contains: sum (is_written=true), i (is_written=true)
/// ```
pub fn analyze_loop(
    condition: &ASTNode,
    body: &[ASTNode],
    parent_defined: &[String],
) -> Result<OwnershipPlan, String> {
    let mut analyzer = AstOwnershipAnalyzer::new();

    // Create temporary function scope for parent context
    let parent_scope = analyzer.alloc_scope(ScopeKind::Function, None);
    analyzer.push_env();
    for var in parent_defined {
        analyzer.declare_binding(parent_scope, var)?;
    }

    // Create loop scope
    let loop_scope = analyzer.alloc_scope(ScopeKind::Loop, Some(parent_scope));
    analyzer.push_env();

    // Analyze body statements
    for stmt in body {
        analyzer.analyze_node(stmt, loop_scope, false)?;
    }

    // Analyze condition (with is_condition=true flag)
    analyzer.analyze_node(condition, loop_scope, true)?;

    analyzer.pop_env(); // loop scope
                        // Propagate to parent
    analyzer.propagate_to_parent(loop_scope);

    // Build plan for loop only
    let plan = analyzer.build_plan_for_scope(loop_scope)?;
    analyzer.pop_env(); // parent scope
    Ok(plan)
}

impl AstOwnershipAnalyzer {
    /// Phase 64: Build OwnershipPlan for a specific scope (helper for analyze_loop).
    ///
    /// This is a private helper used by `analyze_loop()` to extract a single
    /// scope's OwnershipPlan without building plans for all scopes.
    pub(super) fn build_plan_for_scope(&self, scope_id: ScopeId) -> Result<OwnershipPlan, String> {
        let scope = self
            .scopes
            .get(&scope_id)
            .ok_or_else(|| format!("Scope {:?} not found", scope_id))?;

        let mut plan = OwnershipPlan::new(scope_id);

        // Collect owned vars
        for (name, binding) in &scope.declared {
            let is_written = scope.writes.contains(binding);
            let is_condition_only = is_written && scope.condition_reads.contains(binding);
            plan.owned_vars.push(ScopeOwnedVar {
                name: name.clone(),
                is_written,
                is_condition_only,
            });
        }

        // Collect relay writes
        for binding in &scope.writes {
            let info = self
                .bindings
                .get(binding)
                .ok_or_else(|| format!("AstOwnershipAnalyzer: unknown binding {:?}", binding))?;
            if info.owner_scope == scope_id {
                continue;
            }
            let relay_path = self.relay_path_to_owner(scope_id, info.owner_scope)?;
            plan.relay_writes.push(RelayVar {
                name: info.name.clone(),
                owner_scope: info.owner_scope,
                relay_path,
            });
        }

        // Collect captures
        for binding in &scope.reads {
            if scope.writes.contains(binding) {
                continue;
            }
            let info = self
                .bindings
                .get(binding)
                .ok_or_else(|| format!("AstOwnershipAnalyzer: unknown binding {:?}", binding))?;
            if info.owner_scope == scope_id {
                continue;
            }
            let captured = CapturedVar {
                name: info.name.clone(),
                owner_scope: info.owner_scope,
            };
            if scope.condition_reads.contains(binding) {
                plan.condition_captures.push(captured.clone());
            }
            plan.captures.push(captured);
        }

        #[cfg(debug_assertions)]
        plan.verify_invariants()?;

        Ok(plan)
    }
}
