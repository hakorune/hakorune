//! Loop-internal source declaration rows derived at admission.
//!
//! A `local` declared inside a loop body (e.g. the nested profile's `j`) is
//! not covered by the entry input set: its declaration must still be adopted
//! by the canonical SSA identity before the first operation that touches it
//! can claim an assignment or satisfy a read. This module derives exactly
//! those declarations from the co-sealed operation/effect product — it never
//! inspects names, never allocates physical identities, and never issues a
//! source claim.
//!
//! Two adoption modes exist because a Recipe models an initializing `local`
//! in one of two ways:
//!
//! - `Activate`: the local's first physical touch is a `WriteBinding`
//!   operation. The dispatcher activates the declaration without a value
//!   immediately before that item emits; the exact write path then
//!   initializes the binding.
//! - `PublishWithEntry`: the local is a carrier binding whose entry value is
//!   a Recipe value (e.g. `local j = 0` feeding the inner loop's carrier).
//!   The dispatcher publishes the declaration with that value's materialized
//!   physical result immediately after the producing item emits.

use crate::mir::resolved_semantics::{
    BindingOriginV1, BindingRefV1, SourceBindingSiteV1, SourceStmtSiteV1,
};

use super::ids::LoopItemKeyV1;
use super::operation_effect::VerifiedLoopOperationEffectProductV1;
use super::schema::{LoopOperationV1, LoopRecipeItemV1};
use super::source_bound_core::{LoopBindingEffectAnchorV1, LoopBindingEffectRoleV1};
use super::LoopValueKeyV1;

/// How the dispatcher adopts one loop-internal declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopInternalDeclarationModeV1 {
    /// Activate without a value before the producing write emits.
    Activate,
    /// Publish with the carrier entry value after the producing item emits.
    PublishWithEntry { value: LoopValueKeyV1 },
}

/// One loop-internal `local` declaration and the Recipe item at which the
/// dispatcher must adopt it around emission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedLoopInternalDeclarationV1 {
    binding: BindingRefV1,
    site: SourceBindingSiteV1,
    producing_item: LoopItemKeyV1,
    mode: LoopInternalDeclarationModeV1,
}

impl VerifiedLoopInternalDeclarationV1 {
    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn site(&self) -> &SourceBindingSiteV1 {
        &self.site
    }

    /// The Recipe item anchoring adoption. `Activate` rows run before this
    /// item emits; `PublishWithEntry` rows run after it emits.
    pub(crate) const fn producing_item(&self) -> LoopItemKeyV1 {
        self.producing_item
    }

    pub(crate) const fn mode(&self) -> LoopInternalDeclarationModeV1 {
        self.mode
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopInternalDeclarationRejectV1 {
    ProducerMissing { binding: BindingRefV1 },
    ProducerNotWrite { item: LoopItemKeyV1 },
    CarrierMissing { binding: BindingRefV1 },
    EntryProducerMissing { binding: BindingRefV1 },
    EntryProducerNotOperation { item: LoopItemKeyV1 },
}

/// The complete set of loop-internal declarations for one physical demand.
/// The set is derived once inside `VerifiedLoopOperationPhysicalDemandV1`;
/// no caller may assemble or filter the rows.
#[derive(Debug)]
pub(crate) struct VerifiedLoopInternalDeclarationSetV1 {
    rows: Box<[VerifiedLoopInternalDeclarationV1]>,
}

impl VerifiedLoopInternalDeclarationSetV1 {
    pub(crate) fn issue(
        operation_effect: &VerifiedLoopOperationEffectProductV1,
        root_loop_site: &SourceStmtSiteV1,
    ) -> Result<Self, LoopInternalDeclarationRejectV1> {
        let core = operation_effect.core();
        let recipe = core.recipe().as_recipe();
        let loop_segments = root_loop_site.node().segments();
        let mut rows = Vec::new();
        for relation in core.binding_relations() {
            let BindingOriginV1::Source(site) = relation.declaration() else {
                continue;
            };
            let SourceBindingSiteV1::Local { statement, .. } = site else {
                continue;
            };
            let declaration_segments = statement.node().segments();
            let internal = declaration_segments.len() > loop_segments.len()
                && declaration_segments.starts_with(loop_segments);
            if !internal {
                continue;
            }
            let binding = relation.source_binding();
            let carrier_entry = core.effect_relations().iter().find_map(|effect| {
                if effect.role() != LoopBindingEffectRoleV1::DerivedCarrierEntry
                    || effect.source_binding() != binding
                {
                    return None;
                }
                let LoopBindingEffectAnchorV1::DerivedCarrierEntry { carrier, .. } =
                    effect.anchor()
                else {
                    return None;
                };
                Some(*carrier)
            });
            let (producing_item, mode) = match carrier_entry {
                Some(carrier) => {
                    let carrier = recipe
                        .carriers
                        .iter()
                        .find(|row| row.key == carrier)
                        .ok_or(LoopInternalDeclarationRejectV1::CarrierMissing { binding })?;
                    let entry_value = carrier.entry_value;
                    let item = recipe
                        .items
                        .iter()
                        .find(|row| {
                            matches!(
                                &row.item,
                                LoopRecipeItemV1::Operation { operation }
                                    if operation_result(operation) == Some(entry_value)
                            )
                        })
                        .map(|row| row.key)
                        .ok_or(LoopInternalDeclarationRejectV1::EntryProducerMissing {
                            binding,
                        })?;
                    (
                        item,
                        LoopInternalDeclarationModeV1::PublishWithEntry { value: entry_value },
                    )
                }
                None => {
                    let producing = operation_effect
                        .evidence()
                        .iter()
                        .filter(|row| row.source_binding() == Some(binding))
                        .min_by_key(|row| row.item())
                        .ok_or(LoopInternalDeclarationRejectV1::ProducerMissing { binding })?;
                    let item = producing.item();
                    let produces_write = recipe.items.iter().any(|row| {
                        row.key == item
                            && matches!(
                                row.item,
                                LoopRecipeItemV1::Operation {
                                    operation: LoopOperationV1::WriteBinding { .. },
                                }
                            )
                    });
                    if !produces_write {
                        return Err(LoopInternalDeclarationRejectV1::ProducerNotWrite { item });
                    }
                    (item, LoopInternalDeclarationModeV1::Activate)
                }
            };
            rows.push(VerifiedLoopInternalDeclarationV1 {
                binding,
                site: site.clone(),
                producing_item,
                mode,
            });
        }
        rows.sort_by_key(|row| row.producing_item);
        Ok(Self {
            rows: rows.into_boxed_slice(),
        })
    }

    pub(crate) fn rows(&self) -> &[VerifiedLoopInternalDeclarationV1] {
        &self.rows
    }

    pub(crate) const fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Rows anchored at `item`; callers pair them with the row's mode to run
    /// `Activate` before emission and `PublishWithEntry` after it.
    pub(crate) fn for_item(
        &self,
        item: LoopItemKeyV1,
    ) -> impl Iterator<Item = &VerifiedLoopInternalDeclarationV1> {
        self.rows
            .iter()
            .filter(move |row| row.producing_item == item)
    }
}

fn operation_result(operation: &LoopOperationV1) -> Option<LoopValueKeyV1> {
    match operation {
        LoopOperationV1::ReadBinding { result, .. }
        | LoopOperationV1::ConstI64 { result, .. }
        | LoopOperationV1::BinaryI64 { result, .. }
        | LoopOperationV1::CompareI64 { result, .. } => Some(*result),
        LoopOperationV1::WriteBinding { .. } => None,
    }
}
