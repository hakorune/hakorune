//! One-shot, source-bound terminality qualification for the ordered loop registry.
//!
//! The carrier fields stay in this parent so only its transaction child can
//! consume the exact source/Facts pair. This is deliberately not a raw-parts API.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::facts::LoopFacts;

mod all_route_preflight;
mod logical_product;
mod transaction;

pub(crate) use all_route_preflight::issue_all_route_preflight_v1;
pub(crate) use logical_product::issue_pre_effect_terminal_v1;
pub(crate) use transaction::qualify_live_loop_facts_v1;

/// Opaque non-Clone capability binding one live loop frame to its derived facts.
#[derive(Debug)]
pub(in crate::mir::builder) struct LiveLoopFactsV1<'src> {
    condition: &'src ASTNode,
    body: &'src [ASTNode],
    facts: LoopFacts,
}

/// The facts builder is the sole bridge from AST observation into this registry box.
pub(in crate::mir::builder) fn bind_live_loop_facts_v1<'src>(
    condition: &'src ASTNode,
    body: &'src [ASTNode],
    facts: LoopFacts,
) -> LiveLoopFactsV1<'src> {
    LiveLoopFactsV1 {
        condition,
        body,
        facts,
    }
}

/// A route that is proven to stop legacy scheduling, without claiming success.
#[derive(Debug)]
pub(crate) struct PreEffectSchedulerTerminalV1<'src> {
    route: super::route_id::LoopRouteId,
    unreached_legacy_tail: Box<[super::route_id::LoopRouteId]>,
    source_lease: DirectTerminalSourceLeaseV1<'src>,
}

#[derive(Debug)]
struct DirectSimpleWhileSourceLeaseV1<'src> {
    condition: &'src ASTNode,
    step: &'src ASTNode,
}

#[derive(Debug)]
enum DirectTerminalSourceLeaseV1<'src> {
    SimpleWhile(DirectSimpleWhileSourceLeaseV1<'src>),
    AccumConstLoop(DirectAccumConstLoopSourceLeaseV1<'src>),
    LoopBreak(DirectLoopBreakSourceLeaseV1<'src>),
}

#[derive(Debug)]
struct DirectAccumConstLoopSourceLeaseV1<'src> {
    condition: &'src ASTNode,
    acc_update: &'src ASTNode,
    step: &'src ASTNode,
}

#[derive(Debug)]
struct DirectLoopBreakSourceLeaseV1<'src> {
    condition: &'src ASTNode,
    break_if: &'src ASTNode,
    carrier_update: &'src ASTNode,
    step: &'src ASTNode,
}

impl<'src> PreEffectSchedulerTerminalV1<'src> {
    pub(crate) fn route(&self) -> super::route_id::LoopRouteId {
        self.route
    }

    pub(crate) fn unreached_legacy_tail(&self) -> &[super::route_id::LoopRouteId] {
        &self.unreached_legacy_tail
    }
}

/// Fail-closed result of the one-shot ordered terminality transaction.
#[derive(Debug)]
pub(crate) enum LiveOrderedTerminalityDispositionV1<'src> {
    NoRoute,
    BlockedCurrent { route: super::route_id::LoopRouteId },
    BlockedEarlier { route: super::route_id::LoopRouteId },
    PreEffectSchedulerTerminal(PreEffectSchedulerTerminalV1<'src>),
}
