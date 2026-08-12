//! Borrowed A-prime source/Recipe relation for the selected exact-i64 cohort.
//!
//! This view closes an already verified relation once.  It does not observe
//! the AST, issue a Recipe, or carry physical values.  The final semantic
//! program remains the owner of the retained source and claims.

use crate::mir::loop_recipe_contract::{
    LoopBindingKeyV1, LoopCarrierKeyV1, LoopValueClassV2, LoopValueKeyV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, ResolvedScopeRegionPairV1,
    SourceBindingSiteV1, SourceExprSiteV1, SourceStmtSiteV1,
};

use super::super::claims::DynamicFullLoopClaimTargetV2;
use super::super::{DynamicFullLoopParameterClassV2, DynamicFullLoopRetainedSourceV1};
use super::coverage::VerifiedDynamicFullLoopClaimCoverageV2;
use super::VerifiedDynamicFullLoopSourceRecipeEnvelopeV2;
use crate::mir::compiler::dynamic_full_body_source::{
    DynamicFullBodyBindingRoleV1, DynamicFullBodyBindingRowV1, DynamicFullBodySourceRoleV1,
    DynamicFullBodySourceSiteV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicAPrimeI64SourceRelationRejectV1 {
    MissingBinding,
    MissingSource,
    ParameterContract,
    ClaimMismatch,
    RecipeMismatch,
    CompletionMismatch,
}

/// Exact source/Recipe facts needed by the Builder-free A-prime demand.
#[derive(Debug, Clone, Copy)]
pub(in crate::mir) struct DynamicAPrimeI64SourceRelationViewV1<'program> {
    owner: FunctionOwnerIdV1,
    frame: &'program LoopExecutionFrameKeyV1,
    scope_region: ResolvedScopeRegionPairV1,
    src_binding: BindingRefV1,
    pos_binding: BindingRefV1,
    end_binding: BindingRefV1,
    pred_chars_binding: BindingRefV1,
    src_declaration: &'program SourceBindingSiteV1,
    pos_declaration: &'program SourceBindingSiteV1,
    end_declaration: &'program SourceBindingSiteV1,
    pred_chars_declaration: &'program SourceBindingSiteV1,
    induction_binding: BindingRefV1,
    induction_declaration: &'program SourceBindingSiteV1,
    initializer: &'program SourceExprSiteV1,
    loop_site: &'program SourceStmtSiteV1,
    condition_i: &'program SourceExprSiteV1,
    step_read_i: &'program SourceExprSiteV1,
    step_target_i: &'program SourceExprSiteV1,
    inner_return_i: &'program SourceExprSiteV1,
    outer_return_i: &'program SourceExprSiteV1,
    completion_sites: [&'program SourceStmtSiteV1; 2],
    src_class: DynamicFullLoopParameterClassV2,
    pos_class: DynamicFullLoopParameterClassV2,
    end_class: DynamicFullLoopParameterClassV2,
    pred_chars_class: DynamicFullLoopParameterClassV2,
    induction_key: LoopBindingKeyV1,
    carrier_key: LoopCarrierKeyV1,
    entry_value: LoopValueKeyV1,
    src_value: LoopValueKeyV1,
    pos_value: LoopValueKeyV1,
    end_value: LoopValueKeyV1,
    pred_chars_value: LoopValueKeyV1,
    inner_return_value: LoopValueKeyV1,
    outer_tail_binding: LoopBindingKeyV1,
}

/// One borrowed formal lane from the already verified source/Recipe relation.
/// This is a physical-session input view, not a second semantic contract.
#[derive(Debug, Clone, Copy)]
pub(in crate::mir) struct DynamicAPrimeFormalRelationRowV1<'program> {
    ordinal: u32,
    declaration: &'program SourceBindingSiteV1,
    binding: BindingRefV1,
    recipe_value: LoopValueKeyV1,
    class: DynamicFullLoopParameterClassV2,
}

impl<'program> DynamicAPrimeFormalRelationRowV1<'program> {
    pub(in crate::mir) const fn ordinal(self) -> u32 {
        self.ordinal
    }

