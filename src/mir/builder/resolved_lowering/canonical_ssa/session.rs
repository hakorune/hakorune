//! One function-scoped owner bundle shared by canonical lowering profiles.

use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::builder::resolved_lowering::draft_seal::ReadyFunctionDraftSealV1;
use crate::mir::builder::MirBuilder;
use crate::mir::checked_callout::{CheckedCallOutNormalResultProjectionV1, CheckedCallOutSiteIdV1};
use crate::mir::compiler::common_v2_physical_function_skeleton::PhysicalFunctionEntryCohortStampV1;
use crate::mir::compiler::dynamic_full_body_recipe::DynamicCanonicalSessionAuthorityRefV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::located::SourceBodySiteV1;
use crate::mir::resolved_control_flow::if_control::{
    FunctionIfControlUseErrorV1, FunctionIfControlUseLedgerV1, ResolvedIfControlMaterializationV1,
    VerifiedResolvedFunctionIfControlV1,
};
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, RegionId};
use crate::mir::{BasicBlockId, MirType, ValueId};

use super::super::common_v2_segment_block_allocation::SegmentBlockAllocationBrandV1;
use super::super::completion_consumption::ResolvedFunctionCompletionConsumptionV1;
use super::super::physical_entry_lane_adoption::PhysicalTextEntryLaneSidecarV1;
use super::super::semantic_stack::{ResolvedSemanticExpectedCountsV1, ResolvedSemanticStackV1};
use super::identity::ResolvedSsaIdentityStateV2;

#[path = "session/draft_seal_close.rs"]
mod draft_seal_close;
#[path = "session/generic_g0_entry_adoption.rs"]
mod generic_g0_entry_adoption;
#[path = "session/physical_entry_lane_adoption.rs"]
mod physical_entry_lane_adoption;
#[path = "session/physical_entry_stamp.rs"]
mod physical_entry_stamp;
#[path = "session/pinned_text_plan.rs"]
mod pinned_text_plan;
#[path = "session/residence_lifecycle.rs"]
mod residence_lifecycle;
#[path = "session/s6c_textref_plan.rs"]
mod s6c_textref_plan;
#[path = "session/segment_scope.rs"]
mod segment_scope;

enum CanonicalIfControlConsumptionV1 {
    Resolved(FunctionIfControlUseLedgerV1),
    // The Dynamic profile's complete operation/control ledger is consumed by
    // the selected physical session before the common terminal is opened.
    // Until that terminal exists, this is intentionally a unit disposition:
    // it must not retain an owner token that finish() cannot validate.
    DynamicProfileOwned,
}

impl CanonicalIfControlConsumptionV1 {
    fn claim(
        &mut self,
        statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    ) -> Result<ResolvedIfControlMaterializationV1, FunctionIfControlUseErrorV1> {
        match self {
            Self::Resolved(ledger) => ledger.claim(statement),
            Self::DynamicProfileOwned => Err(FunctionIfControlUseErrorV1::Unexpected),
        }
    }

    fn finish(self) -> Result<(), FunctionIfControlUseErrorV1> {
        match self {
            Self::Resolved(ledger) => ledger.finish(),
            Self::DynamicProfileOwned => Ok(()),
        }
    }
}

