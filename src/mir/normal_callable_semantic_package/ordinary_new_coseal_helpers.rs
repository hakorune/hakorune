//! Small source-side helpers for ordinary-`New` co-sealing.
//!
//! These helpers only translate already-issued observations or validate the
//! direct initializer shape. They do not issue keys, select a target, or
//! recover missing semantic facts.

use super::{
    OrdinaryNewCoSealIssueV1, OrdinaryNewConstructorDispositionV1,
    OrdinaryNewTrivialArgumentKindV1, OrdinaryNewTrivialArgumentV1,
};
use crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentUnavailableV1;
use crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1;
use crate::mir::resolved_semantics::home_new_prefix::TerminalReturnedSourceV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    SelectedNewArgumentKindV1, SelectedNewArgumentObservationV1,
};
use crate::mir::resolved_semantics::{OwnedExprSiteV1, SourcePathSegmentV1};

pub(super) fn convert_selected_new_arguments(
    observation: SelectedNewArgumentObservationV1,
) -> Result<Box<[OrdinaryNewTrivialArgumentV1]>, SelectedNewArgumentUnavailableV1> {
    let rows = observation.arguments().map_err(Clone::clone)?;
    rows.iter().map(|row| {
        let kind = match row.kind() {
            SelectedNewArgumentKindV1::Integer(value) => OrdinaryNewTrivialArgumentKindV1::Integer(*value),
            SelectedNewArgumentKindV1::Bool(value) => OrdinaryNewTrivialArgumentKindV1::Bool(*value),
            SelectedNewArgumentKindV1::Null => OrdinaryNewTrivialArgumentKindV1::Null,
            SelectedNewArgumentKindV1::Local { binding } => OrdinaryNewTrivialArgumentKindV1::Local { binding: *binding },
            SelectedNewArgumentKindV1::Handle { binding } => OrdinaryNewTrivialArgumentKindV1::Handle { binding: *binding },
            SelectedNewArgumentKindV1::BoundValue { binding } => OrdinaryNewTrivialArgumentKindV1::BoundValue { binding: *binding },
            SelectedNewArgumentKindV1::I64Field { object } => OrdinaryNewTrivialArgumentKindV1::I64Field { object: *object },
            SelectedNewArgumentKindV1::Expression { evaluation } => {
                if evaluation.source().owner() != observation.new_site().owner()
                    || !evaluation.reference_coverage_is_exact()
                    || !matches!(evaluation.source().root(),
                        crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::NewArgument { construction, ordinal, .. }
                            if construction.site() == observation.new_site().site() && *ordinal == row.ordinal()
                                && construction.arguments().get(*ordinal as usize) == Some(row.site()))
                    || evaluation.source().nodes().first().is_none_or(|node| node.site() != row.site()) {
                    return Err(SelectedNewArgumentUnavailableV1::SourceMismatch { new_site: observation.new_site().clone() });
                }
                OrdinaryNewTrivialArgumentKindV1::Expression { evaluation: std::rc::Rc::clone(evaluation) }
            }
        };
        Ok(OrdinaryNewTrivialArgumentV1::new(observation.new_site().owner(),
            observation.new_site().clone(), row.ordinal(), row.site().clone(), kind)
            .with_explicit_move(row.explicit_move()))
    }).collect()
}

pub(super) fn is_direct_local_initializer(segments: &[SourcePathSegmentV1]) -> bool {
    matches!(
        segments,
        [
            SourcePathSegmentV1::Body(_),
            SourcePathSegmentV1::Initializer(_)
        ]
    )
}

pub(super) fn no_birth_constructor_disposition(
    site: &OwnedExprSiteV1,
    class: &str,
    arity: usize,
) -> Result<OrdinaryNewConstructorDispositionV1, OrdinaryNewCoSealIssueV1> {
    if arity == 0 {
        return Ok(OrdinaryNewConstructorDispositionV1::NoBirthZero);
    }
    Err(OrdinaryNewCoSealIssueV1::BirthConstructorMissing {
        site: site.clone(),
        class: class.into(),
        arity,
    })
}

