//! Behavior-neutral recursive child-lowering port.
//!
//! This module owns the typed body, statement, and expression entry boundary.
//! It owns no source navigation, callable-result plan, location, ledger,
//! MethodCall route, or result-publication policy.

use crate::ast::ASTNode;
use crate::mir::{MirBuilder, ValueId};

use super::module_lowering_invocation::{LoweringHeaderPortV1, ModuleLoweringPortV1};
use super::raw_expression_dispatch::RawExpressionDispatchPortV1;

const MAX_RAW_EXPRESSION_RECURSION_DEPTH: usize = 200;

pub(in crate::mir::builder) trait RecursiveChildLoweringPortV1 {
    type BodyInput;
    type StatementInput;
    type ExpressionInput;

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::BodyInput,
    ) -> Result<ValueId, String>;

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::StatementInput,
    ) -> Result<ValueId, String>;

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String>;
}

/// Raw AST specialization shared by the legacy facade and the future
/// invocation-aware carrier.
///
/// Located/source-branded ports intentionally do not implement this marker.
/// It permits raw syntax adapters to have one blanket implementation without
/// fabricating a second AST representation or copying any source policy.
pub(in crate::mir::builder) trait RawAstChildLoweringPortV1:
    RecursiveChildLoweringPortV1<
    BodyInput = Vec<ASTNode>,
    StatementInput = ASTNode,
    ExpressionInput = ASTNode,
>
{
}

impl<Port> RawAstChildLoweringPortV1 for Port where
    Port: RecursiveChildLoweringPortV1<
        BodyInput = Vec<ASTNode>,
        StatementInput = ASTNode,
        ExpressionInput = ASTNode,
    >
{
}

pub(in crate::mir::builder) fn drive_legacy_body_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: Port::BodyInput,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1,
{
    port.lower_body(builder, input)
}

pub(in crate::mir::builder) fn drive_legacy_statement_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: Port::StatementInput,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1,
{
    port.lower_statement(builder, input)
}

pub(in crate::mir::builder) fn drive_legacy_expression_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: Port::ExpressionInput,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1,
{
    port.lower_expression(builder, input)
}

pub(in crate::mir::builder) struct RawLegacyChildLoweringPortV1;

/// Stack-owned raw-recursion capability for one module-lowering invocation.
///
/// This is intentionally only the carrier in the first RAWPORT0-M0-R0
/// refactor commit.  The following port-aware dispatcher series consumes it
/// for body, statement, and expression descent.  It owns neither a Builder,
/// collector, header view, AST cache, nor child-terminal authority; all it can
/// do is reborrow the exact invocation port for a shorter recursive frame.
///
/// Keeping this wrapper separate from `RawLegacyChildLoweringPortV1` makes a
/// port drop mechanically visible while the legacy facade remains the sole
/// production route through M0.
pub(in crate::mir::builder) struct RawInvocationChildPortV1<'port, 'collector> {
    module_port: &'port mut ModuleLoweringPortV1<'collector>,
    _seal: RawInvocationChildPortSealV1,
}

struct RawInvocationChildPortSealV1;

impl<'port, 'collector> RawInvocationChildPortV1<'port, 'collector> {
    /// Start one raw recursive frame from the exact invocation port.
    pub(in crate::mir::builder) fn new(
        module_port: &'port mut ModuleLoweringPortV1<'collector>,
    ) -> Self {
        Self {
            module_port,
            _seal: RawInvocationChildPortSealV1,
        }
    }

    /// Reborrow the same invocation capability for one nested raw frame.
    ///
    /// No header borrow crosses this boundary: `with_headers` consumes the
    /// observation closure before the next descendant can mutate state.
    pub(in crate::mir::builder) fn reborrow(&mut self) -> RawInvocationChildPortV1<'_, 'collector> {
        RawInvocationChildPortV1::new(&mut *self.module_port)
    }

    /// Lend the exact collector-backed header view for one observation only.
    pub(in crate::mir::builder) fn with_headers<R>(
        &self,
        observe: impl for<'header> FnOnce(&'header LoweringHeaderPortV1<'header>) -> R,
    ) -> R {
        self.module_port.with_headers(observe)
    }
}

impl RecursiveChildLoweringPortV1 for RawLegacyChildLoweringPortV1 {
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        super::stmts::block_stmt::build_block_with_port_v1(builder, self, input)
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        super::stmts::block_stmt::build_statement_with_port_v1(builder, self, input)
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        lower_raw_expression_with_recursion_guard_v1(builder, self, input)
    }
}

fn lower_raw_expression_with_recursion_guard_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: ASTNode,
) -> Result<ValueId, String>
where
    Port: RawExpressionDispatchPortV1,
{
    let node_kind = std::mem::discriminant(&input);
    with_legacy_expression_recursion_guard_v1(builder, node_kind, move |builder| {
        builder.build_expression_impl_with_port_v1(port, input)
    })
}

pub(in crate::mir::builder) fn with_legacy_expression_recursion_guard_v1<F>(
    builder: &mut MirBuilder,
    node_kind: std::mem::Discriminant<ASTNode>,
    lower: F,
) -> Result<ValueId, String>
where
    F: FnOnce(&mut MirBuilder) -> Result<ValueId, String>,
{
    builder.recursion_depth += 1;
    let current_depth = builder.recursion_depth;
    if current_depth > MAX_RAW_EXPRESSION_RECURSION_DEPTH {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .error("\n[FATAL] ============================================");
        ring0.log.error(&format!(
            "[FATAL] Recursion depth exceeded {} in build_expression",
            MAX_RAW_EXPRESSION_RECURSION_DEPTH
        ));
        ring0
            .log
            .error(&format!("[FATAL] Current depth: {current_depth}"));
        ring0
            .log
            .error(&format!("[FATAL] AST node type: {:?}", node_kind));
        ring0
            .log
            .error("[FATAL] ============================================\n");
        builder.recursion_depth -= 1;
        return Err(format!(
            "Recursion depth exceeded: {current_depth} (possible infinite loop)"
        ));
    }

    let result = lower(builder);
    builder.recursion_depth -= 1;
    result
}

pub(in crate::mir::builder) fn drive_raw_legacy_body_v1(
    builder: &mut MirBuilder,
    input: Vec<ASTNode>,
) -> Result<ValueId, String> {
    let mut port = RawLegacyChildLoweringPortV1;
    drive_legacy_body_v1(builder, &mut port, input)
}

pub(in crate::mir::builder) fn drive_raw_legacy_statement_v1(
    builder: &mut MirBuilder,
    input: ASTNode,
) -> Result<ValueId, String> {
    let mut port = RawLegacyChildLoweringPortV1;
    drive_legacy_statement_v1(builder, &mut port, input)
}

pub(in crate::mir::builder) fn drive_raw_legacy_expression_v1(
    builder: &mut MirBuilder,
    input: ASTNode,
) -> Result<ValueId, String> {
    let mut port = RawLegacyChildLoweringPortV1;
    drive_legacy_expression_v1(builder, &mut port, input)
}