/// The only mutable SSA/CFG/PHI owner for one canonical function session.
///
/// A profile may add source-specific admission receipts, but it must borrow
/// this bundle rather than create another reaching-value or PHI authority.
pub(in crate::mir::builder::resolved_lowering) struct CanonicalSsaFunctionSessionV2<'source> {
    owner: FunctionOwnerIdV1,
    root_body: SourceBodySiteV1,
    root_body_end: u32,
    target_function: RegionId,
    pub(in crate::mir::builder::resolved_lowering) identity: ResolvedSsaIdentityStateV2<'source>,
    pub(in crate::mir::builder::resolved_lowering) semantics: ResolvedSemanticStackV1,
    pub(in crate::mir::builder::resolved_lowering) if_control: CanonicalIfControlConsumptionV1,
    pub(in crate::mir::builder::resolved_lowering) completion:
        ResolvedFunctionCompletionConsumptionV1,
    pub(in crate::mir::builder::resolved_lowering) cfg: CanonicalCfgSessionV1,
    pub(in crate::mir::builder::resolved_lowering) phis: PhiTxn,
    pub(in crate::mir::builder::resolved_lowering) implicit_completion: bool,
    physical_entry_sidecar: Option<PhysicalTextEntryLaneSidecarV1>,
    physical_entry_stamp: Option<PhysicalFunctionEntryCohortStampV1>,
    generic_entry_adopted: bool,
    segment_block_brand: SegmentBlockAllocationBrandV1,
    segment_blocks_issued: bool,
    pinned_text_residence: Option<residence_lifecycle::PinnedTextResidenceLifecycleStateV1>,
}

/// One-shot evidence that a profile-specific ledger has closed before the
/// common function terminal consumes the session.
#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) struct ReadyCanonicalProfileCloseV1 {
    owner: FunctionOwnerIdV1,
    terminal_block: BasicBlockId,
}

impl ReadyCanonicalProfileCloseV1 {
    fn from_closed_profile(owner: FunctionOwnerIdV1, terminal_block: BasicBlockId) -> Self {
        Self {
            owner,
            terminal_block,
        }
    }

    fn parts(self) -> (FunctionOwnerIdV1, BasicBlockId) {
        (self.owner, self.terminal_block)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) enum CanonicalFunctionFinishErrorV1 {
    ProfileOwnerMismatch,
    TerminalBlockMismatch,
    FunctionMissing,
    Cfg(String),
    Semantic(String),
    IfControl(String),
    Identity(String),
    Phi(String),
    CheckedCallOut(String),
    Binding(String),
    Completion(String),
    ReturnOperandMissing,
}

impl std::fmt::Display for CanonicalFunctionFinishErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProfileOwnerMismatch => {
                formatter.write_str("[freeze:contract][canonical_finish/profile_owner_mismatch]")
            }
            Self::TerminalBlockMismatch => {
                formatter.write_str("[freeze:contract][canonical_finish/terminal_block_mismatch]")
            }
            Self::FunctionMissing => {
                formatter.write_str("[freeze:contract][canonical_finish/function_missing]")
            }
            Self::Cfg(error) => {
                write!(formatter, "[freeze:contract][canonical_finish/cfg] {error}")
            }
            Self::Semantic(error) => write!(
                formatter,
                "[freeze:contract][canonical_finish/semantic] {error}"
            ),
            Self::IfControl(error) => write!(
                formatter,
                "[freeze:contract][canonical_finish/if_control] {error}"
            ),
            Self::Identity(error) => write!(
                formatter,
                "[freeze:contract][canonical_finish/identity] {error}"
            ),
            Self::Phi(error) => {
                write!(formatter, "[freeze:contract][canonical_finish/phi] {error}")
            }
            Self::CheckedCallOut(error) => write!(
                formatter,
                "[freeze:contract][canonical_finish/checked_callout] {error}"
            ),
            Self::Binding(error) => write!(
                formatter,
                "[freeze:contract][canonical_finish/binding] {error}"
            ),
            Self::Completion(error) => write!(
                formatter,
                "[freeze:contract][canonical_finish/completion] {error}"
            ),
            Self::ReturnOperandMissing => {
                formatter.write_str("[freeze:contract][canonical_finish/return_operand_missing]")
            }
        }
    }
}

pub(in crate::mir::builder::resolved_lowering) fn finish_profile_close(
    owner: FunctionOwnerIdV1,
    terminal_block: BasicBlockId,
    close: impl FnOnce() -> Result<(), String>,
) -> Result<ReadyCanonicalProfileCloseV1, String> {
    close()?;
    Ok(ReadyCanonicalProfileCloseV1::from_closed_profile(
        owner,
        terminal_block,
    ))
}

