//! Fresh callable anchors issued from exact parser generator receipts.

use crate::ast::{
    ASTNode, BoxMethodGeneratedProvenanceV1, BoxMethodInventoryV1, BoxMethodProvenanceV1,
};

use super::callable_gate_projection::MemberGateSelectionReceiptV1;
use super::callable_source_anchor::{
    GeneratedCallableOriginV1, GeneratedDelegateCallableOriginV1,
    GeneratedPropertyCallableOriginV1, PreparedGeneratedCallableSourceV1,
};
use super::delegate_source_relation::GeneratedDelegateSourceRelationV1;
use super::source_authority::{
    MethodSourceRelationV1, ParserInvocationBrandV1, PreparedBoxSourceSealV1, SourceBoxMethodSiteV1,
};
use super::source_path::SourceBoxDeclarationPathV1;
use super::source_path::{SourceProgramCallablePathV1, SourceProgramDeclarationPathV1};
use super::ParseError;

const REJECT_TAG: &str = "parser/generated-callable-source";

pub(super) fn issue_property_callable_rows(
    brand: &ParserInvocationBrandV1,
    inventory: &BoxMethodInventoryV1,
    relations: &[MethodSourceRelationV1],
    member_receipts: &[MemberGateSelectionReceiptV1],
) -> Result<Box<[PreparedGeneratedCallableSourceV1]>, ParseError> {
    let generated_count = inventory
        .iter_selected_declaration_order()
        .filter(|entry| {
            matches!(
                entry.provenance(),
                BoxMethodProvenanceV1::Generated(BoxMethodGeneratedProvenanceV1::Property { .. })
            )
        })
        .count();
    let mut rows = Vec::with_capacity(generated_count);
    let mut placements = Vec::with_capacity(generated_count);
    for relation in relations {
        let MethodSourceRelationV1::GeneratedProperty {
            source_site,
            placement,
        } = relation
        else {
            continue;
        };
        if !source_site.box_site().path().brand().same_as(brand) {
            return Err(reject(
                "generated property source member has a foreign parser brand",
            ));
        }
        if placements.contains(&placement.inventory_ordinal()) {
            return Err(reject("duplicate generated property placement receipt"));
        }
        let entry = inventory
            .get(placement.name())
            .filter(|entry| entry.site() == placement.inventory_ordinal())
            .ok_or_else(|| reject("generated property placement is absent from inventory"))?;
        let BoxMethodProvenanceV1::Generated(
            provenance @ BoxMethodGeneratedProvenanceV1::Property { selection, .. },
        ) = entry.provenance()
        else {
            return Err(reject(
                "generated property placement has the wrong provenance",
            ));
        };
        let declaration =
            SourceProgramDeclarationPathV1::from_parser_path(source_site.box_site().path().clone());
        if !source_site.matches_ast_selection(selection) {
            return Err(reject(
                "generated property provenance does not match its source site",
            ));
        }
        let gate_path = exact_generated_gate_path(&declaration, source_site, member_receipts)
            .map_err(reject)?;
        placements.push(placement.inventory_ordinal());
        rows.push(PreparedGeneratedCallableSourceV1::issue(
            brand.clone(),
            GeneratedCallableOriginV1::Property(GeneratedPropertyCallableOriginV1::new(
                SourceProgramCallablePathV1::box_method(
                    declaration,
                    gate_path,
                    source_site.source_member_ordinal(),
                ),
                placement.clone(),
                provenance.clone(),
            )),
            placement.name(),
        ));
    }
    if rows.len() != generated_count {
        return Err(reject("generated property relation coverage is incomplete"));
    }
    for entry in inventory.iter_selected_declaration_order() {
        if matches!(
            entry.provenance(),
            BoxMethodProvenanceV1::Generated(
                BoxMethodGeneratedProvenanceV1::MacroOrImport { .. }
                    | BoxMethodGeneratedProvenanceV1::Delegate { .. }
            ) | BoxMethodProvenanceV1::CompatibilityOnly { .. }
        ) {
            return Err(reject(
                "unsupported generated or compatibility origin entered property issuer",
            ));
        }
    }
    Ok(rows.into_boxed_slice())
}

