//! Original child-field cohort and source witness, independent of the Home query owner.
use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StoredChildFieldSourceV1 {
    field: hakorune_mir_defs::CanonicalFieldRefV1,
    child: hakorune_mir_defs::CanonicalObjectIdV1,
    child_class: Box<str>,
    nullable: bool,
    paired_fields: BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>,
    parent_class: Box<str>,
    field_name: Box<str>,
    declared: bool,
    paired_all: bool,
    relations: Vec<
        std::rc::Rc<
            super::super::super::super::result_class_claim::SourceConstructionChildRelationV1,
        >,
    >,
    initialization:
        Option<super::super::super::super::field_write_claim::NullFieldInitializationSourceV1>,
}

/// The same original witness alternatives feed issuance and revalidation.
/// A caller supplies the appropriate original-child class correspondence.
fn child_witness_kinds_v1(
    relation: &super::super::super::super::result_class_claim::SourceConstructionChildRelationV1,
    mut fresh_matches: impl FnMut(&str) -> bool,
) -> Option<u8> {
    use super::super::super::super::result_class_claim::ResultValueOriginV1;
    let mut kinds = 0u8;
    for witness in relation.witnesses() {
        match witness.origin() {
            ResultValueOriginV1::Null => kinds |= 1,
            ResultValueOriginV1::Fresh(class) if fresh_matches(class) => kinds |= 2,
            _ => return None,
        }
    }
    (kinds != 0).then_some(kinds)
}

fn paired_child_kinds_v1(
    per_outer: &BTreeMap<OwnedExprSiteV1, BTreeMap<hakorune_mir_defs::CanonicalFieldRefV1, u8>>,
    paired_fields: &BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>,
) -> bool {
    !per_outer.is_empty()
        && !paired_fields.is_empty()
        && per_outer.values().all(|fields| {
            fields.keys().copied().collect::<BTreeSet<_>>() == *paired_fields
                && fields.values().copied().collect::<BTreeSet<_>>().len() == 1
                && (paired_fields.len() == 1 || !fields.values().any(|kinds| *kinds == 3))
        })
}

/// Summarize the same complete source cohort for issuance and revalidation.
fn summarize_child_relations_v1(
    relations: &[std::rc::Rc<
        super::super::super::super::result_class_claim::SourceConstructionChildRelationV1,
    >],
    parent: &str,
    field: hakorune_mir_defs::CanonicalFieldRefV1,
    declared: bool,
    mut fresh_matches: impl FnMut(&str, hakorune_mir_defs::CanonicalObjectIdV1) -> bool,
) -> Option<(
    hakorune_mir_defs::CanonicalObjectIdV1,
    bool,
    BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>,
)> {
    let mut per_outer = BTreeMap::<OwnedExprSiteV1, BTreeMap<_, u8>>::new();
    let mut child = None;
    let mut nullable = false;
    for relation in relations {
        if (if declared {
            relation.provided_field_source_v1(parent)
        } else {
            relation.unqualified_field_source_v1(parent)
        })
        .is_err()
            || child.is_some_and(|prior| prior != relation.child())
        {
            return None;
        }
        let kinds =
            child_witness_kinds_v1(relation, |class| fresh_matches(class, relation.child()))?;
        child = Some(relation.child());
        nullable |= kinds & 1 != 0;
        if per_outer
            .entry(relation.outer().clone())
            .or_default()
            .insert(relation.field(), kinds)
            .is_some()
        {
            return None;
        }
    }
    let paired = per_outer
        .values()
        .find(|fields| fields.contains_key(&field))?
        .keys()
        .copied()
        .collect();
    paired_child_kinds_v1(&per_outer, &paired).then_some((child?, nullable, paired))
}

