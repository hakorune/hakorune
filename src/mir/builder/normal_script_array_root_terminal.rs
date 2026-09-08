//! Root completion derived from the same Script source inventory as Array rows.
//! Return admission alone proves neither the outgoing value nor cleanup.
use crate::mir::resolved_semantics::{
    BindingRefV1, BodyStatementShapeV1, FunctionOwnerIdV1, RegionId, ResolvedControlTransferV1,
    ResolvedExitOriginV1, ResolvedExitSiteV1, ResolvedLiteralSourceV1, ScopeId,
    ScriptRootResolvedDemandV1, ScriptRootSemanticDispositionV1, SourceExprSiteV1,
    SourcePathSegmentV1, SourceStmtSiteV1, VerifiedResolvedScriptV1,
    VerifiedScriptRootDemandWindowV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RootResult {
    Unit,
    Integer { site: SourceExprSiteV1, value: i64 },
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct RootTerminal {
    owner: FunctionOwnerIdV1,
    scope: ScopeId,
    body_scope: ScopeId,
    source_region: RegionId,
    target: RegionId,
    site: SourceStmtSiteV1,
    result: RootResult,
    homes: Box<[BindingRefV1]>,
}
impl RootTerminal {
    pub(in crate::mir::builder) fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }
    pub(in crate::mir::builder) fn result(&self) -> &RootResult {
        &self.result
    }

    pub(super) fn homes(&self) -> &[BindingRefV1] {
        &self.homes
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum RootTerminalCoverage {
    NotSelected,
    Available(RootTerminal),
    Unavailable(&'static str),
}
impl Default for RootTerminalCoverage {
    fn default() -> Self {
        Self::Unavailable("root-terminal-unissued")
    }
}
impl RootTerminalCoverage {
    pub(super) fn issue(
        product: &VerifiedResolvedScriptV1,
        window: &VerifiedScriptRootDemandWindowV1,
        ordinal: usize,
        prefix_available: bool,
        homes: &[BindingRefV1],
    ) -> Self {
        match issue(product, window, ordinal, prefix_available, homes) {
            Ok(terminal) => Self::Available(terminal),
            Err(reason) => Self::Unavailable(reason),
        }
    }
    pub(super) fn require(&self) -> Result<&RootTerminal, &'static str> {
        match self {
            Self::NotSelected => Err("root-not-selected"),
            Self::Available(row) => Ok(row),
            Self::Unavailable(reason) => Err(reason),
        }
    }
}

fn issue(
    product: &VerifiedResolvedScriptV1,
    window: &VerifiedScriptRootDemandWindowV1,
    ordinal: usize,
    prefix_available: bool,
    homes: &[BindingRefV1],
) -> Result<RootTerminal, &'static str> {
    if !prefix_available {
        return Err("root-prefix-capability");
    }
    if !window.is_final_ordinal(ordinal) {
        return Err("root-return-not-final");
    }
    let entry = window.entries().get(ordinal).ok_or("root-return-missing")?;
    if !matches!(
        entry.semantic(),
        ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::ReturnExit(_))
    ) {
        return Err("root-return-admission");
    }
    let core = product.core().data();
    let shape = product.body_shape();
    let Some(BodyStatementShapeV1::Return { site, value }) = shape.statements().last() else {
        return Err("root-return-shape");
    };
    if site != entry.site() || shape.owner() != core.owner {
        return Err("root-return-owner-site");
    }
    let exit = core
        .resolved_exits
        .get(&ResolvedExitSiteV1::Statement(site.clone()))
        .ok_or("root-return-exit")?;
    validate_return_target(exit, core.function_region)?;
    if core
        .scopes
        .get(&core.function_scope)
        .map(|scope| scope.owner_region())
        != Some(core.function_region)
        || core
            .regions
            .get(&core.function_region)
            .and_then(|region| region.lexical_scope())
            != Some(core.function_scope)
    {
        return Err("root-scope");
    }
    let source_region = core
        .regions
        .get(&exit.source_region())
        .ok_or("root-return-source-region")?;
    if source_region.kind() != crate::mir::resolved_semantics::RegionKindV1::Sequence
        || source_region.parent() != Some(core.function_region)
        || source_region.origin()
            != &crate::mir::resolved_semantics::RegionOriginV1::Source(
                crate::mir::resolved_semantics::SourcePathV1::program_body().node(),
            )
    {
        return Err("root-return-source-region");
    }
    let body_scope = source_region.lexical_scope().ok_or("root-body-scope")?;
    let body = core.scopes.get(&body_scope).ok_or("root-body-scope")?;
    if body.parent() != Some(core.function_scope)
        || body.owner_region() != exit.source_region()
        || homes
            .iter()
            .any(|binding| !body.declarations().contains(binding))
    {
        return Err("root-body-home-scope");
    }
    let relations = shape
        .relations()
        .iter()
        .filter(|row| row.parent() == site.node())
        .collect::<Vec<_>>();
    let result = validate_return_value(
        value.as_ref(),
        value
            .as_ref()
            .and_then(|site| core.expression_source.literal(site)),
        &relations,
    )?;
    if homes.iter().any(|binding| binding.owner() != core.owner)
        || homes
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != homes.len()
    {
        return Err("root-home-owner-duplicate");
    }
    Ok(RootTerminal {
        owner: core.owner,
        scope: core.function_scope,
        body_scope,
        source_region: exit.source_region(),
        target: core.function_region,
        site: site.clone(),
        result,
        homes: homes.iter().rev().copied().collect(),
    })
}

fn validate_return_target(
    exit: &crate::mir::resolved_semantics::ResolvedExitRecordV1,
    target: RegionId,
) -> Result<(), &'static str> {
    if exit.origin() != ResolvedExitOriginV1::ExplicitReturn
        || exit.transfer()
            != (ResolvedControlTransferV1::Return {
                target_function: target,
            })
    {
        return Err("root-return-target");
    }
    Ok(())
}
fn validate_return_value(
    value: Option<&SourceExprSiteV1>,
    literal: Option<&ResolvedLiteralSourceV1>,
    relations: &[&crate::mir::resolved_semantics::BodyShapeRelationV1],
) -> Result<RootResult, &'static str> {
    match value {
        None if relations.is_empty() => Ok(RootResult::Unit),
        Some(value)
            if relations.len() == 1
                && relations[0].role() == &SourcePathSegmentV1::Value
                && relations[0].child() == value =>
        {
            let Some(ResolvedLiteralSourceV1::Integer(integer)) = literal else {
                return Err("root-result-capability");
            };
            Ok(RootResult::Integer {
                site: value.clone(),
                value: *integer,
            })
        }
        _ => Err("root-return-value-relation"),
    }
}
#[cfg(test)]
#[path = "normal_script_array_root_terminal_tests.rs"]
mod tests;