pub(super) fn issue_delegate_callable_rows(
    relations: &[GeneratedDelegateSourceRelationV1],
    member_receipts: &[MemberGateSelectionReceiptV1],
) -> Result<Vec<PreparedGeneratedCallableSourceV1>, String> {
    let mut rows = Vec::with_capacity(relations.len());
    let mut placements = Vec::with_capacity(relations.len());
    for relation in relations {
        let brand = relation.host_box_path().brand();
        if !relation
            .host_delegate_member()
            .box_site()
            .path()
            .brand()
            .same_as(brand)
            || !relation.target_box_path().brand().same_as(brand)
        {
            return Err("generated delegate relation has a foreign parser brand".to_owned());
        }
        let placement = relation.generated_inventory_placement();
        if placement.name() != relation.exposed_method_name() {
            return Err("generated delegate placement/name mismatch".to_owned());
        }
        if placements.iter().any(|(path, ordinal)| {
            path == relation.host_box_path() && *ordinal == placement.inventory_ordinal()
        }) {
            return Err("duplicate generated delegate placement receipt".to_owned());
        }
        let BoxMethodGeneratedProvenanceV1::Delegate {
            field_name,
            exposed_name,
            selection,
        } = relation.generated_name_provenance()
        else {
            return Err("generated delegate relation has the wrong provenance".to_owned());
        };
        if field_name.as_ref() != relation.delegate_field_name()
            || exposed_name.as_ref() != relation.exposed_method_name()
            || !relation
                .host_delegate_member()
                .matches_ast_selection(selection)
        {
            return Err(
                "generated delegate provenance does not match its source relation".to_owned(),
            );
        }
        placements.push((
            relation.host_box_path().clone(),
            placement.inventory_ordinal(),
        ));
        let declaration =
            SourceProgramDeclarationPathV1::from_parser_path(relation.host_box_path().clone());
        let gate_path = exact_generated_gate_path(
            &declaration,
            relation.host_delegate_member(),
            member_receipts,
        )?;
        rows.push(PreparedGeneratedCallableSourceV1::issue(
            brand.clone(),
            GeneratedCallableOriginV1::Delegate(GeneratedDelegateCallableOriginV1::new(
                SourceProgramCallablePathV1::box_method(
                    declaration,
                    gate_path,
                    relation.host_delegate_member().source_member_ordinal(),
                ),
                relation.clone(),
            )),
            relation.exposed_method_name(),
        ));
    }
    Ok(rows)
}

pub(super) fn issue_covered_delegate_callable_rows(
    ast: &ASTNode,
    final_box_paths: &[SourceBoxDeclarationPathV1],
    seals: &[PreparedBoxSourceSealV1],
) -> Result<Vec<PreparedGeneratedCallableSourceV1>, String> {
    let ASTNode::Program { statements, .. } = ast else {
        return Err("generated delegate anchor commit requires a Program".to_owned());
    };
    let final_inventories = statements
        .iter()
        .filter_map(|statement| match statement {
            ASTNode::BoxDeclaration {
                methods,
                is_interface: false,
                is_record: false,
                is_static: false,
                ..
            } => Some(methods),
            _ => None,
        })
        .collect::<Vec<_>>();
    if final_inventories.len() != final_box_paths.len() || seals.len() != final_box_paths.len() {
        return Err("generated delegate anchor host coverage is incomplete".to_owned());
    }
    let mut rows = Vec::new();
    for seal in seals {
        let final_index = final_box_paths
            .iter()
            .position(|path| path == seal.box_site().path())
            .ok_or_else(|| "generated delegate anchor host path is missing".to_owned())?;
        super::source_seal_finalizer::validate_generated_delegate_coverage(
            seal,
            final_inventories[final_index],
        )
        .map_err(|error| format!("generated delegate callable coverage: {error:?}"))?;
        rows.extend(issue_delegate_callable_rows(
            seal.generated_delegate_source_relations(),
            seal.member_gate_selection_receipts(),
        )?);
    }
    Ok(rows)
}

fn exact_generated_gate_path(
    declaration: &SourceProgramDeclarationPathV1,
    source_site: &SourceBoxMethodSiteV1,
    receipts: &[MemberGateSelectionReceiptV1],
) -> Result<Box<[super::source_path::SourceProgramMemberGateStepV1]>, String> {
    match source_site.selected_gate_path() {
        None => Ok(Box::new([])),
        Some(path)
            if path.last().is_some_and(|step| {
                step.branch_member_ordinal() == source_site.source_member_ordinal()
            }) =>
        {
            let matches = receipts
                .iter()
                .filter_map(|receipt| {
                    receipt.exact_selected_path_for_method_site(declaration, source_site)
                })
                .collect::<Vec<_>>();
            if matches.len() != 1 {
                return Err(format!(
                    "generated callable member-gate receipt coverage mismatch: {}",
                    matches.len()
                ));
            }
            Ok(matches[0].to_vec().into_boxed_slice())
        }
        Some(_) => Err("generated callable selection does not match its source member".to_owned()),
    }
}