    pub(in crate::mir) const fn declaration(self) -> &'program SourceBindingSiteV1 {
        self.declaration
    }

    pub(in crate::mir) const fn binding(self) -> BindingRefV1 {
        self.binding
    }

    pub(in crate::mir) const fn recipe_value(self) -> LoopValueKeyV1 {
        self.recipe_value
    }

    pub(in crate::mir) const fn class(self) -> DynamicFullLoopParameterClassV2 {
        self.class
    }
}

impl DynamicAPrimeI64SourceRelationViewV1<'_> {
    pub(in crate::mir) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir) const fn frame(&self) -> &LoopExecutionFrameKeyV1 {
        self.frame
    }

    pub(in crate::mir) const fn scope_region(&self) -> ResolvedScopeRegionPairV1 {
        self.scope_region
    }

    pub(in crate::mir) const fn pos_binding(&self) -> BindingRefV1 {
        self.pos_binding
    }

    pub(in crate::mir) const fn src_binding(&self) -> BindingRefV1 {
        self.src_binding
    }

    pub(in crate::mir) const fn end_binding(&self) -> BindingRefV1 {
        self.end_binding
    }

    pub(in crate::mir) const fn pred_chars_binding(&self) -> BindingRefV1 {
        self.pred_chars_binding
    }

    pub(in crate::mir) const fn induction_binding(&self) -> BindingRefV1 {
        self.induction_binding
    }

    pub(in crate::mir) const fn induction_declaration(&self) -> &SourceBindingSiteV1 {
        self.induction_declaration
    }

    pub(in crate::mir) const fn initializer(&self) -> &SourceExprSiteV1 {
        self.initializer
    }

    pub(in crate::mir) const fn loop_site(&self) -> &SourceStmtSiteV1 {
        self.loop_site
    }

    pub(in crate::mir) const fn condition_i(&self) -> &SourceExprSiteV1 {
        self.condition_i
    }

    pub(in crate::mir) const fn step_read_i(&self) -> &SourceExprSiteV1 {
        self.step_read_i
    }

    pub(in crate::mir) const fn step_target_i(&self) -> &SourceExprSiteV1 {
        self.step_target_i
    }

    pub(in crate::mir) const fn inner_return_i(&self) -> &SourceExprSiteV1 {
        self.inner_return_i
    }

    pub(in crate::mir) const fn outer_return_i(&self) -> &SourceExprSiteV1 {
        self.outer_return_i
    }

    pub(in crate::mir) const fn completion_sites(&self) -> &[&SourceStmtSiteV1; 2] {
        &self.completion_sites
    }

    pub(in crate::mir) const fn pos_class(&self) -> DynamicFullLoopParameterClassV2 {
        self.pos_class
    }

    pub(in crate::mir) const fn end_class(&self) -> DynamicFullLoopParameterClassV2 {
        self.end_class
    }

    pub(in crate::mir) const fn src_class(&self) -> DynamicFullLoopParameterClassV2 {
        self.src_class
    }

    pub(in crate::mir) const fn pred_chars_class(&self) -> DynamicFullLoopParameterClassV2 {
        self.pred_chars_class
    }

    pub(in crate::mir) fn formal_rows(
        &self,
    ) -> [DynamicAPrimeFormalRelationRowV1<'_>; 4] {
        [
            DynamicAPrimeFormalRelationRowV1 {
                ordinal: 0,
                declaration: self.src_declaration,
                binding: self.src_binding,
                recipe_value: self.src_value,
                class: self.src_class,
            },
            DynamicAPrimeFormalRelationRowV1 {
                ordinal: 1,
                declaration: self.pos_declaration,
                binding: self.pos_binding,
                recipe_value: self.pos_value,
                class: self.pos_class,
            },
            DynamicAPrimeFormalRelationRowV1 {
                ordinal: 2,
                declaration: self.end_declaration,
                binding: self.end_binding,
                recipe_value: self.end_value,
                class: self.end_class,
            },
            DynamicAPrimeFormalRelationRowV1 {
                ordinal: 3,
                declaration: self.pred_chars_declaration,
                binding: self.pred_chars_binding,
                recipe_value: self.pred_chars_value,
                class: self.pred_chars_class,
            },
        ]
    }

    pub(in crate::mir) const fn induction_key(&self) -> LoopBindingKeyV1 {
        self.induction_key
    }

    pub(in crate::mir) const fn carrier_key(&self) -> LoopCarrierKeyV1 {
        self.carrier_key
    }

    pub(in crate::mir) const fn entry_value(&self) -> LoopValueKeyV1 {
        self.entry_value
    }

    pub(in crate::mir) const fn inner_return_value(&self) -> LoopValueKeyV1 {
        self.inner_return_value
    }

    pub(in crate::mir) const fn outer_tail_binding(&self) -> LoopBindingKeyV1 {
        self.outer_tail_binding
    }
}

