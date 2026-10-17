//! Original append/read requirements over the sole named construction issuer.
//! No element class, transferable Home, live slot or provider capability is issued.
use super::named_array_requirement::{
    NamedArrayConstructionSourceV1, NamedArrayRequirementIssueV1,
};
use super::{
    CallableSemanticSourceLedgerView, ResolvedLexicalRefV1, ResolvedLiteralSourceV1,
    ResolvedMethodCallReceiverSourceV1, SourceExprSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
    VerifiedResolvedMethodCallSourceV1,
};
use crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1;
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::core_method_result_kind::lookup_core_method_result_row_v2;
use crate::parser::ParserOrdinaryBoxSourceCoverageV1;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NamedArrayAppendOnlySourceIssueV1 {
    Construction(NamedArrayRequirementIssueV1),
    Declaration,
    SourceOrder,
    UnsupportedUse,
    Argument,
    CandidateSource,
    IndexSource,
    HolderEscape,
}

#[derive(Debug)]
pub(crate) struct NamedArrayAppendSourceV1 {
    call: SourceExprSiteV1,
    receiver: SourceExprSiteV1,
    candidate: SourceExprSiteV1,
    null_candidate: bool,
}
impl NamedArrayAppendSourceV1 {
    pub(crate) fn call(&self) -> &SourceExprSiteV1 {
        &self.call
    }
    pub(crate) fn candidate(&self) -> &SourceExprSiteV1 {
        &self.candidate
    }
    pub(crate) fn receiver(&self) -> &SourceExprSiteV1 { &self.receiver }
    pub(crate) fn null_candidate(&self) -> bool { self.null_candidate }
}

#[derive(Debug)]
pub(crate) struct NamedArrayReadSourceV1 {
    source: std::sync::Arc<VerifiedResolvedMethodCallSourceV1>,
    integer: i64,
}
impl NamedArrayReadSourceV1 {
    pub(crate) fn source(&self) -> &std::sync::Arc<VerifiedResolvedMethodCallSourceV1> { &self.source }
    pub(crate) fn receiver(&self) -> &SourceExprSiteV1 { self.source.receiver_site() }
    pub(crate) fn call(&self) -> &SourceExprSiteV1 {
        self.source.site()
    }
    pub(crate) fn index(&self) -> &SourceExprSiteV1 {
        self.source.arguments()[0].site()
    }
    pub(crate) fn integer(&self) -> i64 {
        self.integer
    }
}

/// Complete original holder use requirements, conditional on producer and
/// provider verification. Future appends are retained, not read-liveness evidence.
#[derive(Debug)]
pub(crate) struct NamedArrayAppendOnlySourceV1 {
    construction: NamedArrayConstructionSourceV1,
    appends: Box<[NamedArrayAppendSourceV1]>,
    reads: Box<[NamedArrayReadSourceV1]>,
}
impl NamedArrayAppendOnlySourceV1 {
    pub(crate) fn construction(&self) -> &NamedArrayConstructionSourceV1 {
        &self.construction
    }
    pub(crate) fn appends(&self) -> &[NamedArrayAppendSourceV1] {
        &self.appends
    }
    pub(crate) fn reads(&self) -> &[NamedArrayReadSourceV1] {
        &self.reads
    }