impl<'source> CanonicalSsaFunctionSessionV2<'source> {
    pub(in crate::mir::builder::resolved_lowering) fn new(
        input: ResolvedFunctionLoweringInputV1<'source>,
        if_control: VerifiedResolvedFunctionIfControlV1,
        completion: VerifiedFunctionCompletionV1,
        block_expr_count: usize,
    ) -> Result<Self, String> {
        let implicit_completion = completion.is_implicit_void();
        let completion = ResolvedFunctionCompletionConsumptionV1::new(input.owner(), completion)?;
        Self::from_consumption(
            input,
            if_control,
            completion,
            block_expr_count,
            implicit_completion,
        )
    }

    /// Generic-only session opener.  It consumes the resolver-issued outer-If
    /// and borrows Completion while projecting the typed BlockExpr count; no
    /// S6C envelope or common-V2 admission is involved.
    pub(in crate::mir::builder::resolved_lowering) fn new_generic(
        input: ResolvedFunctionLoweringInputV1<'source>,
        if_control: VerifiedResolvedFunctionIfControlV1,
        expectation: &crate::mir::resolved_semantics::VerifiedResolvedBlockExpressionExpectationV1,
        completion: &VerifiedFunctionCompletionV1,
    ) -> Result<Self, String> {
        if expectation.owner() != input.owner()
            || expectation.function_origin() != input.function().function_origin()
            || expectation.body_root() != input.function().root_profile().body_root()
            || if_control.owner() != input.owner()
            || completion.owner() != input.owner()
            || completion.target_function() != input.function().function_region()
        {
            return Err("[freeze:contract][generic_session/source_cohort_mismatch]".to_owned());
        }
        let block_expr_count = usize::try_from(expectation.pair_count())
            .map_err(|_| "[freeze:contract][generic_session/block_expr_overflow]".to_owned())?;
        let implicit_completion = completion.is_implicit_void();
        let completion =
            ResolvedFunctionCompletionConsumptionV1::new_borrowed(input.owner(), completion)?;
        Self::from_consumption(
            input,
            if_control,
            completion,
            block_expr_count,
            implicit_completion,
        )
    }

    /// Common V2 session opener.  It consumes the one-shot admission parts,
    /// projects the typed BlockExpr expectation here, and snapshots the
    /// installed parent's Completion exactly once into the session-local
    /// physical consumer.  It does not emit CFG, SSA, PHI, or DraftSeal work.
    pub(in crate::mir) fn new_common_v2(
        parts: crate::mir::compiler::common_v2_session_admission::LoopV2CanonicalSessionPartsV1<
            'source,
            '_,
            '_,
        >,
    ) -> Result<Self, String> {
        let (input, if_control, expectation, _envelope, completion) = parts.into_parts();
        if expectation.owner() != input.owner()
            || expectation.function_origin() != input.function().function_origin()
            || expectation.body_root() != input.function().root_profile().body_root()
        {
            return Err(
                "[freeze:contract][common_v2_session/typed_expectation_mismatch]".to_owned(),
            );
        }
        let block_expr_count = usize::try_from(expectation.pair_count()).map_err(|_| {
            "[freeze:contract][common_v2_session/block_expr_count_overflow]".to_owned()
        })?;
        let implicit_completion = completion.is_implicit_void();
        let completion =
            ResolvedFunctionCompletionConsumptionV1::new_borrowed(input.owner(), completion)?;
        Self::from_consumption(
            input,
            if_control,
            completion,
            block_expr_count,
            implicit_completion,
        )
    }

