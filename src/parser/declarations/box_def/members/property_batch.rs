//! Atomic generated-method transaction for one source property.

use crate::ast::{
    ASTNode, BoxMethodGeneratedProvenanceV1, BoxMethodSourceSelectionV1,
    PreparedGeneratedBoxMethodBatchV1, PreparedGeneratedBoxMethodV1, Span,
};
use crate::parser::declarations::box_def::members::{pending_method, property_emit};
use crate::parser::source_authority::GeneratedPropertySink;
use crate::parser::ParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PropertyMemberKindV1 {
    Computed,
    Once,
    BirthOnce,
}

impl PropertyMemberKindV1 {
    pub(super) fn from_keyword(keyword: &str) -> Option<Self> {
        match keyword {
            "once" => Some(Self::Once),
            "birth_once" => Some(Self::BirthOnce),
            _ => None,
        }
    }

    fn emit(
        self,
        property_name: String,
        body: Vec<ASTNode>,
    ) -> Vec<property_emit::GeneratedPropertyMethodV1> {
        match self {
            Self::Computed => property_emit::computed_getter(property_name, body),
            Self::Once => property_emit::once_methods(property_name, body),
            Self::BirthOnce => property_emit::birth_once_methods(property_name, body),
        }
    }
}

/// All generated methods for one property remain unpublished until every row
/// and collision has passed preflight.
pub(super) struct PreparedGeneratedPropertyMethodBatchV1 {
    property_name: String,
    birth_once: bool,
    methods: PreparedGeneratedBoxMethodBatchV1,
}

impl PreparedGeneratedPropertyMethodBatchV1 {
    pub(super) fn prepare(
        kind: PropertyMemberKindV1,
        property_name: String,
        body: Vec<ASTNode>,
        diagnostic_span: Span,
    ) -> Result<Self, ParseError> {
        let generated = kind.emit(property_name.clone(), body);
        let provenance = || BoxMethodGeneratedProvenanceV1::Property {
            property_name: property_name.clone().into_boxed_str(),
            selection: BoxMethodSourceSelectionV1::Direct,
        };
        let rows = generated
            .into_iter()
            .map(|method| {
                let (name, declaration) = method.into_parts();
                PreparedGeneratedBoxMethodV1::new(name, declaration, provenance(), diagnostic_span)
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(pending_method::map_inventory_error)?;
        let methods = PreparedGeneratedBoxMethodBatchV1::try_new(rows)
            .map_err(pending_method::map_inventory_error)?;
        Ok(Self {
            property_name,
            birth_once: kind == PropertyMemberKindV1::BirthOnce,
            methods,
        })
    }

    pub(super) fn commit(
        self,
        sink: &mut impl GeneratedPropertySink,
    ) -> Result<Option<String>, ParseError> {
        sink.commit_generated_property_batch_at_current(self.methods)?;
        Ok(self.birth_once.then_some(self.property_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BoxMethodInventoryV1, BoxMethodProvenanceV1, DeclarationAttrs};

    fn function(name: &str, span: Span) -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: name.to_owned(),
            params: vec![],
            param_decls: vec![],
            return_type_name: None,
            body: vec![],
            uses: vec![],
            contracts: vec![],
            is_static: false,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span,
        }
    }

    #[test]
    fn property_rows_commit_in_emission_order_with_generated_provenance() {
        let span = Span::new(3, 4, 5, 6);
        let batch = PreparedGeneratedPropertyMethodBatchV1::prepare(
            PropertyMemberKindV1::Once,
            "value".to_owned(),
            vec![],
            span,
        )
        .unwrap();
        let mut inventory = BoxMethodInventoryV1::empty();

        let birth_once = batch.commit(&mut inventory).unwrap();

        assert_eq!(
            inventory.names_in_selected_order().collect::<Vec<_>>(),
            vec!["__compute_once_value", "__get_once_value"]
        );
        for entry in inventory.iter_selected_declaration_order() {
            assert_eq!(entry.diagnostic_span(), span);
            assert!(matches!(
                entry.provenance(),
                BoxMethodProvenanceV1::Generated(
                    BoxMethodGeneratedProvenanceV1::Property {
                        property_name,
                        selection: BoxMethodSourceSelectionV1::Direct,
                    }
                ) if property_name.as_ref() == "value"
            ));
        }
        assert!(birth_once.is_none());
    }

    #[test]
    fn collision_rejects_whole_batch_and_birth_once_side_effect() {
        let first_span = Span::new(0, 0, 2, 3);
        let duplicate_span = Span::new(0, 0, 7, 8);
        let mut inventory = BoxMethodInventoryV1::empty();
        inventory
            .try_push_explicit_source(
                "__get_birth_value",
                function("__get_birth_value", first_span),
                first_span,
            )
            .unwrap();
        let batch = PreparedGeneratedPropertyMethodBatchV1::prepare(
            PropertyMemberKindV1::BirthOnce,
            "value".to_owned(),
            vec![],
            duplicate_span,
        )
        .unwrap();

        let error = batch.commit(&mut inventory).unwrap_err();

        assert!(matches!(
            error,
            ParseError::DuplicateBoxMethod {
                first_line: 2,
                first_column: 3,
                duplicate_line: 7,
                duplicate_column: 8,
                ..
            }
        ));
        assert_eq!(inventory.len(), 1);
        assert!(!inventory.contains_name("__compute_birth_value"));
    }

    #[test]
    fn birth_once_tracking_happens_after_successful_commit() {
        let batch = PreparedGeneratedPropertyMethodBatchV1::prepare(
            PropertyMemberKindV1::BirthOnce,
            "value".to_owned(),
            vec![],
            Span::new(0, 0, 4, 2),
        )
        .unwrap();
        let mut inventory = BoxMethodInventoryV1::empty();

        let birth_once = batch.commit(&mut inventory).unwrap();

        assert_eq!(birth_once.as_deref(), Some("value"));
        assert_eq!(inventory.len(), 2);
    }
}
