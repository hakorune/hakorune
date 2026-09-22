//! Emission observations extend the package-owned source relation. They cannot
//! replace its canonical caller, constructor site, or resolver contract.
use super::SelectedSourceCoreMethodCallV1;
use crate::mir::named_array_obligation::{
    fault, validate_physical_marker, NamedArrayWriteMarkerV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceExprSiteV1};
use crate::mir::{ArrayWriteSiteId, MirModule, ValueId};

#[derive(Debug)]
pub(crate) struct NamedArrayEmissionDraftV1 {
    source: SelectedSourceCoreMethodCallV1,
    allocation: Option<ValueId>,
}

#[derive(Debug)]
pub(crate) struct EmittedNamedArrayRequirementV1 {
    source: SelectedSourceCoreMethodCallV1,
    marker: NamedArrayWriteMarkerV1,
}

impl SelectedSourceCoreMethodCallV1 {
    pub(crate) fn into_named_array_emission(self) -> Result<NamedArrayEmissionDraftV1, String> {
        if self.contract().named_array_requirement().is_none() {
            return Err(fault("source-requirement-missing"));
        }
        Ok(NamedArrayEmissionDraftV1 {
            source: self,
            allocation: None,
        })
    }
}

impl NamedArrayEmissionDraftV1 {
    pub(crate) fn record_allocation(
        &mut self,
        owner: FunctionOwnerIdV1,
        site: &SourceExprSiteV1,
        destination: ValueId,
    ) -> Result<(), String> {
        let requirement = self
            .source
            .contract()
            .named_array_requirement()
            .ok_or_else(|| fault("source-requirement-missing"))?;
        if requirement.owner() != owner || requirement.construction() != site {
            return Err(fault("allocation-source-mismatch"));
        }
        if self.allocation.is_some() {
            return Err(fault("duplicate-allocation"));
        }
        self.allocation = Some(destination);
        Ok(())
    }

    pub(crate) fn record_write(
        self,
        owner: FunctionOwnerIdV1,
        site: &SourceExprSiteV1,
        argument_site: &SourceExprSiteV1,
        write: ArrayWriteSiteId,
        receiver: ValueId,
        argument: ValueId,
    ) -> Result<EmittedNamedArrayRequirementV1, String> {
        let requirement = self
            .source
            .contract()
            .named_array_requirement()
            .ok_or_else(|| fault("source-requirement-missing"))?;
        if requirement.owner() != owner
            || requirement.call() != site
            || requirement.argument() != argument_site
        {
            return Err(fault("write-source-mismatch"));
        }
        let allocation = self
            .allocation
            .ok_or_else(|| fault("allocation-not-emitted"))?;
        Ok(EmittedNamedArrayRequirementV1 {
            source: self.source,
            marker: NamedArrayWriteMarkerV1 {
                allocation,
                receiver,
                argument,
                write,
            },
        })
    }
}

impl EmittedNamedArrayRequirementV1 {
    pub(crate) fn marker(&self) -> NamedArrayWriteMarkerV1 {
        self.marker
    }

    pub(crate) fn validate<'m>(&self, module: &'m MirModule) -> Result<&'m str, String> {
        let symbol = module
            .canonical_callable_definition_symbol(&self.source.caller)
            .ok_or_else(|| fault("canonical-caller-missing"))?;
        let function = module
            .functions
            .get(symbol)
            .ok_or_else(|| fault("physical-caller-missing"))?;
        if function.signature.name != symbol {
            return Err(fault("physical-caller-drift"));
        }
        validate_physical_marker(function, &self.marker)?;
        Ok(symbol)
    }
}

/// Check both directions: neither markers without source nor dropped markers
/// on a retained row can pass. Shared allocation is allowed; writes are unique.
pub(crate) fn validate_named_array_coverage(
    module: &MirModule,
    rows: &[EmittedNamedArrayRequirementV1],
) -> Result<(), String> {
    let mut covered = std::collections::BTreeSet::new();
    let mut source_calls = std::collections::BTreeSet::new();
    let mut constructions = std::collections::BTreeMap::new();
    let mut allocations = std::collections::BTreeMap::new();
    for row in rows {
        let symbol = row.validate(module)?;
        let requirement = row
            .source
            .contract()
            .named_array_requirement()
            .ok_or_else(|| fault("source-requirement-missing"))?;
        if !source_calls.insert((symbol, requirement.call())) {
            return Err(fault("duplicate-source-write"));
        }
        if constructions
            .insert((symbol, requirement.construction()), row.marker.allocation)
            .is_some_and(|previous| previous != row.marker.allocation)
            || allocations
                .insert((symbol, row.marker.allocation), requirement.construction())
                .is_some_and(|previous| previous != requirement.construction())
        {
            return Err(fault("construction-allocation-mismatch"));
        }
        if !covered.insert((symbol, row.marker.write.0)) {
            return Err(fault("duplicate-retained-write"));
        }
        let markers = &module.functions[symbol]
            .metadata
            .named_array_write_obligations;
        if markers
            .iter()
            .filter(|marker| **marker == row.marker)
            .count()
            != 1
        {
            return Err(fault("marker-source-mismatch"));
        }
    }
    let marker_count: usize = module
        .functions
        .iter()
        .map(|(symbol, function)| {
            function
                .metadata
                .named_array_write_obligations
                .iter()
                .map(|marker| {
                    if !covered.contains(&(symbol.as_str(), marker.write.0)) {
                        return Err(fault("uncovered-write-marker"));
                    }
                    Ok(1usize)
                })
                .sum::<Result<usize, String>>()
        })
        .sum::<Result<usize, String>>()?;
    if marker_count != rows.len() {
        return Err(fault("marker-cardinality"));
    }
    Ok(())
}
