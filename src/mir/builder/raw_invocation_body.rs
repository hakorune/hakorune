//! Located raw invocation body driver.
//!
//! The source transport owns the receipt; this box only walks the already
//! selected body and scopes each statement through that receipt.

use crate::ast::ASTNode;
use crate::mir::builder::stmts::block_driver::{drive_legacy_block_v1, LegacyBlockDescentPortV1};
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

use super::raw_expression_dispatch::RawExpressionDispatchPortV1;
use super::raw_invocation_source_transport::{
    RawInvocationSourceContextV1, RawSourceTransportPortV1,
};
use super::recursive_child_lowering::RecursiveChildLoweringPortV1;

pub(in crate::mir::builder) fn drive_located_invocation_body_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    statements: Vec<ASTNode>,
    context: RawInvocationSourceContextV1,
) -> Result<ValueId, String>
where
    Port: RawSourceTransportPortV1 + RawExpressionDispatchPortV1,
{
    let mut body = LocatedInvocationBlockPortV1 {
        statements: statements.into_iter(),
        source: context,
        child: port,
    };
    drive_legacy_block_v1(builder, &mut body)
}

struct LocatedInvocationBlockPortV1<'port, Port> {
    statements: std::vec::IntoIter<ASTNode>,
    source: RawInvocationSourceContextV1,
    child: &'port mut Port,
}

impl<Port> LegacyBlockDescentPortV1 for LocatedInvocationBlockPortV1<'_, Port>
where
    Port: RawSourceTransportPortV1
        + RawExpressionDispatchPortV1
        + RecursiveChildLoweringPortV1<StatementInput = ASTNode>,
{
    fn len(&self) -> usize {
        self.statements.len()
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        index: usize,
    ) -> Result<ValueId, String> {
        let statement = self
            .statements
            .next()
            .expect("block driver index stays within the owned source iterator");
        let transport = self.source.body_statement(statement, index);
        self.child
            .with_source_transport_v1(transport, |child, statement| {
                child.lower_statement(builder, statement)
            })
    }
}
