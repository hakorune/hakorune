use crate::ast::ASTNode;
use crate::mir::join_ir::ownership::{OwnershipPlan, ScopeId, ScopeKind};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct BindingId(u32);

#[derive(Debug, Clone)]
pub(super) struct BindingInfo {
    pub(super) name: String,
    pub(super) owner_scope: ScopeId,
}

#[derive(Debug)]
pub(super) struct ScopeInfo {
    pub(super) id: ScopeId,
    pub(super) kind: ScopeKind,
    pub(super) parent: Option<ScopeId>,
    pub(super) declared: BTreeMap<String, BindingId>,
    pub(super) reads: BTreeSet<BindingId>,
    pub(super) writes: BTreeSet<BindingId>,
    pub(super) condition_reads: BTreeSet<BindingId>,
}

/// Analyzes real AST and produces `OwnershipPlan`.
///
/// This analyzer:
/// - Treats `ASTNode::Local` as "definition" (no JSON v0 rebind hack)
/// - Records writes via `ASTNode::Assignment` / `ASTNode::GroupedAssignmentExpr`
/// - Treats `Loop/While/ForRange` and `If` conditions as `condition_reads`
pub struct AstOwnershipAnalyzer {
    pub(super) scopes: BTreeMap<ScopeId, ScopeInfo>,
    pub(super) bindings: BTreeMap<BindingId, BindingInfo>,
    next_scope_id: u32,
    next_binding_id: u32,
    pub(super) env_stack: Vec<BTreeMap<String, BindingId>>,
}

impl AstOwnershipAnalyzer {
    pub fn new() -> Self {
        Self {
            scopes: BTreeMap::new(),
            bindings: BTreeMap::new(),
            next_scope_id: 0,
            next_binding_id: 0,
            env_stack: Vec::new(),
        }
    }

    pub fn analyze_ast(&mut self, ast: &ASTNode) -> Result<Vec<OwnershipPlan>, String> {
        self.scopes.clear();
        self.bindings.clear();
        self.next_scope_id = 0;
        self.next_binding_id = 0;
        self.env_stack.clear();

        match ast {
            ASTNode::Program { statements, .. } => {
                for stmt in statements {
                    self.analyze_toplevel(stmt, None)?;
                }
            }
            ASTNode::FunctionDeclaration { .. } => {
                self.analyze_function_decl(ast, None)?;
            }
            _ => {
                return Err(
                    "AstOwnershipAnalyzer: expected Program or FunctionDeclaration".to_string(),
                );
            }
        }

        self.build_plans()
    }

