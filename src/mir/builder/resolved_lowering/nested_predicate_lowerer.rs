//! Canonical function-draft lowerer for the bounded Nested Predicate pilot.
//!
//! This is the sibling of the DirectAccum lowerer. It owns no module
//! transaction and borrows the existing function-wide SSA/CFG/PHI services.

use crate::mir::builder::resolved_lowering::canonical_ssa::{
    finish_profile_close, CanonicalSsaFunctionSessionV2,
};
use crate::mir::builder::resolved_lowering::draft_seal::ReadyFunctionDraftSealV1;
use crate::mir::builder::resolved_lowering::nested_predicate_adapter::CanonicalNestedBindingPort;
use crate::mir::builder::resolved_lowering::nested_predicate_physicalizer::physicalize_nested_predicate_v1;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::nested_predicate_physical_input::{
    VerifiedNestedPhysicalBlockProjectionV1, VerifiedNestedPhysicalCandidateInputV1,
};
use crate::mir::compiler::nested_predicate_profile::CanonicalNestedPredicatePlanV1;
use crate::mir::function::MirParamDecl;
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_semantics::{RegionKindV1, ScopeKindV1};
use crate::mir::{BasicBlockId, MirFunction};

pub(in crate::mir::builder::resolved_lowering) struct CanonicalNestedPredicateSsaLowererV1<
    'builder,
    'source,
> {
    builder: &'builder mut MirBuilder,
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'source>,
    claims:
        crate::mir::compiler::nested_predicate_effect_plan::VerifiedNestedBindingExecutionClaimsV1,
    emission: Option<
        crate::mir::compiler::nested_predicate_topology::VerifiedNestedPhysicalEmissionInputV1,
    >,
    session: CanonicalSsaFunctionSessionV2<'source>,
}

impl<'builder, 'source> CanonicalNestedPredicateSsaLowererV1<'builder, 'source> {
    pub(in crate::mir::builder::resolved_lowering) fn new(
        builder: &'builder mut MirBuilder,
        plan: CanonicalNestedPredicatePlanV1<'source>,
    ) -> Result<Self, String> {
        let (input, _loop_stmt, claims, emission, completion) = plan.into_parts();
        if input.owner() != claims.prefix().owner()
            || input.owner() != claims.effect_plan().owner()
            || claims.prefix().frame_key() != emission.topology().root_frame_key()
        {
            return Err("[freeze:contract][nested_lowerer/owner_or_frame]".into());
        }
        if !builder
            .function_state
            .resolved_binding_state
            .is_installed_for(input.owner())
        {
            return Err("[freeze:contract][canonical_binding_ssa/authority_not_installed]".into());
        }
        let if_control = VerifiedResolvedFunctionIfControlV1::empty_for_loop_profile(input)?;
        let session = CanonicalSsaFunctionSessionV2::new(input, if_control, completion, 0)?;
        Ok(Self {
            builder,
            input,
            claims,
            emission: Some(emission),
            session,
        })
    }

    pub(in crate::mir::builder::resolved_lowering) fn lower(
        mut self,
    ) -> Result<ReadyFunctionDraftSealV1, String> {
        let preheader = self.current_block()?;
        let root_fresh = self.fresh_blocks();
        let child_fresh = self.fresh_blocks();
        let parent_resume = self.builder.next_block_id();
        let emission = self
            .emission
            .take()
            .ok_or_else(|| "[freeze:contract][nested_lowerer/emission_reconsumed]".to_string())?;
        let blocks = VerifiedNestedPhysicalBlockProjectionV1::try_new(
            &emission,
            self.input.owner(),
            emission.topology().root_frame_key(),
            preheader,
            root_fresh,
            child_fresh,
            parent_resume,
        )
        .map_err(|error| format!("[freeze:contract][nested_blocks] {error:?}"))?;

        let root_scope = self.session.semantics.enter_scope_region(
            self.input.function(),
            self.claims.prefix().root_loop_pair(),
            ScopeKindV1::LoopBody,
            RegionKindV1::Loop,
        )?;
        let child_scope = self.session.semantics.enter_scope_region(
            self.input.function(),
            self.claims.prefix().child_loop_pair(),
            ScopeKindV1::LoopBody,
            RegionKindV1::Loop,
        )?;

        let candidate = VerifiedNestedPhysicalCandidateInputV1::new(emission, blocks);
        let mut port = CanonicalNestedBindingPort::new(
            &mut self.session.identity,
            &self.claims,
            self.input.owner(),
            self.claims.effect_plan().frame_key(),
        )?;
        port.publish_prefix(self.builder, preheader, &self.claims)?;
        port.activate_child(&self.claims)?;
        let continuation = physicalize_nested_predicate_v1(
            self.builder,
            candidate,
            &mut self.session.cfg,
            &mut port,
            &mut self.session.phis,
        )?;
        let profile_close =
            finish_profile_close(self.input.owner(), continuation.continuation_block, || {
                port.finish_effect_claims()
            })?;
        drop(port);
        self.finish_after(continuation.continuation_block)?;
        self.session
            .semantics
            .close_scope_region_success(child_scope, &mut self.session.identity)?;
        self.session
            .semantics
            .close_scope_region_success(root_scope, &mut self.session.identity)?;

        self.session
            .finish_for_draft_seal(self.builder, profile_close)
            .map_err(|error| error.to_string())
    }

