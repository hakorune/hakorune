//! Opaque direct-SimpleWhile product issuance from terminality evidence only.

use super::{LiveOrderedTerminalityDispositionV1, PreEffectSchedulerTerminalV1};
use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;

#[derive(Debug)]
pub(crate) struct VerifiedDirectSimpleWhileLogicalProductV1<'src> {
    terminality: PreEffectSchedulerTerminalV1<'src>,
    roles: [DirectSimpleWhileLogicalRoleV1; 2],
}

#[derive(Debug)]
pub(crate) struct VerifiedDirectAccumConstLoopLogicalProductV1<'src> {
    terminality: PreEffectSchedulerTerminalV1<'src>,
    roles: [DirectAccumConstLoopLogicalRoleV1; 3],
}

#[derive(Debug)]
pub(crate) struct VerifiedDirectLoopBreakLogicalProductV1<'src> {
    terminality: PreEffectSchedulerTerminalV1<'src>,
    roles: [DirectLoopBreakLogicalRoleV1; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DirectAccumConstLoopLogicalRoleV1 {
    LoopBinding,
    AccumulatorBinding,
    LoopBackContinuation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DirectLoopBreakLogicalRoleV1 {
    LoopCondition,
    BreakIfSubtree,
    CarrierUpdate,
    LoopBackContinuation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DirectSimpleWhileLogicalRoleV1 {
    LoopBinding,
    LoopBackContinuation,
}

impl<'src> VerifiedDirectSimpleWhileLogicalProductV1<'src> {
    pub(crate) fn route(&self) -> LoopRouteId {
        self.terminality.route()
    }

    pub(crate) fn unreached_legacy_tail(&self) -> &[LoopRouteId] {
        self.terminality.unreached_legacy_tail()
    }
}

impl<'src> VerifiedDirectAccumConstLoopLogicalProductV1<'src> {
    pub(crate) fn route(&self) -> LoopRouteId {
        self.terminality.route()
    }

    pub(crate) fn unreached_legacy_tail(&self) -> &[LoopRouteId] {
        self.terminality.unreached_legacy_tail()
    }
}

impl<'src> VerifiedDirectLoopBreakLogicalProductV1<'src> {
    pub(crate) fn route(&self) -> LoopRouteId {
        self.terminality.route()
    }

    pub(crate) fn unreached_legacy_tail(&self) -> &[LoopRouteId] {
        self.terminality.unreached_legacy_tail()
    }
}

#[derive(Debug)]
pub(crate) enum VerifiedLoopLogicalProductV1<'src> {
    DirectSimpleWhile(VerifiedDirectSimpleWhileLogicalProductV1<'src>),
    DirectAccumConstLoop(VerifiedDirectAccumConstLoopLogicalProductV1<'src>),
    DirectLoopBreak(VerifiedDirectLoopBreakLogicalProductV1<'src>),
}

impl<'src> VerifiedLoopLogicalProductV1<'src> {
    pub(crate) fn route(&self) -> LoopRouteId {
        match self {
            Self::DirectSimpleWhile(product) => product.route(),
            Self::DirectAccumConstLoop(product) => product.route(),
            Self::DirectLoopBreak(product) => product.route(),
        }
    }

    pub(crate) fn unreached_legacy_tail(&self) -> &[LoopRouteId] {
        match self {
            Self::DirectSimpleWhile(product) => product.unreached_legacy_tail(),
            Self::DirectAccumConstLoop(product) => product.unreached_legacy_tail(),
            Self::DirectLoopBreak(product) => product.unreached_legacy_tail(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum LoopLogicalProductDispositionV1<'src> {
    NoRoute,
    BlockedCurrent { route: LoopRouteId },
    BlockedEarlier { route: LoopRouteId },
    Issued(VerifiedLoopLogicalProductV1<'src>),
}

/// The issuer consumes terminality proof and has no source/Facts/selection input.
pub(crate) fn issue_pre_effect_terminal_v1<'src>(
    disposition: LiveOrderedTerminalityDispositionV1<'src>,
) -> LoopLogicalProductDispositionV1<'src> {
    match disposition {
        LiveOrderedTerminalityDispositionV1::NoRoute => LoopLogicalProductDispositionV1::NoRoute,
        LiveOrderedTerminalityDispositionV1::BlockedCurrent { route } => {
            LoopLogicalProductDispositionV1::BlockedCurrent { route }
        }
        LiveOrderedTerminalityDispositionV1::BlockedEarlier { route } => {
            LoopLogicalProductDispositionV1::BlockedEarlier { route }
        }
        LiveOrderedTerminalityDispositionV1::PreEffectSchedulerTerminal(terminality) => {
            let product = match terminality.route() {
                LoopRouteId::LoopSimpleWhile => VerifiedLoopLogicalProductV1::DirectSimpleWhile(
                    VerifiedDirectSimpleWhileLogicalProductV1 {
                        terminality,
                        roles: [
                            DirectSimpleWhileLogicalRoleV1::LoopBinding,
                            DirectSimpleWhileLogicalRoleV1::LoopBackContinuation,
                        ],
                    },
                ),
                LoopRouteId::AccumConstLoop => VerifiedLoopLogicalProductV1::DirectAccumConstLoop(
                    VerifiedDirectAccumConstLoopLogicalProductV1 {
                        terminality,
                        roles: [
                            DirectAccumConstLoopLogicalRoleV1::LoopBinding,
                            DirectAccumConstLoopLogicalRoleV1::AccumulatorBinding,
                            DirectAccumConstLoopLogicalRoleV1::LoopBackContinuation,
                        ],
                    },
                ),
                LoopRouteId::LoopBreakRecipe => VerifiedLoopLogicalProductV1::DirectLoopBreak(
                    VerifiedDirectLoopBreakLogicalProductV1 {
                        terminality,
                        roles: [
                            DirectLoopBreakLogicalRoleV1::LoopCondition,
                            DirectLoopBreakLogicalRoleV1::BreakIfSubtree,
                            DirectLoopBreakLogicalRoleV1::CarrierUpdate,
                            DirectLoopBreakLogicalRoleV1::LoopBackContinuation,
                        ],
                    },
                ),
                route => return LoopLogicalProductDispositionV1::BlockedCurrent { route },
            };
            LoopLogicalProductDispositionV1::Issued(product)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{issue_pre_effect_terminal_v1, LoopLogicalProductDispositionV1};
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::joinir::route_entry::registry::live_ordered_terminality::qualify_live_loop_facts_v1;
    use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
    use crate::mir::builder::control_flow::plan::facts::try_build_live_loop_facts;

    #[test]
    fn issues_only_actual_direct_simple_while_with_ordered_tail() {
        let variable = |name: &str| ASTNode::Variable {
            name: name.into(),
            span: Span::unknown(),
        };
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(variable("i")),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(3),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let body = vec![ASTNode::Assignment {
            target: Box::new(variable("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(variable("i")),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];
        let live = try_build_live_loop_facts(&condition, &body)
            .unwrap()
            .unwrap();
        assert!(matches!(
            issue_pre_effect_terminal_v1(qualify_live_loop_facts_v1(live)),
            LoopLogicalProductDispositionV1::Issued(product)
                if product.route() == LoopRouteId::LoopSimpleWhile
                && product.unreached_legacy_tail() == [LoopRouteId::GenericLoopV0]
        ));
    }

    #[test]
    fn issues_only_actual_direct_accum_with_empty_tail() {
        let variable = |name: &str| ASTNode::Variable {
            name: name.into(),
            span: Span::unknown(),
        };
        let integer = |value| ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        };
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(variable("i")),
            right: Box::new(integer(3)),
            span: Span::unknown(),
        };
        let increment = |name: &str| ASTNode::Assignment {
            target: Box::new(variable(name)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(variable(name)),
                right: Box::new(integer(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let body = vec![increment("sum"), increment("i")];
        let live = try_build_live_loop_facts(&condition, &body)
            .unwrap()
            .unwrap();
        assert!(matches!(
            issue_pre_effect_terminal_v1(qualify_live_loop_facts_v1(live)),
            LoopLogicalProductDispositionV1::Issued(product)
                if product.route() == LoopRouteId::AccumConstLoop
                && product.unreached_legacy_tail().is_empty()
        ));
    }

    #[test]
    fn issues_only_actual_direct_loop_break_with_empty_tail() {
        let variable = |name: &str| ASTNode::Variable {
            name: name.into(),
            span: Span::unknown(),
        };
        let integer = |value| ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        };
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(variable("i")),
            right: Box::new(integer(3)),
            span: Span::unknown(),
        };
        let increment = |name: &str| ASTNode::Assignment {
            target: Box::new(variable(name)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(variable(name)),
                right: Box::new(integer(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let body = vec![
            ASTNode::If {
                condition: Box::new(variable("stop")),
                then_body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            },
            increment("sum"),
            increment("i"),
        ];
        let live = try_build_live_loop_facts(&condition, &body)
            .unwrap()
            .unwrap();
        assert!(matches!(
            issue_pre_effect_terminal_v1(qualify_live_loop_facts_v1(live)),
            LoopLogicalProductDispositionV1::Issued(product)
                if product.route() == LoopRouteId::LoopBreakRecipe
                && product.unreached_legacy_tail().is_empty()
        ));
    }
}
