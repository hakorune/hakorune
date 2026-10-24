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
    pub(crate) fn receiver(&self) -> &SourceExprSiteV1 {
        &self.receiver
    }
    pub(crate) fn null_candidate(&self) -> bool {
        self.null_candidate
    }
}

#[derive(Debug)]
pub(crate) struct NamedArrayReadSourceV1 {
    source: std::sync::Arc<VerifiedResolvedMethodCallSourceV1>,
    integer: i64,
}
impl NamedArrayReadSourceV1 {
    pub(crate) fn source(&self) -> &std::sync::Arc<VerifiedResolvedMethodCallSourceV1> {
        &self.source
    }
    pub(crate) fn receiver(&self) -> &SourceExprSiteV1 {
        self.source.receiver_site()
    }
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
    returned_values: Box<[SourceExprSiteV1]>,
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

    pub(crate) fn has_appends(&self) -> bool {
        !self.appends.is_empty()
    }

    pub(crate) fn append_candidate_v1(&self, call: &SourceExprSiteV1) -> Option<&SourceExprSiteV1> {
        self.appends
            .iter()
            .find(|row| row.call() == call)
            .map(NamedArrayAppendSourceV1::candidate)
    }

    pub(crate) fn append_candidate_is_null_v1(&self, call: &SourceExprSiteV1) -> Option<bool> {
        self.appends
            .iter()
            .find(|row| row.call() == call)
            .map(NamedArrayAppendSourceV1::null_candidate)
    }

    pub(crate) fn returned_values(&self) -> &[SourceExprSiteV1] {
        &self.returned_values
    }

    pub(crate) fn issue(
        ledger: &CallableSemanticSourceLedgerView<'_>,
        read: &VerifiedResolvedMethodCallSourceV1,
        ordinary: &ParserOrdinaryBoxSourceCoverageV1,
        brands: &VerifiedBrandProgramDeclarationCatalogV1,
    ) -> Result<Self, NamedArrayAppendOnlySourceIssueV1> {
        let construction = NamedArrayConstructionSourceV1::issue(ledger, read, ordinary, brands)
            .map_err(NamedArrayAppendOnlySourceIssueV1::Construction)?;
        Self::issue_for_construction_with_read_v1(
            ledger,
            construction,
            ordinary,
            brands,
            Some(read),
            None,
        )
    }

    /// Complete original uses for a named Array even when it has no Get.
    /// This is source membership only; Dynamic result ownership is checked at
    /// the candidate's canonical producer and physical consumer.
    pub(crate) fn issue_for_construction_v1(
        ledger: &CallableSemanticSourceLedgerView<'_>,
        construction: NamedArrayConstructionSourceV1,
        ordinary: &ParserOrdinaryBoxSourceCoverageV1,
        brands: &VerifiedBrandProgramDeclarationCatalogV1,
        body_shape: &super::VerifiedResolvedBodyShapeInventoryV1,
    ) -> Result<Self, NamedArrayAppendOnlySourceIssueV1> {
        Self::issue_for_construction_with_read_v1(
            ledger,
            construction,
            ordinary,
            brands,
            None,
            Some(body_shape),
        )
    }

