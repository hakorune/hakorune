//! Parser-private gate selection and pruning for direct callable rows.

use super::callable_source_anchor::PreparedDirectCallableSourceV1;
use super::source_authority::{ParserInvocationBrandV1, PreparedBoxSourceSealV1};
use super::source_path::{
    SourceBuildGateBranchV1, SourceProgramCallablePathV1, SourceProgramDeclarationPathV1,
    SourceProgramMemberGateStepV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MemberGateSelectionReceiptV1 {
    brand: ParserInvocationBrandV1,
    declaration: SourceProgramDeclarationPathV1,
    selected_path: Box<[SourceProgramMemberGateStepV1]>,
}

impl MemberGateSelectionReceiptV1 {
    pub(super) fn issue_from_selected_path(
        declaration: SourceProgramDeclarationPathV1,
        parent_path: &[SourceProgramMemberGateStepV1],
        selected_path: &[SourceProgramMemberGateStepV1],
        gate_member_ordinal: u32,
    ) -> Result<Option<Self>, &'static str> {
        if selected_path == parent_path {
            return Ok(None);
        }
        if selected_path.len() != parent_path.len() + 1
            || selected_path[..parent_path.len()] != *parent_path
        {
            return Err("selected member gate path is not one exact child of its parent");
        }
        let selected = selected_path
            .last()
            .expect("exact child member-gate path is non-empty");
        if selected.gate_member_ordinal() != gate_member_ordinal {
            return Err("selected member gate path does not end at the merged gate");
        }
        Ok(Some(Self {
            brand: declaration.brand().clone(),
            declaration,
            selected_path: selected_path.to_vec().into_boxed_slice(),
        }))
    }

    fn branch(&self) -> SourceBuildGateBranchV1 {
        self.selected_path
            .last()
            .expect("member gate receipt has a non-empty selected path")
            .branch()
    }

    fn matches_gate(
        &self,
        declaration: &SourceProgramDeclarationPathV1,
        path: &[SourceProgramMemberGateStepV1],
        gate_index: usize,
    ) -> bool {
        self.declaration == *declaration
            && self.selected_path.len() == gate_index + 1
            && self.selected_path[..gate_index] == path[..gate_index]
            && self.selected_path[gate_index].gate_member_ordinal()
                == path[gate_index].gate_member_ordinal()
    }

    pub(super) fn same_gate_as(&self, other: &Self) -> bool {
        self.matches_gate(
            &other.declaration,
            &other.selected_path,
            other.selected_path.len() - 1,
        )
    }

    pub(super) fn brand_matches(&self, brand: &ParserInvocationBrandV1) -> bool {
        self.brand.same_as(brand) && self.declaration.brand().same_as(brand)
    }
}

pub(super) fn prune_direct_callable_rows(
    rows: Vec<PreparedDirectCallableSourceV1>,
    seals: &[PreparedBoxSourceSealV1],
    declaration_survives: impl Fn(&SourceProgramDeclarationPathV1) -> Result<bool, String>,
) -> Result<Vec<PreparedDirectCallableSourceV1>, String> {
    let mut retained = Vec::with_capacity(rows.len());
    for row in rows {
        if !declaration_survives(row.path().declaration())? {
            continue;
        }
        let SourceProgramCallablePathV1::BoxMethod {
            declaration,
            gate_path,
            ..
        } = row.path()
        else {
            retained.push(row);
            continue;
        };
        if gate_path.is_empty() {
            retained.push(row);
            continue;
        }
        let seal = seals
            .iter()
            .find(|seal| seal.box_site().path() == declaration.compatibility_box_path())
            .ok_or_else(|| "gated callable row has no exact Box source seal".to_owned())?;
        if !row.parser_brand().same_as(&seal.brand) {
            return Err("foreign parser brand in callable member-gate row".to_owned());
        }
        let mut selected = true;
        for gate_index in 0..gate_path.len() {
            let matches = seal
                .member_gate_selection_receipts()
                .iter()
                .filter(|receipt| receipt.matches_gate(declaration, gate_path, gate_index))
                .collect::<Vec<_>>();
            if matches.len() != 1 {
                return Err(format!(
                    "callable member-gate receipt coverage mismatch at depth {}: {}",
                    gate_index,
                    matches.len()
                ));
            }
            if !matches[0].brand_matches(row.parser_brand()) {
                return Err("foreign parser brand in callable member-gate receipt".to_owned());
            }
            if matches[0].branch() != gate_path[gate_index].branch() {
                selected = false;
                break;
            }
        }
        if selected {
            retained.push(row);
        }
    }
    Ok(retained)
}

#[cfg(test)]
mod tests {
    use super::MemberGateSelectionReceiptV1;
    use crate::parser::build_cfg::project_build_gates;
    use crate::parser::source_authority::{
        ParserInvocationBrandV1, SourceBoxDeclarationPathV1, SourceBuildGateBranchV1,
        SourceProgramDeclarationPathV1, SourceProgramMemberGateStepV1,
    };
    use crate::parser::NyashParser;
    use crate::tokenizer::NyashTokenizer;

