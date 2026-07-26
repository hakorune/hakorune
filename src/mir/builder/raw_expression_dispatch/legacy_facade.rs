//! Legacy AST ingress for the single Raw expression matcher.

use crate::ast::ASTNode;
use crate::mir::{MirBuilder, ValueId};

use super::input_view::{RawExpressionInputViewV1, RawLegacyExpressionInputV1};
use super::RawExpressionDispatchPortV1;
use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;

impl MirBuilder {
    /// Legacy facade for the one generic Raw dispatcher.
    pub(in crate::mir::builder) fn build_expression_impl(
        &mut self,
        ast: ASTNode,
    ) -> Result<ValueId, String> {
        let mut port = RawLegacyChildLoweringPortV1;
        self.build_expression_input_view_with_port_v1(
            &mut port,
            RawLegacyExpressionInputV1::new(ast),
        )
    }

    /// Thin transport boundary into the sole AST match tree.
    pub(in crate::mir::builder) fn build_expression_input_view_with_port_v1<Port, Input>(
        &mut self,
        port: &mut Port,
        input: Input,
    ) -> Result<ValueId, String>
    where
        Port: RawExpressionDispatchPortV1,
        Input: RawExpressionInputViewV1,
    {
        self.build_expression_impl_with_port_v1(port, input.into_legacy_expression())
    }
}