    pub(crate) fn issue(
        ledger: &CallableSemanticSourceLedgerView<'_>,
        read: &VerifiedResolvedMethodCallSourceV1,
        ordinary: &ParserOrdinaryBoxSourceCoverageV1,
        brands: &VerifiedBrandProgramDeclarationCatalogV1,
    ) -> Result<Self, NamedArrayAppendOnlySourceIssueV1> {
        use NamedArrayAppendOnlySourceIssueV1 as E;
        let construction = NamedArrayConstructionSourceV1::issue(ledger, read, ordinary, brands)
            .map_err(E::Construction)?;
        let binding = construction.binding();
        let initializer = ledger
            .initializer_relations()
            .find(|row| row.binding() == binding)
            .ok_or(E::Declaration)?;
        if ledger.declaration_binding(initializer.declaration_site()) != Some(binding) {
            return Err(E::Declaration);
        }
        let [SourcePathSegmentV1::Body(construction_order), SourcePathSegmentV1::Initializer(_)] =
            construction.construction().node().segments()
        else {
            return Err(E::SourceOrder);
        };
        let mut receivers = BTreeSet::new();
        let mut appends = Vec::new();
        let mut reads = Vec::new();
        for (site, call) in ledger.method_calls().filter(|(_, call)| {
            call.receiver()
                == ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding))
        }) {
            if call.owner() != ledger.owner()
                || call.site() != site
                || ledger.variable_ref(call.receiver_site())
                    != Some(ResolvedLexicalRefV1::Local(binding))
                || !receivers.insert(call.receiver_site().clone())
            {
                return Err(E::UnsupportedUse);
            }
            let [argument] = call.arguments() else {
                return Err(E::Argument);
            };
            if argument.ordinal() != 0 || call.arity() != 1 {
                return Err(E::Argument);
            }
            let segments = site.node().segments();
            let Some(SourcePathSegmentV1::Body(order)) = segments.first() else {
                return Err(E::SourceOrder);
            };
            if order <= construction_order {
                return Err(E::SourceOrder);
            }
            let op = lookup_core_method_result_row_v2("ArrayBox", call.selector(), call.arity())
                .map(|row| row.op)
                .ok_or(E::UnsupportedUse)?;
            match op {
                CoreMethodOp::ArrayPush => {
                    if segments.len() != 1
                        || !ledger
                            .source_site_inventory()
                            .contains_statement(&SourceStmtSiteV1::from_node(site.node().clone()))
                    {
                        return Err(E::SourceOrder);
                    }
                    let null_candidate = match (
                        ledger.literal_source(argument.site()),
                        ledger.method_call(argument.site()),
                    ) {
                        (Some(ResolvedLiteralSourceV1::Null), None) => true,
                        (None, Some(child)) if child.receiver() != call.receiver() => false,
                        _ => return Err(E::CandidateSource),
                    };
                    appends.push(NamedArrayAppendSourceV1 {
                        call: site.clone(),
                        receiver: call.receiver_site().clone(),
                        candidate: argument.site().clone(),
                        null_candidate,
                    });
                }
                CoreMethodOp::ArrayGet => {
                    // A direct argument read has no stored result/alias to outlive its holder.
                    if segments.len() < 2
                        || !segments[1..]
                            .iter()
                            .all(|segment| matches!(segment, SourcePathSegmentV1::Argument(_)))
                    {
                        return Err(E::SourceOrder);
                    }
                    let Some(ResolvedLiteralSourceV1::Integer(integer)) =
                        ledger.literal_source(argument.site())
                    else {
                        return Err(E::IndexSource);
                    };
                    let source = ledger.method_call_source_v1(site).ok_or(E::UnsupportedUse)?;
                    if !std::ptr::eq(source.as_ref(), call) { return Err(E::UnsupportedUse); }
                    reads.push(NamedArrayReadSourceV1 {
                        source: std::sync::Arc::clone(source), integer: *integer,
                    });
                }
                _ => return Err(E::UnsupportedUse),
            }
        }
        if ledger.variable_refs().any(|(site, reference)| {
            *reference == ResolvedLexicalRefV1::Local(binding) && !receivers.contains(site)
        }) {
            return Err(E::HolderEscape);
        }
        if !reads.iter().any(|row| row.call() == read.site()) {
            return Err(E::UnsupportedUse);
        }
        // Sorting is safe only after the exact straight-line source shapes above.
        appends.sort_by(|a, b| a.call.cmp(&b.call));
        reads.sort_by(|a, b| a.call().cmp(b.call()));
        Ok(Self {
            construction,
            appends: appends.into_boxed_slice(),
            reads: reads.into_boxed_slice(),
        })
    }
}
