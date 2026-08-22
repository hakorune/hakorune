use crate::ast::BoxMethodInventoryErrorV1;
use crate::parser::ParseError;

use super::SourceAuthorityErrorV1;

pub(in crate::parser) fn source_authority_to_parse_error(
    error: SourceAuthorityErrorV1,
) -> ParseError {
    let message = match error {
        SourceAuthorityErrorV1::ForeignBoxSite => {
            "Box source site belongs to another parser invocation".to_owned()
        }
        SourceAuthorityErrorV1::StaleMemberSite => "Box source member site is stale".to_owned(),
        SourceAuthorityErrorV1::MemberOrdinalOverflow => {
            "Box member ordinal exceeds u32".to_owned()
        }
        SourceAuthorityErrorV1::ExposeOrdinalOverflow => {
            "delegate expose ordinal exceeds u32".to_owned()
        }
        SourceAuthorityErrorV1::DelegateCompatibilityOnly => {
            "compatibility-only delegate cannot issue parser source declarations".to_owned()
        }
        SourceAuthorityErrorV1::MissingMethodSourceRelation { inventory_ordinal } => {
            format!("Box source relation missing for inventory ordinal {inventory_ordinal}")
        }
        SourceAuthorityErrorV1::MethodSourceRelationMismatch { name } => {
            format!("Box source relation does not match method `{name}`")
        }
        SourceAuthorityErrorV1::DuplicateConstructorKey(key) => {
            format!("duplicate constructor source key `{key}`")
        }
        SourceAuthorityErrorV1::ConstructorMissing(key) => {
            format!("constructor source row `{key}` is missing from the AST inventory")
        }
        SourceAuthorityErrorV1::ConstructorShapeMismatch(key) => {
            format!("constructor source row `{key}` does not match its AST declaration")
        }
        SourceAuthorityErrorV1::ConstructorCoverageMismatch(relations, constructors) => format!(
            "constructor source coverage mismatch: relations={relations}, constructors={constructors}"
        ),
        SourceAuthorityErrorV1::Inventory(error) => {
            return crate::parser::declarations::box_def::members::pending_method::map_inventory_error(
                error,
            );
        }
    };
    ParseError::BuildCfg { message, line: 0 }
}

pub(in crate::parser) fn inventory_error_to_parse_error(
    error: BoxMethodInventoryErrorV1,
) -> ParseError {
    crate::parser::declarations::box_def::members::pending_method::map_inventory_error(error)
}