pub(super) fn issue<R>(
    envelope: &VerifiedDynamicFullLoopSourceRecipeEnvelopeV2,
    callback: impl for<'program> FnOnce(DynamicAPrimeI64SourceRelationViewV1<'program>) -> R,
) -> Result<R, DynamicAPrimeI64SourceRelationRejectV1> {
    let view = issue_view(envelope)?;
    Ok(callback(view))
}

pub(super) fn issue_view(
    envelope: &VerifiedDynamicFullLoopSourceRecipeEnvelopeV2,
) -> Result<DynamicAPrimeI64SourceRelationViewV1<'_>, DynamicAPrimeI64SourceRelationRejectV1> {
    let source = &envelope.source;
    let artifact = &envelope.artifact;
    let coverage = &envelope.coverage;
    let src = binding(source, DynamicFullBodyBindingRoleV1::Src)?;
    let pos = binding(source, DynamicFullBodyBindingRoleV1::Pos)?;
    let end = binding(source, DynamicFullBodyBindingRoleV1::End)?;
    let pred_chars = binding(source, DynamicFullBodyBindingRoleV1::PredChars)?;
    let induction = binding(source, DynamicFullBodyBindingRoleV1::Induction)?;
    let initializer = expression(source, DynamicFullBodySourceRoleV1::PreludeInitializerPos)?;
    let loop_site = statement(source, DynamicFullBodySourceRoleV1::Loop)?;
    let condition_i = expression(source, DynamicFullBodySourceRoleV1::LoopConditionI)?;
    let step_read_i = expression(source, DynamicFullBodySourceRoleV1::StepReadI)?;
    let step_target_i = expression(source, DynamicFullBodySourceRoleV1::StepTargetI)?;
    let inner_return_i = expression(source, DynamicFullBodySourceRoleV1::InnerReturnI)?;
    let outer_return_i = expression(source, DynamicFullBodySourceRoleV1::OuterReturnI)?;

    let src_class = parameter_class(source, 0)?;
    let pos_class = parameter_class(source, 1)?;
    let end_class = parameter_class(source, 2)?;
    let pred_chars_class = parameter_class(source, 3)?;
    if src_class != DynamicFullLoopParameterClassV2::Dynamic
        || pos_class != DynamicFullLoopParameterClassV2::I64
        || end_class != DynamicFullLoopParameterClassV2::I64
        || pred_chars_class != DynamicFullLoopParameterClassV2::Dynamic
    {
        return Err(DynamicAPrimeI64SourceRelationRejectV1::ParameterContract);
    }

    let inner_site = statement_site(source, DynamicFullBodySourceRoleV1::InnerReturn)?;
    let outer_site = statement_site(source, DynamicFullBodySourceRoleV1::OuterReturn)?;
    let completion = source.completion.explicit_sites();
    if completion.len() != 2
        || !completion.iter().any(|site| site == inner_site)
        || !completion.iter().any(|site| site == outer_site)
    {
        return Err(DynamicAPrimeI64SourceRelationRejectV1::CompletionMismatch);
    }

    let expected_binding = |role, target| {
        (coverage.binding_target(role) == Some(target))
            .then_some(())
            .ok_or(DynamicAPrimeI64SourceRelationRejectV1::ClaimMismatch)
    };
    expected_binding(
        DynamicFullBodyBindingRoleV1::Src,
        DynamicFullLoopClaimTargetV2::Value(value(0)),
    )?;
    expected_binding(
        DynamicFullBodyBindingRoleV1::Pos,
        DynamicFullLoopClaimTargetV2::Value(value(1)),
    )?;
    expected_binding(
        DynamicFullBodyBindingRoleV1::End,
        DynamicFullLoopClaimTargetV2::Value(value(2)),
    )?;
    let induction_key = LoopBindingKeyV1::new(0);
    let carrier_key = LoopCarrierKeyV1::new(0);
    expected_binding(
        DynamicFullBodyBindingRoleV1::PredChars,
        DynamicFullLoopClaimTargetV2::Value(value(3)),
    )?;
    expected_binding(
        DynamicFullBodyBindingRoleV1::Induction,
        DynamicFullLoopClaimTargetV2::Binding(induction_key),
    )?;
    expected_source(
        coverage,
        DynamicFullBodySourceRoleV1::PreludeInitializerPos,
        DynamicFullLoopClaimTargetV2::Value(value(1)),
    )?;
    expected_source(
        coverage,
        DynamicFullBodySourceRoleV1::Loop,
        DynamicFullLoopClaimTargetV2::Loop(crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(0)),
    )?;
    for (role, item) in [
        (DynamicFullBodySourceRoleV1::LoopConditionI, 0),
        (DynamicFullBodySourceRoleV1::StepReadI, 13),
        (DynamicFullBodySourceRoleV1::StepTargetI, 16),
        (DynamicFullBodySourceRoleV1::InnerReturnI, 11),
    ] {
        expected_source(
            coverage,
            role,
            DynamicFullLoopClaimTargetV2::Item(
                crate::mir::loop_recipe_contract::LoopItemKeyV1::new(item),
            ),
        )?;
    }
    expected_source(
        coverage,
        DynamicFullBodySourceRoleV1::OuterReturnI,
        DynamicFullLoopClaimTargetV2::CallableTail {
            binding: induction_key,
        },
    )?;
    expected_source(
        coverage,
        DynamicFullBodySourceRoleV1::PreludeLocalI,
        DynamicFullLoopClaimTargetV2::PreludeInduction {
            binding: induction_key,
            carrier: carrier_key,
            entry: value(1),
        },
    )?;

    let recipe = artifact.recipe().as_recipe();
    if class(recipe, value(1)) != Some(LoopValueClassV2::I64)
        || class(recipe, value(2)) != Some(LoopValueClassV2::I64)
        || class(recipe, value(14)) != Some(LoopValueClassV2::I64)
        || recipe
            .bindings
            .iter()
            .find(|row| row.key == induction_key)
            .map(|row| row.class)
            != Some(LoopValueClassV2::I64)
        || recipe
            .carriers
            .iter()
            .find(|row| row.key == carrier_key)
            .map(|row| (row.binding, row.entry_value, row.class))
            != Some((induction_key, value(1), LoopValueClassV2::I64))
    {
        return Err(DynamicAPrimeI64SourceRelationRejectV1::RecipeMismatch);
    }

    let view = DynamicAPrimeI64SourceRelationViewV1 {
        owner: source.owner,
        frame: &source.frame,
        scope_region: source.scope_region,
        src_binding: src.binding(),
        pos_binding: pos.binding(),
        end_binding: end.binding(),
        pred_chars_binding: pred_chars.binding(),
        src_declaration: src.declaration(),
        pos_declaration: pos.declaration(),
        end_declaration: end.declaration(),
        pred_chars_declaration: pred_chars.declaration(),
        induction_binding: induction.binding(),
        induction_declaration: induction.declaration(),
        initializer,
        loop_site,
        condition_i,
        step_read_i,
        step_target_i,
        inner_return_i,
        outer_return_i,
        completion_sites: [inner_site, outer_site],
        src_class,
        pos_class,
        end_class,
        pred_chars_class,
        induction_key,
        carrier_key,
        entry_value: value(1),
        src_value: binding_value(coverage, DynamicFullBodyBindingRoleV1::Src)?,
        pos_value: binding_value(coverage, DynamicFullBodyBindingRoleV1::Pos)?,
        end_value: binding_value(coverage, DynamicFullBodyBindingRoleV1::End)?,
        pred_chars_value: binding_value(coverage, DynamicFullBodyBindingRoleV1::PredChars)?,
        inner_return_value: value(14),
        outer_tail_binding: induction_key,
    };
    Ok(view)
}