/// Project the exact New-site child relations for one untyped field. The
/// relation cohort is already complete at issuance; recheck same-site field
/// pairing before using it as a stored-call source. A single field may have
/// mixed Null/Fresh witnesses; multiple fields require correlated witnesses.
pub(in super::super) fn dynamic_stored_child_field_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    facts: &super::super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    parent_class: &str,
    field_name: &str,
) -> Result<Option<StoredChildFieldSourceV1>, String> {
    let Some(parent_source) = batch
        .ordinary_box_coverage()
        .row_for(parent_class)
        .map_err(|error| format!("[freeze:contract][stored-child/definition]{error:?}"))?
    else {
        return Ok(None);
    };
    let parent_object = constructors
        .object_for(parent_source)
        .map_err(|error| format!("[freeze:contract][stored-child/definition]{error:?}"))?;
    let field = constructors
        .with_source_object_definition(parent_source, |object, definition| {
            let mut matches = definition
                .fields()
                .iter()
                .enumerate()
                .filter(|(_, row)| row.name == field_name);
            let (ordinal, declaration) = matches.next()?;
            if matches.next().is_some()
                || declaration.is_weak
                || declaration.declared_type_name.is_some()
            {
                return None;
            }
            hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(object, ordinal)
        })
        .map_err(|error| format!("[freeze:contract][stored-child/definition]{error:?}"))?;
    let Some(field) = field else {
        return Ok(None);
    };
    if field.object() != parent_object {
        return Ok(None);
    }

    let relations: Vec<_> = facts
        .construction_child_relation_loans()
        .filter(|relation| {
            relation.is_unqualified_dynamic_v1() && relation.field().object() == parent_object
        })
        .cloned()
        .collect();
    let Some((child, nullable, paired_fields)) =
        summarize_child_relations_v1(&relations, parent_class, field, false, |class, child| {
            batch
                .ordinary_box_coverage()
                .row_for(class)
                .ok()
                .flatten()
                .and_then(|source| constructors.object_for(&source).ok())
                == Some(child)
        })
    else {
        return Ok(None);
    };
    let mut child_classes = batch
        .ordinary_box_coverage()
        .rows()
        .iter()
        .filter(|source| constructors.object_for(source).ok() == Some(child));
    let Some(child_source) = child_classes.next() else {
        return Ok(None);
    };
    if child_classes.next().is_some() {
        return Ok(None);
    }
    Ok(Some(StoredChildFieldSourceV1 {
        field,
        child,
        child_class: child_source.name().into(),
        nullable,
        paired_fields,
        parent_class: parent_class.into(),
        field_name: field_name.into(),
        declared: false,
        paired_all: true,
        relations,
        initialization: None,
    }))
}