    fn from_consumption(
        input: ResolvedFunctionLoweringInputV1<'source>,
        if_control: VerifiedResolvedFunctionIfControlV1,
        completion: ResolvedFunctionCompletionConsumptionV1,
        block_expr_count: usize,
        implicit_completion: bool,
    ) -> Result<Self, String> {
        let if_controls = if_control.row_count();
        let if_branches = if_controls + if_control.explicit_else_count();
        let semantics = ResolvedSemanticStackV1::new_with_expectations(
            input.function(),
            input.function().lowering_roots(),
            ResolvedSemanticExpectedCountsV1::new(block_expr_count, if_controls, if_branches),
        )?;
        let root_body = input
            .source()
            .root_body()
            .map_err(|error| error.to_string())?;
        let root_body_end = u32::try_from(root_body.statements().len()).map_err(|_| {
            "[freeze:contract][canonical_completion/body_length_overflow]".to_string()
        })?;
        let target_function = input.function().function_region();
        Ok(Self {
            owner: input.owner(),
            root_body: root_body.site().clone(),
            root_body_end,
            target_function,
            identity: ResolvedSsaIdentityStateV2::new(input.function()),
            semantics,
            if_control: CanonicalIfControlConsumptionV1::Resolved(if_control.into_use_ledger()),
            completion,
            cfg: CanonicalCfgSessionV1::new(),
            phis: PhiTxn::begin("canonical_binding_ssa"),
            implicit_completion,
            physical_entry_sidecar: None,
            physical_entry_stamp: None,
            generic_entry_adopted: false,
            segment_block_brand: SegmentBlockAllocationBrandV1::new(),
            segment_blocks_issued: false,
            pinned_text_residence: None,
        })
    }

    /// Dynamic-only admission from the final semantic program.  The
    /// authority view is borrowed only while this constructor snapshots the
    /// Completion expectations and validates the sealed Loop control; the
    /// returned session owns no semantic borrow.
    pub(in crate::mir::builder::resolved_lowering) fn new_selected_dynamic(
        input: ResolvedFunctionLoweringInputV1<'source>,
        authority: DynamicCanonicalSessionAuthorityRefV1<'_>,
    ) -> Result<Self, String> {
        if authority.owner() != input.owner()
            || authority.target_function() != input.function().function_region()
        {
            return Err("[freeze:contract][dynamic_session/identity_mismatch]".to_string());
        }
        authority.validate_loop_control()?;
        let if_control_regions = 0;
        let if_branch_pairs = 0;
        let semantics = ResolvedSemanticStackV1::new_with_expectations(
            input.function(),
            input.function().lowering_roots(),
            ResolvedSemanticExpectedCountsV1::new(0, if_control_regions, if_branch_pairs),
        )?;
        let root_body = input
            .source()
            .root_body()
            .map_err(|error| error.to_string())?;
        let root_body_end = u32::try_from(root_body.statements().len()).map_err(|_| {
            "[freeze:contract][canonical_completion/body_length_overflow]".to_string()
        })?;
        let implicit_completion = authority.completion().is_implicit_void();
        let target_function = input.function().function_region();
        let completion = ResolvedFunctionCompletionConsumptionV1::new_borrowed(
            input.owner(),
            authority.completion(),
        )?;
        Ok(Self {
            owner: input.owner(),
            root_body: root_body.site().clone(),
            root_body_end,
            target_function,
            identity: ResolvedSsaIdentityStateV2::new(input.function()),
            semantics,
            if_control: CanonicalIfControlConsumptionV1::DynamicProfileOwned,
            completion,
            cfg: CanonicalCfgSessionV1::new(),
            phis: PhiTxn::begin("canonical_binding_ssa"),
            implicit_completion,
            physical_entry_sidecar: None,
            physical_entry_stamp: None,
            generic_entry_adopted: false,
            segment_block_brand: SegmentBlockAllocationBrandV1::new(),
            segment_blocks_issued: false,
            pinned_text_residence: None,
        })
    }

    pub(in crate::mir::builder::resolved_lowering) fn claim_if_control(
        &mut self,
        statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    ) -> Result<ResolvedIfControlMaterializationV1, FunctionIfControlUseErrorV1> {
        self.if_control.claim(statement)
    }