    fn issue_for_construction_with_read_v1(
        ledger: &CallableSemanticSourceLedgerView<'_>,
        construction: NamedArrayConstructionSourceV1,
        ordinary: &ParserOrdinaryBoxSourceCoverageV1,
        brands: &VerifiedBrandProgramDeclarationCatalogV1,
        required_read: Option<&VerifiedResolvedMethodCallSourceV1>,
        body_shape: Option<&super::VerifiedResolvedBodyShapeInventoryV1>,
    ) -> Result<Self, NamedArrayAppendOnlySourceIssueV1> {
        use NamedArrayAppendOnlySourceIssueV1 as E;
        if construction.owner() != ledger.owner()
            || NamedArrayConstructionSourceV1::issue_for_binding(
                ledger,
                construction.binding(),
                ordinary,
                brands,
            )
            .as_ref()
                != Ok(&construction)
        {
            return Err(E::Construction(NamedArrayRequirementIssueV1::ForeignCall));
        }
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
            // Callable method observations may retain the function-body root
            // prefix; compare the top-level body ordinal from the same path
            // instead of requiring it to occupy segment zero.
            let direct_order = segments.iter().find_map(|segment| match segment {
                SourcePathSegmentV1::Body(order) => Some(*order),
                _ => None,
            });
            let enclosing_loop_orders: BTreeSet<_> =
                ledger
                    .loop_sites()
                    .filter(|loop_site| {
                        ledger.resolved_loop_source(loop_site).is_ok()
                            && ledger.resolved_loop_placement(loop_site, site).is_ok_and(
                                |placement| placement == Some(super::ResolvedLoopPlacementV1::Body),
                            )
                    })
                    .filter_map(|loop_site| {
                        loop_site
                            .node()
                            .segments()
                            .iter()
                            .find_map(|segment| match segment {
                                SourcePathSegmentV1::Body(order) => Some(*order),
                                _ => None,
                            })
                    })
                    .collect();
            let order = direct_order.or_else(|| {
                (enclosing_loop_orders.len() == 1)
                    .then(|| *enclosing_loop_orders.iter().next().expect("one loop order"))
            });
            let Some(order) = order else {
                return Err(E::SourceOrder);
            };
            if order <= *construction_order {
                return Err(E::SourceOrder);
            }
            let op = lookup_core_method_result_row_v2("ArrayBox", call.selector(), call.arity())
                .map(|row| row.op)
                .ok_or(E::UnsupportedUse)?;
            match op {
                CoreMethodOp::ArrayPush => {
                    // Nested branches and loops remain eligible only when the
                    // resolver inventory ties this exact call expression to a
                    // source statement. The top-level Body ordinal above
                    // preserves ordering relative to the original New.
                    if ledger
                        .source_site_inventory()
                        .containing_statement(site)
                        .is_none()
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
                        || (!segments[1..]
                            .iter()
                            .all(|segment| matches!(segment, SourcePathSegmentV1::Argument(_)))
                            && !ledger.method_calls().any(|(_, parent)| {
                                parent
                                    .arguments()
                                    .iter()
                                    .any(|argument| argument.site() == site)
                            }))
                    {
                        return Err(E::SourceOrder);
                    }
                    let Some(ResolvedLiteralSourceV1::Integer(integer)) =
                        ledger.literal_source(argument.site())
                    else {
                        return Err(E::IndexSource);
                    };
                    let source = ledger
                        .method_call_source_v1(site)
                        .ok_or(E::UnsupportedUse)?;
                    if !std::ptr::eq(source.as_ref(), call) {
                        return Err(E::UnsupportedUse);
                    }
                    reads.push(NamedArrayReadSourceV1 {
                        source: std::sync::Arc::clone(source),
                        integer: *integer,
                    });
                }
                _ => return Err(E::UnsupportedUse),
            }
        }
        let returned_values: Vec<_> = if let Some(shape) = body_shape {
            if shape.owner() != ledger.owner() {
                return Err(E::HolderEscape);
            }
            shape
                .return_value_sites()
                .filter(|site| {
                    ledger.variable_ref(site) == Some(ResolvedLexicalRefV1::Local(binding))
                })
                .cloned()
                .collect()
        } else {
            Vec::new()
        };
        let mut allowed_uses = receivers;
        allowed_uses.extend(returned_values.iter().cloned());
        if !only_original_local_uses(ledger, binding, &allowed_uses) {
            return Err(E::HolderEscape);
        }
        if required_read.is_some_and(|read| !reads.iter().any(|row| row.call() == read.site())) {
            return Err(E::UnsupportedUse);
        }
        // Deterministic source-site identity order only. This is neither
        // runtime append order nor dominance or a lower bound on array length.
        appends.sort_by(|a, b| a.call.cmp(&b.call));
        reads.sort_by(|a, b| a.call().cmp(b.call()));
        Ok(Self {
            construction,
            appends: appends.into_boxed_slice(),
            reads: reads.into_boxed_slice(),
            returned_values: returned_values.into_boxed_slice(),
        })
    }
}

