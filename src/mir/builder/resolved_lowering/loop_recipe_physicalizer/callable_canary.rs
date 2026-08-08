//! Caller-zero Prelude materialization for the callable Loop canary.
//!
//! This adapter consumes only already-sealed resolver capabilities.  It
//! publishes parameter declarations through the canonical identity owner,
//! emits the exact resolver-issued static call through the shared direct-call
//! emitter, and returns one `ReadyLoopEntryV1`.  Tail, Loop operations,
//! Completion, and DraftSeal remain outside this cell.

use super::super::canonical_ssa::{CanonicalBindingReadReceiptV1, CanonicalSsaFunctionSessionV2};
use super::super::trivial_ssa::emit_resolved_header;
use super::topology::{ReadyLoopEntryRowV1, ReadyLoopEntryV1};
use crate::ast::{ASTNode, LiteralValue};
use crate::mir::builder::emission::constant;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::callable_loop_physical_canary::into_canary_parts;
use crate::mir::compiler::loop_physical_prepare::{
    VerifiedCallableFunctionLoweringInputV1, VerifiedCallablePreludeCapabilityV1,
};
use crate::mir::loop_recipe_contract::VerifiedLoopInitializedLocalInputSourceSetV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingOriginV1, OwnedExprSiteV1, SourceBindingSiteV1,
};
use crate::mir::{BasicBlockId, MirType, ValueId};
use hakorune_mir_core::MirValueKind;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum CallablePreludeMaterializationRejectV1 {
    OwnerMismatch,
    InputBindingMissing,
    InputBindingMismatch,
    InputInitializerNavigation(String),
    InputInitializerUnsupported,
    InputDeclaration(String),
    FunctionMissing,
    RootNotFunction,
    ParameterCountMismatch,
    ParameterBindingMissing(u32),
    ParameterRecordMissing,
    ParameterOriginMismatch,
    ParameterKindMismatch(u32),
    ParameterAbiUnsupported(u32),
    PreludeResultBindingMissing,
    PreludeResultOriginMismatch,
    PreludeResultKindUnsupported,
    TargetHeaderMissing,
    ArgumentIdentity(String),
    DirectCall(String),
    ResultDeclaration(String),
}

impl std::fmt::Display for CallablePreludeMaterializationRejectV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OwnerMismatch => {
                formatter.write_str("[freeze:contract][callable_prelude/owner_mismatch]")
            }
            Self::InputBindingMissing => {
                formatter.write_str("[freeze:contract][callable_prelude/input_binding_missing]")
            }
            Self::InputBindingMismatch => {
                formatter.write_str("[freeze:contract][callable_prelude/input_binding_mismatch]")
            }
            Self::InputInitializerNavigation(error) => write!(
                formatter,
                "[freeze:contract][callable_prelude/input_initializer_navigation] {error}"
            ),
            Self::InputInitializerUnsupported => formatter
                .write_str("[freeze:contract][callable_prelude/input_initializer_unsupported]"),
            Self::InputDeclaration(error) => write!(
                formatter,
                "[freeze:contract][callable_prelude/input_declaration] {error}"
            ),
            Self::FunctionMissing => {
                formatter.write_str("[freeze:contract][callable_prelude/function_missing]")
            }
            Self::RootNotFunction => {
                formatter.write_str("[freeze:contract][callable_prelude/root_not_function]")
            }
            Self::ParameterCountMismatch => {
                formatter.write_str("[freeze:contract][callable_prelude/parameter_count_mismatch]")
            }
            Self::ParameterBindingMissing(index) => write!(
                formatter,
                "[freeze:contract][callable_prelude/parameter_binding_missing] index={index}"
            ),
            Self::ParameterRecordMissing => {
                formatter.write_str("[freeze:contract][callable_prelude/parameter_record_missing]")
            }
            Self::ParameterOriginMismatch => {
                formatter.write_str("[freeze:contract][callable_prelude/parameter_origin_mismatch]")
            }
            Self::ParameterKindMismatch(index) => write!(
                formatter,
                "[freeze:contract][callable_prelude/parameter_kind_mismatch] index={index}"
            ),
            Self::ParameterAbiUnsupported(index) => write!(
                formatter,
                "[freeze:contract][callable_prelude/parameter_abi_unsupported] index={index}"
            ),
            Self::PreludeResultBindingMissing => {
                formatter.write_str("[freeze:contract][callable_prelude/result_binding_missing]")
            }
            Self::PreludeResultOriginMismatch => {
                formatter.write_str("[freeze:contract][callable_prelude/result_origin_mismatch]")
            }
            Self::PreludeResultKindUnsupported => {
                formatter.write_str("[freeze:contract][callable_prelude/result_kind_unsupported]")
            }
            Self::TargetHeaderMissing => {
                formatter.write_str("[freeze:contract][callable_prelude/target_header_missing]")
            }
            Self::ArgumentIdentity(error) => write!(
                formatter,
                "[freeze:contract][callable_prelude/argument_identity] {error}"
            ),
            Self::DirectCall(error) => write!(
                formatter,
                "[freeze:contract][callable_prelude/direct_call] {error}"
            ),
            Self::ResultDeclaration(error) => write!(
                formatter,
                "[freeze:contract][callable_prelude/result_declaration] {error}"
            ),
        }
    }
}

