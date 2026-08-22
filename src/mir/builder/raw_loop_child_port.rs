//! Raw Loop child-entry port boundary.
//!
//! This module owns the two raw Loop-entry implementations. The legacy port
//! still delegates to JoinIR; the invocation port now consumes a source-backed
//! Ready product through the named semantic Recipe/physical adapter and keeps
//! Outside terminal.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::GenericLoopFactsPolicyFrameV1;
use crate::mir::{MirBuilder, ValueId};

use super::raw_loop_child_entry::PreparedLocatedRawLoopChildEntryV1;
use super::recursive_child_lowering::{RawInvocationChildPortV1, RawLegacyChildLoweringPortV1};

/// One raw Loop child-entry boundary.
///
/// This boundary owns the raw invocation entry. A non-callable handoff may
/// delegate to the legacy JoinIR owner, while a source-backed Ready handoff
/// enters the named Recipe/physical adapter exactly once.
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
        let function_name = builder
            .function_state
            .current_function
            .as_ref()
            .map(|function| function.signature.name.clone())
            .unwrap_or_else(|| "<unknown>".to_owned());
        let debug = crate::config::env::joinir_dev::debug_enabled();
        let in_static_box = builder.comp_ctx.current_static_box.is_some();
        let policy = GenericLoopFactsPolicyFrameV1::from_environment();
        PreparedLocatedRawLoopChildEntryV1::prepare_with_method_source_observation(
            source,
            loop_node,
            callable_handoff,
            self.generic_loop_diagnostic.method_source().cloned(),
            admission_observation,
        )?
        .lower_v1(builder, &function_name, debug, in_static_box, policy)
    }
}