fn parameter_class(
    source: &DynamicFullLoopRetainedSourceV1,
    ordinal: u32,
) -> Result<DynamicFullLoopParameterClassV2, DynamicAPrimeI64SourceRelationRejectV1> {
    source
        .parameter_contract
        .rows()
        .iter()
        .find(|row| row.ordinal == ordinal)
        .map(|row| row.class)
        .ok_or(DynamicAPrimeI64SourceRelationRejectV1::ParameterContract)
}

fn binding_value(
    coverage: &VerifiedDynamicFullLoopClaimCoverageV2,
    role: DynamicFullBodyBindingRoleV1,
) -> Result<LoopValueKeyV1, DynamicAPrimeI64SourceRelationRejectV1> {
    match coverage.binding_target(role) {
        Some(DynamicFullLoopClaimTargetV2::Value(value)) => Ok(value),
        _ => Err(DynamicAPrimeI64SourceRelationRejectV1::ClaimMismatch),
    }
}

fn expected_source(
    coverage: &VerifiedDynamicFullLoopClaimCoverageV2,
    role: DynamicFullBodySourceRoleV1,
    target: DynamicFullLoopClaimTargetV2,
) -> Result<(), DynamicAPrimeI64SourceRelationRejectV1> {
    (coverage.source_target(role) == Some(target))
        .then_some(())
        .ok_or(DynamicAPrimeI64SourceRelationRejectV1::ClaimMismatch)
}