    fn open_member_gate() -> (
        NyashParser,
        crate::parser::source_seal::OpenParserPostpassProductV1,
    ) {
        let tokens =
            NyashTokenizer::new("box Choice { gate Build.test { run() {} } else { run() {} } }\n")
                .tokenize()
                .unwrap();
        let mut parser = NyashParser::new(tokens);
        let ast = parser.parse_program().unwrap();
        let product = parser.open_postpass_product(ast).unwrap();
        (parser, product)
    }

    fn open_top_level_gate() -> (
        NyashParser,
        crate::parser::source_seal::OpenParserPostpassProductV1,
    ) {
        let tokens = NyashTokenizer::new(
            "gate Build.test { function chosen() {} } else { function hidden() {} }\n",
        )
        .tokenize()
        .unwrap();
        let mut parser = NyashParser::new(tokens);
        let ast = parser.parse_program().unwrap();
        let product = parser.open_postpass_product(ast).unwrap();
        (parser, product)
    }

    #[test]
    fn mismatched_member_gate_path_cannot_issue_a_receipt() {
        let brand = ParserInvocationBrandV1::issue();
        let declaration = SourceProgramDeclarationPathV1::from_parser_path(
            SourceBoxDeclarationPathV1::root(brand, 0),
        );
        let path = [SourceProgramMemberGateStepV1::new(
            1,
            SourceBuildGateBranchV1::Then,
        )];
        assert!(
            MemberGateSelectionReceiptV1::issue_from_selected_path(declaration, &[], &path, 2,)
                .is_err()
        );
    }

    #[test]
    fn exact_no_else_paths_issue_no_member_receipt() {
        let brand = ParserInvocationBrandV1::issue();
        let declaration = SourceProgramDeclarationPathV1::from_parser_path(
            SourceBoxDeclarationPathV1::root(brand, 0),
        );
        assert!(MemberGateSelectionReceiptV1::issue_from_selected_path(
            declaration.clone(),
            &[],
            &[],
            0,
        )
        .unwrap()
        .is_none());
        let parent = [SourceProgramMemberGateStepV1::new(
            3,
            SourceBuildGateBranchV1::Else,
        )];
        assert!(MemberGateSelectionReceiptV1::issue_from_selected_path(
            declaration,
            &parent,
            &parent,
            4,
        )
        .unwrap()
        .is_none());
    }

    #[test]
    fn missing_top_level_receipt_rejects_before_callable_publication() {
        let (_parser, product) = open_top_level_gate();
        let error = product
            .source_session
            .prepare_prune(Vec::new())
            .unwrap_err();
        assert!(format!("{error}").contains("receipt coverage mismatch"));
    }

    #[test]
    fn duplicate_top_level_receipt_rejects_before_callable_publication() {
        let (parser, product) = open_top_level_gate();
        let projection = project_build_gates(
            &parser,
            product.ast,
            &product.build_gate_decision_set,
            product.source_session.gate_records(),
            false,
        )
        .unwrap();
        let receipt = projection.receipts[0].clone();
        let error = product
            .source_session
            .prepare_prune(vec![receipt.clone(), receipt])
            .unwrap_err();
        assert!(format!("{error}").contains("receipt coverage mismatch"));
    }

    #[test]
    fn foreign_top_level_receipt_rejects_before_callable_publication() {
        let (parser, product) = open_top_level_gate();
        let mut projection = project_build_gates(
            &parser,
            product.ast,
            &product.build_gate_decision_set,
            product.source_session.gate_records(),
            false,
        )
        .unwrap();
        projection.receipts[0].brand = ParserInvocationBrandV1::issue();
        let error = product
            .source_session
            .prepare_prune(projection.receipts)
            .unwrap_err();
        assert!(format!("{error}").contains("foreign parser brand"));
    }

    #[test]
    fn missing_member_receipt_rejects_before_callable_publication() {
        let (parser, mut product) = open_member_gate();
        product.source_session.prepared_source_seals[0].member_gate_selection_receipts =
            Box::new([]);
        let error = product.prune_build_gates(&parser).unwrap_err();
        assert!(format!("{error}").contains("receipt coverage mismatch"));
    }

    #[test]
    fn duplicate_member_receipt_rejects_before_callable_publication() {
        let (parser, mut product) = open_member_gate();
        let duplicate = product.source_session.prepared_source_seals[0]
            .member_gate_selection_receipts[0]
            .clone();
        product.source_session.prepared_source_seals[0].member_gate_selection_receipts =
            vec![duplicate.clone(), duplicate].into_boxed_slice();
        let error = product.prune_build_gates(&parser).unwrap_err();
        assert!(format!("{error}").contains("receipt coverage mismatch"));
    }

    #[test]
    fn foreign_member_receipt_rejects_before_callable_publication() {
        let (parser, mut product) = open_member_gate();
        product.source_session.prepared_source_seals[0].member_gate_selection_receipts[0].brand =
            ParserInvocationBrandV1::issue();
        let error = product.prune_build_gates(&parser).unwrap_err();
        assert!(format!("{error}").contains("foreign parser brand"));
    }
}
