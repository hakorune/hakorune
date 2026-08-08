use std::collections::HashMap;

use crate::{ASTNode, Span};

use super::{
    BoxMethodDeclarationSiteV1, BoxMethodEntryV1, BoxMethodInventoryErrorV1, BoxMethodInventoryV1,
    BoxMethodProvenanceV1,
};

/// One complete descriptive row from the roundtrip-v2 wire format.
///
/// This row carries no resolver capability. Its ordinal and provenance are
/// accepted only after the complete transaction passes preflight.
#[derive(Debug, Clone, PartialEq)]
pub struct BoxMethodInventoryRoundtripRowV2 {
    name: Box<str>,
    selected_method_ordinal: u32,
    declaration: ASTNode,
    provenance: BoxMethodProvenanceV1,
    diagnostic_span: Span,
}

impl BoxMethodInventoryRoundtripRowV2 {
    pub fn new(
        name: impl Into<Box<str>>,
        selected_method_ordinal: u32,
        declaration: ASTNode,
        provenance: BoxMethodProvenanceV1,
        diagnostic_span: Span,
    ) -> Self {
        Self {
            name: name.into(),
            selected_method_ordinal,
            declaration,
            provenance,
            diagnostic_span,
        }
    }
}

/// A fully preflighted, unpublished roundtrip-v2 reconstruction transaction.
///
/// `commit` is infallible: declaration identity, duplicate names, contiguous
/// selected ordinals, and selected-gate provenance are checked for every row
/// before this product can exist.
#[derive(Debug, Clone, PartialEq)]
pub struct PreparedBoxMethodInventoryRoundtripV2 {
    entries: Box<[BoxMethodEntryV1]>,
}