impl std::error::Error for CallablePreludeMaterializationRejectV1 {}

#[derive(Debug)]
pub(super) struct CallablePreludeMaterializationReceiptV1 {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    preheader: BasicBlockId,
    binding: crate::mir::resolved_semantics::BindingRefV1,
    result: ValueId,
    arguments: Box<[CanonicalBindingReadReceiptV1]>,
    entry: ReadyLoopEntryV1,
}

impl CallablePreludeMaterializationReceiptV1 {
    pub(super) const fn owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn preheader(&self) -> BasicBlockId {
        self.preheader
    }

    pub(super) const fn binding(&self) -> crate::mir::resolved_semantics::BindingRefV1 {
        self.binding
    }

    pub(super) const fn result(&self) -> ValueId {
        self.result
    }

    pub(super) fn arguments(&self) -> &[CanonicalBindingReadReceiptV1] {
        &self.arguments
    }

    pub(super) fn entry(&self) -> &ReadyLoopEntryV1 {
        &self.entry
    }
}

pub(super) fn materialize_callable_prelude_v1(
    builder: &mut MirBuilder,
    session: &mut CanonicalSsaFunctionSessionV2<'_>,
    input: &VerifiedCallableFunctionLoweringInputV1<'_>,
    input_relations: &VerifiedLoopInitializedLocalInputSourceSetV1,
    prelude: &VerifiedCallablePreludeCapabilityV1,
) -> Result<CallablePreludeMaterializationReceiptV1, CallablePreludeMaterializationRejectV1> {
    if input.owner() != prelude.owner() || input.owner() != input_relations.owner() {
        return Err(CallablePreludeMaterializationRejectV1::OwnerMismatch);
    }
    let preheader = builder
        .function_state
        .current_block
        .ok_or(CallablePreludeMaterializationRejectV1::FunctionMissing)?;

    materialize_parameters(builder, session, input, preheader)?;

    let mut arguments = Vec::with_capacity(prelude.arguments().rows().len());
    for row in prelude.arguments().rows() {
        session
            .identity
            .claim_variable_use_binding(row.site(), row.binding())
            .map_err(CallablePreludeMaterializationRejectV1::ArgumentIdentity)?;
        let receipt = session
            .identity
            .read_entry_receipt(builder, &mut session.phis, preheader, row.binding())
            .map_err(CallablePreludeMaterializationRejectV1::ArgumentIdentity)?;
        arguments.push(receipt);
    }

    let target_header = input
        .index()
        .header_for_callable(prelude.target())
        .map_err(|_| CallablePreludeMaterializationRejectV1::TargetHeaderMissing)?;
    let (result, _) = emit_resolved_header(
        builder,
        input.input(),
        target_header,
        prelude.result_abi(),
        arguments
            .iter()
            .map(|receipt| receipt.physical_value())
            .collect(),
    )
    .map_err(CallablePreludeMaterializationRejectV1::DirectCall)?;

    let result_record = input
        .input()
        .function()
        .binding(prelude.binding())
        .ok_or(CallablePreludeMaterializationRejectV1::PreludeResultBindingMissing)?;
    let BindingOriginV1::Source(site) = result_record.origin() else {
        return Err(CallablePreludeMaterializationRejectV1::PreludeResultOriginMismatch);
    };
    if !matches!(result_record.kind(), BindingKindV1::Local { .. }) {
        return Err(CallablePreludeMaterializationRejectV1::PreludeResultKindUnsupported);
    }
    session
        .identity
        .publish_declaration(
            site,
            result_record.kind(),
            result_record.diagnostic_name(),
            preheader,
            result,
        )
        .map_err(CallablePreludeMaterializationRejectV1::ResultDeclaration)?;

    let mut entry_rows = Vec::with_capacity(input_relations.rows().len());
    for input_relation in input_relations.rows() {
        let initializer_site =
            OwnedExprSiteV1::new(input.owner(), input_relation.initializer().clone());
        let initializer = input
            .input()
            .source()
            .expr_at(&initializer_site)
            .map_err(|error| {
                CallablePreludeMaterializationRejectV1::InputInitializerNavigation(
                    error.to_string(),
                )
            })?;
        let initial_value = match initializer.node() {
            ASTNode::Literal {
                value: LiteralValue::Integer(value),
                ..
            } => *value,
            _ => return Err(CallablePreludeMaterializationRejectV1::InputInitializerUnsupported),
        };
        let input_binding = input
            .input()
            .function()
            .declaration_binding(input_relation.declaration())
            .ok_or(CallablePreludeMaterializationRejectV1::InputBindingMissing)?;
        if input_binding != input_relation.source_binding() {
            return Err(CallablePreludeMaterializationRejectV1::InputBindingMismatch);
        }
        let input_record = input
            .input()
            .function()
            .binding(input_binding)
            .ok_or(CallablePreludeMaterializationRejectV1::InputBindingMissing)?;
        let BindingKindV1::Local { .. } = input_record.kind() else {
            return Err(CallablePreludeMaterializationRejectV1::InputBindingMismatch);
        };
        if !matches!(
            input_record.origin(),
            BindingOriginV1::Source(site) if site == input_relation.declaration()
        ) {
            return Err(CallablePreludeMaterializationRejectV1::InputBindingMismatch);
        }
        let input_value = constant::emit_integer(builder, initial_value)
            .map_err(CallablePreludeMaterializationRejectV1::InputDeclaration)?;
        session
            .identity
            .publish_declaration(
                input_relation.declaration(),
                input_record.kind(),
                input_record.diagnostic_name(),
                preheader,
                input_value,
            )
            .map_err(CallablePreludeMaterializationRejectV1::InputDeclaration)?;
        entry_rows.push(ReadyLoopEntryRowV1::new(
            input_relation.recipe_value(),
            input_binding,
            input_value,
        ));
    }
    let entry = ReadyLoopEntryV1::new_for_test(input.owner(), preheader, entry_rows);
    Ok(CallablePreludeMaterializationReceiptV1 {
        owner: input.owner(),
        preheader,
        binding: prelude.binding(),
        result,
        arguments: arguments.into_boxed_slice(),
        entry,
    })
}

