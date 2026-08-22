/*!
 * Special Call Handlers
 *
 * Handles math functions, type operations, and other special cases
 * These require custom processing beyond standard method calls
 */

use crate::ast::ASTNode;
use crate::mir::numeric_substrate::is_numeric_integer_type_name;
use crate::mir::policies::source_method_typeop_route::{
    classify_source_method_typeop_route_v1, extract_source_string_literal_v1,
    SourceMethodTypeOpDispositionV1,
};
use crate::mir::MirType;

/// Check if a function is a math function
pub fn is_math_function(name: &str) -> bool {
    matches!(
        name,
        "sin" | "cos" | "abs" | "min" | "max" | "sqrt" | "pow" | "floor" | "ceil"
    )
}

/// Check if a method is a type operation (.is() or .as())
pub fn is_typeop_method(method: &str, arguments: &[ASTNode]) -> Option<String> {
    match classify_source_method_typeop_route_v1(method, arguments) {
        SourceMethodTypeOpDispositionV1::TypeOp { type_name, .. } => Some(type_name.to_string()),
        SourceMethodTypeOpDispositionV1::Ordinary => None,
    }
}

/// Extract string literal from AST node if possible
/// Handles both direct literals and StringBox constructors
pub fn extract_string_literal(node: &ASTNode) -> Option<String> {
    extract_source_string_literal_v1(node)
}

/// Map a user-facing type name to MIR type
pub fn parse_type_name_to_mir(name: &str) -> MirType {
    if is_numeric_integer_type_name(name) {
        return MirType::Integer;
    }

    match name {
        // Core primitive types only (no Box suffixes)
        "Integer" | "Int" | "I64" => MirType::Integer,
        "Float" | "F64" => MirType::Float,
        "Bool" | "Boolean" => MirType::Bool,
        "String" => MirType::String,
        "Void" | "Unit" => MirType::Void,
        // Phase 15.5: All Box types (including former core IntegerBox, StringBox, etc.) treated uniformly
        other => MirType::Box(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::parse_type_name_to_mir;
    use crate::mir::MirType;

    #[test]
    fn parse_type_name_to_mir_maps_numeric_substrate_names_to_integer_lane() {
        for name in [
            "i8", "i16", "i32", "i64", "isize", "u8", "u16", "u32", "u64", "usize",
        ] {
            assert_eq!(parse_type_name_to_mir(name), MirType::Integer);
        }
    }

    #[test]
    fn parse_type_name_to_mir_keeps_box_suffix_types_as_boxes() {
        assert_eq!(
            parse_type_name_to_mir("IntegerBox"),
            MirType::Box("IntegerBox".to_string())
        );
    }
}

/// Check if an AST node contains a return statement with value
pub fn contains_value_return(nodes: &[ASTNode]) -> bool {
    fn node_has_value_return(node: &ASTNode) -> bool {
        match node {
            ASTNode::Return { value: Some(_), .. } => true,
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                contains_value_return(then_body)
                    || else_body
                        .as_ref()
                        .map_or(false, |body| contains_value_return(body))
            }
            ASTNode::Loop { body, .. } => contains_value_return(body),
            ASTNode::TryCatch {
                try_body,
                catch_clauses,
                finally_body,
                ..
            } => {
                contains_value_return(try_body)
                    || catch_clauses
                        .iter()
                        .any(|clause| contains_value_return(&clause.body))
                    || finally_body
                        .as_ref()
                        .map_or(false, |body| contains_value_return(body))
            }
            ASTNode::Program { statements, .. } => contains_value_return(statements),
            ASTNode::ScopeBox { body, .. } => contains_value_return(body),
            ASTNode::FunctionDeclaration { body, .. } => contains_value_return(body),
            _ => false,
        }
    }

    nodes.iter().any(node_has_value_return)
}