    pub(in crate::mir::builder::resolved_lowering) const fn completion_is_implicit(&self) -> bool {
        self.implicit_completion
    }

    pub(in crate::mir::builder::resolved_lowering) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    /// Allocate one unpublished physical block through the canonical CFG
    /// owner.  Profile emitters may borrow the returned id only while their
    /// opaque target is alive; they must not call `ensure_block_exists` or
    /// mutate the function CFG directly.
    pub(in crate::mir::builder::resolved_lowering) fn create_unpublished_block(
        &mut self,
        builder: &mut MirBuilder,
    ) -> Result<BasicBlockId, String> {
        if builder.function_state.current_function.is_none() {
            return Err("canonical physical target requires current function".to_owned());
        }
        let block = builder.next_block_id();
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .expect("current function checked above");
        self.cfg
            .create_block(function, block)
            .map_err(|error| error.to_string())?;
        Ok(block)
    }

    pub(in crate::mir::builder::resolved_lowering) fn entry_block(
        &self,
        builder: &MirBuilder,
    ) -> Result<BasicBlockId, String> {
        builder
            .function_state
            .current_function
            .as_ref()
            .map(|function| function.entry_block)
            .ok_or_else(|| "canonical physical target requires current function".to_owned())
    }

    /// Issue one physical SSA value id for a selected unpublished operation.
    /// This is the only selected-lane value-id issuer; operation leaves only
    /// receive the id and publish their typed receipt through the session ledger.
    pub(in crate::mir::builder::resolved_lowering) fn issue_physical_value_id(
        &mut self,
        builder: &mut MirBuilder,
    ) -> Result<ValueId, String> {
        builder
            .function_state
            .current_function
            .as_mut()
            .map(|function| function.next_value_id())
            .ok_or_else(|| "canonical physical value requires current function".to_owned())
    }

    /// Commit one physical value type through the canonical SSA owner.  The
    /// selected emitter may project an already-verified operation shape, but
    /// it may not write a competing type fact directly into the Builder.
    pub(in crate::mir::builder::resolved_lowering) fn publish_physical_value_type(
        &mut self,
        builder: &mut MirBuilder,
        value: ValueId,
        expected: MirType,
    ) -> Result<(), String> {
        match builder.function_state.type_ctx.get_type(value) {
            None | Some(MirType::Unknown) => {
                builder
                    .function_state
                    .type_ctx
                    .value_types
                    .insert(value, expected);
                Ok(())
            }
            Some(actual) if *actual == expected => Ok(()),
            Some(actual) => Err(format!(
                "[freeze:contract][canonical_physical_value/type_drift] value={value:?} expected={expected:?} actual={actual:?}"
            )),
        }
    }

