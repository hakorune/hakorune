use crate::ast::ASTNode;
use crate::mir::builder::control_flow::generic_loop_canon::matches_loop_increment;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) struct CandidateSiteV0 {
    pub preorder_index: usize,
    pub top_level_stmt_index: usize,
    pub conditional: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CandidateObservationV0 {
    pub candidate: String,
    pub condition_anchored: bool,
    pub existing_true_loop_increment_derived: bool,
    pub writes: Vec<CandidateSiteV0>,
    pub uses: Vec<CandidateSiteV0>,
    pub canonical_step_sites: Vec<CandidateSiteV0>,
    pub uses_outside_canonical_step: Vec<CandidateSiteV0>,
    pub post_update_uses: Vec<CandidateSiteV0>,
    pub conditional_writes: Vec<CandidateSiteV0>,
}

impl CandidateObservationV0 {
    fn new(
        candidate: &str,
        condition_anchored: bool,
        existing_true_loop_increment_derived: bool,
    ) -> Self {
        Self {
            candidate: candidate.to_string(),
            condition_anchored,
            existing_true_loop_increment_derived,
            writes: Vec::new(),
            uses: Vec::new(),
            canonical_step_sites: Vec::new(),
            uses_outside_canonical_step: Vec::new(),
            post_update_uses: Vec::new(),
            conditional_writes: Vec::new(),
        }
    }
}

struct ObservationBuilder<'a> {
    candidate: &'a str,
    canonical_increment: Option<&'a ASTNode>,
    next_preorder_index: usize,
    first_write_index: Option<usize>,
    observation: CandidateObservationV0,
}

/// Observes one already-discovered loop candidate without selecting its role.
///
/// `condition_anchored` and `canonical_increment` are supplied by their
/// existing owners. This observer records structure only and never discovers
/// candidates or recurrence policy by itself.
pub(in crate::mir::builder) fn observe_candidate_progression_v0(
    candidate: &str,
    condition_anchored: bool,
    existing_true_loop_increment_derived: bool,
    body: &[ASTNode],
    canonical_increment: Option<&ASTNode>,
) -> CandidateObservationV0 {
    let mut builder = ObservationBuilder {
        candidate,
        canonical_increment,
        next_preorder_index: 0,
        first_write_index: None,
        observation: CandidateObservationV0::new(
            candidate,
            condition_anchored,
            existing_true_loop_increment_derived,
        ),
    };
    for (top_level_stmt_index, stmt) in body.iter().enumerate() {
        builder.visit_stmt(stmt, top_level_stmt_index, false);
    }
    builder.observation
}

impl ObservationBuilder<'_> {
    fn next_site(&mut self, top_level_stmt_index: usize, conditional: bool) -> CandidateSiteV0 {
        let site = CandidateSiteV0 {
            preorder_index: self.next_preorder_index,
            top_level_stmt_index,
            conditional,
        };
        self.next_preorder_index += 1;
        site
    }

    fn visit_stmt(&mut self, node: &ASTNode, top_level_stmt_index: usize, conditional: bool) {
        match node {
            ASTNode::Assignment { target, value, .. } => {
                let writes_candidate = matches!(target.as_ref(), ASTNode::Variable { name, .. } if name == self.candidate);
                let canonical_step = writes_candidate
                    && self.canonical_increment.is_some_and(|increment| {
                        matches_loop_increment(node, self.candidate, increment)
                    });
                self.visit_expr(value, top_level_stmt_index, conditional, canonical_step);
                if writes_candidate {
                    self.record_write(node, top_level_stmt_index, conditional);
                } else {
                    self.visit_expr(target, top_level_stmt_index, conditional, false);
                }
            }
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => {
                for value in initial_values.iter().flatten() {
                    self.visit_expr(value, top_level_stmt_index, conditional, false);
                }
                if variables.iter().any(|name| name == self.candidate) {
                    self.record_write(node, top_level_stmt_index, conditional);
                }
            }
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                self.visit_expr(condition, top_level_stmt_index, conditional, false);
                for stmt in then_body {
                    self.visit_stmt(stmt, top_level_stmt_index, true);
                }
                if let Some(body) = else_body {
                    for stmt in body {
                        self.visit_stmt(stmt, top_level_stmt_index, true);
                    }
                }
            }
            ASTNode::Program { statements, .. }
            | ASTNode::ScopeBox {
                body: statements, ..
            } => {
                for stmt in statements {
                    self.visit_stmt(stmt, top_level_stmt_index, conditional);
                }
            }
            ASTNode::Loop { .. } | ASTNode::LoopRange { .. } => {
                // Nested-loop state belongs to another loop observation.
            }
            _ => self.visit_expr(node, top_level_stmt_index, conditional, false),
        }
    }

    fn visit_expr(
        &mut self,
        node: &ASTNode,
        top_level_stmt_index: usize,
        conditional: bool,
        within_canonical_step: bool,
    ) {
        match node {
            ASTNode::Variable { name, .. } if name == self.candidate => {
                self.record_use(top_level_stmt_index, conditional, within_canonical_step);
            }
            ASTNode::UnaryOp { operand, .. } => {
                self.visit_expr(
                    operand,
                    top_level_stmt_index,
                    conditional,
                    within_canonical_step,
                );
            }
            ASTNode::BinaryOp { left, right, .. } => {
                self.visit_expr(
                    left,
                    top_level_stmt_index,
                    conditional,
                    within_canonical_step,
                );
                self.visit_expr(
                    right,
                    top_level_stmt_index,
                    conditional,
                    within_canonical_step,
                );
            }
            ASTNode::MethodCall {
                object, arguments, ..
            } => {
                self.visit_expr(
                    object,
                    top_level_stmt_index,
                    conditional,
                    within_canonical_step,
                );
                for argument in arguments {
                    self.visit_expr(
                        argument,
                        top_level_stmt_index,
                        conditional,
                        within_canonical_step,
                    );
                }
            }
            ASTNode::FunctionCall { arguments, .. } => {
                for argument in arguments {
                    self.visit_expr(
                        argument,
                        top_level_stmt_index,
                        conditional,
                        within_canonical_step,
                    );
                }
            }
            ASTNode::FieldAccess { object, .. } => {
                self.visit_expr(
                    object,
                    top_level_stmt_index,
                    conditional,
                    within_canonical_step,
                );
            }
            ASTNode::If { .. }
            | ASTNode::Assignment { .. }
            | ASTNode::Local { .. }
            | ASTNode::Program { .. }
            | ASTNode::ScopeBox { .. } => {
                self.visit_stmt(node, top_level_stmt_index, conditional);
            }
            _ => {}
        }
    }

    fn record_write(&mut self, node: &ASTNode, top_level_stmt_index: usize, conditional: bool) {
        let site = self.next_site(top_level_stmt_index, conditional);
        if self.first_write_index.is_none() {
            self.first_write_index = Some(site.preorder_index);
        }
        if conditional {
            self.observation.conditional_writes.push(site.clone());
        }
        if self
            .canonical_increment
            .is_some_and(|increment| matches_loop_increment(node, self.candidate, increment))
        {
            self.observation.canonical_step_sites.push(site.clone());
        }
        self.observation.writes.push(site);
    }

    fn record_use(
        &mut self,
        top_level_stmt_index: usize,
        conditional: bool,
        within_canonical_step: bool,
    ) {
        let site = self.next_site(top_level_stmt_index, conditional);
        if !within_canonical_step {
            self.observation
                .uses_outside_canonical_step
                .push(site.clone());
        }
        if self
            .first_write_index
            .is_some_and(|write_index| site.preorder_index > write_index)
        {
            self.observation.post_update_uses.push(site.clone());
        }
        self.observation.uses.push(site);
    }
}

