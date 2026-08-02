//! Opaque direct-SimpleWhile product issuance from terminality evidence only.

use super::{LiveOrderedTerminalityDispositionV1, PreEffectSchedulerTerminalV1};
use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;

#[derive(Debug)]
pub(crate) struct VerifiedDirectSimpleWhileLogicalProductV1<'src> {
    terminality: PreEffectSchedulerTerminalV1<'src>,
    roles: [DirectSimpleWhileLogicalRoleV1; 2],
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

#[derive(Debug)]
pub(crate) enum LoopLogicalProductDispositionV1<'src> {
    NoRoute,
    BlockedCurrent { route: LoopRouteId },
    BlockedEarlier { route: LoopRouteId },
    Issued(VerifiedDirectSimpleWhileLogicalProductV1<'src>),
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
            LoopLogicalProductDispositionV1::Issued(VerifiedDirectSimpleWhileLogicalProductV1 {
                terminality,
                roles: [
                    DirectSimpleWhileLogicalRoleV1::LoopBinding,
                    DirectSimpleWhileLogicalRoleV1::LoopBackContinuation,
                ],
            })
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
}