fn binding(
    source: &DynamicFullLoopRetainedSourceV1,
    role: DynamicFullBodyBindingRoleV1,
) -> Result<&DynamicFullBodyBindingRowV1, DynamicAPrimeI64SourceRelationRejectV1> {
    source
        .bindings
        .iter()
        .find(|row| row.role() == role)
        .ok_or(DynamicAPrimeI64SourceRelationRejectV1::MissingBinding)
}

fn statement_site(
    source: &DynamicFullLoopRetainedSourceV1,
    role: DynamicFullBodySourceRoleV1,
) -> Result<&SourceStmtSiteV1, DynamicAPrimeI64SourceRelationRejectV1> {
    source
        .rows
        .iter()
        .find_map(|row| {
            (row.role() == role).then(|| match row.site() {
                DynamicFullBodySourceSiteV1::Statement(site) => Some(site),
                DynamicFullBodySourceSiteV1::Expression(_) => None,
            })?
        })
        .ok_or(DynamicAPrimeI64SourceRelationRejectV1::MissingSource)
}

fn statement(
    source: &DynamicFullLoopRetainedSourceV1,
    role: DynamicFullBodySourceRoleV1,
) -> Result<&SourceStmtSiteV1, DynamicAPrimeI64SourceRelationRejectV1> {
    statement_site(source, role)
}

fn expression(
    source: &DynamicFullLoopRetainedSourceV1,
    role: DynamicFullBodySourceRoleV1,
) -> Result<&SourceExprSiteV1, DynamicAPrimeI64SourceRelationRejectV1> {
    source
        .rows
        .iter()
        .find_map(|row| {
            (row.role() == role).then(|| match row.site() {
                DynamicFullBodySourceSiteV1::Expression(site) => Some(site),
                DynamicFullBodySourceSiteV1::Statement(_) => None,
            })?
        })
        .ok_or(DynamicAPrimeI64SourceRelationRejectV1::MissingSource)
}

fn class(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV2,
    key: LoopValueKeyV1,
) -> Option<LoopValueClassV2> {
    recipe
        .values
        .iter()
        .find(|row| row.key == key)
        .map(|row| row.class)
}

const fn value(raw: u32) -> LoopValueKeyV1 {
    LoopValueKeyV1::new(raw)
}
