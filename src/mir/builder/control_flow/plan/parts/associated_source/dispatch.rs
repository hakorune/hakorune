//! Sole acceptance dispatcher for verified Parts recipe/source items.
//!
//! This module is a child of `associated_source` so it can consume the private
//! port/item pair by value. It selects no source carrier, performs no Builder
//! mutation, and owns no raw/located alternate route.

use crate::mir::builder::control_flow::plan::recipe_tree::{ExitKind, IfContractKind, IfMode};

use super::{PartsAssociatedRecipeItemV1, PartsAssociatedSourceV1, VerifiedPartsAssociatedItemV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder::control_flow::plan::parts) enum PartsAssociatedBlockModeV1 {
    ExitOnly,
    ExitAllowed,
    StmtOnly,
    NoExit,
}

pub(in crate::mir::builder::control_flow::plan::parts) trait PartsAssociatedLoweringHooksV1<S>
where
    S: PartsAssociatedSourceV1,
{
    type Output;

    fn lower_opaque_stmt(
        &mut self,
        port: S::PortHandle,
        source: S::StmtInput,
    ) -> Result<Self::Output, String>;

    fn lower_opaque_exit(
        &mut self,
        port: S::PortHandle,
        source: S::StmtInput,
        kind: ExitKind,
    ) -> Result<Self::Output, String>;

    #[allow(clippy::too_many_arguments)]
    fn lower_explicit_if(
        &mut self,
        port: S::PortHandle,
        source: S::StmtInput,
        condition: S::ConditionInput,
        then_body: S::BodyInput,
        else_body: Option<S::BodyInput>,
        contract: IfContractKind,
        then_block: S::BlockInput,
        else_block: Option<S::BlockInput>,
    ) -> Result<Self::Output, String>;

    fn lower_stmt_wrapped_join_if(
        &mut self,
        port: S::PortHandle,
        bridge: S::WrappedJoinInput,
    ) -> Result<Self::Output, String>;

    fn lower_raw_loop_v0(
        &mut self,
        port: S::PortHandle,
        loop_input: S::LoopInput,
    ) -> Result<Self::Output, String>;
}

pub(in crate::mir::builder::control_flow::plan::parts) fn lower_verified_parts_associated_item<
    S,
    H,
>(
    mode: PartsAssociatedBlockModeV1,
    verified: VerifiedPartsAssociatedItemV1<
        S::PortHandle,
        S::StmtInput,
        S::ConditionInput,
        S::BodyInput,
        S::BlockInput,
        S::WrappedJoinInput,
        S::LoopInput,
    >,
    hooks: &mut H,
    error_prefix: &str,
) -> Result<H::Output, String>
where
    S: PartsAssociatedSourceV1,
    H: PartsAssociatedLoweringHooksV1<S>,
{
    let VerifiedPartsAssociatedItemV1 { port, item } = verified;
    match (mode, item) {
        (
            PartsAssociatedBlockModeV1::ExitOnly
            | PartsAssociatedBlockModeV1::ExitAllowed
            | PartsAssociatedBlockModeV1::StmtOnly
            | PartsAssociatedBlockModeV1::NoExit,
            PartsAssociatedRecipeItemV1::OpaqueStmt { source },
        ) => hooks.lower_opaque_stmt(port, source),
        (
            PartsAssociatedBlockModeV1::ExitOnly | PartsAssociatedBlockModeV1::ExitAllowed,
            PartsAssociatedRecipeItemV1::OpaqueExit { source, kind },
        ) => hooks.lower_opaque_exit(port, source, kind),
        (
            PartsAssociatedBlockModeV1::ExitOnly | PartsAssociatedBlockModeV1::ExitAllowed,
            PartsAssociatedRecipeItemV1::ExplicitIfV2 {
                source,
                condition,
                then_body,
                else_body,
                contract,
                then_block,
                else_block,
            },
        ) if matches!(
            contract,
            IfContractKind::ExitOnly {
                mode: IfMode::ExitIf | IfMode::ExitAll
            } | IfContractKind::ExitAllowed {
                mode: IfMode::ElseOnlyExit | IfMode::ThenOnlyExit
            }
        ) =>
        {
            hooks.lower_explicit_if(
                port, source, condition, then_body, else_body, contract, then_block, else_block,
            )
        }
        (
            PartsAssociatedBlockModeV1::NoExit,
            PartsAssociatedRecipeItemV1::ExplicitIfV2 {
                source,
                condition,
                then_body,
                else_body,
                contract: IfContractKind::Join,
                then_block,
                else_block,
            },
        ) => hooks.lower_explicit_if(
            port,
            source,
            condition,
            then_body,
            else_body,
            IfContractKind::Join,
            then_block,
            else_block,
        ),
        (
            PartsAssociatedBlockModeV1::ExitAllowed,
            PartsAssociatedRecipeItemV1::StmtWrappedJoinIf { bridge },
        ) => hooks.lower_stmt_wrapped_join_if(port, bridge),
        (
            PartsAssociatedBlockModeV1::ExitOnly
            | PartsAssociatedBlockModeV1::ExitAllowed
            | PartsAssociatedBlockModeV1::NoExit,
            PartsAssociatedRecipeItemV1::RawLoopV0 { loop_input },
        ) => hooks.lower_raw_loop_v0(port, loop_input),
        (PartsAssociatedBlockModeV1::StmtOnly, _) => Err(format!(
            "[freeze:contract][recipe] stmt_only_block_contains_non_stmt_item: ctx={error_prefix}"
        )),
        _ => Err(format!(
            "[freeze:contract][recipe] dispatch_saw_unsupported_item: ctx={error_prefix}"
        )),
    }
}
