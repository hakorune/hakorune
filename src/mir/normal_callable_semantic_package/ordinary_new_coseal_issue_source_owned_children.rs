//! Owned-field child inventory sealing for the ordinary-New cohort.
//! One responsibility: prove the sealed residence inventory of a
//! user-object child — for `new`-claim parents and for provider-created
//! children that mint no claim — so every downstream cleanup consumer
//! reads one ledger. This private move preserves call order and failure
//! boundaries.
use std::collections::BTreeMap;

use super::super::super::{
    field_write_claim, OrdinaryNewCoSealIssueV1, OwnedFieldChildKindV1, OwnedFieldChildV1,
};
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::normal_callable_semantic_package::instance_constructor_semantic::VerifiedInstanceConstructorSemanticBatchV1;
use crate::mir::normal_callable_semantic_package::ConstructionStoreRhsV1;
use crate::mir::resolved_semantics::OwnedExprSiteV1;

/// Prove the owned field children of one canonical object.
/// `OwnedArrayFieldsNoHook`/`OwnedObjectFieldsNoHook` objects carry their
/// residence-capable declared fields in declaration order only when each
/// field has a sealed birth-side residence whose written class equals the
/// declared type. A user-object child additionally admits exactly one
/// bounded nesting level: a `PlainI64NoHook` object always resolves,
/// while an `OwnedArrayFieldsNoHook` child resolves only when its own
/// `ArrayBox` residences seal — releasing its slots is sound exactly
/// while every residence is a proven provider store. The child's sealed
/// inventory is inserted into `owned_field_children` for the downstream
/// reclaim/discharge and release consumers; deeper teardown, cycles and
/// unproven child residences stay unadmitted. Any unproven field — or a
/// missing object definition — yields `None`, which every consumer
/// treats as unadmitted, never silently plain.
pub(super) fn owned_field_children_of(
    site: &OwnedExprSiteV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    box_source: &crate::parser::ParserOrdinaryBoxSourceRowV1,
    destruction: crate::mir::function::ObjectDestructionDispositionV1,
    residences: &field_write_claim::OwnedFieldResidencesV1,
    owned_field_children: &mut BTreeMap<
        hakorune_mir_defs::CanonicalObjectIdV1,
        Option<Box<[OwnedFieldChildV1]>>,
    >,
) -> Result<Option<Box<[OwnedFieldChildV1]>>, OrdinaryNewCoSealIssueV1> {
    use crate::mir::function::ObjectDestructionDispositionV1;
    if !matches!(
        destruction,
        ObjectDestructionDispositionV1::OwnedArrayFieldsNoHook
            | ObjectDestructionDispositionV1::OwnedObjectFieldsNoHook
    ) {
        return Ok(None);
    }
    // The definition-view closure returns `Option`, so a nested seal
    // failure reaches the caller through `issue_error` — `None` stays the
    // unadmitted marker and the typed error is never collapsed into it.
    let mut issue_error = None;
    let children = instance_constructors
        .with_source_object_definition(box_source, |object, definition| {
            let mut children = Vec::new();
            for (ordinal, field) in definition.fields().iter().enumerate() {
                let declared = field.declared_type_name.as_deref();
                if declared
                    .and_then(
                        crate::mir::declared_type_storage::exact_numeric_storage_for_declared_type,
                    )
                    .is_some()
                {
                    continue;
                }
                let key = (
                    box_source.name().into(),
                    field.name.clone().into_boxed_str(),
                );
                let Some(residence) = residences.get(&key) else {
                    return None;
                };
                let class: &str = match residence {
                    field_write_claim::OwnedFieldResidenceV1::Provider(class) => class.as_ref(),
                    // A provided store names no class at the write site;
                    // the declared field type is the sole authority and
                    // it must be a user class — builtin/`ArrayBox` fields
                    // keep their provider-only boundary.
                    field_write_claim::OwnedFieldResidenceV1::Provided => {
                        let name = declared?;
                        if name == "ArrayBox" || crate::box_trait::is_builtin_box(name) {
                            return None;
                        }
                        name
                    }
                };
                // The sole birth write must store the declared class
                // exactly — a proven residence of a different class does
                // not satisfy the typed field.
                if declared != Some(class) {
                    return None;
                }
                let field_ref = hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(
                    object, ordinal,
                )
                .expect("declared field ordinal resolves canonically");
                let kind = if class == "ArrayBox" {
                    OwnedFieldChildKindV1::Array
                } else {
                    let child_source = match batch.ordinary_box_coverage().row_for(class) {
                        Ok(Some(row)) => row,
                        _ => return None,
                    };
                    let (child, child_destruction) =
                        match instance_constructors.destruction_for(child_source) {
                            Ok((child, destruction)) if child != object => (child, destruction),
                            _ => return None,
                        };
                    match child_destruction {
                        ObjectDestructionDispositionV1::PlainI64NoHook => {}
                        ObjectDestructionDispositionV1::OwnedArrayFieldsNoHook => {
                            // The bounded nested admission: the child owns
                            // `ArrayBox` residences only when its own
                            // teardown inventory seals — releasing its
                            // slots is sound exactly while every residence
                            // is a proven provider store.
                            let nested = match owned_field_children_of(
                                site,
                                batch,
                                instance_constructors,
                                child_source,
                                child_destruction,
                                residences,
                                owned_field_children,
                            ) {
                                Ok(Some(nested)) => nested,
                                Ok(None) => return None,
                                Err(error) => {
                                    issue_error = Some(error);
                                    return None;
                                }
                            };
                            if nested
                                .iter()
                                .any(|row| !matches!(row.kind, OwnedFieldChildKindV1::Array))
                            {
                                return None;
                            }
                            match owned_field_children.entry(child) {
                                std::collections::btree_map::Entry::Vacant(entry) => {
                                    entry.insert(Some(nested));
                                }
                                std::collections::btree_map::Entry::Occupied(entry)
                                    if entry.get().as_deref() == Some(nested.as_ref()) => {}
                                std::collections::btree_map::Entry::Occupied(_) => {
                                    issue_error = Some(OrdinaryNewCoSealIssueV1::DuplicateSite {
                                        site: site.clone(),
                                    });
                                    return None;
                                }
                            }
                        }
                        _ => return None,
                    }
                    OwnedFieldChildKindV1::Object(child)
                };
                children.push(OwnedFieldChildV1 {
                    field: field_ref,
                    kind,
                });
            }
            Some(children.into_boxed_slice())
        })
        .map_err(|error| OrdinaryNewCoSealIssueV1::ConstructorLookup {
            site: site.clone(),
            class: box_source.name().into(),
            error,
        })?;
    if let Some(error) = issue_error {
        return Err(error);
    }
    Ok(children)
}

