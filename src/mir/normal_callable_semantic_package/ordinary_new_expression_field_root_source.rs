//! Passive original root correspondence; no construction or execution permission.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1;

#[derive(Debug, Clone)]
enum RootOriginV1 {
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
    pub(in crate::mir::normal_callable_semantic_package) fn issue(
        constructors: &VerifiedInstanceConstructorSemanticBatchV1,
        candidates: &[OrdinaryNewCandidate], coverage: &crate::parser::ParserOrdinaryBoxSourceCoverageV1,
        receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
        entry: Option<&crate::mir::resolved_semantics::VerifiedInstanceEntryHomeLoanV1>,
        received: Option<&super::super::field_reads::ReceivedFieldReadSourceV1>,
        request: &LocalFieldReadRequestV1, field: CanonicalFieldRefV1,
    ) -> Result<Rc<Self>, String> {
        let (box_source, origin) = if let Some(source) = received {
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
            let mut roots = candidates.iter().filter(|row| row.destination == path.root && row.site == path.acquisition);
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
                    (source, RootOriginV1::Entry { slot: entry.batch_slot(), identity: entry.identity().clone() })
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
    pub(in crate::mir::normal_callable_semantic_package) fn same_source(&self, other: &Self) -> bool {
        self.receiver == other.receiver && self.home == other.home && self.object == other.object
            && self.box_source.same_source_as(&other.box_source) && match (&self.origin, &other.origin) {
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
            RootOriginV1::Entry { slot, identity } => {
                let (selected, signatures) = ledger.original_instance_input_issuance.as_ref()
                    .ok_or("expression-path/root-entry-issuance")?;
                let signature = signatures.row(*slot).ok_or("expression-path/root-entry-signature")?;
                let key = selected.key_for_batch_slot(*slot);
                if !selected.identity_for_batch_slot(*slot).is_some_and(|actual| actual.same_as(identity))
                    || !matches!(key, Some(SelectedNormalCallableKeyV1::Cataloged(key))
                        if key.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod && key.owner() == self.class())
                    || signature.owner() != request.site.owner() || !signature.identity().same_as(identity)
                    || signature.mode() != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod {
                    return Err("expression-path/root-entry-drift".into());
                }
                let receivers: Vec<_> = signature.lanes().iter().filter(|lane| lane.role()
                    == super::super::super::physical_signature::PhysicalCallableLaneRoleV1::InstanceReceiver).collect();
                if receivers.len() != 1 || receivers[0].binding() != self.home {
                    return Err("expression-path/root-entry-receiver".into());
                }
            },
        }
        Ok(())
    }
}
