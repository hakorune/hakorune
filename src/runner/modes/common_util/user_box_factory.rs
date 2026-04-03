use crate::{
    box_factory::{BoxFactory, RuntimeError},
    core::model::BoxDeclaration as CoreBoxDecl,
    instance_v2::InstanceBox,
};
use nyash_rust::ast::ASTNode;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Collect inline user-defined box declarations and register a small factory
/// for the current runner.
pub(crate) fn install_inline_user_box_factory(
    ast: &ASTNode,
    include_static_box_decls: bool,
    alias_static_instances: bool,
) -> HashMap<String, CoreBoxDecl> {
    let mut nonstatic_decls: HashMap<String, CoreBoxDecl> = HashMap::new();
    let mut static_names: Vec<String> = Vec::new();
    let mut static_box_decls: HashMap<String, CoreBoxDecl> = HashMap::new();

    if let ASTNode::Program { statements, .. } = ast {
        for st in statements {
            if let ASTNode::BoxDeclaration {
                name,
                fields,
                public_fields,
                private_fields,
                methods,
                constructors,
                init_fields,
                weak_fields,
                is_interface,
                extends,
                implements,
                type_parameters,
                is_static,
                ..
            } = st
            {
                let decl = CoreBoxDecl {
                    name: name.clone(),
                    fields: fields.clone(),
                    public_fields: public_fields.clone(),
                    private_fields: private_fields.clone(),
                    methods: methods.clone(),
                    constructors: constructors.clone(),
                    init_fields: init_fields.clone(),
                    weak_fields: weak_fields.clone(),
                    is_interface: *is_interface,
                    extends: extends.clone(),
                    implements: implements.clone(),
                    type_parameters: type_parameters.clone(),
                };

                if *is_static {
                    static_names.push(name.clone());
                    if include_static_box_decls {
                        static_box_decls.insert(name.clone(), decl);
                    }
                } else {
                    nonstatic_decls.insert(name.clone(), decl);
                }
            }
        }
    }

    let mut decls = nonstatic_decls.clone();
    if include_static_box_decls {
        for (name, sdecl) in static_box_decls.iter() {
            if !decls.contains_key(name) {
                decls.insert(name.clone(), sdecl.clone());
            }
        }
    }
    if alias_static_instances {
        for s in static_names.into_iter() {
            let inst = format!("{}Instance", s);
            if let Some(d) = nonstatic_decls.get(&inst) {
                decls.insert(s, d.clone());
            }
        }
    }

    if !decls.is_empty() {
        struct InlineUserBoxFactory {
            decls: Arc<RwLock<HashMap<String, CoreBoxDecl>>>,
        }

        impl BoxFactory for InlineUserBoxFactory {
            fn create_box(
                &self,
                name: &str,
                args: &[Box<dyn crate::box_trait::NyashBox>],
            ) -> Result<Box<dyn crate::box_trait::NyashBox>, RuntimeError> {
                let guard = self.decls.read().unwrap();
                let decl = match guard.get(name).cloned() {
                    Some(d) => d,
                    None => {
                        let mut available: Vec<_> = guard.keys().cloned().collect();
                        available.sort();
                        let hint = if available.is_empty() {
                            "No user-defined boxes available".to_string()
                        } else if available.len() <= 10 {
                            format!("Available: {}", available.join(", "))
                        } else {
                            format!(
                                "Available ({} boxes): {}, ...",
                                available.len(),
                                available[..10].join(", ")
                            )
                        };
                        return Err(RuntimeError::InvalidOperation {
                            message: format!("Unknown Box type: {}. {}", name, hint),
                        });
                    }
                };

                let mut inst = InstanceBox::from_declaration(
                    decl.name.clone(),
                    decl.fields.clone(),
                    decl.methods.clone(),
                );
                let _ = inst.init(args);
                Ok(Box::new(inst))
            }

            fn box_types(&self) -> Vec<&str> {
                vec![]
            }

            fn is_available(&self) -> bool {
                true
            }

            fn factory_type(&self) -> crate::box_factory::FactoryType {
                crate::box_factory::FactoryType::User
            }
        }

        let factory = InlineUserBoxFactory {
            decls: Arc::new(RwLock::new(decls)),
        };
        crate::runtime::unified_registry::register_user_defined_factory(Arc::new(factory));
    }

    if include_static_box_decls {
        static_box_decls
    } else {
        HashMap::new()
    }
}