/// Seal the owned-field inventory of every provider-created user-class
/// child. A provider `new` mints no `local`-bound claim, so the claim-side
/// seal never reaches the child's object: each plan `owned_fields`
/// admission must be joined here with the same residence ledger a
/// `new`-claim seal would use. `Some(children)` is admitted and `None`
/// stays explicitly unproven — consumers reject missing and unproven
/// rows alike, never silently releasing the child as plain.
pub(super) fn seal_provider_owned_children_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    residences: &field_write_claim::OwnedFieldResidencesV1,
    owned_field_children: &mut BTreeMap<
        hakorune_mir_defs::CanonicalObjectIdV1,
        Option<Box<[OwnedFieldChildV1]>>,
    >,
) -> Result<(), OrdinaryNewCoSealIssueV1> {
    for row in instance_constructors.rows() {
        let Ok(plan) = row.construction() else {
            continue;
        };
        for store in plan.stores() {
            let ConstructionStoreRhsV1::ProviderConstruction {
                site,
                class,
                object: Some(child),
                arguments,
                owned_fields,
                ..
            } = store.rhs()
            else {
                continue;
            };
            if owned_fields.is_empty() || owned_field_children.contains_key(child) {
                continue;
            }
            let owner = plan
                .constructor()
                .expect("provider store requires a constructor owner")
                .1;
            let owned_site = OwnedExprSiteV1::new(owner, site.clone());
            let child_source = match batch.ordinary_box_coverage().row_for(class) {
                Ok(Some(row)) => row,
                Ok(None) => {
                    return Err(OrdinaryNewCoSealIssueV1::OrdinaryBoxCoverageMissing {
                        site: owned_site,
                        class: class.clone(),
                    })
                }
                Err(_) => {
                    return Err(OrdinaryNewCoSealIssueV1::OrdinaryBoxCoverageDuplicate {
                        site: owned_site,
                        class: class.clone(),
                    })
                }
            };
            let (child_object, child_destruction) = instance_constructors
                .destruction_for(child_source)
                .map_err(|error| OrdinaryNewCoSealIssueV1::ConstructorLookup {
                    site: owned_site.clone(),
                    class: class.clone(),
                    error,
                })?;
            if child_object != *child {
                return Err(OrdinaryNewCoSealIssueV1::ConstructorRelationMismatch {
                    site: owned_site,
                    class: class.clone(),
                    arity: arguments.len(),
                });
            }
            let children = owned_field_children_of(
                &owned_site,
                batch,
                instance_constructors,
                child_source,
                child_destruction,
                residences,
                owned_field_children,
            )?;
            match owned_field_children.entry(child_object) {
                std::collections::btree_map::Entry::Vacant(entry) => {
                    entry.insert(children);
                }
                std::collections::btree_map::Entry::Occupied(entry) if *entry.get() == children => {
                }
                std::collections::btree_map::Entry::Occupied(_) => {
                    return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site: owned_site });
                }
            }
        }
    }
    Ok(())
}