    fn analyze_toplevel(&mut self, node: &ASTNode, parent: Option<ScopeId>) -> Result<(), String> {
        match node {
            ASTNode::FunctionDeclaration { .. } => {
                self.analyze_function_decl(node, parent)?;
            }
            ASTNode::BoxDeclaration {
                methods,
                constructors,
                ..
            } => {
                for (_, f) in methods {
                    self.analyze_function_decl(f, parent)?;
                }
                for (_, f) in constructors {
                    self.analyze_function_decl(f, parent)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub(super) fn alloc_scope(&mut self, kind: ScopeKind, parent: Option<ScopeId>) -> ScopeId {
        let id = ScopeId(self.next_scope_id);
        self.next_scope_id += 1;
        self.scopes.insert(
            id,
            ScopeInfo {
                id,
                kind,
                parent,
                declared: BTreeMap::new(),
                reads: BTreeSet::new(),
                writes: BTreeSet::new(),
                condition_reads: BTreeSet::new(),
            },
        );
        id
    }

    pub(super) fn analyze_function_decl(
        &mut self,
        node: &ASTNode,
        parent: Option<ScopeId>,
    ) -> Result<ScopeId, String> {
        let ASTNode::FunctionDeclaration { params, body, .. } = node else {
            return Err("AstOwnershipAnalyzer: expected FunctionDeclaration".to_string());
        };

        let scope_id = self.alloc_scope(ScopeKind::Function, parent);

        self.push_env();
        for name in params {
            self.declare_binding(scope_id, name)?;
        }

        let result: Result<(), String> = body
            .iter()
            .try_for_each(|stmt| self.analyze_node(stmt, scope_id, false));
        self.pop_env();
        result?;

        Ok(scope_id)
    }

    pub(super) fn propagate_to_parent(&mut self, child_id: ScopeId) {
        let (parent_id, reads, writes, cond_reads) = {
            let child = &self.scopes[&child_id];
            (
                child.parent,
                child.reads.clone(),
                child.writes.clone(),
                child.condition_reads.clone(),
            )
        };

        if let Some(parent_id) = parent_id {
            let parent = self.scopes.get_mut(&parent_id).unwrap();
            for b in reads {
                if let Some(info) = self.bindings.get(&b) {
                    if info.owner_scope == child_id {
                        continue;
                    }
                }
                parent.reads.insert(b);
            }
            for b in writes {
                if let Some(info) = self.bindings.get(&b) {
                    if info.owner_scope == child_id {
                        continue;
                    }
                }
                parent.writes.insert(b);
            }
            for b in cond_reads {
                if let Some(info) = self.bindings.get(&b) {
                    if info.owner_scope == child_id {
                        continue;
                    }
                }
                parent.condition_reads.insert(b);
            }
        }
    }

    fn build_plans(&self) -> Result<Vec<OwnershipPlan>, String> {
        let mut plans = Vec::new();

        for (_, scope) in &self.scopes {
            if scope.kind != ScopeKind::Loop && scope.kind != ScopeKind::Function {
                continue;
            }
            plans.push(self.build_plan_for_scope(scope.id)?);
        }

        Ok(plans)
    }

    pub(super) fn relay_path_to_owner(
        &self,
        from_scope: ScopeId,
        owner_scope: ScopeId,
    ) -> Result<Vec<ScopeId>, String> {
        let mut current = from_scope;
        let mut path = Vec::new();

        while current != owner_scope {
            let scope = self
                .scopes
                .get(&current)
                .ok_or_else(|| format!("AstOwnershipAnalyzer: scope {:?} not found", current))?;
            if scope.kind == ScopeKind::Loop {
                path.push(current);
            }
            current = scope.parent.ok_or_else(|| {
                format!(
                    "AstOwnershipAnalyzer: no parent while searching relay path: from={:?} owner={:?}",
                    from_scope, owner_scope
                )
            })?;
        }

        Ok(path)
    }

    pub(super) fn resolve_binding(&self, name: &str) -> Option<BindingId> {
        self.env_stack
            .iter()
            .rev()
            .find_map(|frame| frame.get(name).copied())
    }

    pub(super) fn push_env(&mut self) {
        self.env_stack.push(BTreeMap::new());
    }

    pub(super) fn pop_env(&mut self) {
        self.env_stack.pop();
    }

    pub(super) fn declare_binding(
        &mut self,
        scope_id: ScopeId,
        name: &str,
    ) -> Result<BindingId, String> {
        let Some(frame) = self.env_stack.last_mut() else {
            return Err(
                "AstOwnershipAnalyzer: internal error: declare_binding with empty env_stack"
                    .to_string(),
            );
        };

        if let Some(existing) = frame.get(name).copied() {
            self.scopes
                .get_mut(&scope_id)
                .ok_or_else(|| format!("AstOwnershipAnalyzer: scope {:?} not found", scope_id))?
                .declared
                .entry(name.to_string())
                .or_insert(existing);
            return Ok(existing);
        }

        let binding = BindingId(self.next_binding_id);
        self.next_binding_id += 1;

        frame.insert(name.to_string(), binding);
        self.bindings.insert(
            binding,
            BindingInfo {
                name: name.to_string(),
                owner_scope: scope_id,
            },
        );
        self.scopes
            .get_mut(&scope_id)
            .ok_or_else(|| format!("AstOwnershipAnalyzer: scope {:?} not found", scope_id))?
            .declared
            .insert(name.to_string(), binding);

        Ok(binding)
    }

    pub(super) fn record_read(
        &mut self,
        binding: BindingId,
        scope_id: ScopeId,
        is_condition: bool,
    ) {
        let scope = self.scopes.get_mut(&scope_id).expect("scope must exist");
        scope.reads.insert(binding);
        if is_condition {
            scope.condition_reads.insert(binding);
        }
    }

    pub(super) fn record_write(&mut self, binding: BindingId, scope_id: ScopeId) {
        let scope = self.scopes.get_mut(&scope_id).expect("scope must exist");
        scope.writes.insert(binding);
    }
}

impl Default for AstOwnershipAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
