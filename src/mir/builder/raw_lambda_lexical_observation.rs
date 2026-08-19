//! Source-only lexical observation for the raw Lambda compatibility edge.
//!
//! This is deliberately not a `resolved_semantics` owner forest: raw lowering
//! has no parent source provenance from which it could issue `BindingRefV1`.
//! It observes only relative external demands and is later bound once to the
//! legacy name-to-ValueId environment.

use crate::ast::ASTNode;
use std::collections::BTreeSet;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RawLambdaExternalUseKindV1 {
    Read,
    Rebind,
    DirectReceiver,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct RawLambdaExternalUseV1 {
    name: Option<Box<str>>,
    kind: RawLambdaExternalUseKindV1,
    relative_site: u32,
}

impl RawLambdaExternalUseV1 {
    #[cfg(test)]
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    #[cfg(test)]
    fn kind(&self) -> RawLambdaExternalUseKindV1 {
        self.kind
    }

    #[cfg(test)]
    fn relative_site(&self) -> u32 {
        self.relative_site
    }
}

/// Immutable source-only receipt. It contains neither an AST nor any source
/// identity/binding/value handle, so it cannot masquerade as a resolver owner.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct RawLambdaLexicalObservationV1 {
    capture_names: Box<[Box<str>]>,
    uses: Box<[RawLambdaExternalUseV1]>,
    receiver_required: bool,
}

impl RawLambdaLexicalObservationV1 {
    pub(super) fn observe(
        params: &[String],
        body: &[ASTNode],
    ) -> Result<Self, RawLambdaLexicalObservationErrorV1> {
        let mut observer = RawLambdaLexicalObserverV1::new(params);
        observer.observe_body(body)?;
        Ok(Self {
            capture_names: observer.capture_names.into_boxed_slice(),
            uses: observer.uses.into_boxed_slice(),
            receiver_required: observer.receiver_required,
        })
    }

    pub(super) fn capture_names(&self) -> &[Box<str>] {
        &self.capture_names
    }

    pub(super) fn receiver_required(&self) -> bool {
        self.receiver_required
    }

    #[cfg(test)]
    fn uses(&self) -> &[RawLambdaExternalUseV1] {
        &self.uses
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum RawLambdaLexicalObservationErrorV1 {
    NestedOwner { kind: &'static str },
    UnsupportedSurface { kind: &'static str },
}

impl fmt::Display for RawLambdaLexicalObservationErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NestedOwner { kind } => write!(
                formatter,
                "[freeze:contract][mir_builder/raw_lambda_nested_owner] kind={kind} capture forwarding is not implemented"
            ),
            Self::UnsupportedSurface { kind } => write!(
                formatter,
                "[freeze:contract][mir_builder/raw_lambda_unsupported_surface] kind={kind}"
            ),
        }
    }
}

struct RawLambdaLexicalObserverV1 {
    scopes: Vec<BTreeSet<Box<str>>>,
    seen_capture_names: BTreeSet<Box<str>>,
    capture_names: Vec<Box<str>>,
    uses: Vec<RawLambdaExternalUseV1>,
    next_relative_site: u32,
    receiver_required: bool,
}

impl RawLambdaLexicalObserverV1 {
    fn new(params: &[String]) -> Self {
        Self {
            scopes: vec![params.iter().map(|name| name.clone().into()).collect()],
            seen_capture_names: BTreeSet::new(),
            capture_names: Vec::new(),
            uses: Vec::new(),
            next_relative_site: 0,
            receiver_required: false,
        }
    }

    fn observe_body(&mut self, body: &[ASTNode]) -> Result<(), RawLambdaLexicalObservationErrorV1> {
        for statement in body {
            self.observe_node(statement)?;
        }
        Ok(())
    }

    fn observe_child_scope(
        &mut self,
        body: &[ASTNode],
    ) -> Result<(), RawLambdaLexicalObservationErrorV1> {
        self.scopes.push(BTreeSet::new());
        let result = self.observe_body(body);
        self.scopes.pop();
        result
    }

    fn observe_child_scope_with_binding(
        &mut self,
        name: &str,
        body: &[ASTNode],
    ) -> Result<(), RawLambdaLexicalObservationErrorV1> {
        self.scopes.push(BTreeSet::from([Box::<str>::from(name)]));
        let result = self.observe_body(body);
        self.scopes.pop();
        result
    }