/// Borrow original receiver identity and residence; issue no child inventory.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::coseal_issue) fn stored_child_source_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    entry: Option<&crate::mir::resolved_semantics::VerifiedInstanceEntryHomeLoanV1>,
    slot: u32,
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    call: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    residences: &field_write_claim::OwnedFieldResidencesV1,
) -> Result<Option<super::super::super::lexical_instance_call::StoredReceiverSourceV1>, String> {
    use crate::mir::resolved_semantics::{BodyExpressionShapeV1, BodyMeReceiverV1};
    let Some(shape) = input.body_shape() else {
        return Ok(None);
    };
    let Some(BodyExpressionShapeV1::FieldAccess {
        object: parent_site,
        field: name,
        ..
    }) = shape.expression_shape(call.receiver_site())
    else {
        return Ok(None);
    };
    let Some(BodyExpressionShapeV1::Me {
        receiver: BodyMeReceiverV1::Lexical(parent_binding),
        ..
    }) = shape.expression_shape(parent_site)
    else {
        return Ok(None);
    };
    let Some((receiver, source)) =
        super::super::super::terminal_home::entry_receiver_box_proof(selected, batch, entry, slot)
    else {
        return Ok(None);
    };
    if receiver != *parent_binding
        || receiver.owner() != input.owner()
        || entry.is_none_or(|loan| loan.owner() != input.owner() || loan.batch_slot() != slot)
    {
        return Ok(None);
    }
    let key = (source.name().into(), name.clone());
    let Some(field_write_claim::OwnedFieldResidenceV1::Provider(class)) = residences.get(&key)
    else {
        return Ok(None);
    };
    Ok(Some(super::super::super::lexical_instance_call::StoredReceiverSourceV1 {
        parent_binding: *parent_binding, parent_site: parent_site.clone(),
        parent_class: source.name().into(), field_name: name.clone(), child_class: class.clone(),
    }))
}

/// Seal inventory only for an eligible source receiver through the existing issuer.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::coseal_issue) fn stored_child_receiver_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    site: &OwnedExprSiteV1,
    receiver: &super::super::super::lexical_instance_call::StoredReceiverSourceV1,
    residences: &field_write_claim::OwnedFieldResidencesV1,
    owned: &mut BTreeMap<hakorune_mir_defs::CanonicalObjectIdV1, Option<Box<[OwnedFieldChildV1]>>>,
) -> Result<Option<super::super::super::lexical_instance_call::LexicalInstanceCallReceiverV1>, String> {
    let class = &receiver.child_class;
    let name = &receiver.field_name;
    let source = batch.ordinary_box_coverage().row_for(&receiver.parent_class)
        .map_err(|error| format!("[freeze:contract][stored-child/definition]{error:?}"))?
        .ok_or_else(|| "[freeze:contract][stored-child/parent-source-missing]".to_owned())?;
    let Some(child_source) = batch.ordinary_box_coverage().row_for(class).ok().flatten() else {
        return Ok(None);
    };
    let (child, _) = constructors
        .destruction_for(child_source)
        .map_err(|error| format!("[freeze:contract][stored-child/definition]{error:?}"))?;
    let (parent, destruction) = constructors
        .destruction_for(source)
        .map_err(|error| format!("[freeze:contract][stored-child/definition]{error:?}"))?;
    let field = constructors
        .with_source_object_definition(source, |object, definition| {
            let mut fields = definition
                .fields()
                .iter()
                .enumerate()
                .filter(|(_, row)| row.name == name.as_ref());
            let (ordinal, declaration) = fields.next()?;
            if fields.next().is_some()
                || declaration.is_weak
                || declaration.declared_type_name.as_deref() != Some(class.as_ref())
            {
                return None;
            }
            hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(object, ordinal)
        })
        .map_err(|error| format!("[freeze:contract][stored-child/definition]{error:?}"))?;
    let Some(field) = field else {
        return Ok(None);
    };
    let inventory = owned_field_children_of(
        site,
        batch,
        constructors,
        source,
        destruction,
        residences,
        owned,
    )
    .map_err(|error| format!("[freeze:contract][stored-child/residence]{error:?}"))?;
    let Some(inventory) = inventory else {
        return Ok(None);
    };
    if inventory
        .iter()
        .filter(|row| row.field == field && row.kind == OwnedFieldChildKindV1::Object(child))
        .count()
        != 1
    {
        return Ok(None);
    }
    match owned.entry(parent) {
        std::collections::btree_map::Entry::Vacant(entry) => {
            entry.insert(Some(inventory));
        }
        std::collections::btree_map::Entry::Occupied(entry)
            if entry.get().as_deref() == Some(inventory.as_ref()) => {}
        _ => return Err("[freeze:contract][stored-child/residence-drift]".into()),
    }
    Ok(Some(super::super::super::lexical_instance_call::LexicalInstanceCallReceiverV1::StoredOwnedChild {
        parent_binding: receiver.parent_binding, parent_site: receiver.parent_site.clone(), field, child,
    }))
}