/// Retain the original source map, including canonical Static continuations
/// supplied by the same issuer. A child cannot borrow another owner's exit.
pub(super) fn retain_child_terminal_relations_v1(
    rows: std::collections::BTreeMap<
        crate::mir::resolved_semantics::SourceStmtSiteV1,
        TerminalRelationV1,
    >,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    has_map: bool,
    mut retain_source: impl FnMut(&TerminalRelationV1) -> Result<bool, OrdinaryNewCoSealIssueV1>,
) -> Result<
    std::collections::BTreeMap<
        crate::mir::resolved_semantics::SourceStmtSiteV1,
        TerminalRelationV1,
    >,
    OrdinaryNewCoSealIssueV1,
> {
    let mut retained = std::collections::BTreeMap::new();
    for (site, row) in rows {
        if row.owner() != owner || row.return_site() != &site {
            return Err(OrdinaryNewCoSealIssueV1::BatchLoan);
        }
        if retain_child_terminal_relation(&row, has_map) || retain_source(&row)? {
            retained.insert(site, row);
        }
    }
    Ok(retained)
}

pub(super) fn retain_child_terminal_relation(row: &TerminalRelationV1, has_map: bool) -> bool {
    has_map
        // The immutable Lexical argument arm is issued only after the source
        // terminal owner proves BorrowedActual; child completion must carry it
        // to the same final terminal emitter instead of dropping that loan.
        || matches!(row, TerminalRelationV1::Call(call)
            if call.arguments().iter().any(|argument| matches!(argument,
                crate::mir::resolved_semantics::home_new_prefix::TerminalCallArgumentV1::Lexical(_))))
        || matches!(
            row,
            TerminalRelationV1::Unit(_)
                | TerminalRelationV1::IntegerLiteral(_)
                | TerminalRelationV1::BoolLiteral(_)
                | TerminalRelationV1::Scalar(_)
                | TerminalRelationV1::MapGet(_)
        )
        // A `return new ...` relation is the co-sealed evidence that the
        // exact site owns a result claim, so it stays on the child contract
        // even without a map obligation. A `return null` relation is
        // retained explicitly for the same reason: dropping it would let a
        // mixed `new`/`null` callee masquerade as a uniform-object callee
        // downstream.
        || matches!(
            row,
            TerminalRelationV1::Value(value)
                if matches!(
                    value.returned(),
                    TerminalReturnedSourceV1::Construction(_)
                        | TerminalReturnedSourceV1::Home { .. }
                        | TerminalReturnedSourceV1::OwnedCall(_)
                        | TerminalReturnedSourceV1::BorrowedNullableLocal(_)
                        | TerminalReturnedSourceV1::StoredMapGet { .. }
                        | TerminalReturnedSourceV1::StoredMapInstalled(_)
                        | TerminalReturnedSourceV1::TaggedDynamic(_)
                        | TerminalReturnedSourceV1::NullLiteral
                        | TerminalReturnedSourceV1::TextResidence { .. }
                        | TerminalReturnedSourceV1::TextExpression(_)
                )
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn child_terminal_filter_retains_borrowed_call_without_widening_strict_call() {
        for borrowed in [false, true] {
            let parameter = if borrowed { "p" } else { "" };
            let argument = if borrowed { "7" } else { "" };
            let source = format!("box Transport {{ birth() {{ }} probe({parameter}): i64 {{ return 7 }} }}
                static box Main {{ main() {{ local recv = new Transport() return recv.probe({argument}) }} }}");
            let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&source).unwrap();
            let ledger = &package.ordinary_new_claim_ledger;
            let row = ledger
                .sole_terminal_relation_for_owner(ledger.root_owner().unwrap())
                .unwrap();
            assert!(matches!(row, TerminalRelationV1::Call(_)), "{row:?}");
            assert_eq!(retain_child_terminal_relation(row, false), borrowed);
            assert!(retain_child_terminal_relation(row, true));
        }
    }
}

/// The same source Text demand required by the final Birth actual verifier.
/// A declared String alone does not supply a connected borrowing contract.
pub(super) fn validate_selected_new_argument_contracts_v1(
    site: &OwnedExprSiteV1,
    rows: &Result<Box<[OrdinaryNewTrivialArgumentV1]>, SelectedNewArgumentUnavailableV1>,
    birth: Option<&super::super::BirthAbiHandoffV1>,
) -> Result<(), OrdinaryNewCoSealIssueV1> {
    use crate::mir::resolved_value_profile::{
        expression_class::ExpressionValueClassV1 as V,
        expression_projection::ExpressionLeafClassV1 as L,
    };
    let Ok(rows) = rows else {
        return Ok(());
    };
    for row in rows.iter() {
        let OrdinaryNewTrivialArgumentKindV1::Expression { evaluation } = row.kind() else {
            continue;
        };
        if evaluation.root_class() != L::Value(V::String) {
            continue;
        }
        let demand = birth
            .map(|birth| birth.text_formal_source_v1(row.ordinal()))
            .transpose()
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue,
            })?
            .flatten();
        if demand.is_none() {
            let issue = SelectedNewArgumentUnavailableV1::ArgumentNotTrivial {
                new_site: site.clone(),
                site: row.site().clone(),
                reason: Some("constructor Text formal contract is not connected"),
            };
            return Err(OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue: crate::mir::resolved_semantics::home_new_prefix::argument_source_unavailable_tag_v1(&issue),
            });
        }
    }
    Ok(())
}

