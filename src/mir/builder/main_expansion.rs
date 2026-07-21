//! HEADERPORT0-I0-MAINROLE0-S0: source-only Main expansion.
//!
//! `Main` is a source declaration which expands into a root entry, ordinary
//! static children, and an optional callable compatibility child.  This
//! product owns only source references and deterministic symbols.  It has no
//! Builder, collector, ValueId, metadata, header cache, or publication route.

use crate::ast::ASTNode;

use super::declaration_order::sorted_method_entries;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum MainExpansionErrorV1 {
    RootMustBeProgram,
    MainBoxMissing,
    DuplicateMainBox,
    MainMethodMissing,
    MainMethodMustBeFunction,
    StaticChildMustBeFunction { method: String },
    StaticChildMustBeStatic { method: String },
}

impl std::fmt::Display for MainExpansionErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][main_expansion] {self:?}")
    }
}

impl std::error::Error for MainExpansionErrorV1 {}

/// The source body which is lowered inline into the root `main` function.
#[derive(Debug)]
pub(in crate::mir::builder) struct VerifiedMainRootBodyV1<'src> {
    box_name: &'src str,
    source: &'src ASTNode,
    _seal: MainRootBodySealV1,
}

#[derive(Debug)]
struct MainRootBodySealV1;

impl VerifiedMainRootBodyV1<'_> {
    pub(in crate::mir::builder) fn box_name(&self) -> &str {
        self.box_name
    }

    pub(in crate::mir::builder) fn source(&self) -> &ASTNode {
        self.source
    }
}

/// One independently lowerable static child of the Main source declaration.
#[derive(Debug)]
pub(in crate::mir::builder) struct VerifiedMainStaticChildV1<'src> {
    method_name: &'src str,
    source: &'src ASTNode,
    symbol: Box<str>,
    arity: usize,
    _seal: MainStaticChildSealV1,
}

#[derive(Debug)]
struct MainStaticChildSealV1;

impl VerifiedMainStaticChildV1<'_> {
    pub(in crate::mir::builder) fn method_name(&self) -> &str {
        self.method_name
    }

    pub(in crate::mir::builder) fn source(&self) -> &ASTNode {
        self.source
    }

    pub(in crate::mir::builder) fn symbol(&self) -> &str {
        &self.symbol
    }

    pub(in crate::mir::builder) fn arity(&self) -> usize {
        self.arity
    }
}

/// Immutable source expansion for one `static box Main` declaration.
///
/// The root and the optional compatibility child deliberately have distinct
/// identities even though they may reference the same source method node.
#[derive(Debug)]
pub(in crate::mir::builder) struct VerifiedMainExpansionV1<'src> {
    root: VerifiedMainRootBodyV1<'src>,
    static_children: Box<[VerifiedMainStaticChildV1<'src>]>,
    callable_main_compat: Option<VerifiedMainStaticChildV1<'src>>,
    _seal: MainExpansionSealV1,
}

#[derive(Debug)]
struct MainExpansionSealV1;

impl<'src> VerifiedMainExpansionV1<'src> {
    /// Classify exactly one top-level static `Main` box without cloning AST.
    pub(in crate::mir::builder) fn from_program(
        source: &'src ASTNode,
    ) -> Result<Self, MainExpansionErrorV1> {
        let ASTNode::Program { statements, .. } = source else {
            return Err(MainExpansionErrorV1::RootMustBeProgram);
        };

        let mut main_box: Option<(&str, &std::collections::HashMap<String, ASTNode>)> = None;
        for statement in statements {
            let ASTNode::BoxDeclaration {
                name,
                methods,
                is_static: true,
                ..
            } = statement
            else {
                continue;
            };
            if name != "Main" {
                continue;
            }
            if main_box.replace((name.as_str(), methods)).is_some() {
                return Err(MainExpansionErrorV1::DuplicateMainBox);
            }
        }
        let Some((box_name, methods)) = main_box else {
            return Err(MainExpansionErrorV1::MainBoxMissing);
        };

        let Some(main_source) = methods.get("main") else {
            return Err(MainExpansionErrorV1::MainMethodMissing);
        };
        let ASTNode::FunctionDeclaration {
            params,
            is_static: main_is_static,
            ..
        } = main_source
        else {
            return Err(MainExpansionErrorV1::MainMethodMustBeFunction);
        };
        if !main_is_static {
            return Err(MainExpansionErrorV1::StaticChildMustBeStatic {
                method: "main".to_owned(),
            });
        }

        let mut static_children = Vec::new();
        for (method_name, child_source) in sorted_method_entries(methods) {
            if method_name == "main" {
                continue;
            }
            let ASTNode::FunctionDeclaration {
                params, is_static, ..
            } = child_source
            else {
                return Err(MainExpansionErrorV1::StaticChildMustBeFunction {
                    method: method_name.to_owned(),
                });
            };
            if !is_static {
                return Err(MainExpansionErrorV1::StaticChildMustBeStatic {
                    method: method_name.to_owned(),
                });
            }
            static_children.push(VerifiedMainStaticChildV1 {
                method_name,
                source: child_source,
                symbol: crate::mir::naming::encode_static_method(
                    box_name,
                    method_name,
                    params.len(),
                )
                .into_boxed_str(),
                arity: params.len(),
                _seal: MainStaticChildSealV1,
            });
        }

        let callable_main_compat = Some(VerifiedMainStaticChildV1 {
            method_name: "main",
            source: main_source,
            symbol: crate::mir::naming::encode_static_method(box_name, "main", params.len())
                .into_boxed_str(),
            arity: params.len(),
            _seal: MainStaticChildSealV1,
        });

        Ok(Self {
            root: VerifiedMainRootBodyV1 {
                box_name,
                source: main_source,
                _seal: MainRootBodySealV1,
            },
            static_children: static_children.into_boxed_slice(),
            callable_main_compat,
            _seal: MainExpansionSealV1,
        })
    }

