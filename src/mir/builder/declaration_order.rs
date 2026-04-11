use crate::ast::ASTNode;
use std::collections::HashMap;

fn sorted_named_ast_entries<'a>(
    entries: &'a HashMap<String, ASTNode>,
) -> Vec<(&'a str, &'a ASTNode)> {
    let mut items: Vec<(&str, &ASTNode)> = entries
        .iter()
        .map(|(name, node)| (name.as_str(), node))
        .collect();
    items.sort_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs));
    items
}

/// MIR builder must not depend on `HashMap` iteration order for box member lowering.
///
/// The current known-receiver rewrite still reads already-lowered module state on a
/// narrow transition slice, so member traversal needs one deterministic owner until
/// declaration presence is split into its own generic authority.
pub(super) fn sorted_method_entries<'a>(
    methods: &'a HashMap<String, ASTNode>,
) -> Vec<(&'a str, &'a ASTNode)> {
    sorted_named_ast_entries(methods)
}

/// Constructors share the same deterministic traversal seam.
pub(super) fn sorted_constructor_entries<'a>(
    constructors: &'a HashMap<String, ASTNode>,
) -> Vec<(&'a str, &'a ASTNode)> {
    sorted_named_ast_entries(constructors)
}

#[cfg(test)]
mod tests {
    use super::{sorted_constructor_entries, sorted_method_entries};
    use crate::ast::{ASTNode, DeclarationAttrs, Span};
    use std::collections::HashMap;

    fn empty_fn() -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: "f".to_string(),
            params: vec![],
            body: vec![],
            is_static: false,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    #[test]
    fn sorted_method_entries_ignore_hashmap_order() {
        let mut methods = HashMap::new();
        methods.insert("step_chain".to_string(), empty_fn());
        methods.insert("birth".to_string(), empty_fn());
        methods.insert("step".to_string(), empty_fn());

        let names: Vec<&str> = sorted_method_entries(&methods)
            .into_iter()
            .map(|(name, _)| name)
            .collect();
        assert_eq!(names, vec!["birth", "step", "step_chain"]);
    }

    #[test]
    fn sorted_constructor_entries_ignore_hashmap_order() {
        let mut ctors = HashMap::new();
        ctors.insert("birth/2".to_string(), empty_fn());
        ctors.insert("birth/0".to_string(), empty_fn());
        ctors.insert("birth/1".to_string(), empty_fn());

        let names: Vec<&str> = sorted_constructor_entries(&ctors)
            .into_iter()
            .map(|(name, _)| name)
            .collect();
        assert_eq!(names, vec!["birth/0", "birth/1", "birth/2"]);
    }
}
