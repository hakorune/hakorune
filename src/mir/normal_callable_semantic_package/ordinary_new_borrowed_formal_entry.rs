//! Entry projection of the same lexical source/use/actual owner.
//! This borrow is prerequisite evidence, not permission to activate an ABI.
use super::borrowed_formal_actuals::PreparedBorrowedFormalActualV1;
use super::borrowed_formal_source::PreparedBorrowedFormalIngressV1;
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::normal_callable_semantic_package::{
    SelectedCallableLoweringInputRefV1, SelectedCallableSemanticRefV1,
};
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

#[derive(Debug)]
pub(crate) struct BorrowedOrdinaryEntrySourceRefV1<'ledger> {
    owner: FunctionOwnerIdV1,
    formals: Box<[(u32, BindingRefV1)]>,
    source: &'ledger PreparedBorrowedFormalIngressV1,
    definition: &'ledger super::borrowed_formal_uses::BorrowedFormalUsesDraftV1,
    incoming_rows: Box<[&'ledger super::borrowed_formal_uses::BorrowedIncomingCallDraftV1]>,
    incoming: Box<
        [(
            &'ledger OwnedExprSiteV1,
            &'ledger [PreparedBorrowedFormalActualV1],
        )],
    >,
}

impl BorrowedOrdinaryEntrySourceRefV1<'_> {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn formals(&self) -> &[(u32, BindingRefV1)] {
        &self.formals
    }
    pub(crate) fn origins(&self) -> &BTreeMap<BindingRefV1, BindingRefV1> {
        &self.definition.origins
    }
    pub(crate) fn incoming(&self) -> &[(&OwnedExprSiteV1, &[PreparedBorrowedFormalActualV1])] {
        &self.incoming
    }
    /// Checked-compare operand admissions this owner's draft proved:
    /// `(binding, formal, binary site)` rows under the operation owner's
    /// `Greater(NormalInteger, NormalInteger)` envelope. Admission evidence
    /// only; publication still counts the exact physical operand uses.
    pub(crate) fn compare_uses(
        &self,
    ) -> impl Iterator<Item = (BindingRefV1, BindingRefV1, &OwnedExprSiteV1)> {
        self.definition
            .uses
            .iter()
            .filter_map(|row| match &row.kind {
                super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::CompareOperand {
                    binary,
                    ..
                } => Some((row.binding, row.formal, binary)),
                _ => None,
            })
    }
    /// Exact Return source rows, tied to the SAME original Compare receipt.
    /// This lends source correspondence only; final dominance is checked independently.
    pub(crate) fn integer_return_uses(
        &self,
    ) -> Result<
        Vec<(
            BindingRefV1,
            BindingRefV1,
            crate::mir::resolved_semantics::SourceStmtSiteV1,
            OwnedExprSiteV1,
            OwnedExprSiteV1,
        )>,
        String,
    > {
        use super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1 as Use;
        let draft = self.definition;
        let mut returns = Vec::new();
        for row in &draft.uses {
            let Use::IntegerReturn { exit, guard } = &row.kind else {
                continue;
            };
            if draft.origins.get(&row.binding) != Some(&row.formal)
                || !draft.uses.iter().any(|compare| match &compare.kind {
                    Use::CompareOperand { binary, source } => {
                        compare.formal == row.formal
                            && guard.matches_compare(row.formal, binary, source)
                    }
                    _ => false,
                })
            {
                return Err("ordinary-new/borrowed-return/compare-source-identity".into());
            }
            returns.push((
                row.binding,
                row.formal,
                exit.clone(),
                row.site.clone(),
                guard.binary().clone(),
            ));
        }
        Ok(returns)
    }

    /// Dominated `+` operand admissions this owner's draft proved under the
    /// operation owner's `Add(NormalInteger, NormalInteger)` envelope:
    /// `(binding, formal, binary site)` rows guarded by an admitted compare.
    pub(crate) fn add_uses(
        &self,
    ) -> impl Iterator<Item = (BindingRefV1, BindingRefV1, &OwnedExprSiteV1)> {
        self.definition
            .uses
            .iter()
            .filter_map(|row| match &row.kind {
                super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::AddOperand {
                    binary,
                } => Some((row.binding, row.formal, binary)),
                _ => None,
            })
    }
    /// Original Mul source products and exact side; no legacy Add counts.
    /// The consumer must retain the source through materialization/finishing.
    pub(crate) fn mul_source_uses(
        &self,
    ) -> impl Iterator<
        Item = (
            &OwnedExprSiteV1,
            &Rc<super::borrowed_formal_uses::BorrowedMulSourceV1>,
            super::borrowed_formal_uses::BorrowedMulSideV1,
        ),
    > {
        self.definition
            .uses
            .iter()
            .filter_map(|row| match &row.kind {
                super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::MulOperand {
                    binary,
                    source,
                    side,
                } => Some((binary, source, *side)),
                _ => None,
            })
    }

    /// Dominated `.set` element-value admissions this owner's draft proved:
    /// `(binding, formal, call site)` rows on a proven `me.<ArrayBox>`
    /// receiver guarded by an admitted compare. Admission evidence only;
    /// publication still counts the exact physical operand uses.
    pub(crate) fn array_element_uses(
        &self,
    ) -> impl Iterator<Item = (BindingRefV1, BindingRefV1, &OwnedExprSiteV1)> {
        self.definition
            .uses
            .iter()
            .filter_map(|row| match &row.kind {
                super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::ArrayElementValue {
                    call,
                } => Some((row.binding, row.formal, call)),
                _ => None,
            })
    }
    /// Dominated `new`-argument admissions this owner's draft proved:
    /// `(binding, formal, new site, ordinal)` rows on the sole admitted
    /// argument position, guarded by an admitted compare of the same
    /// formal. Admission evidence only; publication still counts the exact
    /// physical operand uses.
    pub(crate) fn new_argument_uses(
        &self,
    ) -> impl Iterator<Item = (BindingRefV1, BindingRefV1, &OwnedExprSiteV1, u32)> {
        self.definition
            .uses
            .iter()
            .filter_map(|row| match &row.kind {
                super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::NewArgument {
                    site,
                    ordinal,
                } => Some((row.binding, row.formal, site, *ordinal)),
                _ => None,
            })
    }
    /// Admitted null-equality operand uses this owner's draft proved:
    /// `(binding, formal, binary site)` rows under the operation owner's
    /// `Equal(Dynamic, Null)` envelope. Admission evidence only;
    /// publication still counts the exact physical operand uses.
    pub(crate) fn null_compare_uses(
        &self,
    ) -> impl Iterator<Item = (BindingRefV1, BindingRefV1, &OwnedExprSiteV1)> {
        self.definition
            .uses
            .iter()
            .filter_map(|row| match &row.kind {
                super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::NullCompareOperand {
                    binary,
                } => Some((row.binding, row.formal, binary)),
                _ => None,
            })
    }
    /// Guarded `formal.field` receiver admissions this owner's draft
    /// proved: `(binding, formal, field-access site)` rows dominated by an
    /// admitted null compare of the same formal. Admission evidence only;
    /// publication still counts the exact physical operand uses.
    pub(crate) fn field_read_uses(
        &self,
    ) -> impl Iterator<Item = (BindingRefV1, BindingRefV1, &OwnedExprSiteV1)> {
        self.definition
            .uses
            .iter()
            .filter_map(|row| match &row.kind {
                super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::FieldReadOperand {
                    site,
                } => Some((row.binding, row.formal, site)),
                _ => None,
            })
    }
    /// The co-sealed class view for one of this owner's formals — `Some`
    /// only when every incoming actual agreed on one ordinary class.
    pub(crate) fn formal_object_view(
        &self,
        formal: BindingRefV1,
    ) -> Option<&super::borrowed_formal_source::BorrowedFormalObjectViewV1> {
        self.source.formal_object_view(formal)
    }
    /// Loan the original target, including its source owner and batch identity.
    pub(crate) fn incoming_targets(
        &self,
    ) -> impl Iterator<Item = Result<super::borrowed_formal_uses::BorrowedCallSourceLoanV1<'_>, String>>
    {
        self.incoming_rows
            .iter()
            .filter(move |row| row.callee == self.owner)
            .map(|row| {
                let loan = row.source.as_loan();
                if loan.call_site() != &row.call
                    || loan.callee_owner() != row.callee
                    || row.callee != self.owner
                {
                    return Err(freeze("borrowed-entry/incoming-source-identity"));
                }
                if let super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
                    &row.source
                {
                    if !original.is_qualified()
                        && !(original.is_current_owner_i64_source_v1()
                            && self.incoming.iter().any(|(site, actuals)| {
                                *site == &row.call && actuals.len() == 1
                            }))
                    {
                        original.require_qualified()?;
                    }
                    let retained = self
                        .source
                        .source_incoming
                        .static_observations()
                        .get(&row.call)
                        .ok_or_else(|| freeze("borrowed-entry/incoming-source-missing"))?
                        .as_ref()
                        .map_err(Clone::clone)?;
                    if !Rc::ptr_eq(retained, original) {
                        return Err(freeze("borrowed-entry/incoming-source-drift"));
                    }
                }
                Ok(row.source.as_loan())
            })
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// Finalization borrows the original source and recorded correspondence;
    /// it must not reconstruct a lowering input or issue new formal contracts.
    pub(crate) fn finalized_borrowed_ordinary_entry_source_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<BorrowedOrdinaryEntrySourceRefV1<'_>, String> {
        let values = self.borrowed_ordinary_entry_values_v1(owner);
        self.check_borrowed_ordinary_entry_values_v1(owner, &values)?;
        let values = values?;
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-entry/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let view = self.checked_entry_owner_view_v1(source, owner)?
            .ok_or_else(|| freeze("borrowed-entry/entry-owner"))?;
        let incoming = self.checked_borrowed_entry_incoming(source, owner)?;
        Ok(BorrowedOrdinaryEntrySourceRefV1 {
            owner,
            formals: values
                .iter()
                .map(|(ordinal, binding, _)| (*ordinal, *binding))
                .collect(),
            source,
            definition: view.definition,
            incoming_rows: view.incoming,
            incoming,
        })
    }

    /// Only the installed Ordinary source input can request this projection.
    /// Nonopaque and Dynamic callers do not demand unrelated pending errors.
    pub(crate) fn borrowed_ordinary_entry_source_v1(
        &self,
        input: &SelectedCallableLoweringInputRefV1<'_>,
    ) -> Result<Option<BorrowedOrdinaryEntrySourceRefV1<'_>>, String> {
        if !matches!(input.semantic(), SelectedCallableSemanticRefV1::Ordinary) {
            return Ok(None);
        }
        let crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key) = input.selected_key()
        else {
            return Ok(None);
        };
        let receiver_required = match key.namespace() {
            hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod => true,
            hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod => false,
            _ => return Ok(None),
        };
        let has_receiver = input.source().function().declaration_sites().any(|site| {
            matches!(
                site,
                crate::mir::resolved_semantics::SourceBindingSiteV1::Receiver
            )
        });
        if has_receiver != receiver_required {
            return Err(freeze("borrowed-entry/source-mode-shape"));
        }
        let parameters = input.parameter_contracts().collect::<Vec<_>>();
        if !parameters
            .iter()
            .any(|(_, _, kind)| kind.is_ordinary_borrowed_handle())
        {
            return Ok(None);
        }
        if !self
            .completion_index
            .get(&input.source().owner())
            .is_some_and(|row| {
                row.as_ref()
                    .is_ok_and(|completion| completion.owner() == input.source().owner())
            })
        {
            return Err(freeze("borrowed-entry/foreign-source-loan"));
        }
        self.borrowed_entry_source_for_contract(input.source().owner(), &parameters)
    }

    /// Original incoming namespace determines the entry's receiver law.
    /// This does not infer mode from the physical receiver's presence.
    pub(crate) fn borrowed_ordinary_entry_receiver_required_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<bool, String> {
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-entry/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let view = self.checked_entry_owner_view_v1(source, owner)?
            .ok_or_else(|| freeze("borrowed-entry/incoming-missing"))?;
        let mut modes = view.incoming.iter().map(|row| row.source.as_loan().declaration_mode());
        let mode = modes
            .next()
            .ok_or_else(|| freeze("borrowed-entry/incoming-missing"))?;
        if modes.any(|other| other != mode) {
            return Err(freeze("borrowed-entry/incoming-mode-drift"));
        }
        match mode {
            crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod => Ok(true),
            crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::StaticBoxMethod => Ok(false),
        }
    }

    /// Original final opaque entry and argument join for completed return acquisition.
    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_opaque_object_arguments_v1(
        &self,
        target: &LexicalInstanceCallSourceTargetV1,
        loan: &crate::mir::normal_callable_semantic_package::ObjectReturnCallQualificationV1,
        contract: &crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1,
    ) -> Result<
        Option<&[crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1]>,
        String,
    > {
        if contract.owner != target.callee_owner()
            || contract.batch_slot != target.target_batch_slot()
            || loan.call() != target.call_site()
            || loan.key() != target.target()
            || self
                .callable_result_classes
                .object_return_qualification(loan.value())
                .as_ref()
                != Some(loan)
            || !target
                .object_return_sources()
                .is_some_and(|rows| rows.contains(loan))
        {
            return Err(freeze("object-return/opaque-input-identity"));
        }
        self.checked_completed_opaque_object_target_arguments_v1(target, contract)
    }

    /// Shares the original whole-final-incoming input check without a caller result loan.
    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_opaque_object_target_arguments_v1(
        &self,
        target: &LexicalInstanceCallSourceTargetV1,
        contract: &crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1,
    ) -> Result<
        Option<&[crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1]>,
        String,
    > {
        if contract.owner != target.callee_owner()
            || contract.batch_slot != target.target_batch_slot()
        {
            return Err(freeze("object-return/opaque-input-identity"));
        }
        let Some(source) = &self.borrowed_formal_source else {
            return Ok(None);
        };
        let ingress = source.as_ref().map_err(Clone::clone)?;
        if !ingress.definitions.contains_key(&contract.owner) {
            return Ok(None);
        }
        let projected = ingress
            .source_incoming
            .project(&[contract.owner].into_iter().collect())
            .map_err(|issue| {
                format!("{}: {issue:?}", freeze("borrowed-formal/incoming-coverage"))
            })?;
        for call in &projected {
            if !ingress
                .incoming
                .iter()
                .any(|final_call| final_call.call == call.call)
            {
                return Ok(None);
            }
        }
        // Preserve the existing whole-final-incoming checker failure scope.
        let mut missing = false;
        for call in &ingress.incoming {
            match self.borrowed_formal_actuals.get(&call.call) {
                None => missing = true,
                Some(Err(issue)) => return Err(issue.clone()),
                Some(Ok(actuals)) => {
                    if actuals.require_executable_v1().is_err() {
                        missing = true;
                    }
                }
            }
        }
        if missing {
            return Ok(None);
        }
        let parameters: Vec<_> = contract
            .parameters
            .iter()
            .map(|row| (row.ordinal, row.binding, row.kind.clone()))
            .collect();
        if self
            .borrowed_entry_source_for_contract(contract.owner, &parameters)?
            .is_none()
        {
            return Ok(None);
        }
        let Some((call, arguments)) =
            super::borrowed_formal_actuals::lend_pending_borrowed_arguments_v1(
                source,
                &self.borrowed_formal_actuals,
                target.call_site(),
            )?
        else {
            return Ok(None);
        };
        if call.source.require_instance()? != target || call.callee != contract.owner {
            return Err(freeze("object-return/opaque-target"));
        }
        Ok(Some(arguments))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_opaque_receiver_argument_v1(
        &self,
        call: &OwnedExprSiteV1,
        ordinal: u32,
        kind: &crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentKindV1,
    ) -> Result<bool, String> {
        use super::borrowed_formal_actuals::BorrowedFormalActualSourceV1 as Source;
        use crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentKindV1 as Kind;
        let Some(actuals) = self.borrowed_formal_actuals.get(call) else {
            return Ok(false);
        };
        let actuals = actuals.as_ref().map_err(Clone::clone)?;
        actuals.require_executable_v1()?;
        let mut rows = actuals
            .opaque_actuals
            .iter()
            .filter(|row| row.ordinal == ordinal);
        let Some(row) = rows.next() else {
            return Ok(false);
        };
        if rows.next().is_some() {
            return Err(freeze("object-return/receiver-ordinal"));
        }
        Ok(match (&row.source, kind) {
            (Source::Integer(a), Kind::Integer(b)) => a == b,
            (Source::Bool(a), Kind::Bool(b)) => a == b,
            (Source::Null, Kind::Null) => true,
            (
                Source::ReceivedNullable { binding, .. }
                | Source::Scalar { binding, .. }
                | Source::TypedHome { binding, .. }
                | Source::EntryReceiver { binding, .. }
                | Source::DeclaredFormal { binding, .. }
                | Source::Forwarded { binding, .. },
                Kind::Local { binding: actual } | Kind::Handle { binding: actual },
            ) => binding == actual,
            _ => false,
        })
    }

    fn borrowed_entry_source_for_contract(
        &self,
        owner: FunctionOwnerIdV1,
        parameters: &[(u32, BindingRefV1, CallableParameterContractKindV1)],
    ) -> Result<Option<BorrowedOrdinaryEntrySourceRefV1<'_>>, String> {
        if !parameters
            .iter()
            .any(|(_, _, kind)| kind.is_ordinary_borrowed_handle())
        {
            return Ok(None);
        }
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-entry/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let Some(view) = self.checked_entry_owner_view_v1(source, owner)? else {
            // A source use outside the closed transport profile was excluded
            // before selection. This is not retry after a selected failure.
            return Ok(None);
        };
        let definition = view.definition;
        let mut formals = Vec::new();
        for (index, (ordinal, binding, kind)) in parameters.iter().enumerate() {
            if *ordinal as usize != index || binding.owner() != owner {
                return Err(freeze("borrowed-entry/formal-identity"));
            }
            if kind.is_ordinary_borrowed_handle() {
                if definition.origins.get(binding) != Some(binding) {
                    return Err(freeze("borrowed-entry/formal-origin"));
                }
                formals.push((*ordinal, *binding));
            }
        }
        let roots: BTreeSet<_> = definition.origins.values().copied().collect();
        if roots != formals.iter().map(|(_, binding)| *binding).collect() {
            return Err(freeze("borrowed-entry/formal-cardinality"));
        }
        let incoming = self.checked_borrowed_entry_incoming(source, owner)?;
        for call in &view.incoming {
            if call.arguments.len() != formals.len()
                || call.arguments.iter().zip(formals.iter()).any(
                    |((ordinal, _, formal), (expected, binding))| {
                        ordinal != expected || formal != binding
                    },
                )
            {
                return Err(freeze("borrowed-entry/incoming-formals"));
            }
        }
        Ok(Some(BorrowedOrdinaryEntrySourceRefV1 {
            owner,
            formals: formals.into_boxed_slice(),
            source,
            definition,
            incoming_rows: view.incoming,
            incoming,
        }))
    }
    /// Borrow the original actual rows only for a consumed, owned lexical row.
    pub(crate) fn borrowed_call_actuals_v1(
        &self,
        row: &super::LexicalInstanceCallDispositionRowV1,
    ) -> Result<Option<&[PreparedBorrowedFormalActualV1]>, String> {
        if !matches!(
            self.lexical_instance_calls.borrow().get(row.call_site()),
            Some(super::LexicalInstanceCallDispositionSlotV1::Taken)
        ) {
            return Err(freeze("borrowed-call/disposition-not-owned-and-taken"));
        }
        if row.source_target().has_object_source_requirement() {
            return row.checked_object_packet_actuals_v1(self).map(Some);
        }
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-call/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let mut incoming = source
            .incoming
            .iter()
            .filter(|call| &call.call == row.call_site());
        let Some(call) = incoming.next() else {
            return if source.definitions.contains_key(&row.callee_owner()) {
                Err(freeze("borrowed-call/incoming-missing"))
            } else {
                Ok(None)
            };
        };
        if incoming.next().is_some()
            || call.callee != row.callee_owner()
            || row.source_target() != call.source.require_instance()?
            || row.argument_sites().len() != row.target().arity() as usize
            || call
                .arguments
                .iter()
                .any(|(ordinal, site, _)| row.argument_sites().get(*ordinal as usize) != Some(site))
        {
            return Err(freeze("borrowed-call/disposition-identity"));
        }
        self.checked_borrowed_entry_incoming(source, call.callee)?;
        let proof = self
            .borrowed_i64_results
            .get(&call.callee)
            .ok_or_else(|| freeze("borrowed-call/result-source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        proof.require_source_sealed_v1()?;
        if !proof.contract_corroborated
            || proof.returns.is_empty()
            || proof.returns.iter().any(|site| site.owner() != call.callee)
        {
            return Err(freeze("borrowed-call/result-not-corroborated"));
        }
        // The source-proved result class names the one invoke result kind
        // this call may carry: scalar i64 transport or the checked
        // nullable-handle transport — never a third shape.
        let expected = match proof.class {
            super::borrowed_formal_result::BorrowedResultClassV1::I64 => {
                crate::mir::instruction::InvokeCallResultKind::I64
            }
            super::borrowed_formal_result::BorrowedResultClassV1::Nullable => {
                crate::mir::instruction::InvokeCallResultKind::NullableHandle
            }
        };
        if row.result() != Some(expected) {
            return Err(freeze("borrowed-call/result-mismatch"));
        }
        self.borrowed_formal_actuals
            .get(row.call_site())
            .ok_or_else(|| freeze("borrowed-entry/actuals-missing"))?
            .as_ref()
            .map(|rows| Some(rows.opaque_actuals.as_ref()))
            .map_err(Clone::clone)
    }

    pub(super) fn checked_borrowed_entry_incoming<'a>(
        &'a self,
        source: &'a PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> Result<Box<[(&'a OwnedExprSiteV1, &'a [PreparedBorrowedFormalActualV1])]>, String> {
        let mut seen = BTreeSet::new();
        let mut incoming = Vec::new();
        let owner_view = self.checked_entry_owner_view_v1(source, owner)?;
        let mut calls: Vec<_> = source.incoming.iter().collect();
        if let Some(cohort) = source.target_static.get(&owner) {
            calls.extend(cohort.incoming.iter());
        }
        for call in calls {
            let definition = source.definitions.get(&call.callee).or_else(|| {
                (call.callee == owner).then(|| owner_view.as_ref().map(|view| view.definition)).flatten()
            });
            let Some(definition) = definition else {
                return Err(freeze("borrowed-entry/incoming-identity"));
            };
            if !seen.insert(&call.call) {
                return Err(freeze("borrowed-entry/incoming-identity"));
            }
            let target_roots: BTreeSet<_> = definition.origins.values().copied().collect();
            let argument_roots: BTreeSet<_> = call
                .arguments
                .iter()
                .map(|(_, _, formal)| *formal)
                .collect();
            if target_roots != argument_roots
                || argument_roots.len() != call.arguments.len()
                || call.arguments.windows(2).any(|rows| rows[0].0 >= rows[1].0)
            {
                return Err(freeze("borrowed-entry/incoming-cardinality"));
            }
            let actuals = self
                .borrowed_formal_actuals
                .get(&call.call)
                .ok_or_else(|| freeze("borrowed-entry/actuals-missing"))?
                .as_ref()
                .map_err(Clone::clone)?;
            actuals.ordered_arguments_for_v1(call)?;
            let actuals = &actuals.opaque_actuals;
            for actual in actuals.iter() {
                if definition.origins.get(&actual.formal)
                    != Some(&actual.formal)
                {
                    return Err(freeze("borrowed-entry/actuals-identity"));
                }
                // A minted object view binds every incoming actual to one
                // agreed class — a sealed actual naming a different class
                // (or no class at all) is drift, never a second authority.
                if let Some(view) = source.object_views.get(&actual.formal) {
                    let agrees = match &actual.source {
                        super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::ReceivedNullable { class, .. }
                        | super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::TypedHome { class, .. }
                        | super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::EntryReceiver { class, .. }
                        | super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::DeclaredFormal { class, .. } => {
                            class.as_ref() == view.class()
                        }
                        super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::Null => true,
                        super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::Forwarded { formal, .. } => {
                            source
                                .object_views
                                .get(formal)
                                .is_some_and(|origin| origin.class() == view.class())
                        }
                        _ => false,
                    };
                    if !agrees {
                        return Err(freeze("borrowed-entry/object-view-drift"));
                    }
                }
            }
            if call.callee == owner {
                incoming.push((&call.call, actuals.as_ref()));
            }
        }
        if incoming.is_empty() {
            return Err(freeze("borrowed-entry/incoming-missing"));
        }
        Ok(incoming.into_boxed_slice())
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_formal_entry_tests.rs"]
mod tests;

#[path = "ordinary_new_borrowed_formal_entry_values.rs"]
mod entry_values;
#[path = "ordinary_new_borrowed_formal_entry_owner_view.rs"]
mod owner_view;

pub(in crate::mir) use entry_values::BorrowedCompareCarrierOperandLoanV1;
pub(crate) use entry_values::BorrowedCompareIntegerLiteralLoanV1;
pub(in crate::mir) use entry_values::BorrowedCompareIntegerLiteralMaterializationV1;
pub(in crate::mir) use entry_values::BorrowedCompareMaterializationV1;
pub(crate) use entry_values::BorrowedCompareSourceLoanV1;
pub(in crate::mir::normal_callable_semantic_package) use entry_values::BorrowedOrdinaryEntryPhysicalV1;
pub(in crate::mir) use entry_values::{BorrowedMulMaterializationV1, BorrowedMulSourceLoanV1};

#[cfg(test)]
#[path = "ordinary_new_borrowed_incoming_kind_tests.rs"]
mod incoming_kind_tests;

#[path = "ordinary_new_borrowed_static_packet_entry.rs"]
mod static_packet_entry;
