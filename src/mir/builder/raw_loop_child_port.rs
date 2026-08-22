//! Raw Loop child-entry port boundary.
//!
//! This module owns only the two existing raw Loop-entry implementations. It
//! is a behavior-neutral BoxShape split: source ownership, callable semantic
//! consumption, JoinIR routing, and physical lowering remain unchanged.

use crate::ast::ASTNode;
use crate::mir::{MirBuilder, ValueId};

use super::raw_loop_child_entry::PreparedLocatedRawLoopChildEntryV1;
use super::recursive_child_lowering::{RawInvocationChildPortV1, RawLegacyChildLoweringPortV1};

/// One raw Loop child-entry boundary.
///
/// This boundary owns only the decision whether a raw invocation may delegate
/// to the existing JoinIR route owner. It does not pass the invocation port
/// into recipe composition, normalization, or plan lowering.
pub(in crate::mir::builder) trait RawLoopChildEntryPortV1 {
    fn lower_loop(
        &mut self,
        builder: &mut MirBuilder,
        loop_node: ASTNode,
    ) -> Result<ValueId, String>;
}

impl RawLoopChildEntryPortV1 for RawLegacyChildLoweringPortV1 {
    fn lower_loop(
        &mut self,
        builder: &mut MirBuilder,
        loop_node: ASTNode,
    ) -> Result<ValueId, String> {
        let ASTNode::Loop {
            condition, body, ..
        } = loop_node
        else {
            return Err("[freeze:contract][raw-loop-child-entry/expected-loop]".to_owned());
        };
        crate::mir::builder::control_flow::joinir::routing::lower_loop_or_freeze_v1(
            builder, *condition, body,
        )
    }
}

impl RawLoopChildEntryPortV1 for RawInvocationChildPortV1<'_, '_> {
    fn lower_loop(
        &mut self,
        builder: &mut MirBuilder,
        loop_node: ASTNode,
    ) -> Result<ValueId, String> {
        let source = self.active_source.as_ref().ok_or_else(|| {
            "[freeze:contract][raw-loop-child-entry/missing-located-source]".to_owned()
        })?;
        let callable_handoff = self.issue_callable_loop_binding_schedule_v1()?;
        let admission_observation = self.generic_loop_diagnostic.issue_for_loop(source);
        PreparedLocatedRawLoopChildEntryV1::prepare_with_method_source_observation(
            source,
            loop_node,
            callable_handoff,
            self.generic_loop_diagnostic.method_source().cloned(),
            admission_observation,
        )?
        .lower_with_existing_route_v1(builder)
    }
}