impl super::OrdinaryNewClaimLedgerV1 {
    /// Constructor Text demands settle with the execution plans, not with
    /// their early co-seal copies. Check once before any physical take.
    pub(in crate::mir::normal_callable_semantic_package) fn validate_new_argument_contracts_v1(
        &self,
    ) -> Result<(), OrdinaryNewCoSealIssueV1> {
        let claims = self.claims.borrow();
        let results = self.result_claims.borrow();
        let births = self.birth_abi_handoffs.borrow();
        for core in claims
            .values()
            .map(|claim| &claim.core)
            .chain(results.values().map(|claim| &claim.core))
        {
            validate_selected_new_argument_contracts_v1(
                &core.site,
                &core.argument_rows,
                births.get(&core.site),
            )?;
        }
        Ok(())
    }
}

use super::*;

impl OrdinaryNewClaimLedgerV1 {
    /// Borrow the original SUM only for this original Birth formal and store.
    /// Its use census stays Unqualified; this is not a physical ABI grant.
    pub(in crate::mir::normal_callable_semantic_package) fn constructor_text_store_sum_v1(
        &self,
        row: &VerifiedInstanceConstructorSemanticRowV1,
        store: &crate::mir::normal_callable_semantic_package::instance_construction::ConstructionStoreV1,
    ) -> Option<Rc<super::SourceValueSumV1>> {
        let crate::mir::normal_callable_semantic_package::instance_construction::ConstructionStoreRhsV1::Parameter { site, binding, provided: None } = store.rhs() else { return None };
        let source = row.construction_source().as_ref().ok()?;
        let (constructor, owner) = source.constructor()?;
        if !constructor.same_as(row.source_id())
            || *owner != binding.owner()
            || source.object() != row.object()
            || !source
                .stores()
                .iter()
                .any(|original| std::ptr::eq(original, store))
        {
            return None;
        }
        let formal = row
            .formal_contracts()
            .iter()
            .enumerate()
            .find_map(|(index, formal)| {
                (formal.binding() == *binding && formal.ordinal() as usize == index)
                    .then_some(formal)
            })?;
        if !formal
            .unqualified_store_sites_v1(true)
            .is_some_and(|sites| sites.contains(site))
        {
            return None;
        }
        self.borrowed_formal_source
            .as_ref()?
            .as_ref()
            .ok()?
            .lend_source_value_sum_v1(*binding)
            .filter(|sum| sum.text_only())
    }