    pub(in crate::mir::builder) fn root(&self) -> &VerifiedMainRootBodyV1<'src> {
        &self.root
    }

    pub(in crate::mir::builder) fn static_children(&self) -> &[VerifiedMainStaticChildV1<'src>] {
        &self.static_children
    }

    pub(in crate::mir::builder) fn callable_main_compat(
        &self,
    ) -> Option<&VerifiedMainStaticChildV1<'src>> {
        self.callable_main_compat.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, DeclarationAttrs, Span};
    use std::collections::HashMap;

    fn function(name: &str, is_static: bool, arity: usize) -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: name.to_owned(),
            params: (0..arity).map(|index| format!("p{index}")).collect(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: Vec::new(),
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    fn program(methods: HashMap<String, ASTNode>) -> ASTNode {
        ASTNode::Program {
            statements: vec![ASTNode::BoxDeclaration {
                name: "Main".to_owned(),
                methods,
                is_static: true,
                fields: Vec::new(),
                field_decls: Vec::new(),
                public_fields: Vec::new(),
                private_fields: Vec::new(),
                constructors: HashMap::new(),
                init_fields: Vec::new(),
                weak_fields: Vec::new(),
                delegates: Vec::new(),
                invariants: Vec::new(),
                transitions: Vec::new(),
                is_interface: false,
                is_sync: false,
                is_record: false,
                type_parameters: Vec::new(),
                extends: Vec::new(),
                implements: Vec::new(),
                static_init: None,
                attrs: DeclarationAttrs::default(),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        }
    }

    #[test]
    fn expansion_separates_root_children_and_compat_identity() {
        let mut methods = HashMap::new();
        methods.insert("zeta".to_owned(), function("zeta", true, 2));
        methods.insert("main".to_owned(), function("main", true, 1));
        methods.insert("alpha".to_owned(), function("alpha", true, 0));
        let source = program(methods);

        let expansion = VerifiedMainExpansionV1::from_program(&source).unwrap();
        assert_eq!(expansion.root().box_name(), "Main");
        assert_eq!(
            expansion
                .static_children()
                .iter()
                .map(|child| child.symbol())
                .collect::<Vec<_>>(),
            vec!["Main.alpha/0", "Main.zeta/2"]
        );
        let compat = expansion.callable_main_compat().unwrap();
        assert_eq!(compat.symbol(), "Main.main/1");
        assert_eq!(expansion.root().source(), compat.source());
    }

    #[test]
    fn malformed_or_missing_main_fails_before_builder_effects() {
        let empty = ASTNode::Program {
            statements: Vec::new(),
            span: Span::unknown(),
        };
        assert_eq!(
            VerifiedMainExpansionV1::from_program(&empty).unwrap_err(),
            MainExpansionErrorV1::MainBoxMissing
        );

        let mut methods = HashMap::new();
        methods.insert(
            "main".to_owned(),
            ASTNode::Literal {
                value: crate::ast::LiteralValue::Integer(1),
                span: Span::unknown(),
            },
        );
        assert_eq!(
            VerifiedMainExpansionV1::from_program(&program(methods)).unwrap_err(),
            MainExpansionErrorV1::MainMethodMustBeFunction
        );
    }
}
