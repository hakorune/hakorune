//! Original field-alias path retained by the existing source-prefix state.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1;

/// Passive original initializer provenance; the package's existing field and
/// residence owners alone can prove that this path stays borrowed and live.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FieldAliasPathV1 {
    pub(crate) root: BindingRefV1,
    pub(crate) acquisition: OwnedExprSiteV1,
    pub(crate) edges: Vec<(BindingRefV1, LocalFieldReadRequestV1)>,
}

impl PrefixLocalFlow<'_> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn field_read_receiver(&self, binding: BindingRefV1) -> Option<FieldReadReceiverV1> {
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
            StoredLocal::ReceivedMixed(loan) if self.is_received_mixed_v1(binding) =>
                Some(FieldReadReceiverV1::CheckedReceived(crate::mir::resolved_semantics::home_new_prefix::ReceivedFieldReadRequestV1::Mixed(loan.clone()))),
            _ => None,
        }
    }


    pub(in crate::mir::resolved_semantics::home_new_prefix) fn install_field_alias(
        &mut self, binding: BindingRefV1, class: Box<str>, request: LocalFieldReadRequestV1,
    ) {
        let root = request.home;
        let mut borrow = match self.locals.get(&request.receiver) {
            Some(StoredLocal::Home { acquisition }) if request.receiver == root =>
                Some(FieldAliasPathV1 { root, acquisition: acquisition.clone(), edges: Vec::new() }),
            Some(StoredLocal::FieldAlias { borrow, .. }) => borrow.clone(),
            _ => None,
        };
        if let Some(path) = &mut borrow { path.edges.push((binding, request)); }
        self.store(binding, StoredLocal::FieldAlias { class, root, borrow });
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn field_alias_call_path(&self, receiver: BindingRefV1) -> Option<FieldAliasPathV1> {
        let StoredLocal::FieldAlias { borrow: Some(path), .. } = self.locals.get(&receiver)? else { return None };
        if self.home_acquisition(path.root) != Some(&path.acquisition)
            || path.edges.last().map(|edge| edge.0) != Some(receiver)
            || self.input.function().assignment_targets().any(|(_, target)| matches!(target,
                crate::mir::resolved_semantics::ResolvedAssignmentTargetV1::BindingRebind(binding)
                if *binding == path.root || path.edges.iter().any(|edge| edge.0 == *binding))) {
            return None;
        }
        Some(path.clone())
    }
    pub(super) fn field_root_is_live(&self, root: BindingRefV1) -> bool {
        match self.locals.get(&root) {
            Some(StoredLocal::Home { .. }) => true,
            Some(StoredLocal::Handle(entry)) => *entry == root,
            _ => false,
        }
    }

}
