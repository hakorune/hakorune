//! Original field-alias path retained by the existing source-prefix state.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1;

/// Passive original initializer provenance; the package's existing field and
/// residence owners alone can prove that this path stays borrowed and live.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FieldAliasPathV1 {
    pub(crate) root: BindingRefV1,
    pub(crate) root_source: FieldAliasRootSourceV1,
    pub(crate) edges: Vec<(BindingRefV1, LocalFieldReadRequestV1)>,
}

#[derive(Debug, Clone)]
pub(crate) enum FieldAliasRootSourceV1 {
    OwnedHome { acquisition: OwnedExprSiteV1 },
    InstanceEntry {
        batch_slot: u32,
        identity: crate::parser::CallableDeclarationIdentityV1,
    },
}
impl PartialEq for FieldAliasRootSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::OwnedHome { acquisition: a },
                Self::OwnedHome { acquisition: b },
            ) => a == b,
            (
                Self::InstanceEntry { batch_slot: a, identity: x },
                Self::InstanceEntry { batch_slot: b, identity: y },
            ) => a == b && x.same_as(y),
            _ => false,
        }
    }
}
impl Eq for FieldAliasRootSourceV1 {}

impl PrefixLocalFlow<'_> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn field_read_receiver(
        &self,
        binding: BindingRefV1,
    ) -> Option<FieldReadReceiverV1> {
        match self.locals.get(&binding)? {
            StoredLocal::Home { .. } => Some(FieldReadReceiverV1::OwnedHome),
            StoredLocal::Handle(root) if self.field_root_is_live(*root) => {
                if *root == binding
                    && self.nonnull.contains(&binding)
                    && matches!(
                        self.input.function().binding(binding).map(|row| row.kind()),
                        Some(crate::mir::resolved_semantics::BindingKindV1::Parameter { .. })
                    )
                {
                    Some(FieldReadReceiverV1::GuardedFormal)
                } else {
                    Some(FieldReadReceiverV1::RootedHandle(*root))
                }
            }
            StoredLocal::FieldAlias { class, root, .. } if self.field_root_is_live(*root) => {
                Some(FieldReadReceiverV1::Alias {
                    class: class.clone(),
                    root: *root,
                })
            }
            // A received nullable carries no field provenance until the
            // branch join proves the surviving path non-null.
            StoredLocal::ReceivedNullable { .. } if self.nonnull.contains(&binding) => {
                Some(FieldReadReceiverV1::ReceivedNullable)
            }
            StoredLocal::ReceivedHandle { .. } | StoredLocal::ReceivedNullable { .. } => {
                let (acquisition, nullable) = self.received_call_acquisition(binding)?;
                Some(FieldReadReceiverV1::CheckedReceived(
                    crate::mir::resolved_semantics::home_new_prefix::ReceivedFieldReadRequestV1::Object {
                        acquisition: acquisition.clone(), nullable,
                    }))
            },
            StoredLocal::StoredMapGet(frame) if self.nonnull.contains(&binding)
                && self.is_self_rooted_handle(frame.source().receiver().receiver()) => {
                let exclusion = self.null_exclusions.get(&binding)?;
                // The request later supplies the exact field receiver use.
                Some(FieldReadReceiverV1::CheckedReceived(crate::mir::resolved_semantics::home_new_prefix::ReceivedFieldReadRequestV1::StoredMapGet {
                    frame: std::rc::Rc::clone(frame), exclusion: std::rc::Rc::clone(exclusion),
                }))
            },
            StoredLocal::ReceivedMixed(loan) if self.is_received_mixed_v1(binding) =>
                Some(FieldReadReceiverV1::CheckedReceived(crate::mir::resolved_semantics::home_new_prefix::ReceivedFieldReadRequestV1::Mixed(loan.clone()))),
            _ => None,
        }
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn install_field_alias(
        &mut self,
        binding: BindingRefV1,
        class: Box<str>,
        request: LocalFieldReadRequestV1,
    ) {
        let root = request.home;
        let mut borrow = match self.locals.get(&request.receiver) {
            Some(StoredLocal::Home { acquisition, .. }) if request.receiver == root => {
                Some(FieldAliasPathV1 {
                    root,
                    root_source: FieldAliasRootSourceV1::OwnedHome {
                        acquisition: acquisition.clone(),
                    },
                    edges: Vec::new(),
                })
            }
            Some(StoredLocal::FieldAlias { borrow, .. }) => borrow.clone(),
            Some(StoredLocal::Handle(entry))
                if request.receiver == *entry
                    && self.entry_receiver.as_ref().is_some_and(|row| {
                        row.binding == *entry
                            && row.binding == root
                            && row.owner == self.input.owner()
                    }) =>
            {
                self.entry_receiver.as_ref().map(|row| FieldAliasPathV1 {
                    root,
                    root_source: FieldAliasRootSourceV1::InstanceEntry {
                        batch_slot: row.batch_slot,
                        identity: row.identity.clone(),
                    },
                    edges: Vec::new(),
                })
            }
            _ => None,
        };
        if let Some(path) = &mut borrow {
            path.edges.push((binding, request));
        }
        self.store(
            binding,
            StoredLocal::FieldAlias {
                class,
                root,
                borrow,
            },
        );
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn field_alias_call_path(
        &self,
        receiver: BindingRefV1,
    ) -> Option<FieldAliasPathV1> {
        let StoredLocal::FieldAlias {
            borrow: Some(path), ..
        } = self.locals.get(&receiver)?
        else {
            return None;
        };
        let owned_root = match &path.root_source {
            FieldAliasRootSourceV1::OwnedHome { acquisition } => {
                self.home_acquisition(path.root) == Some(acquisition)
            }
            FieldAliasRootSourceV1::InstanceEntry { .. } => false,
        };
        if !owned_root
            || path.edges.last().map(|edge| edge.0) != Some(receiver)
            || self
                .input
                .function()
                .assignment_targets()
                .any(|(_, target)| {
                    matches!(target,
                crate::mir::resolved_semantics::ResolvedAssignmentTargetV1::BindingRebind(binding)
                if *binding == path.root || path.edges.iter().any(|edge| edge.0 == *binding))
                })
        {
            return None;
        }
        Some(path.clone())
    }

    /// Call-only borrowed aliases may be rooted in the exact instance-entry
    /// receiver. Expression field paths still require an owned root.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn received_field_alias_call_path_v1(
        &self,
        receiver: BindingRefV1,
    ) -> Option<FieldAliasPathV1> {
        let StoredLocal::FieldAlias {
            borrow: Some(path), ..
        } = self.locals.get(&receiver)?
        else {
            return None;
        };
        let live_root = match &path.root_source {
            FieldAliasRootSourceV1::OwnedHome { acquisition } => {
                self.home_acquisition(path.root) == Some(acquisition)
            }
            FieldAliasRootSourceV1::InstanceEntry { batch_slot, identity } => {
                self.entry_receiver.as_ref().is_some_and(|entry| {
                    entry.binding == path.root
                        && entry.owner == self.input.owner()
                        && entry.batch_slot == *batch_slot
                        && entry.identity.same_as(identity)
                        && self.is_self_rooted_handle(path.root)
                        && matches!(
                            self.input.function().binding(path.root).map(|row| row.kind()),
                            Some(crate::mir::resolved_semantics::BindingKindV1::Receiver)
                        )
                })
            }
        };
        if !live_root
            || path.edges.last().map(|edge| edge.0) != Some(receiver)
            || self.input.function().assignment_targets().any(|(_, target)| {
                matches!(target,
                    crate::mir::resolved_semantics::ResolvedAssignmentTargetV1::BindingRebind(binding)
                    if *binding == path.root || path.edges.iter().any(|edge| edge.0 == *binding))
            })
        {
            return None;
        }
        Some(path.clone())
    }

    /// Current declared-box aliases are borrowed receiver values, not live
    /// Homes. The stored class is installed only by the source-backed field
    /// read lane; require its original root to remain live before lending it
    /// to a received-object call.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn field_alias_receiver_class_v1(
        &self,
        receiver: BindingRefV1,
    ) -> Option<Box<str>> {
        if receiver.owner() != self.input.owner()
            || !matches!(
                self.input.function().binding(receiver).map(|row| row.kind()),
                Some(crate::mir::resolved_semantics::BindingKindV1::Local { .. })
            )
        {
            return None;
        }
        let StoredLocal::FieldAlias { class, root, .. } = self.locals.get(&receiver)? else {
            return None;
        };
        (root.owner() == self.input.owner() && self.field_root_is_live(*root))
            .then(|| class.clone())
    }

    pub(super) fn field_root_is_live(&self, root: BindingRefV1) -> bool {
        match self.locals.get(&root) {
            Some(StoredLocal::Home { .. }) => true,
            Some(StoredLocal::Handle(entry)) => *entry == root,
            _ => false,
        }
    }
}
