//! One source-origin dispatch for the shared FieldSet packet. No physical
//! operand, Home installation or Normal/Fault execution permission is lent here.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::QualifiedFieldWriteRhsV1;
use hakorune_mir_defs::CanonicalFieldRefV1;
use crate::mir::instruction::FieldSetModeV1;

#[derive(Debug, Clone)]
pub(crate) struct FieldWriteSourceLoanV1 {
    origin: FieldWriteSourceOriginV1,
}
#[derive(Debug, Clone)]
enum FieldWriteSourceOriginV1 {
    FormalBorrowed(BorrowedFieldWriteSourceLoanV1),
    Qualified {
        frame: Rc<CheckedFieldWriteFrameV1>,
        completion: Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
        rhs: Rc<QualifiedFieldWriteRhsV1>,
    },
}
impl FieldWriteSourceLoanV1 {
    pub(crate) fn frame(&self) -> &Rc<CheckedFieldWriteFrameV1> {
        match &self.origin { FieldWriteSourceOriginV1::FormalBorrowed(source) => source.frame(),
            FieldWriteSourceOriginV1::Qualified { frame, .. } => frame }
    }
    pub(crate) fn receiver(&self) -> Result<(BindingRefV1, crate::mir::resolved_semantics::OwnedExprSiteV1), String> {
        if let Some(formal) = self.borrowed() {
            let (binding, _, site) = formal.receiver();
            return Ok((binding, site.clone()));
        }
        if let Some(entry) = self.frame().entry() {
            return Ok((entry.receiver(), crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                self.frame().source().owner(), entry.original().write().receiver_site().clone())));
        }
        let local = self.frame().owned_local().ok_or_else(|| freeze("field-write/local-origin"))?;
        Ok((local.root(), crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            self.frame().source().owner(), local.original().write().receiver_site().clone())))
    }
    /// Canonical target comes from the sealed source origin, independently of
    /// receiver acquisition. This accessor grants no storage/execution lease.
    pub(crate) fn field(&self) -> Result<CanonicalFieldRefV1, String> {
        if let Some(formal) = self.borrowed() { return Ok(formal.field_source().field()); }
        if let Some(entry) = self.frame().entry() { return Ok(entry.field()); }
        self.frame().owned_local().map(|local| local.field())
            .ok_or_else(|| freeze("field-write/local-origin"))
    }
    pub(crate) fn field_name(&self) -> Result<&str, String> {
        if let Some(formal) = self.borrowed() { return Ok(formal.field_name()); }
        if let Some(entry) = self.frame().entry() { return Ok(entry.original().write().field()); }
        self.frame().owned_local().map(|local| local.original().write().field())
            .ok_or_else(|| freeze("field-write/local-origin"))
    }
    pub(crate) fn value_site(&self) -> crate::mir::resolved_semantics::OwnedExprSiteV1 {
        let source = self.frame().source();
        crate::mir::resolved_semantics::OwnedExprSiteV1::new(source.owner(), source.nodes()[0].site().clone())
    }
    pub(crate) fn borrowed(&self) -> Option<&BorrowedFieldWriteSourceLoanV1> {
        match &self.origin { FieldWriteSourceOriginV1::FormalBorrowed(source) => Some(source), _ => None }
    }
    pub(crate) fn qualified_rhs(&self) -> Option<&Rc<QualifiedFieldWriteRhsV1>> {
        match &self.origin { FieldWriteSourceOriginV1::Qualified { rhs, .. } => Some(rhs), _ => None }
    }
    fn original_source_error(&self, premise: &str) -> String {
        format!("{} premise={premise} owner={:?} statement={:?} field={:?}",
            freeze("owned-field-write/original-source"), self.frame().source().owner(),
            self.frame().source().root().statement(), self.field().ok())
    }
    /// Lend the checked mutation law from the same retained operand/source proof.
    /// Dynamic scalar mode transfers no candidate Home; owned replacement requires
    /// separate pre/postcommit ownership states and is not admitted here.
    pub(crate) fn require_current_execution(&self, context: crate::mir::normal_callable_semantic_package::CallValidationContextV1<'_>) -> Result<FieldSetModeV1, String> {
        let ledger = context.ledger();
        self.validate_source(ledger)?;
        if self.borrowed().is_some() { return Ok(FieldSetModeV1::ExactSlot); }
        self.require_signed_rhs_v1(context)?;
        let fail = || self.original_source_error("execution-storage");
        if let Some(entry) = self.frame().entry() {
            // validate_source replays the original all-writer ClosedNumeric loan
            // for an absent declaration. It remains Dynamic, never an I64 slot.
            return match entry.declared_type() {
                Some("i64") => Ok(FieldSetModeV1::ExactSlot),
                None => Ok(FieldSetModeV1::DynamicI64Replace),
                _ => Err(fail()),
            };
        }
        let local = self.frame().owned_local().ok_or_else(fail)?;
        let field = local.construction().allocation_only_field(local.original().write().field())
            .filter(|field| field.field() == local.field())
            .ok_or_else(fail)?;
        if field.storage() == crate::mir::function::TypedObjectFieldStorage::Dynamic {
            let rhs = self.qualified_rhs().ok_or_else(fail)?;
            if rhs.owned_array().is_some_and(|array| array.proves_empty_transfer(
                local.original().write().assignment(), local.root(), local.original().write().field())) {
                let contract = crate::mir::instruction::DynamicFieldSetContractV1::issue(
                    crate::mir::instruction::DynamicFieldWriteIntentV1::Initialize,
                    crate::mir::instruction::DynamicFieldValueTagV1::Array,
                    crate::mir::instruction::DynamicFieldResidenceV1::EmptyArrayHome,
                ).ok_or_else(fail)?;
                return Ok(FieldSetModeV1::DynamicValue(contract));
            }
            // The source and same-expression check above has already proved a
            // signed I64 RHS with no Home. Dynamic replacement retains that
            // value law and lets the runtime detach any prior residence.
            return Ok(FieldSetModeV1::DynamicI64Replace);
        }
        if local.construction().allocation_only_i64_field(local.original().write().field()) != Some(local.field()) {
            return Err(fail());
        }
        ledger.with_local_new_source_v1(local.acquisition(), local.root(), |source| {
            if source.allocation_execution.is_none_or(|lease| !Rc::ptr_eq(lease.source(), local.construction())) {
                return Err(fail());
            }
            Ok(())
        })?.ok_or_else(fail).map(|()| FieldSetModeV1::ExactSlot)
    }
    /// Discharge an Integer view from SAME source facts, never Class::I64 or
    /// the eventual MIR carrier. Wide unsigned/exact lanes remain obligations.
    fn require_signed_rhs_v1(&self, context: crate::mir::normal_callable_semantic_package::CallValidationContextV1<'_>) -> Result<(), String> {
        let ledger = context.ledger();
        use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
        use crate::mir::resolved_semantics::ResolvedLiteralSourceV1 as Literal;
        let fail = || self.original_source_error("signed-rhs");
        let rhs = self.qualified_rhs().ok_or_else(fail)?;
        for (index, node) in rhs.source().nodes().iter().enumerate() {
            if rhs.node_class(index) == Some(crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Identity) { continue; }
            match node.source() {
                Node::Literal(Literal::Integer(_)) => {},
                Node::Literal(Literal::TypedInteger { declared_type_name, .. }) if declared_type_name.as_ref() == "i64" => {},
                Node::Binary(_) => {},
                Node::Reference(_) => {
                    let source = ledger.borrowed_formal_source.as_ref().ok_or_else(fail)?.as_ref().map_err(Clone::clone)?;
                    if let Some((read, sum)) = rhs.signed_reference(index) {
                        let current = source.lend_current_scalar_sum_v1(read).ok_or_else(fail)?;
                        if !Rc::ptr_eq(sum, &current) || !sum.signed_only() { return Err(fail()); }
                    } else if let Some(crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1::FiniteEntry { read, exclusion: None })
                        = rhs.reference_source(index) {
                        let current = source.lend_source_value_sum_v1(read.requirement().formal().1).ok_or_else(fail)?;
                        if !read.signed_only() || !read.corroborates_finite_domain_v1(&current) { return Err(fail()); }
                    } else { return Err(fail()); }
                },
                Node::Shape(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. }) => {
                    let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(rhs.source().owner(), node.site().clone());
                    let required = ledger.source_field_requirement_v1(&site)?.ok_or_else(fail)?;
                    let reads = ledger.local_field_reads.borrow();
                    let read = reads.get(&site).ok_or_else(fail)?;
                    if !Rc::ptr_eq(required.plan(), rhs.source()) || !match read.source.declared_type_name() {
                        Some("i64") => true,
                        None => read.numeric_source.is_some(),
                        _ => false,
                    } { return Err(fail()); }
                    // Reuse the same all-writer/read proof; an absent annotation
                    // remains absent and grants no Dynamic execution lease.
                    ledger.recheck_local_field_source_v1(&site, read, context)?;
                },
                _ => return Err(fail()),
            }
        }
        Ok(())
    }
    pub(crate) fn validate_source(&self, ledger: &OrdinaryNewClaimLedgerV1) -> Result<(), String> {
        let FieldWriteSourceOriginV1::Qualified { frame, completion, rhs } = &self.origin else {
            return self.borrowed().expect("closed formal origin").validate_source(ledger);
        };
        let fail = || self.original_source_error("source-identity");
        if ledger.completion_source_for_owner_v1(frame.source().owner())
            .is_none_or(|current| !Rc::ptr_eq(current, completion)) { return Err(fail()); }
        if let Some(entry) = frame.entry() {
            if !Rc::ptr_eq(entry.source(), frame.source()) || entry.original().write().owner() != frame.source().owner() {
                return Err(fail());
            }
            entry.recheck(ledger)?;
        } else {
            let local = frame.owned_local().ok_or_else(fail)?;
            if !Rc::ptr_eq(local.source(), frame.source())
                || local.original().write().owner() != frame.source().owner()
                || local.construction().allocation_only_field(local.original().write().field()).map(|field| field.field()) != Some(local.field())
            { return Err(fail()); }
            ledger.field_write_claims.recheck_original_write_source_v1(local.original()).map_err(|_| fail())?;
            ledger.with_local_new_source_v1(local.acquisition(), local.root(), |source| {
                if source.construction != &Err(crate::mir::normal_callable_semantic_package::ConstructionUnavailableV1::InitializationContractMissing)
                    || local.construction().allocation_only_fields().is_none()
                    || source.object != local.field().object() || !source.construction_source.as_ref()
                    .is_ok_and(|original| Rc::ptr_eq(original, local.construction())) { return Err(fail()); }
                Ok(())
            })?.ok_or_else(fail)?;
        }
        let flow = completion.cleanup().root_flow().ok_or_else(fail)?;
        let mut plans = flow.expression_plans().iter().filter(|plan|
            plan.field_write_frame().is_some_and(|original| Rc::ptr_eq(original, frame)));
        let plan = plans.next().ok_or_else(fail)?;
        if plans.next().is_some() || plan.field_write_rhs().is_none_or(|original| !Rc::ptr_eq(original, rhs))
            || !rhs.is_consistent(frame.source()) { return Err(fail()); }
        Ok(())
    }
}
impl OrdinaryNewClaimLedgerV1 {
    /// The original Completion chooses the origin at its exact statement.
    pub(crate) fn field_write_source_at_v1(&self, owner: FunctionOwnerIdV1,
        statement: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    ) -> Result<Option<FieldWriteSourceLoanV1>, String> {
        let Some(completion) = self.completion_source_for_owner_v1(owner) else { return Ok(None) };
        let Some(flow) = completion.cleanup().root_flow() else { return Ok(None) };
        let mut plans = flow.expression_plans().iter().filter(|plan| plan.field_write_frame()
            .is_some_and(|frame| frame.source().root().statement() == statement));
        let Some(plan) = plans.next() else { return Ok(None) };
        if plans.next().is_some() { return Err(freeze("field-write/duplicate-original-plan")); }
        let frame = plan.field_write_frame().expect("selected original frame");
        let origin = if frame.is_borrowed() {
            let source = self.borrowed_field_write_source_at_v1(owner, statement)?
                .ok_or_else(|| freeze("field-write/formal-source-missing"))?;
            if !Rc::ptr_eq(source.frame(), frame) { return Err(freeze("field-write/formal-frame-drift")); }
            FieldWriteSourceOriginV1::FormalBorrowed(source)
        } else {
            let rhs = plan.field_write_rhs().ok_or_else(|| freeze("field-write/owned-rhs-missing"))?;
            FieldWriteSourceOriginV1::Qualified { frame: Rc::clone(frame),
                completion: Rc::clone(completion), rhs: Rc::clone(rhs) }
        };
        let source = FieldWriteSourceLoanV1 { origin };
        source.validate_source(self)?;
        Ok(Some(source))
    }
}
