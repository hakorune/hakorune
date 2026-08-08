use crate::ast::{ASTNode, BoxMethodInventoryV1};

/// MIR builder must not depend on `HashMap` iteration order for box member lowering.
///
/// The current known-receiver rewrite still reads already-lowered module state on a
/// narrow transition slice, so member traversal needs one deterministic owner until
/// declaration presence is split into its own generic authority.
pub(super) fn sorted_method_entries<'a>(
    methods: &'a BoxMethodInventoryV1,
) -> Vec<(&'a str, &'a ASTNode)> {
    methods
        .iter_compat_name_order()
        .map(|entry| (entry.name(), entry.declaration()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::sorted_method_entries;
    use crate::ast::{
        ASTNode, BoxMethodCompatibilityOriginV1, BoxMethodInventoryV1, DeclarationAttrs, Span,
    };
    use std::collections::HashMap;

    fn empty_fn() -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: "f".to_string(),
            params: vec![],
            param_decls: vec![],
            return_type_name: None,
            body: vec![],
            is_static: false,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            uses: vec![],

            contracts: vec![],
            span: Span::unknown(),
        }
    }

    #[test]
    fn sorted_method_entries_ignore_hashmap_order() {
        let mut methods = HashMap::new();
        methods.insert("step_chain".to_string(), empty_fn());
        methods.insert("birth".to_string(), empty_fn());
        methods.insert("step".to_string(), empty_fn());

        let methods = BoxMethodInventoryV1::try_from_compatibility_map(
            methods,
            BoxMethodCompatibilityOriginV1::LegacyAstConstruction,
        )
        .unwrap();
        let names: Vec<&str> = sorted_method_entries(&methods)
            .into_iter()
            .map(|(name, _)| name)
            .collect();
        assert_eq!(names, vec!["birth", "step", "step_chain"]);
    }
}