impl PreparedBoxMethodInventoryRoundtripV2 {
    pub fn try_new(
        rows: impl IntoIterator<Item = BoxMethodInventoryRoundtripRowV2>,
    ) -> Result<Self, BoxMethodInventoryErrorV1> {
        let rows = rows.into_iter().collect::<Vec<_>>();
        u32::try_from(rows.len()).map_err(|_| BoxMethodInventoryErrorV1::OrdinalOverflow)?;

        let mut names = HashMap::<&str, Span>::with_capacity(rows.len());
        for (index, row) in rows.iter().enumerate() {
            let expected =
                u32::try_from(index).map_err(|_| BoxMethodInventoryErrorV1::OrdinalOverflow)?;
            if row.selected_method_ordinal != expected {
                return Err(
                    BoxMethodInventoryErrorV1::NonContiguousSelectedMethodOrdinal {
                        expected,
                        found: row.selected_method_ordinal,
                    },
                );
            }
            BoxMethodInventoryV1::validate_declaration_name(&row.name, &row.declaration)?;
            row.provenance.validate_transport()?;
            if let Some(first_span) = names.insert(row.name.as_ref(), row.diagnostic_span) {
                return Err(BoxMethodInventoryErrorV1::DuplicateMethod {
                    name: row.name.clone(),
                    first_span,
                    duplicate_span: row.diagnostic_span,
                });
            }
        }

        let entries = rows
            .into_iter()
            .map(|row| BoxMethodEntryV1 {
                name: row.name,
                declaration: row.declaration,
                provenance: row.provenance,
                site: BoxMethodDeclarationSiteV1 {
                    selected_method_ordinal: row.selected_method_ordinal,
                },
                diagnostic_span: row.diagnostic_span,
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Ok(Self { entries })
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn commit(self) -> BoxMethodInventoryV1 {
        let mut lookup = HashMap::with_capacity(self.entries.len());
        for (index, entry) in self.entries.iter().enumerate() {
            lookup.insert(entry.name.clone(), index);
        }
        BoxMethodInventoryV1 {
            entries: self.entries.into_vec(),
            lookup,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        BoxMemberGateSiteV1, BoxMethodCompatibilityOriginV1, BoxMethodGateSelectionV1,
        BoxMethodGeneratedProvenanceV1, BoxMethodSourceSelectionV1, DeclarationAttrs,
    };

    use super::*;

    fn function(name: &str, span: Span) -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: name.to_owned(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: Vec::new(),
            contracts: Vec::new(),
            uses: Vec::new(),
            is_static: false,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span,
        }
    }

    fn row(
        name: &str,
        ordinal: u32,
        provenance: BoxMethodProvenanceV1,
        span: Span,
    ) -> BoxMethodInventoryRoundtripRowV2 {
        BoxMethodInventoryRoundtripRowV2::new(name, ordinal, function(name, span), provenance, span)
    }

    #[test]
    fn complete_transaction_preserves_order_provenance_and_spans() {
        let direct_span = Span::new(0, 2, 3, 4);
        let selected_span = Span::new(3, 7, 8, 2);
        let selection = BoxMethodSourceSelectionV1::selected_build_gate([
            BoxMethodGateSelectionV1::from_parts(
                BoxMemberGateSiteV1::from_box_member_ordinal(6),
                2,
            ),
            BoxMethodGateSelectionV1::from_parts(
                BoxMemberGateSiteV1::from_box_member_ordinal(9),
                1,
            ),
        ])
        .unwrap();
        let transaction = PreparedBoxMethodInventoryRoundtripV2::try_new([
            row(
                "direct",
                0,
                BoxMethodProvenanceV1::ExplicitSource {
                    selection: BoxMethodSourceSelectionV1::Direct,
                },
                direct_span,
            ),
            row(
                "__get_value",
                1,
                BoxMethodProvenanceV1::Generated(BoxMethodGeneratedProvenanceV1::Property {
                    property_name: "value".into(),
                    selection: selection.clone(),
                }),
                selected_span,
            ),
            row(
                "legacy",
                2,
                BoxMethodProvenanceV1::CompatibilityOnly {
                    origin: BoxMethodCompatibilityOriginV1::LegacyJsonV1,
                },
                Span::unknown(),
            ),
        ])
        .unwrap();

        assert_eq!(transaction.len(), 3);
        let inventory = transaction.commit();
        assert_eq!(
            inventory.names_in_selected_order().collect::<Vec<_>>(),
            vec!["direct", "__get_value", "legacy"]
        );
        assert_eq!(
            inventory.get("direct").unwrap().diagnostic_span(),
            direct_span
        );
        let selected = inventory.get("__get_value").unwrap();
        assert_eq!(selected.site().selected_method_ordinal(), 1);
        assert_eq!(selected.diagnostic_span(), selected_span);
        assert_eq!(
            selected.provenance(),
            &BoxMethodProvenanceV1::Generated(BoxMethodGeneratedProvenanceV1::Property {
                property_name: "value".into(),
                selection,
            })
        );
    }

    #[test]
    fn non_contiguous_ordinal_rejects_before_transaction_exists() {
        let error = PreparedBoxMethodInventoryRoundtripV2::try_new([
            row(
                "first",
                0,
                BoxMethodProvenanceV1::ExplicitSource {
                    selection: BoxMethodSourceSelectionV1::Direct,
                },
                Span::unknown(),
            ),
            row(
                "third",
                2,
                BoxMethodProvenanceV1::ExplicitSource {
                    selection: BoxMethodSourceSelectionV1::Direct,
                },
                Span::unknown(),
            ),
        ])
        .unwrap_err();

        assert_eq!(
            error,
            BoxMethodInventoryErrorV1::NonContiguousSelectedMethodOrdinal {
                expected: 1,
                found: 2,
            }
        );
    }

    #[test]
    fn duplicate_name_rejects_complete_batch() {
        let first_span = Span::new(0, 1, 1, 1);
        let duplicate_span = Span::new(2, 3, 2, 1);
        let error = PreparedBoxMethodInventoryRoundtripV2::try_new([
            row(
                "run",
                0,
                BoxMethodProvenanceV1::ExplicitSource {
                    selection: BoxMethodSourceSelectionV1::Direct,
                },
                first_span,
            ),
            row(
                "run",
                1,
                BoxMethodProvenanceV1::ExplicitSource {
                    selection: BoxMethodSourceSelectionV1::Direct,
                },
                duplicate_span,
            ),
        ])
        .unwrap_err();

        assert_eq!(
            error,
            BoxMethodInventoryErrorV1::DuplicateMethod {
                name: "run".into(),
                first_span,
                duplicate_span,
            }
        );
    }

    #[test]
    fn declaration_name_mismatch_rejects_complete_batch() {
        let error = PreparedBoxMethodInventoryRoundtripV2::try_new([
            BoxMethodInventoryRoundtripRowV2::new(
                "declared",
                0,
                function("different", Span::unknown()),
                BoxMethodProvenanceV1::ExplicitSource {
                    selection: BoxMethodSourceSelectionV1::Direct,
                },
                Span::unknown(),
            ),
        ])
        .unwrap_err();

        assert!(matches!(
            error,
            BoxMethodInventoryErrorV1::DeclarationNameMismatch { .. }
        ));
    }

    #[test]
    fn empty_selected_gate_path_rejects_complete_batch() {
        let error = PreparedBoxMethodInventoryRoundtripV2::try_new([row(
            "selected",
            0,
            BoxMethodProvenanceV1::ExplicitSource {
                selection: BoxMethodSourceSelectionV1::SelectedBuildGate { path: Box::new([]) },
            },
            Span::unknown(),
        )])
        .unwrap_err();

        assert_eq!(error, BoxMethodInventoryErrorV1::EmptySelectedBuildGatePath);
    }
}