    /// Select the executable lane for one untyped Birth store from the
    /// retained formal source: a signed source sum initializes I64, while a
    /// complete same-field constructor relation initializes an exact object
    /// residence. No MIR carrier or field spelling selects the tag.
    pub(in crate::mir::normal_callable_semantic_package) fn constructor_dynamic_store_mode_v1(
        &self,
        row: &VerifiedInstanceConstructorSemanticRowV1,
        store: &crate::mir::normal_callable_semantic_package::instance_construction::ConstructionStoreV1,
    ) -> Option<crate::mir::instruction::FieldSetModeV1> {
        use crate::mir::instruction::{
            DynamicFieldResidenceV1 as Residence, DynamicFieldSetContractV1 as Contract,
            DynamicFieldValueTagV1 as Tag, DynamicFieldWriteIntentV1 as Intent,
            FieldSetModeV1 as Mode,
        };
        use crate::mir::normal_callable_semantic_package::instance_construction::ConstructionStoreRhsV1;
        use crate::mir::normal_callable_semantic_package::instance_constructor_semantic::BirthFormalUseCoverageV1 as Uses;

        let ConstructionStoreRhsV1::Parameter {
            site,
            binding,
            provided: None,
        } = store.rhs()
        else {
            return store_mode_declined(0);
        };
        let source = row.construction_source().as_ref().ok()?;
        let (constructor, owner) = source.constructor()?;
        if !constructor.same_as(row.source_id())
            || *owner != binding.owner()
            || source.object() != row.object()
            || !source
                .stores()
                .iter()
                .any(|original| std::ptr::eq(original, store))
            || source
                .stores()
                .iter()
                .filter(|other| other.field() == store.field())
                .count()
                != 1
        {
            return store_mode_declined(1);
        }
        let formal = row
            .formal_contracts()
            .iter()
            .find(|formal| formal.binding() == *binding)
            .or_else(|| {
                let _ = store_mode_declined::<()>(50);
                None
            })?;
        let Uses::UnqualifiedFieldStores { sites } = formal.uses() else {
            return None;
        };
        if formal.binding() != *binding
            || formal.ordinal() as usize >= row.source_arity() as usize
            || sites.len() != 1
            || !sites.contains(site)
        {
            return store_mode_declined(2);
        }

        let mut child = None;
        let mut nullable = false;
        let mut outer_sites = std::collections::BTreeSet::new();
        let relations: Vec<_> = self
            .callable_result_classes
            .construction_child_relations()
            .filter(|relation| {
                relation.is_unqualified_dynamic_v1()
                    && relation.field() == store.field()
                    && relation.constructor_source().same_as(constructor)
                    && relation.constructor_owner() == *owner
                    && relation.formal_binding() == Some(*binding)
                    && relation.formal_ordinal() == Some(formal.ordinal())
            })
            .collect();
        for relation in &relations {
            if relation.field().object() != row.object()
                || !outer_sites.insert(relation.outer().clone())
                || relation
                    .unqualified_field_source_v1(row.box_name())
                    .is_err()
                || relation.witnesses().is_empty()
            {
                return store_mode_declined(3);
            }
            for witness in relation.witnesses() {
                match witness.origin() {
                    ResultValueOriginV1::Null => {
                        nullable = true;
                    }
                    ResultValueOriginV1::Fresh(_) => {}
                    _ => return None,
                }
            }
            if child.is_some_and(|prior| prior != relation.child()) {
                return store_mode_declined(4);
            }
            child = Some(relation.child());
        }
        if let Some(child) = child {
            let tag = if nullable {
                Tag::NullableObject
            } else {
                Tag::Object
            };
            let contract = Contract::issue(Intent::Initialize, tag, Residence::ObjectHome(child))?;
            return Some(Mode::DynamicValue(contract));
        }

        self.borrowed_formal_source
            .as_ref()?
            .as_ref()
            .ok()?
            .lend_source_value_sum_v1(*binding)
            .filter(|sum| sum.signed_only())
            .map(|_| Mode::DYNAMIC_I64_INITIALIZE)
    }

