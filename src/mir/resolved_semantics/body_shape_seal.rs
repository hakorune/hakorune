//! Canonical seal for the existing passive body-shape owner.
//! Checks exact source coverage; never assigns Home, effect or Recipe meaning.
use super::*;

pub(crate) fn seal_shadow_body_shape(
    owner: FunctionOwnerIdV1,
    root_profile: SemanticOwnerRootProfileV1,
    draft: ShadowBodyShapeDraftV0,
    variable_refs: &BTreeMap<SourceExprSiteV1, ResolvedLexicalRefV1>,
    statement_sites: &BTreeSet<SourceStmtSiteV1>,
    expression_sites: &BTreeSet<SourceExprSiteV1>,
) -> Result<VerifiedResolvedBodyShapeInventoryV1, &'static str> {
    if draft.statements.len() != statement_sites.len()
        || draft
            .statements
            .keys()
            .any(|site| !statement_sites.contains(site))
        || draft.expressions.len() != expression_sites.len()
        || draft
            .expressions
            .keys()
            .any(|site| !expression_sites.contains(site))
    {
        return Err("body shape coverage does not match resolver source inventory");
    }

    let statements = draft
        .statements
        .into_values()
        .map(|row| match row {
            ShadowStatementShapeV0::SequenceItem { site } => {
                BodyStatementShapeV1::SequenceItem { site }
            }
            ShadowStatementShapeV0::Return { site, value } => {
                BodyStatementShapeV1::Return { site, value }
            }
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let method_call_parents = draft
        .expressions
        .values()
        .filter_map(|row| match row {
            ShadowExpressionShapeV0::MethodCall { site, .. } => Some(site.node().clone()),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let method_receiver_sites = draft
        .relations
        .iter()
        .filter_map(|row| {
            (row.role == SourcePathSegmentV1::Receiver && method_call_parents.contains(&row.parent))
                .then(|| row.child.clone())
        })
        .collect::<BTreeSet<_>>();

    let expressions = draft
        .expressions
        .into_values()
        .map(|row| match row {
            ShadowExpressionShapeV0::Variable { site } => match variable_refs.get(&site).copied() {
                Some(resolved) => Ok(BodyExpressionShapeV1::Variable { site, resolved }),
                None if method_receiver_sites.contains(&site) => {
                    Ok(BodyExpressionShapeV1::QualifiedReceiver { site })
                }
                None => Err("body variable shape lacks lexical resolution"),
            },
            ShadowExpressionShapeV0::Me { site } => {
                let receiver = match variable_refs.get(&site).copied() {
                    Some(ResolvedLexicalRefV1::Local(receiver)) => {
                        BodyMeReceiverV1::Lexical(receiver)
                    }
                    None if matches!(
                        root_profile,
                        SemanticOwnerRootProfileV1::DeclaredFunction {
                            receiver_policy:
                                crate::mir::resolved_semantics::ReceiverPolicyV1::StaticCurrentOwner,
                        }
                    ) =>
                    {
                        BodyMeReceiverV1::StaticCurrentOwner
                    }
                    _ => return Err("body Me shape lacks exact receiver authority"),
                };
                Ok(BodyExpressionShapeV1::Me { site, receiver })
            }
            ShadowExpressionShapeV0::FieldAccess {
                site,
                object,
                field,
            } => Ok(BodyExpressionShapeV1::FieldAccess {
                site,
                object,
                field,
            }),
            ShadowExpressionShapeV0::MethodCall {
                site,
                object,
                method,
                arity,
            } => {
                let arity = u32::try_from(arity)
                    .map_err(|_| "method-call arity exceeds resolver source identity")?;
                Ok(BodyExpressionShapeV1::MethodCall {
                    site,
                    object,
                    method,
                    arity,
                })
            }
            ShadowExpressionShapeV0::BlockExpr { site } => {
                Ok(BodyExpressionShapeV1::BlockExpr { site })
            }
            ShadowExpressionShapeV0::MapLiteral { site, keys } => {
                u32::try_from(keys.len()).map_err(|_| "map count exceeds resolver source identity")?;
                Ok(BodyExpressionShapeV1::MapLiteral { site, keys })
            }
            ShadowExpressionShapeV0::ArrayLiteral { site, element_count } => {
                let element_count = u32::try_from(element_count)
                    .map_err(|_| "array count exceeds resolver source identity")?;
                Ok(BodyExpressionShapeV1::ArrayLiteral { site, element_count })
            }
            ShadowExpressionShapeV0::Other { site, kind } => {
                Ok(BodyExpressionShapeV1::Other { site, kind })
            }
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_boxed_slice();

    let effects = draft
        .effects
        .into_iter()
        .map(|(site, kind)| BodyEffectShapeV1 { site, kind })
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let relations = seal_shadow_body_shape_relations(draft.relations)?;
    validate_array_relations(&expressions, &relations)?;
    validate_map_relations(&expressions, &relations)?;
    let assignment_sources = draft
        .assignment_sources
        .into_values()
        .collect::<Vec<_>>()
        .into_boxed_slice();

    Ok(VerifiedResolvedBodyShapeInventoryV1 {
        owner,
        body_root: root_profile.body_root(),
        statements,
        expressions,
        effects,
        relations,
        assignment_sources,
    })
}

fn validate_array_relations(
    expressions: &[BodyExpressionShapeV1],
    relations: &[BodyShapeRelationV1],
) -> Result<(), &'static str> {
    let sites = expressions
        .iter()
        .map(expression_shape_site)
        .collect::<BTreeSet<_>>();
    let arrays = expressions
        .iter()
        .filter_map(|expression| match expression {
            BodyExpressionShapeV1::ArrayLiteral {
                site,
                element_count,
            } => Some((site.node(), *element_count)),
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();
    let mut children = BTreeMap::<_, Vec<_>>::new();
    let mut parent_counts = BTreeMap::<_, usize>::new();
    for row in relations {
        if matches!(row.role(), SourcePathSegmentV1::Element(_))
            && !arrays.contains_key(row.parent())
        {
            return Err("array source child parent outside array inventory");
        }
        children.entry(row.parent()).or_default().push(row);
        *parent_counts.entry(row.child()).or_default() += 1;
    }
    for (site, element_count) in arrays {
        let rows = children.get(site).map(Vec::as_slice).unwrap_or(&[]);
        if rows.len() != element_count as usize {
            return Err("array source child cardinality mismatch");
        }
        let mut ordinals = BTreeSet::new();
        for row in rows {
            let SourcePathSegmentV1::Element(ordinal) = row.role() else {
                return Err("array source child role mismatch");
            };
            if *ordinal >= element_count || !ordinals.insert(*ordinal) {
                return Err("array source child ordinal mismatch");
            }
            let expected = super::super::source_site::SourcePathV1::from_node(site)
                .child(SourcePathSegmentV1::Element(*ordinal))
                .expr();
            if row.child() != &expected || !sites.contains(row.child()) {
                return Err("array source child endpoint mismatch");
            }
            if parent_counts.get(row.child()) != Some(&1) {
                return Err("array source child has multiple parents");
            }
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "body_shape_array_tests.rs"]
mod array_tests;

fn validate_map_relations(
    expressions: &[BodyExpressionShapeV1],
    relations: &[BodyShapeRelationV1],
) -> Result<(), &'static str> {
    let sites = expressions
        .iter()
        .map(expression_shape_site)
        .collect::<BTreeSet<_>>();
    let maps = expressions
        .iter()
        .filter_map(|expression| match expression {
            BodyExpressionShapeV1::MapLiteral { site, keys } => Some((site.node(), keys.len())),
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();
    let mut children = BTreeMap::<_, Vec<_>>::new();
    let mut parent_counts = BTreeMap::<_, usize>::new();
    for row in relations {
        children.entry(row.parent()).or_default().push(row);
        *parent_counts.entry(row.child()).or_default() += 1;
        if matches!(row.role(), SourcePathSegmentV1::EntryValue(_))
            && !maps.contains_key(row.parent())
        {
            return Err("map source child parent outside map inventory");
        }
    }
    for (site, count) in maps {
        let rows = children.get(site).map(Vec::as_slice).unwrap_or(&[]);
        if rows.len() != count {
            return Err("map source child cardinality mismatch");
        }
        let mut ordinals = BTreeSet::new();
        for row in rows {
            let SourcePathSegmentV1::EntryValue(ordinal) = row.role() else {
                return Err("map source child role mismatch");
            };
            if *ordinal as usize >= count || !ordinals.insert(*ordinal) {
                return Err("map source child ordinal mismatch");
            }
            let expected = super::super::source_site::SourcePathV1::from_node(site)
                .child(SourcePathSegmentV1::EntryValue(*ordinal))
                .expr();
            if row.child() != &expected || !sites.contains(row.child()) {
                return Err("map source child endpoint mismatch");
            }
            if parent_counts.get(row.child()) != Some(&1) {
                return Err("map source child has multiple parents");
            }
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "body_shape_map_tests.rs"]
mod map_tests;