#[cfg(test)]
mod tests {
    use super::observe_candidate_progression_v0;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn lit_i(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn add(left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    fn assign(name: &str, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(var(name)),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    fn call_with_arg(argument: ASTNode) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(var("source")),
            method: "operation".to_string(),
            arguments: vec![argument],
            span: Span::unknown(),
        }
    }

    #[test]
    fn observes_canonical_tail_step_without_post_update_use() {
        let increment = add(var("cursor"), lit_i(1));
        let body = vec![assign("cursor", increment.clone())];

        let observation =
            observe_candidate_progression_v0("cursor", true, false, &body, Some(&increment));

        assert!(observation.condition_anchored);
        assert_eq!(observation.writes.len(), 1);
        assert_eq!(observation.uses.len(), 1);
        assert_eq!(observation.canonical_step_sites.len(), 1);
        assert!(observation.post_update_uses.is_empty());
        assert!(observation.conditional_writes.is_empty());
    }

    #[test]
    fn observes_rebased_write_and_post_update_use() {
        let body = vec![
            assign("cursor", call_with_arg(var("cursor"))),
            call_with_arg(var("cursor")),
        ];

        let observation = observe_candidate_progression_v0("cursor", true, false, &body, None);

        assert_eq!(observation.writes.len(), 1);
        assert_eq!(observation.uses.len(), 2);
        assert!(observation.canonical_step_sites.is_empty());
        assert_eq!(observation.post_update_uses.len(), 1);
        assert_eq!(observation.post_update_uses[0].top_level_stmt_index, 1);
    }

    #[test]
    fn observes_multiple_and_conditional_writes() {
        let conditional_write = ASTNode::If {
            condition: Box::new(var("flag")),
            then_body: vec![assign("cursor", call_with_arg(var("cursor")))],
            else_body: None,
            span: Span::unknown(),
        };
        let body = vec![
            assign("cursor", call_with_arg(var("cursor"))),
            conditional_write,
            call_with_arg(var("cursor")),
        ];

        let observation = observe_candidate_progression_v0("cursor", true, false, &body, None);

        assert_eq!(observation.writes.len(), 2);
        assert_eq!(observation.conditional_writes.len(), 1);
        assert_eq!(observation.conditional_writes[0].top_level_stmt_index, 1);
        assert_eq!(observation.post_update_uses.len(), 2);
    }

    #[test]
    fn nested_loop_state_is_not_attributed_to_outer_candidate() {
        let body = vec![ASTNode::Loop {
            condition: Box::new(var("cursor")),
            body: vec![assign("cursor", add(var("cursor"), lit_i(1)))],
            span: Span::unknown(),
        }];

        let observation = observe_candidate_progression_v0("cursor", true, false, &body, None);

        assert!(observation.writes.is_empty());
        assert!(observation.uses.is_empty());
    }
}