/// Complete original-use coverage, shared by element holders and empty transfers.
/// This is source membership only; Home availability remains on the SAME flow.
fn only_original_local_uses(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    binding: super::BindingRefV1,
    allowed: &BTreeSet<SourceExprSiteV1>,
) -> bool {
    !ledger.variable_refs().any(|(site, reference)| {
        *reference == ResolvedLexicalRefV1::Local(binding) && !allowed.contains(site)
    })
}

/// Original empty contents through one direct field transfer. This has no Home,
/// canonical field, runtime storage or End execution authority of its own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NamedArrayEmptyTransferSourceV1 {
    construction: NamedArrayConstructionSourceV1,
    assignment: super::ResolvedAssignmentSourceV1,
    destination: super::BindingRefV1,
    field: Box<str>,
}
impl NamedArrayEmptyTransferSourceV1 {
    pub(crate) fn issue(
        ledger: &CallableSemanticSourceLedgerView<'_>,
        shape: &super::VerifiedResolvedBodyShapeInventoryV1,
        construction: &NamedArrayConstructionSourceV1,
    ) -> Option<Self> {
        use super::{BodyExpressionShapeV1 as Shape, ResolvedAssignmentTargetV1 as Target};
        let binding = construction.binding();
        if ledger.owner() != construction.owner()
            || shape.owner() != ledger.owner()
            || !ledger.uncaptured_binding(binding)
        {
            return None;
        }
        let mut uses = ledger.variable_refs().filter_map(|(site, reference)| {
            (*reference == ResolvedLexicalRefV1::Local(binding)).then_some(site)
        });
        let rhs = uses.next()?;
        if uses.next().is_some() {
            return None;
        }
        let mut assignments = shape
            .assignment_sources()
            .iter()
            .filter(|row| row.value_site() == rhs);
        let assignment = assignments.next()?;
        if assignments.next().is_some()
            || assignment.form() != super::ResolvedAssignmentFormV1::Plain
        {
            return None;
        }
        let [SourcePathSegmentV1::Body(created), SourcePathSegmentV1::Initializer(_)] =
            construction.construction().node().segments()
        else {
            return None;
        };
        let [SourcePathSegmentV1::Body(transferred)] =
            assignment.statement_site().node().segments()
        else {
            return None;
        };
        if transferred <= created {
            return None;
        }
        let Shape::FieldAccess { object, field, .. } =
            shape.expression_shape(assignment.target_site())?
        else {
            return None;
        };
        let ResolvedLexicalRefV1::Local(destination) = ledger.variable_ref(object)? else {
            return None;
        };
        if destination == binding
            || !ledger.uncaptured_binding(destination)
            || !matches!(ledger.assignment_target(assignment.target_site()), Some(Target::FieldWrite { receiver }) if receiver == object)
            || ledger.assignment_targets().any(|(_, target)| {
                matches!(target,
                Target::BindingRebind(actual) if *actual == binding || *actual == destination)
            })
        {
            return None;
        }
        let mut receivers = BTreeSet::new();
        for expression in shape.expressions() {
            let Shape::FieldAccess {
                site,
                object,
                field: accessed,
            } = expression
            else {
                continue;
            };
            if ledger.variable_ref(object) != Some(ResolvedLexicalRefV1::Local(destination)) {
                continue;
            }
            // No direct/nested read, second writer or method receiver for this
            // residence. Other fields acquire no scalar permission here.
            if accessed == field && site != assignment.target_site() {
                return None;
            }
            receivers.insert(object.clone());
        }
        if !only_original_local_uses(ledger, binding, &BTreeSet::from([rhs.clone()]))
            || !only_original_local_uses(ledger, destination, &receivers)
        {
            return None;
        }
        Some(Self {
            construction: construction.clone(),
            assignment: assignment.clone(),
            destination,
            field: field.clone(),
        })
    }
    pub(crate) fn matches_transfer(
        &self,
        construction: &NamedArrayConstructionSourceV1,
        assignment: &super::ResolvedAssignmentSourceV1,
        destination: super::BindingRefV1,
        field: &str,
    ) -> bool {
        self.construction == *construction
            && self.assignment == *assignment
            && self.destination == destination
            && self.field.as_ref() == field
    }
}
