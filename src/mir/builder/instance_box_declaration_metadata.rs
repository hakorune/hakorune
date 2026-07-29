//! Source-only metadata projection for an ordinary instance-Box declaration.
//!
//! This is the one owner for the declaration facts that must be emitted before
//! constructors and instance methods.  It deliberately contains no lowering
//! route or callable-catalog policy: Program-root and raw Box lowering share
//! the same prepared metadata and keep their distinct child terminals.

use std::collections::{HashMap, HashSet};

use crate::ast::ASTNode;
use crate::mir::slot_registry::{get_or_assign_type_id, reserve_method_slot};

use super::declaration_order::sorted_method_entries;
use super::MirBuilder;

pub(super) struct PreparedInstanceBoxDeclarationMetadataV1 {
    name: String,
    fields: Box<[String]>,
    weak_fields: Option<HashSet<String>>,
    instance_method_slots: Box<[String]>,
    declared_methods: Box<[String]>,
}

impl PreparedInstanceBoxDeclarationMetadataV1 {
    pub(super) fn prepare(
        name: &str,
        methods: &HashMap<String, ASTNode>,
        fields: &[String],
        weak_fields: &[String],
    ) -> Self {
        let mut instance_method_slots = Vec::new();
        let mut declared_methods = Vec::new();
        for (method_name, method) in sorted_method_entries(methods) {
            let ASTNode::FunctionDeclaration { is_static, .. } = method else {
                continue;
            };
            declared_methods.push(method_name.to_owned());
            if !*is_static {
                instance_method_slots.push(method_name.to_owned());
            }
        }
        Self {
            name: name.to_owned(),
            fields: fields.to_vec().into_boxed_slice(),
            weak_fields: (!weak_fields.is_empty()).then(|| weak_fields.iter().cloned().collect()),
            instance_method_slots: instance_method_slots.into_boxed_slice(),
            declared_methods: declared_methods.into_boxed_slice(),
        }
    }

    pub(super) fn lower_with_builder_v1(self, builder: &mut MirBuilder) -> Result<(), String> {
        crate::mir::builder::emission::constant::emit_string(
            builder,
            format!("__box_type_{}", self.name),
        )?;
        for field in &self.fields {
            crate::mir::builder::emission::constant::emit_string(
                builder,
                format!("__field_{}_{}", self.name, field),
            )?;
        }
        if let Some(weak_fields) = self.weak_fields {
            builder
                .comp_ctx
                .weak_fields_by_box
                .insert(self.name.clone(), weak_fields);
        }
        if !self.instance_method_slots.is_empty() {
            let type_id = get_or_assign_type_id(&self.name);
            for (index, method_name) in self.instance_method_slots.iter().enumerate() {
                reserve_method_slot(type_id, method_name, 4u16.saturating_add(index as u16));
            }
        }
        for method_name in self.declared_methods {
            crate::mir::builder::emission::constant::emit_string(
                builder,
                format!("__method_{}_{}", self.name, method_name),
            )?;
            builder
                .comp_ctx
                .register_property_getter_method(self.name.clone(), &method_name);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::ast::{ASTNode, DeclarationAttrs, Span};

    use super::PreparedInstanceBoxDeclarationMetadataV1;

    fn function(is_static: bool) -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: "fixture".to_owned(),
            params: vec![],
            param_decls: vec![],
            return_type_name: None,
            body: vec![],
            is_static,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            uses: vec![],
            contracts: vec![],
            span: Span::unknown(),
        }
    }

    #[test]
    fn projection_sorts_declared_methods_and_reserves_only_instance_methods() {
        let mut methods = HashMap::new();
        methods.insert("zeta".to_owned(), function(false));
        methods.insert("alpha".to_owned(), function(true));
        methods.insert("beta".to_owned(), function(false));

        let metadata = PreparedInstanceBoxDeclarationMetadataV1::prepare(
            "Page",
            &methods,
            &["value".to_owned()],
            &["parent".to_owned()],
        );

        assert_eq!(&*metadata.fields, ["value"]);
        assert!(metadata
            .weak_fields
            .as_ref()
            .is_some_and(|fields| fields.contains("parent")));
        assert_eq!(&*metadata.instance_method_slots, ["beta", "zeta"]);
        assert_eq!(&*metadata.declared_methods, ["alpha", "beta", "zeta"]);
    }
}