    fn fresh_blocks(&mut self) -> [BasicBlockId; 4] {
        [
            self.builder.next_block_id(),
            self.builder.next_block_id(),
            self.builder.next_block_id(),
            self.builder.next_block_id(),
        ]
    }

    fn finish_after(&mut self, after: BasicBlockId) -> Result<(), String> {
        if self.builder.function_state.current_block != Some(after) {
            return Err("[freeze:contract][nested_lowerer/after_not_current]".into());
        }
        let function = self
            .builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][nested_lowerer/function_missing]".to_string())?;
        let witness = self
            .session
            .cfg
            .seal_block(function, after)
            .map_err(|error| format!("[freeze:contract][nested_cfg/after_seal] {error:?}"))?;
        self.session
            .identity
            .seal_block(self.builder, &mut self.session.phis, after, &witness)
    }

    fn current_block(&self) -> Result<BasicBlockId, String> {
        self.builder
            .function_state
            .current_block
            .ok_or_else(|| "[freeze:contract][nested_lowerer/current_block_missing]".into())
    }
}

pub(in crate::mir) fn lower_nested_predicate_function_draft(
    builder: &mut MirBuilder,
    plan: CanonicalNestedPredicatePlanV1<'_>,
) -> Result<MirFunction, super::CanonicalResolvedBuildErrorV1> {
    let input = plan.input();
    let crate::ast::ASTNode::FunctionDeclaration {
        name,
        params,
        body,
        return_type_name,
        attrs,
        uses,
        ..
    } = input.source().root()
    else {
        return Err(super::CanonicalResolvedBuildErrorV1::BuilderContract(
            "[freeze:contract][nested_predicate/root_not_function]".into(),
        ));
    };
    let function_name = format!("{name}/{}", params.len());
    let mut session = builder.open_resolved_function_draft_seal_session_v1(&function_name);
    let lowering = {
        let draft_builder = session.builder_view_mut_for_lowering();
        (|| {
            draft_builder
                .function_state
                .resolved_binding_state
                .install(input.function())?;
            draft_builder.create_function_skeleton(function_name, params, body)?;
            draft_builder.set_current_function_declared_signature(
                params
                    .iter()
                    .map(|name| MirParamDecl {
                        name: name.clone(),
                        declared_type_name: None,
                        implicit_receiver: false,
                    })
                    .collect(),
                return_type_name.clone(),
            );
            draft_builder.set_current_function_runes(attrs);
            draft_builder.set_current_function_declared_capability_uses(uses);
            let ready = CanonicalNestedPredicateSsaLowererV1::new(draft_builder, plan)?.lower()?;
            Ok::<_, String>(ready)
        })()
    };
    let ready = match lowering {
        Ok(ready) => ready,
        Err(error) => {
            session.discard_unpublished();
            return Err(error.into());
        }
    };
    let open = ready.open(session);
    let prepared = open.prepare().map_err(|rejected| {
        let stage = rejected.stage();
        let error = format!("{:?}", rejected.error());
        rejected.discard();
        super::CanonicalResolvedBuildErrorV1::BuilderContract(format!(
            "[freeze:contract][nested_predicate/draft_seal/{stage:?}] {error}"
        ))
    })?;
    Ok(prepared.commit().into_draft())
}

impl<'builder, 'source> std::fmt::Debug
    for CanonicalNestedPredicateSsaLowererV1<'builder, 'source>
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CanonicalNestedPredicateSsaLowererV1")
            .field("input_owner", &self.input.owner())
            .finish()
    }
}