fn reject(detail: impl Into<String>) -> ParseError {
    ParseError::GrammarContract {
        stable_reject_tag: REJECT_TAG,
        detail: detail.into(),
        line: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::callable_source_anchor::GeneratedCallableOriginV1;
    use crate::parser::source_authority::ParserInvocationBrandV1;
    use crate::parser::{BuildMode, NyashParser, ParserBuildConfig};
    use crate::tokenizer::NyashTokenizer;

    fn open_property_product() -> crate::parser::source_seal::OpenParserPostpassProductV1 {
        let tokens = NyashTokenizer::new("box Generated { once value: i64 => 1 }\n")
            .tokenize()
            .unwrap();
        let mut parser = NyashParser::new(tokens);
        let ast = parser.parse_program().unwrap();
        parser.open_postpass_product(ast).unwrap()
    }

    #[test]
    fn property_generator_issues_fresh_anchor_per_exact_placement() {
        let product = open_property_product();
        let rows =
            &product.source_session.prepared_source_seals[0].generated_property_callable_rows;
        assert_eq!(rows.len(), 2);
        assert!(!rows[0].anchor().same_as(rows[1].anchor()));
        assert!(rows.iter().all(|row| matches!(
            row.origin(),
            GeneratedCallableOriginV1::Property(origin)
                if origin.placement().name() == row.diagnostic_name()
        )));
    }

    #[test]
    fn property_generator_rejects_missing_duplicate_and_foreign_receipts() {
        let product = open_property_product();
        let seal = &product.source_session.prepared_source_seals[0];
        let brand = seal.brand.clone();
        assert!(issue_property_callable_rows(&brand, seal.inventory(), &[], &[]).is_err());

        let mut duplicate = seal.method_relations().to_vec();
        duplicate.push(duplicate[0].clone());
        assert!(issue_property_callable_rows(&brand, seal.inventory(), &duplicate, &[]).is_err());

        let foreign = ParserInvocationBrandV1::issue();
        assert!(issue_property_callable_rows(
            &foreign,
            seal.inventory(),
            seal.method_relations(),
            &[],
        )
        .is_err());
    }

    fn open_lowered_delegate(
        source: &str,
        mode: BuildMode,
    ) -> crate::parser::source_seal::OpenParserPostpassProductV1 {
        let tokens = NyashTokenizer::new(source).tokenize().unwrap();
        let mut parser = NyashParser::new(tokens).with_build_config(ParserBuildConfig {
            mode,
            ..ParserBuildConfig::default()
        });
        let ast = parser.parse_program().unwrap();
        parser
            .open_postpass_product(ast)
            .unwrap()
            .prune_build_gates(&parser)
            .unwrap()
            .lower_delegates()
            .unwrap()
    }

    #[test]
    fn delegate_generator_issues_fresh_anchor_from_exact_relation() {
        let product = open_lowered_delegate(
            r#"
box Target { run() { return 1 } }
box Host {
    target: Target
    delegate target exposes { run as runAlias }
}
"#,
            BuildMode::Release,
        );
        let rows = product.source_session.callable_rows();
        let generated = rows
            .iter()
            .filter_map(|row| row.generated())
            .find(|row| row.diagnostic_name() == "runAlias")
            .expect("delegate generator must issue one fresh callable anchor");
        assert!(matches!(
            generated.origin(),
            GeneratedCallableOriginV1::Delegate(origin)
                if origin.relation().exposed_method_name() == "runAlias"
        ));
        assert!(rows
            .iter()
            .filter_map(|row| row.direct())
            .all(|direct| !direct.anchor().same_as(generated.anchor())));
    }

    #[test]
    fn selected_member_gate_preserves_exact_delegate_origin_branch() {
        let product = open_lowered_delegate(
            r#"
box Target { run() { return 1 } }
box Host {
    target: Target
    gate Build.test {
        delegate target exposes { run as runAlias }
    } else {
        delegate target exposes { run as runAlias }
    }
}
"#,
            BuildMode::Test,
        );
        let generated = product
            .source_session
            .callable_rows()
            .iter()
            .filter_map(|row| row.generated())
            .find(|row| row.diagnostic_name() == "runAlias")
            .expect("selected delegate must issue one callable anchor");
        assert!(matches!(
            generated.origin(),
            GeneratedCallableOriginV1::Delegate(origin)
                if matches!(
                    origin.source_path(),
                    SourceProgramCallablePathV1::BoxMethod { gate_path, .. }
                        if gate_path.len() == 1
                            && gate_path[0].branch()
                                == super::super::source_authority::SourceBuildGateBranchV1::Then
                )
        ));
    }

    #[test]
    fn selected_member_gate_keeps_only_selected_generated_property_origins() {
        let tokens = NyashTokenizer::new(
            "box Choice { gate Build.test { once value: i64 => 1 } else { once value: i64 => 2 } }\n",
        )
        .tokenize()
        .unwrap();
        let mut parser = NyashParser::new(tokens).with_build_config(ParserBuildConfig {
            mode: BuildMode::Test,
            ..ParserBuildConfig::default()
        });
        let ast = parser.parse_program().unwrap();
        let product = parser
            .open_postpass_product(ast)
            .unwrap()
            .prune_build_gates(&parser)
            .unwrap();
        let names = product
            .source_session
            .callable_rows()
            .iter()
            .filter_map(|row| row.generated())
            .map(|row| row.diagnostic_name())
            .collect::<Vec<_>>();
        assert_eq!(names, vec!["__compute_once_value", "__get_once_value"]);
        assert!(product
            .source_session
            .callable_rows()
            .iter()
            .filter_map(|row| row.generated())
            .all(|row| matches!(
                row.origin(),
                GeneratedCallableOriginV1::Property(origin)
                    if matches!(
                        origin.source_path(),
                        SourceProgramCallablePathV1::BoxMethod { gate_path, .. }
                            if gate_path.len() == 1
                                && gate_path[0].branch()
                                    == super::super::source_authority::SourceBuildGateBranchV1::Then
                    )
            )));
    }

    #[test]
    fn delegate_anchor_coverage_rejects_missing_and_duplicate_relations() {
        let mut product = open_lowered_delegate(
            r#"
box Target { run() { return 1 } }
box Host {
    target: Target
    delegate target exposes { run as runAlias }
}
"#,
            BuildMode::Release,
        );
        let relation = {
            let host = product
                .source_session
                .prepared_source_seals
                .iter_mut()
                .find(|seal| !seal.generated_delegate_source_relations().is_empty())
                .unwrap();
            let relation = host.generated_delegate_source_relations()[0].clone();
            host.generated_delegate_source_relations = Box::new([]);
            relation
        };
        assert!(issue_covered_delegate_callable_rows(
            &product.ast,
            &product.final_box_paths,
            &product.source_session.prepared_source_seals,
        )
        .is_err());
        let host = product
            .source_session
            .prepared_source_seals
            .iter_mut()
            .find(|seal| seal.box_site().path() == relation.host_box_path())
            .unwrap();
        host.generated_delegate_source_relations =
            vec![relation.clone(), relation].into_boxed_slice();
        assert!(issue_covered_delegate_callable_rows(
            &product.ast,
            &product.final_box_paths,
            &product.source_session.prepared_source_seals,
        )
        .is_err());
    }

    #[test]
    fn delegate_generator_rejects_foreign_source_relation() {
        let product = open_lowered_delegate(
            r#"
box Target { run() { return 1 } }
box Host {
    target: Target
    delegate target exposes { run as runAlias }
}
"#,
            BuildMode::Release,
        );
        let host = product
            .source_session
            .prepared_source_seals
            .iter()
            .find(|seal| !seal.generated_delegate_source_relations().is_empty())
            .unwrap();
        let relation = &host.generated_delegate_source_relations()[0];
        let foreign_host = SourceBoxDeclarationPathV1::root(
            ParserInvocationBrandV1::issue(),
            relation
                .host_box_path()
                .root_statement_ordinal()
                .expect("direct delegate fixture has one root host"),
        );
        let foreign = GeneratedDelegateSourceRelationV1::new(
            foreign_host,
            relation.host_delegate_member().clone(),
            relation.expose_ordinal(),
            relation.delegate_field_name(),
            relation.source_method_name(),
            relation.exposed_method_name(),
            relation.target_box_path().clone(),
            relation.target_method_source_ref().clone(),
            relation.generated_inventory_placement().clone(),
            relation.generated_name_provenance().clone(),
        );
        assert!(
            issue_delegate_callable_rows(&[foreign], host.member_gate_selection_receipts(),)
                .is_err()
        );
    }
}