    /// Define the checked-call result only in its Normal landing block.  The
    /// terminator has no destination, so the existing block-local definition
    /// and dominance machinery remains the sole SSA authority.
    pub(in crate::mir::builder::resolved_lowering) fn define_checked_callout_normal_result(
        &mut self,
        builder: &mut MirBuilder,
        source: BasicBlockId,
        normal_landing: BasicBlockId,
        site_id: CheckedCallOutSiteIdV1,
        result_type: MirType,
    ) -> Result<CheckedCallOutNormalResultProjectionV1, String> {
        let dst = {
            let function = builder
                .function_state
                .current_function
                .as_mut()
                .ok_or_else(|| "checked callout result requires current function".to_owned())?;
            if function.metadata.checked_callout_plan(site_id).is_none() {
                return Err(
                    "checked callout Normal projection has no admitted site plan".to_owned(),
                );
            }
            let source_term = function
                .get_block(source)
                .and_then(|block| block.terminator.as_ref())
                .ok_or_else(|| "checked callout source has no terminator".to_owned())?;
            match source_term {
                crate::mir::MirInstruction::CheckedCallOut {
                    site_id: actual_site,
                    normal_landing: actual_normal,
                    ..
                } if *actual_site == site_id && *actual_normal == normal_landing => {}
                _ => return Err("checked callout source/Normal site mismatch".to_owned()),
            }
            let landing = function
                .get_block(normal_landing)
                .ok_or_else(|| "checked callout Normal landing is missing".to_owned())?;
            if landing.is_sealed() {
                return Err("checked callout Normal landing is already sealed".to_owned());
            }
            if landing.predecessors.len() != 1 || !landing.predecessors.contains(&source) {
                return Err(
                    "checked callout Normal landing must have exactly one predecessor".to_owned(),
                );
            }
            if landing.instructions.iter().any(|inst| {
                matches!(
                    inst,
                    crate::mir::MirInstruction::CheckedCallOutNormalResult {
                        site_id: existing,
                        ..
                    } if *existing == site_id
                )
            }) {
                return Err("checked callout Normal projection was already issued".to_owned());
            }
            function.next_value_id()
        };
        // Type publication belongs to this canonical issuer, immediately
        // beside the SSA definition.  Selected lanes provide only an already
        // projected physical type; they do not mutate type_ctx themselves.
        self.publish_physical_value_type(builder, dst, result_type)?;
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| "checked callout result requires current function".to_owned())?;
        let projection = crate::mir::MirInstruction::CheckedCallOutNormalResult { site_id, dst };
        function
            .get_block_mut(normal_landing)
            .expect("Normal landing was checked")
            .insert_instruction_after_phis(projection);
        Ok(CheckedCallOutNormalResultProjectionV1::new(
            site_id,
            normal_landing,
            dst,
        ))
    }

    /// Sole canonical SSA/session writer for the physical End instruction.
    /// End has no SSA result, but it must still be emitted through the
    /// canonical session so a caller cannot pair a foreign site or lease slot
    /// with an otherwise valid block.
    pub(in crate::mir::builder::resolved_lowering) fn emit_checked_callout_end(
        &mut self,
        builder: &mut MirBuilder,
        block: BasicBlockId,
        site_id: CheckedCallOutSiteIdV1,
        lease_slot: crate::mir::checked_callout::CheckedCallOutLeaseSlotIdV1,
    ) -> Result<(), String> {
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| "checked callout End requires current function".to_owned())?;
        let plan = function
            .metadata
            .checked_callout_plan(site_id)
            .ok_or_else(|| "checked callout End has no admitted site plan".to_owned())?;
        match plan.normal_shape() {
            crate::mir::checked_callout::CheckedCallOutNormalShapeV1::EndAuthorizedHandle {
                lease_slot: expected,
            } if expected == lease_slot => {}
            _ => return Err("checked callout End lease shape mismatch".to_owned()),
        }
        if builder.function_state.current_block != Some(block) {
            return Err("checked callout End requires selected current block".to_owned());
        }
        let target = function
            .get_block_mut(block)
            .ok_or_else(|| "checked callout End block is missing".to_owned())?;
        if target.is_sealed() || target.is_terminated() {
            return Err("checked callout End block is sealed or terminated".to_owned());
        }
        target.add_instruction(crate::mir::MirInstruction::CheckedCallOutEnd {
            site_id,
            lease_slot,
        });
        Ok(())
    }

    /// Adopt one resolver-issued formal lane into the canonical identity/SSA
    /// owner. The function skeleton has already reserved the parameter
    /// ValueIds; this method only validates and publishes those exact values.
    pub(in crate::mir::builder::resolved_lowering) fn adopt_exact_formal_parameter(
        &mut self,
        builder: &mut MirBuilder,
        site: &crate::mir::resolved_semantics::SourceBindingSiteV1,
        binding: crate::mir::resolved_semantics::BindingRefV1,
        ordinal: u32,
    ) -> Result<ValueId, String> {
        let index = usize::try_from(ordinal)
            .map_err(|_| "[freeze:contract][formal_parameter/ordinal_overflow]".to_owned())?;
        let (entry, value, ty) = {
            let function = builder
                .function_state
                .current_function
                .as_ref()
                .ok_or_else(|| "[freeze:contract][formal_parameter/function_missing]".to_owned())?;
            if function.params.len() != function.signature.params.len()
                || index >= function.params.len()
                || builder.function_state.current_block != Some(function.entry_block)
            {
                return Err("[freeze:contract][formal_parameter/reserved_entry_drift]".to_owned());
            }
            let value = function.params[index];
            if value != ValueId::new(ordinal) {
                return Err(format!(
                    "[freeze:contract][formal_parameter/value_drift] ordinal={ordinal} value={value:?}"
                ));
            }
            (
                function.entry_block,
                value,
                function.signature.params[index].clone(),
            )
        };
        self.identity
            .publish_declaration_exact(site, binding, entry, value)?;
        builder.register_value_kind(value, hakorune_mir_core::MirValueKind::Parameter(ordinal));
        if ty != MirType::Unknown {
            builder
                .function_state
                .type_ctx
                .value_types
                .insert(value, ty);
        }
        Ok(value)
    }

    pub(in crate::mir::builder) fn adopt_physical_entry_lanes(
        &mut self,
        builder: &mut MirBuilder,
        descriptors: &[crate::mir::compiler::common_v2_physical_function_entry_input::
            PhysicalCallableParameterDescriptorV1],
    ) -> Result<(), String> {
        physical_entry_lane_adoption::adopt(self, builder, descriptors)
    }

    pub(in crate::mir::builder::resolved_lowering) fn with_exact_text_sidecar_row<R>(
        &self,
        binding: crate::mir::resolved_semantics::BindingRefV1,
        logical_ordinal: u32,
        callback: impl FnOnce(
            &crate::mir::builder::resolved_lowering::physical_entry_lane_adoption::
                PhysicalTextEntryLaneSidecarRowV1,
        ) -> R,
    ) -> Result<R, String> {
        physical_entry_lane_adoption::with_exact_text_sidecar_row(
            self,
            binding,
            logical_ordinal,
            callback,
        )
    }

    pub(in crate::mir::builder::resolved_lowering) fn physical_entry_sidecar_entry(
        &self,
    ) -> Result<BasicBlockId, String> {
        self.physical_entry_sidecar
            .as_ref()
            .map(|sidecar| sidecar.entry())
            .ok_or_else(|| "physical entry ExactText sidecar is missing".to_owned())
    }

    pub(in crate::mir::builder) fn adopt_generic_g0_entry_lanes(
        &mut self,
        builder: &mut MirBuilder,
        descriptors: &[crate::mir::compiler::generic_g0_physical_function_entry_input::
            GenericG0PhysicalParameterDescriptorV1],
    ) -> Result<(), String> {
        generic_g0_entry_adoption::adopt(self, builder, descriptors)
    }

    #[cfg(test)]
    pub(in crate::mir::builder::resolved_lowering) fn physical_entry_sidecar_row_count(
        &self,
    ) -> usize {
        self.physical_entry_sidecar
            .as_ref()
            .map_or(0, |sidecar| sidecar.rows().len())
    }
}

#[cfg(test)]
mod tests {
    use super::finish_profile_close;
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;
    use crate::mir::BasicBlockId;

    #[test]
    fn profile_close_receipt_is_minted_only_after_profile_success() {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
        let owner = issuer.issue().expect("owner");
        let receipt = finish_profile_close(owner, BasicBlockId::new(7), || Ok(()))
            .expect("profile close receipt");
        assert_eq!(receipt.parts(), (owner, BasicBlockId::new(7)));
    }

    #[test]
    fn profile_close_failure_does_not_mint_a_receipt() {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
        let owner = issuer.issue().expect("owner");
        let error = finish_profile_close(owner, BasicBlockId::new(7), || {
            Err("profile ledger remained open".to_string())
        })
        .expect_err("profile close must fail");
        assert_eq!(error, "profile ledger remained open");
    }
}