impl StoredChildFieldSourceV1 {
    pub(in crate::mir) fn field(&self) -> hakorune_mir_defs::CanonicalFieldRefV1 {
        self.field
    }
    pub(in crate::mir) fn child(&self) -> hakorune_mir_defs::CanonicalObjectIdV1 {
        self.child
    }
    pub(in crate::mir) fn child_class(&self) -> &str {
        &self.child_class
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn nullable(
        &self,
    ) -> bool {
        self.nullable
    }
    pub(in crate::mir) fn declared_v1(&self) -> bool {
        self.declared
    }
    pub(in super::super) fn paired_field(
        &self,
        field: hakorune_mir_defs::CanonicalFieldRefV1,
    ) -> bool {
        self.paired_fields.contains(&field)
    }
    /// A provisional receiver can only use its paired children as method
    /// receivers or in null comparisons. This does not establish dispatch,
    /// incoming-call closure, ownership, or permission to execute.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn local_child_uses_v1(
        &self,
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        receiver: crate::mir::resolved_semantics::BindingRefV1,
    ) -> bool {
        use crate::mir::resolved_semantics::{
            BodyExpressionShapeV1, BodyMeReceiverV1, ResolvedBinaryOperatorV1,
            ResolvedLexicalRefV1, ResolvedLiteralSourceV1,
        };
        let Some(shape) = input.body_shape() else {
            return false;
        };
        let mut names = BTreeSet::new();
        if self.initialization.is_some() {
            names.insert(self.field_name.as_ref());
        }
        for relation in &self.relations {
            if self.paired_fields.contains(&relation.field()) {
                let Ok(name) = relation.original_field_name_v1() else {
                    return false;
                };
                names.insert(name);
            }
        }
        if names.len() != self.paired_fields.len() {
            return false;
        }
        let mut parents = BTreeSet::new();
        let mut children = BTreeSet::new();
        for expression in shape.expressions() {
            if let BodyExpressionShapeV1::FieldAccess {
                site,
                object,
                field,
            } = expression
            {
                if matches!(shape.expression_shape(object), Some(BodyExpressionShapeV1::Me {
                    receiver: BodyMeReceiverV1::Lexical(binding), .. }) if *binding == receiver)
                {
                    parents.insert(object.clone());
                    if names.contains(field.as_ref()) {
                        children.insert(site.clone());
                    }
                }
            }
        }
        // Do not treat a local alias as a direct field access; a later closure
        // proof can explicitly add aliases using its own source identities.
        if shape
            .expressions()
            .iter()
            .any(|expression| match expression {
                BodyExpressionShapeV1::Me {
                    site,
                    receiver: BodyMeReceiverV1::Lexical(binding),
                }
                | BodyExpressionShapeV1::Variable {
                    site,
                    resolved: ResolvedLexicalRefV1::Local(binding),
                } if *binding == receiver => !parents.contains(site),
                _ => false,
            })
        {
            return false;
        }
        let mut permitted = BTreeSet::new();
        for expression in shape.expressions() {
            if let BodyExpressionShapeV1::MethodCall { object, .. } = expression {
                permitted.insert(object.clone());
            }
        }
        let source = input.function().expression_source();
        for binary in source.binaries() {
            if !matches!(
                binary.operator(),
                ResolvedBinaryOperatorV1::Equal | ResolvedBinaryOperatorV1::NotEqual
            ) {
                continue;
            }
            if matches!(
                source.literal(binary.lhs()),
                Some(ResolvedLiteralSourceV1::Null)
            ) {
                permitted.insert(binary.rhs().clone());
            }
            if matches!(
                source.literal(binary.rhs()),
                Some(ResolvedLiteralSourceV1::Null)
            ) {
                permitted.insert(binary.lhs().clone());
            }
        }
        !children.is_empty() && children.is_subset(&permitted)
    }
    /// The same complete immutable parent cohort, not a new class inference.
    pub(in crate::mir::normal_callable_semantic_package) fn recheck_v1(
        &self,
        facts: &super::super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
        field_name: &str,
    ) -> bool {
        if let Some(initialization) = &self.initialization {
            return self.declared
                && self.nullable
                && self.relations.is_empty()
                && self.field_name.as_ref() == field_name
                && self.paired_fields == BTreeSet::from([self.field])
                && initialization.recheck(self.field, self.child, field_name)
                && facts
                    .field_inventory_v1()
                    .is_some_and(|inventory| initialization.same_inventory(inventory));
        }
        let current: Vec<_> = facts
            .construction_child_relation_loans()
            .filter(|row| {
                row.is_unqualified_dynamic_v1() != self.declared
                    && row.field().object() == self.field.object()
                    && (self.paired_all || row.field() == self.field)
            })
            .collect();
        if self.declared
            && facts
                .child_construction_sites_v1(self.field.object())
                .is_none_or(|sites| {
                    sites.is_empty()
                        || current
                            .iter()
                            .map(|row| row.outer().clone())
                            .collect::<BTreeSet<_>>()
                            != *sites
                })
        {
            return false;
        }
        if self.field_name.as_ref() != field_name
            || self.relations.is_empty()
            || current.len() != self.relations.len()
            || !current.iter().zip(&self.relations).all(|(a, b)| {
                std::rc::Rc::ptr_eq(a, b)
                    && (if self.declared {
                        a.provided_field_source_v1(&self.parent_class)
                    } else {
                        a.unqualified_field_source_v1(&self.parent_class)
                    })
                    .is_ok()
            })
        {
            return false;
        }
        if self.relations.iter().any(|relation| {
            relation.field() == self.field && relation.original_field_name_v1() != Ok(field_name)
        }) {
            return false;
        }
        summarize_child_relations_v1(
            &self.relations,
            &self.parent_class,
            self.field,
            self.declared,
            |class, child| child == self.child && class == self.child_class(),
        )
        .is_some_and(|(child, nullable, paired)| {
            child == self.child && nullable == self.nullable && paired == self.paired_fields
        })
    }
}