    fn declared(&self, name: &str) -> bool {
        self.scopes.iter().rev().any(|scope| scope.contains(name))
    }

    fn declare_current(&mut self, name: &str) {
        self.scopes
            .last_mut()
            .expect("[freeze:contract][mir_builder/raw_lambda_missing_scope]")
            .insert(name.into());
    }

    fn observe_named_use(&mut self, name: &str, kind: RawLambdaExternalUseKindV1) {
        if self.declared(name) {
            return;
        }
        self.record_external(Some(name.into()), kind);
    }

    fn observe_receiver(&mut self) {
        self.receiver_required = true;
        self.record_external(None, RawLambdaExternalUseKindV1::DirectReceiver);
    }

    fn record_external(&mut self, name: Option<Box<str>>, kind: RawLambdaExternalUseKindV1) {
        let relative_site = self.next_relative_site;
        self.next_relative_site = self.next_relative_site.saturating_add(1);
        if let Some(name) = name.as_ref() {
            if self.seen_capture_names.insert(name.clone()) {
                self.capture_names.push(name.clone());
            }
        }
        self.uses.push(RawLambdaExternalUseV1 {
            name,
            kind,
            relative_site,
        });
    }

    fn observe_assignment_target(
        &mut self,
        target: &ASTNode,
        compound: bool,
    ) -> Result<(), RawLambdaLexicalObservationErrorV1> {
        match target {
            ASTNode::Variable { name, .. } => {
                if compound {
                    self.observe_named_use(name, RawLambdaExternalUseKindV1::Read);
                }
                self.observe_named_use(name, RawLambdaExternalUseKindV1::Rebind);
                Ok(())
            }
            ASTNode::Me { .. } => {
                self.observe_receiver();
                Ok(())
            }
            other => self.observe_node(other),
        }
    }

    fn observe_declaration(
        &mut self,
        variables: &[String],
        initial_values: &[Option<Box<ASTNode>>],
        observe_initializers: bool,
    ) -> Result<(), RawLambdaLexicalObservationErrorV1> {
        if observe_initializers {
            for initial in initial_values.iter().flatten() {
                self.observe_node(initial)?;
            }
        }
        for name in variables {
            self.declare_current(name);
        }
        Ok(())
    }

    fn observe_arguments(
        &mut self,
        arguments: &[ASTNode],
    ) -> Result<(), RawLambdaLexicalObservationErrorV1> {
        for argument in arguments {
            self.observe_node(argument)?;
        }
        Ok(())
    }

