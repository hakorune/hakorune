//! Borrowed receiver correspondence on the existing local-field-read rows.
//! A field class alone proves no lifetime: every edge needs the sealed owning
//! residence, and the physical receiver must be the original initializer local.
use super::*;
use crate::mir::resolved_semantics::{ResolvedInitializerRelationV1, home_new_prefix::{
    LocalCallObservationV1, LocalFieldReadResultV1,
}};

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct FieldAliasLocalV1 {
    relation: ResolvedInitializerRelationV1,
    source: Rc<LocalFieldReadSourceV1>,
    initializer: ValueId,
    value: ValueId,
    copy: Option<(BasicBlockId, MirInstruction)>,
}
impl FieldAliasLocalV1 {
    fn corroborate(&self, row: &LocalFieldRead) -> Result<(), String> {
        let LocalReadProgress::Emitted { instruction: MirInstruction::ObjectFieldGet { dst, .. }, source, .. } = &row.progress
            else { return Err(fault("alias/read-not-emitted")); };
        if !Rc::ptr_eq(&self.source, &row.source) || !Rc::ptr_eq(source, &row.source)
            || *dst != self.initializer || !matches!(row.source.result(), LocalFieldReadResultV1::Alias(_))
            || match &self.copy {
                Some((_, MirInstruction::Copy { dst, src })) => *dst != self.value || *src != self.initializer,
                None => self.value != self.initializer,
                _ => true,
            } { return Err(fault("alias/local-correspondence")); }
        Ok(())
    }
    pub(super) fn validate_function(&self, function: &MirFunction, row: &LocalFieldRead) -> Result<(), String> {
        self.corroborate(row)?;
        if let Some((_, copy)) = &self.copy {
            if function.blocks.values().flat_map(|block| block.all_instructions()).filter(|i| *i == copy).count() != 1 {
                return Err(fault("alias/finished-copy-missing-or-duplicate"));
            }
        }
        Ok(())
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// Hand back the SAME completed initializer to its existing read row. Other
    /// locals do nothing; this neither creates a Home nor emits a second read.
    pub(crate) fn record_field_alias_local_v1(
        &self, owner: FunctionOwnerIdV1, relation: &ResolvedInitializerRelationV1,
        initializer: ValueId, value: ValueId, copy: Option<&(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        let Some(site) = relation.initializer_site() else { return Ok(()) };
        let site = OwnedExprSiteV1::new(owner, site.clone());
        let mut rows = self.local_field_reads.borrow_mut();
        let Some(row) = rows.get_mut(&site) else { return Ok(()) };
        if !matches!(row.source.result(), LocalFieldReadResultV1::Alias(_)) { return Ok(()) }
        if relation.binding().owner() != owner || row.local.is_some() { return Err(fault("alias/local-owner-or-repeat")); }
        let local = FieldAliasLocalV1 { relation: relation.clone(), source: Rc::clone(&row.source),
            initializer, value, copy: copy.cloned() };
        local.corroborate(row)?;
        row.local = Some(local);
        Ok(())
    }

    /// Recheck the original alias dependencies against the current finalized
    /// function, not just the saved draft/finishing image.
    pub(crate) fn validate_field_alias_function_v1(
        &self, site: &OwnedExprSiteV1, function: &MirFunction,
    ) -> Result<(), String> {
        let call = self.handle_call_source(site)
            .or_else(|| self.nullable_call_source(site))
            .or_else(|| self.lexical_i64_call_source(site));
        let Some(path) = call.and_then(|call| call.field_alias())
            else { return Ok(()) };
        let rows = self.local_field_reads.borrow();
        for (_, request) in &path.edges {
            let row = rows.get(&request.site).ok_or_else(|| fault("alias/final-read-missing"))?;
            let local = row.local.as_ref().ok_or_else(|| fault("alias/final-local-missing"))?;
            local.validate_function(function, row)?;
            let LocalReadProgress::Emitted { instruction, .. } = &row.progress
                else { return Err(fault("alias/final-read-not-emitted")); };
            if function.blocks.values().flat_map(|block| block.all_instructions()).filter(|i| *i == instruction).count() != 1 {
                return Err(fault("alias/finished-read-missing-or-duplicate"));
            }
        }
        Ok(())
    }

    /// Read-only lender. The canonical source walk issued the passive path;
    /// each lookup below borrows the existing sealed source and physical rows.
    pub(crate) fn checked_field_alias_receiver_v1(
        &self, row: &LexicalInstanceCallDispositionRowV1, observation: &LocalCallObservationV1,
        receiver: Option<ValueId>,
    ) -> Result<bool, String> {
        let Some(path) = observation.field_alias() else { return Ok(false) };
        use crate::mir::resolved_semantics::home_new_prefix::FieldAliasRootSourceV1;
        let sealed = self.handle_call_source(row.call_site()) == Some(observation)
            || self.nullable_call_source(row.call_site()) == Some(observation)
            || self.lexical_i64_call_source(row.call_site()) == Some(observation);
        let result_matches = matches!(
            (row.result(), observation.result()),
            (Some(crate::mir::instruction::InvokeCallResultKind::Handle),
                crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Handle)
                | (Some(crate::mir::instruction::InvokeCallResultKind::NullableHandle),
                    crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Nullable)
                | (Some(crate::mir::instruction::InvokeCallResultKind::I64),
                    crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::I64)
                | (Some(crate::mir::instruction::InvokeCallResultKind::Bool),
                    crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Bool)
        );
        let root_is_live = match &path.root_source {
            FieldAliasRootSourceV1::OwnedHome { acquisition } => {
                acquisition.owner() == observation.owner()
                    && observation.prior_homes().contains(
                        &crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1::Stable(path.root),
                    )
            }
            FieldAliasRootSourceV1::InstanceEntry { batch_slot, identity } => {
                self.checked_field_alias_entry_root_v1(path.root, *batch_slot, identity, observation.owner())?
                    && !observation.prior_homes().contains(
                        &crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1::Stable(path.root),
                    )
            }
        };
        if !sealed
            || observation.site() != row.call_site() || !result_matches
            || (observation.local_binding().is_none() && !observation.is_expression_value())
            || !root_is_live
            || row.source_target().stored_receiver().is_some()
            || path.edges.last().map(|edge| edge.0) != Some(row.receiver_binding()?) {
            return Err(fault("alias/call-source-drift"));
        }
        let reads = self.local_field_reads.borrow();
        let mut object = match &path.root_source {
            FieldAliasRootSourceV1::OwnedHome { acquisition } => {
                let commits = self.local_commits.borrow();
                let root = commits.get(acquisition).ok_or_else(|| fault("alias/root-acquisition-missing"))?;
                if !root.installs_ordinary(path.root) || !root.end_available() {
                    return Err(fault("alias/root-home-drift"));
                }
                root.ordinary_object().ok_or_else(|| fault("alias/root-object"))?
            }
            FieldAliasRootSourceV1::InstanceEntry { .. } => {
                let (binding, request) = path.edges.first()
                    .ok_or_else(|| fault("alias/entry-root-empty"))?;
                let read = reads.get(&request.site)
                    .ok_or_else(|| fault("alias/original-read-missing"))?;
                if request.receiver != path.root || request.home != path.root
                    || read.receiver != path.root || read.home != path.root
                    || binding.owner() != observation.owner()
                {
                    return Err(fault("alias/entry-root-field-drift"));
                }
                read.source.field().object()
            }
        };
        let mut parent = path.root;
        for (ordinal, (binding, request)) in path.edges.iter().enumerate() {
            let read = reads.get(&request.site).ok_or_else(|| fault("alias/original-read-missing"))?;
            if binding.owner() != observation.owner() || request.site.owner() != observation.owner()
                || request.receiver != parent || request.home != path.root
                || read.receiver != request.receiver || read.home != request.home
                || read.receiver_site != request.receiver_site || read.source.field().object() != object {
                return Err(fault("alias/original-read-drift"));
            }
            let children = self.owned_field_children_for(object).flatten()
                .ok_or_else(|| fault("alias/owned-residence-missing"))?;
            let mut children = children.iter().filter(|child| child.field == read.source.field());
            let child = children.next().ok_or_else(|| fault("alias/owning-edge-missing"))?;
            if children.next().is_some() { return Err(fault("alias/owning-edge-duplicate")); }
            let super::super::OwnedFieldChildKindV1::Object(child) = child.kind
                else { return Err(fault("alias/object-child-required")); };
            let LocalFieldReadResultV1::Alias(class) = read.source.result()
                else { return Err(fault("alias/declared-object-required")); };
            let local = read.local.as_ref().ok_or_else(|| fault("alias/completed-local-missing"))?;
            local.corroborate(read)?;
            if local.relation.binding() != *binding || local.relation.initializer_site() != Some(request.site.site()) {
                return Err(fault("alias/initializer-relation-drift"));
            }
            if ordinal + 1 == path.edges.len() && (class.as_ref() != row.target().owner()
                || receiver.is_some_and(|value| value != local.value)) {
                return Err(fault("alias/call-receiver-drift"));
            }
            object = child;
            parent = *binding;
        }
        if path.edges.is_empty() { return Err(fault("alias/empty-path")); }
        Ok(true)
    }

    fn checked_field_alias_entry_root_v1(
        &self,
        root: BindingRefV1,
        batch_slot: u32,
        identity: &crate::parser::CallableDeclarationIdentityV1,
        owner: FunctionOwnerIdV1,
    ) -> Result<bool, String> {
        let Some((selected, signatures)) = &self.original_instance_input_issuance else {
            return Ok(false);
        };
        let Some(signature) = signatures.row(batch_slot) else {
            return Ok(false);
        };
        let selected_key = selected.key_for_batch_slot(batch_slot);
        let selected_identity = selected.identity_for_batch_slot(batch_slot);
        let mut receivers = signature.lanes().iter().filter(|lane| {
            lane.role()
                == crate::mir::normal_callable_semantic_package::PhysicalCallableLaneRoleV1::InstanceReceiver
        });
        let exact_receiver = receivers.next().is_some_and(|lane| lane.binding() == root)
            && receivers.next().is_none();
        Ok(exact_receiver
            && root.owner() == owner
            && signature.owner() == owner
            && signature.mode()
                == crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
            && signature.identity().same_as(identity)
            && selected_identity.is_some_and(|actual| actual.same_as(identity))
            && matches!(selected_key,
                Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
                    if key.namespace() == hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod)
        )
    }

    /// Passive source correspondence for an original expression's alias root.
    /// Physical initializer/Copy membership stays with the read/LocalSSA owners.
    pub(in crate::mir::normal_callable_semantic_package) fn field_alias_source_object_v1(
        &self, path: &crate::mir::resolved_semantics::home_new_prefix::FieldAliasPathV1,
        owner: FunctionOwnerIdV1, receiver: BindingRefV1,
        root_source: &crate::parser::ParserOrdinaryBoxSourceRowV1,
        root_object: hakorune_mir_defs::CanonicalObjectIdV1,
        edges: &[(hakorune_mir_defs::CanonicalFieldRefV1, hakorune_mir_defs::CanonicalObjectIdV1)],
    ) -> Result<hakorune_mir_defs::CanonicalObjectIdV1, String> {
        let crate::mir::resolved_semantics::home_new_prefix::FieldAliasRootSourceV1::OwnedHome { acquisition } = &path.root_source
            else { return Err(fault("alias/entry-root-expression-unsupported")); };
        let mut object = self.with_local_new_source_v1(acquisition, path.root, |source| {
            if !source.box_source.same_source_as(root_source) || source.object != root_object { return Err(fault("alias/root-source-drift")); }
            Ok(source.object)
        })?.ok_or_else(|| fault("alias/root-source-missing"))?;
        if edges.len() != path.edges.len() { return Err(fault("alias/source-edge-census")); }
        let mut parent = path.root;
        for ((binding, request), (field, child)) in path.edges.iter().zip(edges) {
            if binding.owner() != owner || request.site.owner() != owner || request.receiver != parent
                || request.home != path.root || request.path.is_some() || field.object() != object {
                return Err(fault("alias/original-source-path"));
            }
            let children = self.owned_field_children_for(object).flatten()
                .ok_or_else(|| fault("alias/owned-residence-missing"))?;
            let mut matching = children.iter().filter(|row| row.field == *field);
            let current = matching.next().ok_or_else(|| fault("alias/owning-edge-missing"))?;
            if matching.next().is_some() || current.kind != super::super::OwnedFieldChildKindV1::Object(*child) {
                return Err(fault("alias/owning-edge-drift"));
            }
            object = *child;
            parent = *binding;
        }
        if path.edges.is_empty() || parent != receiver { return Err(fault("alias/source-result-drift")); }
        Ok(object)
    }
}