/// Source-only paired-field candidate. It does not prove that an arbitrary
/// receiver came from this cohort, nor grant a non-null or Home loan.
pub(in crate::mir::normal_callable_semantic_package) fn declared_stored_child_field_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    facts: &super::super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    parent_class: &str,
    field_name: &str,
    site: &OwnedExprSiteV1,
    paired_all: bool,
) -> Result<Option<StoredChildFieldSourceV1>, String> {
    let Some(parent) = batch
        .ordinary_box_coverage()
        .row_for(parent_class)
        .map_err(|error| format!("[freeze:contract][stored-child/definition]{error:?}"))?
    else {
        return Ok(None);
    };
    let Some((field, Some(class))) =
        super::super::super::super::terminal_home::source_declared_field(
            constructors,
            parent,
            site,
            field_name,
        )
        .map_err(|error| format!("[freeze:contract][stored-child/definition]{error:?}"))?
    else {
        return Ok(None);
    };
    let Some(nominal) = crate::mir::declared_type_storage::declared_object_field_type_v1(&class)
    else {
        return Ok(None);
    };
    let class = nominal.class_name();
    let Some(child_source) = batch
        .ordinary_box_coverage()
        .row_for(class)
        .map_err(|error| format!("[freeze:contract][stored-child/definition]{error:?}"))?
    else {
        return Ok(None);
    };
    let child = constructors
        .object_for(child_source)
        .map_err(|error| format!("[freeze:contract][stored-child/definition]{error:?}"))?;
    if let Some(initialization) = constructors.original_field_value_source_v1(parent_class, field_name)
        .and_then(|source| source.null_initialization_v1(constructors))
        .or_else(|| super::super::super::super::field_write_claim::NullFieldInitializationSourceV1::declaration_default_v1(
            constructors, parent_class, field_name, field, child)) {
        let source = StoredChildFieldSourceV1 {
            field, child, child_class: class.into(), nullable: true, paired_fields: BTreeSet::from([field]),
            parent_class: parent_class.into(), field_name: field_name.into(), declared: true, paired_all,
            relations: Vec::new(), initialization: Some(initialization),
        };
        return Ok(source.recheck_v1(facts, field_name).then_some(source));
    }
    let Some(sites) = facts.child_construction_sites_v1(field.object()) else {
        return Ok(None);
    };
    let relations: Vec<_> = facts
        .construction_child_relation_loans()
        .filter(|row| {
            !row.is_unqualified_dynamic_v1()
                && row.field().object() == field.object()
                && (paired_all || row.field() == field)
        })
        .cloned()
        .collect();
    if sites.is_empty()
        || relations
            .iter()
            .map(|row| row.outer().clone())
            .collect::<BTreeSet<_>>()
            != *sites
    {
        return Ok(None);
    }
    let Some((actual_child, nullable, paired_fields)) =
        summarize_child_relations_v1(&relations, parent_class, field, true, |name, object| {
            name == class && object == child
        })
    else {
        return Ok(None);
    };
    if actual_child != child {
        return Ok(None);
    }
    Ok(Some(StoredChildFieldSourceV1 {
        field,
        child,
        child_class: class.into(),
        nullable,
        paired_fields,
        parent_class: parent_class.into(),
        field_name: field_name.into(),
        declared: true,
        paired_all,
        relations,
        initialization: None,
    }))
}
