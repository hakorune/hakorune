//! Initialized-local input materialization shared by loop physicalizers.
//!
//! This cell emits the resolver-issued initializer literal for each verified
//! input relation into the preheader and returns the loop-entry rows. It is
//! the single owner for "declared local -> seeded carrier" entry
//! materialization; prelude call emission and tail handling stay with their
//! own owners. No names are inspected and no semantic claim is minted here.

use super::super::canonical_ssa::CanonicalSsaFunctionSessionV2;
use super::topology::{ReadyLoopEntryRowV1, ReadyLoopEntryV1};
use crate::ast::{ASTNode, LiteralValue};
use crate::mir::builder::emission::constant;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::loop_recipe_contract::VerifiedLoopInitializedLocalInputSourceSetV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingOriginV1, FunctionOwnerIdV1, OwnedExprSiteV1,
};
use crate::mir::BasicBlockId;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum InitializedLocalInputMaterializationRejectV1 {
    OwnerMismatch,
    InputBindingMissing,
    InputBindingMismatch,
    InputInitializerNavigation(String),
    InputInitializerUnsupported,
    InputDeclaration(String),
}

impl std::fmt::Display for InitializedLocalInputMaterializationRejectV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OwnerMismatch => {
                formatter.write_str("[freeze:contract][initialized_input/owner_mismatch]")
            }
            Self::InputBindingMissing => formatter
                .write_str("[freeze:contract][initialized_input/input_binding_missing]"),
            Self::InputBindingMismatch => formatter
                .write_str("[freeze:contract][initialized_input/input_binding_mismatch]"),
            Self::InputInitializerNavigation(error) => write!(
                formatter,
                "[freeze:contract][initialized_input/input_initializer_navigation] {error}"
            ),
            Self::InputInitializerUnsupported => formatter
                .write_str("[freeze:contract][initialized_input/input_initializer_unsupported]"),
            Self::InputDeclaration(error) => write!(
                formatter,
                "[freeze:contract][initialized_input/input_declaration] {error}"
            ),
        }
    }
}

impl std::error::Error for InitializedLocalInputMaterializationRejectV1 {}

/// Emit every initialized-local declaration and seed the loop-entry rows in
/// the preheader. The input relations were issued by the same co-seal that
/// produced the Recipe, so this cell only publishes declarations and returns
/// the entry view; it never re-pairs source rows with Recipe keys.
pub(super) fn materialize_initialized_local_inputs_v1(
    builder: &mut MirBuilder,
    session: &mut CanonicalSsaFunctionSessionV2<'_>,
    owner: FunctionOwnerIdV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    input_relations: &VerifiedLoopInitializedLocalInputSourceSetV1,
    preheader: BasicBlockId,
) -> Result<ReadyLoopEntryV1, InitializedLocalInputMaterializationRejectV1> {
    if input.owner() != owner || input_relations.owner() != owner {
        return Err(InitializedLocalInputMaterializationRejectV1::OwnerMismatch);
    }
    let mut entry_rows = Vec::with_capacity(input_relations.rows().len());
    for input_relation in input_relations.rows() {
        let initializer_site =
            OwnedExprSiteV1::new(owner, input_relation.initializer().clone());
        let initializer = input.source().expr_at(&initializer_site).map_err(|error| {
            InitializedLocalInputMaterializationRejectV1::InputInitializerNavigation(
                error.to_string(),
            )
        })?;
        let initial_value = match initializer.node() {
            ASTNode::Literal {
                value: LiteralValue::Integer(value),
                ..
            } => value,
            _ => {
                return Err(
                    InitializedLocalInputMaterializationRejectV1::InputInitializerUnsupported,
                )
            }
        };
        let input_binding = input
            .function()
            .declaration_binding(input_relation.declaration())
            .ok_or(InitializedLocalInputMaterializationRejectV1::InputBindingMissing)?;
        if input_binding != input_relation.source_binding() {
            return Err(InitializedLocalInputMaterializationRejectV1::InputBindingMismatch);
        }
        let input_record = input
            .function()
            .binding(input_binding)
            .ok_or(InitializedLocalInputMaterializationRejectV1::InputBindingMissing)?;
        let BindingKindV1::Local { .. } = input_record.kind() else {
            return Err(InitializedLocalInputMaterializationRejectV1::InputBindingMismatch);
        };
        if !matches!(
            input_record.origin(),
            BindingOriginV1::Source(site) if *site == *input_relation.declaration()
        ) {
            return Err(InitializedLocalInputMaterializationRejectV1::InputBindingMismatch);
        }
        let input_value = constant::emit_integer(builder, *initial_value)
            .map_err(InitializedLocalInputMaterializationRejectV1::InputDeclaration)?;
        session
            .identity
            .publish_declaration(
                input_relation.declaration(),
                input_record.kind(),
                input_record.diagnostic_name(),
                preheader,
                input_value,
            )
            .map_err(InitializedLocalInputMaterializationRejectV1::InputDeclaration)?;
        entry_rows.push(ReadyLoopEntryRowV1::new(
            input_relation.recipe_value(),
            input_binding,
            input_value,
        ));
    }
    Ok(ReadyLoopEntryV1::from_rows(owner, preheader, entry_rows))
}