fn materialize_parameters(
    builder: &mut MirBuilder,
    session: &mut CanonicalSsaFunctionSessionV2<'_>,
    input: &VerifiedCallableFunctionLoweringInputV1<'_>,
    preheader: BasicBlockId,
) -> Result<(), CallablePreludeMaterializationRejectV1> {
    let ASTNode::FunctionDeclaration {
        params,
        param_decls,
        ..
    } = input.input().source().root()
    else {
        return Err(CallablePreludeMaterializationRejectV1::RootNotFunction);
    };
    let parameter_values = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or(CallablePreludeMaterializationRejectV1::FunctionMissing)?
        .params
        .clone();
    if params.len() != parameter_values.len() || params.len() != param_decls.len() {
        return Err(CallablePreludeMaterializationRejectV1::ParameterCountMismatch);
    }
    for (index, ((name, declaration), value)) in params
        .iter()
        .zip(param_decls.iter())
        .zip(parameter_values)
        .enumerate()
    {
        if declaration.declared_type_name.as_deref() != Some("i64") {
            return Err(
                CallablePreludeMaterializationRejectV1::ParameterAbiUnsupported(index as u32),
            );
        }
        let source_site = SourceBindingSiteV1::Parameter {
            index: index as u32,
        };
        let binding = input
            .input()
            .function()
            .declaration_binding(&source_site)
            .ok_or(CallablePreludeMaterializationRejectV1::ParameterBindingMissing(index as u32))?;
        let record = input
            .input()
            .function()
            .binding(binding)
            .ok_or(CallablePreludeMaterializationRejectV1::ParameterRecordMissing)?;
        if !matches!(
            record.origin(),
            BindingOriginV1::Source(site) if site == &source_site
        ) {
            return Err(CallablePreludeMaterializationRejectV1::ParameterOriginMismatch);
        }
        if record.kind()
            != (BindingKindV1::Parameter {
                index: index as u32,
            })
        {
            return Err(
                CallablePreludeMaterializationRejectV1::ParameterKindMismatch(index as u32),
            );
        }
        builder.register_value_kind(value, MirValueKind::Parameter(index as u32));
        builder
            .function_state
            .type_ctx
            .set_type(value, MirType::Integer);
        session
            .identity
            .publish_declaration(&source_site, record.kind(), name, preheader, value)
            .map_err(CallablePreludeMaterializationRejectV1::ResultDeclaration)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
    use crate::mir::builder::resolved_lowering::MirBuilder;
    use crate::mir::canonical_direct_static_call_capability::CanonicalDirectStaticCallCapabilityV1;
    use crate::mir::compiler::callable_single_loop_recipe_coseal::issue_callable_single_loop_recipe_v1;
    use crate::mir::compiler::callable_single_loop_source_map::issue_callable_single_loop_source_map_v1;
    use crate::mir::compiler::callable_single_loop_source_shapes::SourceReceiverShapeV1;
    use crate::mir::compiler::callable_single_loop_static_fixture_tests::static_fixture_for_test;
    use crate::mir::compiler::callable_single_loop_syntax_facts::issue_callable_single_loop_syntax_facts_v1;
    use crate::mir::compiler::loop_physical_prepare::issue_callable_loop_physicalization_v1;
    use crate::mir::function::MirParamDecl;
    use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
    use crate::mir::resolved_control_flow::verify_function_completion_v1;
    use crate::mir::resolved_semantics::CanonicalCallableKeyV1;

    #[test]
    fn resolver_backed_prelude_emits_once_and_returns_ready_entry() {
        let module = static_fixture_for_test();
        let key = CanonicalCallableKeyV1::free_static_for_test("int_to_str", 1);
        let input = module.function_input(&key).expect("callable input");
        let index = module.source().catalog().index();
        let header = index.lookup(&key).expect("callable header");
        let body = input.source().root_body().expect("root body");
        let loop_stmt = input.source().body_stmt(&body, 2).expect("loop statement");
        let ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .expect("source ledger");
        let context = ledger
            .resolved_loop_source(loop_stmt.site())
            .expect("loop source");
        let syntax = issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context)
            .expect("syntax facts");
        let map = issue_callable_single_loop_source_map_v1(&ledger, syntax).expect("source map");
        let product = issue_callable_single_loop_recipe_v1(&ledger, map).expect("recipe product");
        let completion = verify_function_completion_v1(input).expect("completion");
        let prepared = issue_callable_loop_physicalization_v1(
            input,
            index,
            header,
            product,
            completion,
            SourceReceiverShapeV1::FreeStatic,
        )
        .expect("prepared callable product");
        let (input, demand, prelude, _tail, _terminal, completion) = into_canary_parts(prepared);
        let root = input.input().source().root();
        let crate::ast::ASTNode::FunctionDeclaration {
            name,
            params,
            param_decls,
            body,
            return_type_name,
            attrs,
            uses,
            ..
        } = root
        else {
            panic!("expected function root");
        };
        let function_name = format!("{name}/{}", params.len());
        let mut builder = MirBuilder::new();
        let mut outer = builder.open_resolved_function_draft_seal_session_v1(&function_name);
        let receipt = {
            let builder = outer.builder_view_mut_for_lowering();
            builder
                .function_state
                .resolved_binding_state
                .install(input.input().function())
                .expect("install resolver authority");
            builder
                .create_function_skeleton(function_name.clone(), params, body)
                .expect("function skeleton");
            builder.set_current_function_declared_signature(
                param_decls
                    .iter()
                    .map(|decl| MirParamDecl {
                        name: decl.name.clone(),
                        declared_type_name: decl.declared_type_name.clone(),
                        implicit_receiver: false,
                    })
                    .collect(),
                return_type_name.clone(),
            );
            builder.set_current_function_runes(attrs);
            builder.set_current_function_declared_capability_uses(uses);
            let function = builder
                .function_state
                .current_function
                .as_mut()
                .expect("function installed");
            CanonicalDirectStaticCallCapabilityV1::install_for_function(
                &mut function.metadata.canonical_direct_static_call_capabilities,
                true,
            )
            .expect("direct-call capability");
            let if_control =
                VerifiedResolvedFunctionIfControlV1::empty_for_loop_profile(input.input())
                    .expect("loop-only If control");
            let mut session =
                CanonicalSsaFunctionSessionV2::new(input.input(), if_control, completion, 0)
                    .expect("canonical session");
            materialize_callable_prelude_v1(
                builder,
                &mut session,
                &input,
                &demand.co_seal().input(),
                &prelude,
            )
            .expect("Prelude receipt")
        };
        assert_eq!(receipt.owner(), input.owner());
        assert_eq!(receipt.preheader(), receipt.entry().preheader());
        assert_eq!(receipt.binding(), prelude.binding());
        assert_eq!(demand.co_seal().input().rows().len(), 1);
        assert_ne!(receipt.result(), receipt.entry().rows[0].value());
        assert_eq!(
            receipt.entry().rows[0].binding(),
            demand.co_seal().input().rows()[0].source_binding()
        );
        assert_eq!(receipt.arguments().len(), 1);
        assert_eq!(receipt.arguments()[0].physical_value(), ValueId::new(0));
        assert_eq!(
            outer
                .builder_view()
                .function_state
                .current_function
                .as_ref()
                .unwrap()
                .blocks
                .values()
                .map(|block| block.instructions.len())
                .sum::<usize>(),
            2
        );
        drop(receipt);
        outer.discard_unpublished();
        assert!(builder.function_state.current_function.is_none());
    }
}
