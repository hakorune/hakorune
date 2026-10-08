//! Read-only common target loan for the same finite opaque-forward graph.
//! A static call keeps its original static source; no instance receiver is made.
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use hakorune_mir_defs::SameModuleCallableNamespaceV1;

#[derive(Debug, Clone, Copy)]
pub(in crate::mir) enum BorrowedCallSourceLoanV1<'a> {
    Instance(&'a super::super::LexicalInstanceCallSourceTargetV1),
    Static(&'a StaticIncomingSourceV1),
}

impl BorrowedCallSourceLoanV1<'_> {
    pub(in crate::mir) fn call_site(&self) -> &OwnedExprSiteV1 {
        match self {
            Self::Instance(row) => row.call_site(),
            Self::Static(row) => row.call_site(),
        }
    }
    pub(in crate::mir) fn target(&self) -> &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1 {
        match self {
            Self::Instance(row) => row.target(),
            Self::Static(row) => row.target(),
        }
    }
    pub(in crate::mir) fn callee_owner(&self) -> FunctionOwnerIdV1 {
        match self {
            Self::Instance(row) => row.callee_owner(),
            Self::Static(row) => row.callee_owner(),
        }
    }
    pub(in crate::mir) fn target_batch_slot(&self) -> u32 {
        match self {
            Self::Instance(row) => row.target_batch_slot(),
            Self::Static(row) => row.target_batch_slot(),
        }
    }
    pub(in crate::mir) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        match self {
            Self::Instance(row) => row.argument_sites(),
            Self::Static(row) => row.argument_sites(),
        }
    }
    pub(in crate::mir) fn namespace(&self) -> SameModuleCallableNamespaceV1 {
        match self {
            Self::Instance(_) => SameModuleCallableNamespaceV1::InstanceBoxMethod,
            Self::Static(_) => SameModuleCallableNamespaceV1::StaticBoxMethod,
        }
    }
    pub(in crate::mir) fn declaration_mode(&self) -> CallableParameterDeclarationModeV1 {
        match self {
            Self::Instance(_) => CallableParameterDeclarationModeV1::InstanceBoxMethod,
            Self::Static(_) => CallableParameterDeclarationModeV1::StaticBoxMethod,
        }
    }
}

/// Lend already retained products. This has no source walk or target resolution.
pub(in crate::mir) fn borrow_call_sources_v1<'a>(
    instance: &BTreeMap<OwnedExprSiteV1, &'a super::super::LexicalInstanceCallSourceTargetV1>,
    static_sources: impl IntoIterator<Item = &'a StaticIncomingSourceV1>,
) -> Result<BTreeMap<OwnedExprSiteV1, BorrowedCallSourceLoanV1<'a>>, String> {
    let mut result: BTreeMap<_, _> = instance
        .iter()
        .map(|(site, row)| (site.clone(), BorrowedCallSourceLoanV1::Instance(row)))
        .collect();
    for source in static_sources {
        match result.get(source.call_site()) {
            Some(BorrowedCallSourceLoanV1::Static(previous))
                if std::ptr::eq(*previous, source) => {}
            Some(_) => {
                return Err("[freeze:contract][borrowed-formal/conflicting-source-call]".into())
            }
            None => {
                result.insert(
                    source.call_site().clone(),
                    BorrowedCallSourceLoanV1::Static(source),
                );
            }
        }
    }
    Ok(result)
}
