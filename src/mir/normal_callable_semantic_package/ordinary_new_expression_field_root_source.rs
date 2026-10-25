//! Passive original root correspondence; no construction or execution permission.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1;

#[derive(Debug, Clone)]
enum RootOriginV1 {
    StoredMapGet { frame: Rc<crate::mir::resolved_semantics::home_new_prefix::MapFieldGetLocalV1>,
        exclusion: Rc<crate::mir::resolved_semantics::home_new_prefix::SourceNullExclusionV1> },
    New(Rc<super::super::LocalNewRootSourceV1>),
    Received(super::super::field_reads::ReceivedFieldReadSourceV1),
    Entry { slot: u32, identity: crate::parser::CallableDeclarationIdentityV1 },
    Alias {
        path: Box<crate::mir::resolved_semantics::home_new_prefix::FieldAliasPathV1>,
        root_box_source: crate::parser::ParserOrdinaryBoxSourceRowV1,
        root_object: CanonicalObjectIdV1,
        edges: Box<[(CanonicalFieldRefV1, CanonicalObjectIdV1)]>,
    },
}
#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package) struct ExpressionFieldRootSourceV1 {
    receiver: BindingRefV1,
    home: BindingRefV1,
    box_source: crate::parser::ParserOrdinaryBoxSourceRowV1,
    object: CanonicalObjectIdV1,
    origin: RootOriginV1,
}
impl ExpressionFieldRootSourceV1 {
    /// Share the original entry receiver owner with reads and writes. This
    /// lends declaration identity only, never a storage or value-kind lease.
    pub(in crate::mir::normal_callable_semantic_package) fn issue_entry_v1(
        constructors: &VerifiedInstanceConstructorSemanticBatchV1,
        receiver: (BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1),
        entry: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeLoanV1,
        owner: FunctionOwnerIdV1, receiver_read: BindingRefV1, field: CanonicalFieldRefV1,
    ) -> Result<Rc<Self>, String> {
        let (binding, box_source) = receiver;
        if entry.receiver() != binding || entry.owner() != owner || binding.owner() != owner || receiver_read.owner() != owner {
            return Err("expression-path/entry-root-drift".into());
        }
        let (object, _) = constructors.destruction_for(box_source).map_err(|_| "expression-path/root-object")?;
        if field.object() != object { return Err("expression-path/root-field-object".into()); }
        Ok(Rc::new(Self { receiver: receiver_read, home: binding, box_source: box_source.clone(), object,
            origin: RootOriginV1::Entry { slot: entry.batch_slot(), identity: entry.identity().clone() } }))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn entry_receiver_v1(&self) -> Option<BindingRefV1> {
        (matches!(self.origin, RootOriginV1::Entry { .. }) && self.receiver == self.home).then_some(self.home)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn recheck_entry_v1(
        &self, ledger: &OrdinaryNewClaimLedgerV1, owner: FunctionOwnerIdV1,
    ) -> Result<(), String> {
        let RootOriginV1::Entry { slot, identity } = &self.origin else {
            return Err("expression-path/root-entry-drift".into());
        };
        let (selected, signatures) = ledger.original_instance_input_issuance.as_ref()
            .ok_or("expression-path/root-entry-issuance")?;
        let signature = signatures.row(*slot).ok_or("expression-path/root-entry-signature")?;
        let key = selected.key_for_batch_slot(*slot);
        if !selected.identity_for_batch_slot(*slot).is_some_and(|actual| actual.same_as(identity))
            || !matches!(key, Some(SelectedNormalCallableKeyV1::Cataloged(key))
                if key.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod && key.owner() == self.class())
            || signature.owner() != owner || !signature.identity().same_as(identity)
            || signature.mode() != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod {
            return Err("expression-path/root-entry-drift".into());
        }
        let mut receivers = signature.lanes().iter().filter(|lane| lane.role()
            == super::super::super::physical_signature::PhysicalCallableLaneRoleV1::InstanceReceiver);
        if receivers.next().is_none_or(|lane| lane.binding() != self.home) || receivers.next().is_some() {
            return Err("expression-path/root-entry-receiver".into());
        }
        Ok(())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn checked_entry_value_v1(
        &self, ledger: &OrdinaryNewClaimLedgerV1, owner: FunctionOwnerIdV1, function: &crate::mir::MirFunction,
    ) -> Result<(usize, crate::mir::ValueId), String> {
        self.recheck_entry_v1(ledger, owner)?;
        let RootOriginV1::Entry { slot, .. } = self.origin else { unreachable!("checked entry root") };
        let signature = ledger.original_instance_input_issuance.as_ref().ok_or("expression-path/root-entry-issuance")?
            .1.row(slot).ok_or("expression-path/root-entry-signature")?;
        let lane = signature.lanes().iter().find(|lane| lane.role()
            == super::super::super::physical_signature::PhysicalCallableLaneRoleV1::InstanceReceiver)
            .ok_or("expression-path/root-entry-receiver")?;
        let index = lane.index() as usize;
        let value = *function.params.get(index).ok_or("expression-path/root-entry-receiver")?;
        if function.metadata.physical_param_carriers.as_ref().and_then(|rows| rows.get(index))
            != Some(&crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::ExistingCallableI64) {
            return Err("expression-path/root-entry-receiver".into());
        }
        Ok((index, value))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn issue(
        constructors: &VerifiedInstanceConstructorSemanticBatchV1,
        candidates: &[OrdinaryNewCandidate], coverage: &crate::parser::ParserOrdinaryBoxSourceCoverageV1,
        receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
        entry: Option<&crate::mir::resolved_semantics::VerifiedInstanceEntryHomeLoanV1>,
        received: Option<&super::super::field_reads::ReceivedFieldReadSourceV1>,
        request: &LocalFieldReadRequestV1, field: CanonicalFieldRefV1,
    ) -> Result<Rc<Self>, String> {
        let (box_source, origin) = if let Some(crate::mir::resolved_semantics::home_new_prefix::ReceivedFieldReadRequestV1::StoredMapGet { frame, exclusion }) = &request.received {
            if received.is_some() || request.receiver != frame.binding() || request.home != frame.binding()
                || request.nullable || request.formal || request.alias_class.is_some() {
                return Err("expression-path/received-root-kind".into());
            }
            let storage = frame.source().payload_source_v1();
            let source = coverage.row_for(storage.payload_class_v1()).map_err(|_| "expression-path/root-object")?
                .ok_or("expression-path/root-object")?;
            if storage.payload_object_v1() != field.object() { return Err("expression-path/root-field-object".into()); }
            (source, RootOriginV1::StoredMapGet { frame: Rc::clone(frame), exclusion: Rc::clone(exclusion) })
        } else if let Some(source) = received {
            if source.object_result_kind_v1() != Some(crate::mir::instruction::InvokeCallResultKind::Handle)
                || request.receiver != request.home || request.received.as_ref() != Some(source.request()) {
                return Err("expression-path/received-root-kind".into());
            }
            let box_source = coverage.row_for(source.class()).map_err(|_| "expression-path/received-root-class")?
                .ok_or("expression-path/received-root-class")?;
            (box_source, RootOriginV1::Received(source.clone()))
        } else if let Some(class) = request.alias_class.as_deref() {
            let path = request.alias_path.as_ref().ok_or("expression-path/alias-root-missing")?;
            if path.root != request.home || path.edges.last().map(|edge| edge.0) != Some(request.receiver) {
                return Err("expression-path/alias-root-drift".into());
            }
            let crate::mir::resolved_semantics::home_new_prefix::FieldAliasRootSourceV1::OwnedHome { acquisition } = &path.root_source
                else { return Err("expression-path/entry-alias-root-unsupported".into()); };
            let mut roots = candidates.iter().filter(|row| row.destination == path.root && row.site == *acquisition);
            let candidate = roots.next().ok_or("expression-path/alias-acquisition-missing")?;
            if roots.next().is_some() { return Err("expression-path/alias-acquisition-ambiguous".into()); }
            let mut parent_source = &candidate.box_source;
            let mut edges = Vec::new();
            for (_, request) in &path.edges {
                let (field, declared) = source_declared_field(constructors, parent_source, &request.site, &request.field)
                    .map_err(|_| "expression-path/alias-declaration")?.ok_or("expression-path/alias-declaration")?;
                parent_source = coverage.row_for(declared.as_deref().ok_or("expression-path/alias-declared-class")?)
                    .map_err(|_| "expression-path/alias-class")?.ok_or("expression-path/alias-class")?;
                let (child, _) = constructors.destruction_for(parent_source).map_err(|_| "expression-path/alias-child")?;
                edges.push((field, child));
            }
            if parent_source.name() != class { return Err("expression-path/alias-result-class".into()); }
            (parent_source, RootOriginV1::Alias { path: path.clone(), root_box_source: candidate.box_source.clone(),
                root_object: candidate.object, edges: edges.into_boxed_slice() })
        } else {
            let mut rows = candidates.iter().filter(|row| row.destination == request.home);
            match (rows.next(), rows.next()) {
                (Some(row), None) => (&row.box_source, RootOriginV1::New(super::super::LocalNewRootSourceV1::from_candidate(row))),
                (None, None) => {
                    let (binding, source) = receiver.ok_or("expression-path/entry-root-missing")?;
                    let entry = entry.ok_or("expression-path/entry-loan-missing")?;
                    if binding != request.home || entry.receiver() != binding || entry.owner() != request.site.owner() {
                        return Err("expression-path/entry-root-drift".into());
                    }
                    return Self::issue_entry_v1(constructors, (binding, source), entry,
                        request.site.owner(), request.receiver, field);
                },
                _ => return Err("expression-path/root-ambiguous".into()),
            }
        };
        let (object, _) = constructors.destruction_for(box_source).map_err(|_| "expression-path/root-object")?;
        if field.object() != object { return Err("expression-path/root-field-object".into()); }
        Ok(Rc::new(Self { receiver: request.receiver, home: request.home,
            box_source: box_source.clone(), object, origin }))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn class(&self) -> &str { self.box_source.name() }
    pub(in crate::mir::normal_callable_semantic_package) fn received_source_v1(&self)
        -> Option<&super::super::field_reads::ReceivedFieldReadSourceV1> {
        match &self.origin { RootOriginV1::Received(source) => Some(source), _ => None }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn validate_read_v1(
        &self, read: &crate::mir::builder::ExactLexicalReadV1, context: CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        // Passive correspondence grants no physical nonnull permission. Every
        // executable purpose must also validate its same Normal/nonNull Branch.
        if let RootOriginV1::StoredMapGet { frame, exclusion } = &self.origin {
            frame.source().recheck_v1(context.ledger())?;
            if self.receiver != frame.binding() || self.home != frame.binding()
                || exclusion.root_binding() != frame.binding() {
                return Err("expression-path/root-source-drift".into());
            }
            read.loan_local_initializer_v1(frame.binding())?;
        }
        if let RootOriginV1::Received(source) = &self.origin {
            let ledger = context.ledger();
            context.with_taken_packet_v1(source.target().call_site(), |packet| {
                let row = packet.original_row()?;
                if row.source_target() != source.target() { return Err("expression-path/received-packet".into()); }
                let original = ledger.borrowed_object_call_packet_v1(row)?;
                if original.result() != crate::mir::instruction::InvokeCallResultKind::Handle {
                    return Err("expression-path/received-result".into());
                }
                let value = packet.value_in_context_v1(self.home.owner(), original.arguments(), context)?;
                read.loan_forwarded_copies(self.home, value)?;
                Ok(())
            })?;
        }
        Ok(())
    }
    /// Original source loan only; execution still requires the Bool owner's
    /// independent same-Normal/nonNull-Branch check in every physical purpose.
    pub(in crate::mir::normal_callable_semantic_package) fn stored_get_v1(&self) -> Option<(
        &Rc<crate::mir::resolved_semantics::home_new_prefix::MapFieldGetLocalV1>,
        &Rc<crate::mir::resolved_semantics::home_new_prefix::SourceNullExclusionV1>,
    )> {
        match &self.origin { RootOriginV1::StoredMapGet { frame, exclusion } => Some((frame, exclusion)), _ => None }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn same_source(&self, other: &Self) -> bool {
        self.receiver == other.receiver && self.home == other.home && self.object == other.object
            && self.box_source.same_source_as(&other.box_source) && match (&self.origin, &other.origin) {
                (RootOriginV1::StoredMapGet { frame: a, exclusion: x }, RootOriginV1::StoredMapGet { frame: b, exclusion: y }) => Rc::ptr_eq(a, b) && x == y,
                (RootOriginV1::New(a), RootOriginV1::New(b)) => a == b,
                (RootOriginV1::Received(a), RootOriginV1::Received(b)) => a == b,
                (RootOriginV1::Entry { slot: a, identity: x }, RootOriginV1::Entry { slot: b, identity: y }) => a == b && x.same_as(y),
                (RootOriginV1::Alias { path: a, root_box_source: x, root_object: u, edges: i },
                 RootOriginV1::Alias { path: b, root_box_source: y, root_object: v, edges: j }) =>
                    a == b && x.same_source_as(y) && u == v && i == j,
                _ => false,
            }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn recheck(
        &self, ledger: &OrdinaryNewClaimLedgerV1, request: &LocalFieldReadRequestV1, field: CanonicalFieldRefV1,
        context: CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        if request.receiver != self.receiver || request.home != self.home || field.object() != self.object {
            return Err("expression-path/root-source-drift".into());
        }
        match &self.origin {
            RootOriginV1::StoredMapGet { frame, exclusion } => {
                let retained = ledger.stored_map_get_local_source_v1(request.site.owner(), frame.statement().node())?
                    .ok_or("expression-path/root-source-drift")?;
                if !Rc::ptr_eq(&retained, frame) || !matches!(&request.received,
                    Some(crate::mir::resolved_semantics::home_new_prefix::ReceivedFieldReadRequestV1::StoredMapGet { frame: actual, exclusion: proof })
                        if Rc::ptr_eq(actual, frame) && proof == exclusion) {
                    return Err("expression-path/root-source-drift".into());
                }
            },
            RootOriginV1::Received(source) => {
                let ingress = ledger.borrowed_formal_source.as_ref().ok_or("expression-path/received-ingress")?
                    .as_ref().map_err(Clone::clone)?;
                context.with_current_instance_source_v1(source.target().call_site(), |target|
                    source.recheck_issuing_target_v1(&ledger.callable_result_classes, ingress, target, request))?;
            },
            RootOriginV1::New(source) => {
                if source.binding() != self.home || !source.matches_definition(&self.box_source, self.object) {
                    return Err("expression-path/root-new-drift".into());
                }
                source.recheck(ledger)?;
            },
            RootOriginV1::Alias { path, root_box_source, root_object, edges } => {
                if request.alias_path.as_ref() != Some(path) || request.alias_class.as_deref() != Some(self.class())
                    || ledger.field_alias_source_object_v1(path, request.site.owner(), self.receiver, root_box_source, *root_object, edges)? != self.object {
                    return Err("expression-path/root-alias-drift".into());
                }
            },
            RootOriginV1::Entry { .. } => self.recheck_entry_v1(ledger, request.site.owner())?,
        }
        Ok(())
    }
}