    /// Existing source New claims already cloned initial eligibility. Update
    /// both families and every early ABI copy before any physical take,
    /// retaining all other core products.
    pub(in crate::mir::normal_callable_semantic_package) fn install_late_constructor_plan_v1(
        &mut self,
        row: &VerifiedInstanceConstructorSemanticRowV1,
        plan: &crate::mir::normal_callable_semantic_package::instance_construction::ConstructionPlanV1,
    ) -> Result<(), String> {
        if !self.local_commits.get_mut().is_empty() || !self.provider_births.get_mut().is_empty() {
            return Err("[freeze:contract][constructor-unit/late-after-take]".into());
        }
        let matches = |core: &OrdinaryNewClaimCoreV1| -> Result<bool, String> {
            if !core.box_source.same_source_as(row.box_source_v1())
                || &core.construction != row.construction()
            {
                return Ok(false);
            }
            let OrdinaryNewConstructorDispositionV1::Birth(birth) = &core.constructor else {
                return Err("[freeze:contract][constructor-unit/new-birth-missing]".into());
            };
            if !birth.source_id().same_as(row.source_id())
                || row.published_birth_key() != Some(birth.target_ref())
                || core.object != row.object()
                || core.arity != row.source_arity() as usize
                || core.class.as_ref() != row.box_name()
                || plan.object() != core.object
                || !matches!((&core.construction_source, row.construction_source()), (Ok(core), Ok(source))
                    if Rc::ptr_eq(core, source) && Rc::ptr_eq(core, plan.source_v1()))
            {
                return Err("[freeze:contract][constructor-unit/new-source-drift]".into());
            }
            Ok(true)
        };
        // Preflight both maps before either mutates.
        for claim in self.claims.get_mut().values() {
            matches(&claim.core)?;
        }
        for claim in self.result_claims.get_mut().values() {
            matches(&claim.core)?;
        }
        // Providers retain another early handoff in their destination-less
        // index. Prepare every replacement before mutating any claim or index.
        let handoffs = self
            .birth_abi_handoffs
            .get_mut()
            .iter()
            .filter(|(_, handoff)| handoff.source_id().same_as(row.source_id()))
            .map(|(site, handoff)| {
                Ok((
                    site.clone(),
                    handoff.with_late_construction_plan_v1(row, plan)?,
                ))
            })
            .collect::<Result<Vec<_>, String>>()?;
        let providers = self
            .birth_site_index
            .get_mut()
            .iter()
            .filter(|(_, (_, handoff))| handoff.source_id().same_as(row.source_id()))
            .map(|(site, (_, handoff))| {
                Ok((
                    site.clone(),
                    handoff.with_late_construction_plan_v1(row, plan)?,
                ))
            })
            .collect::<Result<Vec<_>, String>>()?;
        for claim in self.claims.get_mut().values_mut() {
            if matches(&claim.core)? {
                claim.core.construction = Ok(plan.clone());
            }
        }
        for claim in self.result_claims.get_mut().values_mut() {
            if matches(&claim.core)? {
                claim.core.construction = Ok(plan.clone());
            }
        }
        for (site, handoff) in handoffs {
            *self
                .birth_abi_handoffs
                .get_mut()
                .get_mut(&site)
                .expect("preflight retained ABI site") = handoff;
        }
        for (site, handoff) in providers {
            self.birth_site_index
                .get_mut()
                .get_mut(&site)
                .expect("preflight retained provider site")
                .1 = handoff;
        }
        Ok(())
    }
}

/// Default-off (builder debug flag): which numbered check declined a dynamic constructor store mode.
#[inline(never)]
fn store_mode_declined<T>(check: u32) -> Option<T> {
    if crate::config::env::builder_debug_enabled() {
        eprintln!("[constructor-store-mode] declined check={check}");
    }
    None
}