    fn observe_node(&mut self, node: &ASTNode) -> Result<(), RawLambdaLexicalObservationErrorV1> {
        match node {
            ASTNode::Program { statements, .. } => self.observe_child_scope(statements),
            ASTNode::Assignment { target, value, .. } => {
                self.observe_assignment_target(target, false)?;
                self.observe_node(value)
            }
            ASTNode::CompoundAssignment { target, value, .. } => {
                self.observe_assignment_target(target, true)?;
                self.observe_node(value)
            }
            ASTNode::Print { expression, .. }
            | ASTNode::AwaitExpression { expression, .. }
            | ASTNode::QMarkPropagate { expression, .. }
            | ASTNode::Throw { expression, .. } => self.observe_node(expression),
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                self.observe_node(condition)?;
                self.observe_child_scope(then_body)?;
                if let Some(else_body) = else_body {
                    self.observe_child_scope(else_body)?;
                }
                Ok(())
            }
            ASTNode::Loop {
                condition, body, ..
            } => {
                self.observe_node(condition)?;
                self.observe_child_scope(body)
            }
            ASTNode::LoopRange {
                var_name,
                start,
                end,
                body,
                ..
            } => {
                self.observe_node(start)?;
                self.observe_node(end)?;
                self.observe_child_scope_with_binding(var_name, body)
            }
            ASTNode::Return { value, .. } => match value {
                Some(value) => self.observe_node(value),
                None => Ok(()),
            },
            ASTNode::Break { .. } | ASTNode::Continue { .. } => Ok(()),
            ASTNode::Nowait {
                variable,
                expression,
                ..
            } => {
                self.observe_node(expression)?;
                self.declare_current(variable);
                Ok(())
            }
            ASTNode::TaskScope { body, .. }
            | ASTNode::FastMemRegion { body, .. }
            | ASTNode::ScopeBox { body, .. } => self.observe_child_scope(body),
            ASTNode::ContextScope {
                name, value, body, ..
            } => {
                self.observe_node(value)?;
                self.observe_child_scope_with_binding(name, body)
            }
            ASTNode::MatchExpr {
                scrutinee,
                arms,
                else_expr,
                ..
            } => {
                self.observe_node(scrutinee)?;
                for (_, arm) in arms {
                    self.observe_node(arm)?;
                }
                self.observe_node(else_expr)
            }
            ASTNode::EnumMatchExpr {
                scrutinee,
                arms,
                else_expr,
                ..
            } => {
                self.observe_node(scrutinee)?;
                for arm in arms {
                    if let Some(name) = arm.binding_name.as_deref() {
                        self.scopes.push(BTreeSet::from([Box::<str>::from(name)]));
                        let result = self.observe_node(&arm.body);
                        self.scopes.pop();
                        result?;
                    } else {
                        self.observe_node(&arm.body)?;
                    }
                }
                if let Some(else_expr) = else_expr {
                    self.observe_node(else_expr)?;
                }
                Ok(())
            }
            ASTNode::ArrayLiteral { elements, .. } => self.observe_arguments(elements),
            ASTNode::MapLiteral { entries, .. }
            | ASTNode::RecordLiteral {
                fields: entries, ..
            } => {
                for (_, value) in entries {
                    self.observe_node(value)?;
                }
                Ok(())
            }
            ASTNode::RecordUpdate { base, updates, .. } => {
                self.observe_node(base)?;
                for (_, value) in updates {
                    self.observe_node(value)?;
                }
                Ok(())
            }
            ASTNode::BlockExpr {
                prelude_stmts,
                tail_expr,
                ..
            } => {
                self.scopes.push(BTreeSet::new());
                let result = self
                    .observe_body(prelude_stmts)
                    .and_then(|()| self.observe_node(tail_expr));
                self.scopes.pop();
                result
            }
            ASTNode::Arrow {
                sender, receiver, ..
            } => {
                self.observe_node(sender)?;
                self.observe_node(receiver)
            }
            ASTNode::TryCatch {
                try_body,
                catch_clauses,
                finally_body,
                ..
            } => {
                self.observe_child_scope(try_body)?;
                for clause in catch_clauses {
                    if let Some(name) = clause.variable_name.as_deref() {
                        self.observe_child_scope_with_binding(name, &clause.body)?;
                    } else {
                        self.observe_child_scope(&clause.body)?;
                    }
                }
                if let Some(finally_body) = finally_body {
                    self.observe_child_scope(finally_body)?;
                }
                Ok(())
            }
            ASTNode::Literal { .. } => Ok(()),
            ASTNode::Variable { name, .. } => {
                self.observe_named_use(name, RawLambdaExternalUseKindV1::Read);
                Ok(())
            }
            ASTNode::UnaryOp { operand, .. } => self.observe_node(operand),
            ASTNode::BinaryOp { left, right, .. } => {
                self.observe_node(left)?;
                self.observe_node(right)
            }
            ASTNode::CheckExpr { items, .. } => {
                for item in items {
                    self.observe_node(&item.expression)?;
                }
                Ok(())
            }
            ASTNode::GroupedAssignmentExpr { lhs, rhs, .. } => {
                self.observe_node(rhs)?;
                self.observe_named_use(lhs, RawLambdaExternalUseKindV1::Rebind);
                Ok(())
            }
            ASTNode::MethodCall {
                object, arguments, ..
            } => {
                self.observe_node(object)?;
                self.observe_arguments(arguments)
            }
            ASTNode::FieldAccess { object, .. } => self.observe_node(object),
            ASTNode::Index { target, index, .. } => {
                self.observe_node(target)?;
                self.observe_node(index)
            }
            ASTNode::New {
                arguments,
                field_initializers,
                ..
            } => {
                self.observe_arguments(arguments)?;
                for (_, value) in field_initializers {
                    self.observe_node(value)?;
                }
                Ok(())
            }
            ASTNode::Me { .. } | ASTNode::MeField { .. } => {
                self.observe_receiver();
                Ok(())
            }
            ASTNode::This { .. } | ASTNode::ThisField { .. } => {
                Err(RawLambdaLexicalObservationErrorV1::UnsupportedSurface {
                    kind: node.node_type(),
                })
            }
            ASTNode::FromCall { arguments, .. }
            | ASTNode::FunctionCall { arguments, .. }
            | ASTNode::ExplicitExternCall { arguments, .. } => self.observe_arguments(arguments),
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => self.observe_declaration(variables, initial_values, true),
            ASTNode::Outbox {
                variables,
                initial_values,
                ..
            } => self.observe_declaration(variables, initial_values, false),
            ASTNode::Call {
                callee, arguments, ..
            } => {
                self.observe_node(callee)?;
                self.observe_arguments(arguments)
            }
            ASTNode::Lambda { .. } => {
                Err(RawLambdaLexicalObservationErrorV1::NestedOwner { kind: "Lambda" })
            }
            ASTNode::FunctionDeclaration { .. } => {
                Err(RawLambdaLexicalObservationErrorV1::NestedOwner {
                    kind: "FunctionDeclaration",
                })
            }
            ASTNode::BoxDeclaration { .. } => {
                Err(RawLambdaLexicalObservationErrorV1::NestedOwner {
                    kind: "BoxDeclaration",
                })
            }
            ASTNode::UsingStatement { .. }
            | ASTNode::Release { .. }
            | ASTNode::ImportStatement { .. }
            | ASTNode::BuildGate { .. }
            | ASTNode::EnumDeclaration { .. }
            | ASTNode::BrandDeclaration { .. }
            | ASTNode::TypeAliasDeclaration { .. }
            | ASTNode::GlobalVar { .. }
            | ASTNode::StaticConstTable { .. } => {
                Err(RawLambdaLexicalObservationErrorV1::UnsupportedSurface {
                    kind: node.node_type(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        RawLambdaExternalUseKindV1, RawLambdaLexicalObservationErrorV1,
        RawLambdaLexicalObservationV1,
    };
    use crate::ast::{ASTNode, Span};

    fn variable(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.into(),
            span: Span::unknown(),
        }
    }

    fn local(name: &str, initial: ASTNode) -> ASTNode {
        ASTNode::Local {
            variables: vec![name.into()],
            initial_values: vec![Some(Box::new(initial))],
            declared_type_names: vec![None],
            span: Span::unknown(),
        }
    }

    #[test]
    fn preserves_first_external_demand_order_and_local_initializer_timing() {
        let observation = RawLambdaLexicalObservationV1::observe(
            &["parameter".into()],
            &[
                local("local", variable("outer")),
                variable("local"),
                variable("parameter"),
                variable("outer"),
            ],
        )
        .unwrap();

        assert_eq!(observation.capture_names(), &[Box::<str>::from("outer")]);
        assert_eq!(observation.uses().len(), 2);
        assert_eq!(observation.uses()[0].name(), Some("outer"));
        assert_eq!(
            observation.uses()[0].kind(),
            RawLambdaExternalUseKindV1::Read
        );
        assert_eq!(observation.uses()[0].relative_site(), 0);
        assert_eq!(observation.uses()[1].relative_site(), 1);
    }

    #[test]
    fn direct_receiver_and_string_name_remain_distinct_demands() {
        let observation = RawLambdaLexicalObservationV1::observe(
            &[],
            &[
                ASTNode::Me {
                    span: Span::unknown(),
                },
                variable("me"),
            ],
        )
        .unwrap();

        assert!(observation.receiver_required());
        assert_eq!(observation.capture_names(), &[Box::<str>::from("me")]);
        assert_eq!(
            observation.uses()[0].kind(),
            RawLambdaExternalUseKindV1::DirectReceiver
        );
        assert_eq!(
            observation.uses()[1].kind(),
            RawLambdaExternalUseKindV1::Read
        );
    }

    #[test]
    fn nested_owner_rejects_before_materialization() {
        let error = RawLambdaLexicalObservationV1::observe(
            &[],
            &[ASTNode::Lambda {
                params: vec![],
                body: vec![],
                span: Span::unknown(),
            }],
        )
        .unwrap_err();

        assert_eq!(
            error,
            RawLambdaLexicalObservationErrorV1::NestedOwner { kind: "Lambda" }
        );
    }
}
