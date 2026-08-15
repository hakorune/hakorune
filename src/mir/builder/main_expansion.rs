//! HEADERPORT0-I0-MAINROLE0-S0: source-only Main expansion.
//!
//! `Main` is a source declaration which expands into a root entry, ordinary
//! static children, and an optional callable compatibility child.  This
//! product owns only source references and deterministic symbols.  It has no
//! Builder, collector, ValueId, metadata, header cache, or publication route.

use crate::ast::{ASTNode, BoxMethodInventoryOrdinalV1, DeclarationAttrs, ParamDecl};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum MainExpansionErrorV1 {
    RootMustBeProgram,
    MainBoxMissing,
    DuplicateMainBox,
    StatementIndexOverflow,
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
    parts: VerifiedMainRootPartsV1<'src>,
    _seal: MainRootBodySealV1,
}

#[derive(Debug)]
struct VerifiedMainRootPartsV1<'src> {
    params: &'src [String],
    param_decls: &'src [ParamDecl],
    return_type_name: Option<&'src str>,
    body: &'src [ASTNode],
    uses: &'src [String],
    attrs: &'src DeclarationAttrs,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct OwnedVerifiedMainRootLoweringV1 {
    box_name: String,
    callable_main_symbol: Option<String>,
    params: Vec<String>,
    param_decls: Vec<ParamDecl>,
    return_type_name: Option<String>,
    body: Vec<ASTNode>,
    uses: Vec<String>,
    attrs: DeclarationAttrs,
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

impl OwnedVerifiedMainRootLoweringV1 {
    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        String,
        Option<String>,
        Vec<String>,
        Vec<ParamDecl>,
        Option<String>,
        Vec<ASTNode>,
        Vec<String>,
        DeclarationAttrs,
    ) {
        (
            self.box_name,
            self.callable_main_symbol,
            self.params,
            self.param_decls,
            self.return_type_name,
            self.body,
            self.uses,
            self.attrs,
        )
    }
}

/// One independently lowerable static child of the Main source declaration.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedMainStaticChildV1<'src> {
    method_name: &'src str,
    source: &'src ASTNode,
    statement: u32,
    method: BoxMethodInventoryOrdinalV1,
    parts: VerifiedMainStaticChildPartsV1<'src>,
    symbol: Box<str>,
    arity: usize,
    _seal: MainStaticChildSealV1,
}

#[derive(Debug)]
struct VerifiedMainStaticChildPartsV1<'src> {
    params: &'src [String],
    param_decls: &'src [ParamDecl],
    return_type_name: Option<&'src str>,
    body: &'src [ASTNode],
    uses: &'src [String],
    attrs: &'src DeclarationAttrs,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct OwnedVerifiedMainStaticChildLoweringV1 {
    symbol: String,
    params: Vec<String>,
    param_decls: Vec<ParamDecl>,
    return_type_name: Option<String>,
    body: Vec<ASTNode>,
    uses: Vec<String>,
    attrs: DeclarationAttrs,
}

#[derive(Debug)]
struct MainStaticChildSealV1;

impl VerifiedMainStaticChildV1<'_> {
    pub(in crate::mir::builder) fn method_name(&self) -> &str {
        self.method_name
    }

    pub(in crate::mir) fn source(&self) -> &ASTNode {
        self.source
    }

    pub(in crate::mir) const fn statement_index(&self) -> u32 {
        self.statement
    }

    pub(in crate::mir) const fn method_ordinal(&self) -> BoxMethodInventoryOrdinalV1 {
        self.method
    }

    pub(in crate::mir::builder) fn symbol(&self) -> &str {
        &self.symbol
    }

    pub(in crate::mir::builder) fn arity(&self) -> usize {
        self.arity
    }

    pub(in crate::mir::builder) fn to_owned_lowering(
        &self,
    ) -> OwnedVerifiedMainStaticChildLoweringV1 {
        OwnedVerifiedMainStaticChildLoweringV1 {
            symbol: self.symbol.to_string(),
            params: self.parts.params.to_vec(),
            param_decls: self.parts.param_decls.to_vec(),
            return_type_name: self.parts.return_type_name.map(str::to_owned),
            body: self.parts.body.to_vec(),
            uses: self.parts.uses.to_vec(),
            attrs: self.parts.attrs.clone(),
        }
    }
}

impl OwnedVerifiedMainStaticChildLoweringV1 {
    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        String,
        Vec<String>,
        Vec<ParamDecl>,
        Option<String>,
        Vec<ASTNode>,
        Vec<String>,
        DeclarationAttrs,
    ) {
        (
            self.symbol,
            self.params,
            self.param_decls,
            self.return_type_name,
            self.body,
            self.uses,
            self.attrs,
        )
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

/// One source-only root selector.  Script and App are classified once before
/// any Builder, candidate, collector, or receipt effect is possible.
#[derive(Debug)]
pub(in crate::mir::builder) enum VerifiedRawRootExpansionV1<'src> {
    Script,
    App(VerifiedMainExpansionV1<'src>),
}

impl<'src> VerifiedRawRootExpansionV1<'src> {
    pub(in crate::mir::builder) fn from_program(
        source: &'src ASTNode,
    ) -> Result<Self, MainExpansionErrorV1> {
        let ASTNode::Program { statements, .. } = source else {
            return Err(MainExpansionErrorV1::RootMustBeProgram);
        };
        let main_count = statements
            .iter()
            .filter(|statement| {
                matches!(
                    statement,
                    ASTNode::BoxDeclaration {
                        name,
                        is_static: true,
                        ..
                    } if name == "Main"
                )
            })
            .count();
        match main_count {
            0 => Ok(Self::Script),
            1 => Ok(Self::App(VerifiedMainExpansionV1::from_program(source)?)),
            _ => Err(MainExpansionErrorV1::DuplicateMainBox),
        }
    }

    /// The verified expansion is the only selected normal Script/App route
    /// authority. Program lowering consumes this disposition without
    /// inspecting the source again.
    pub(in crate::mir::builder) const fn is_app_mode(&self) -> bool {
        matches!(self, Self::App(_))
    }
}

impl<'src> VerifiedMainExpansionV1<'src> {
    /// Classify exactly one top-level static `Main` box without cloning AST.
    pub(in crate::mir::builder) fn from_program(
        source: &'src ASTNode,
    ) -> Result<Self, MainExpansionErrorV1> {
        let ASTNode::Program { statements, .. } = source else {
            return Err(MainExpansionErrorV1::RootMustBeProgram);
        };

        let mut main_box: Option<(u32, &str, &crate::ast::BoxMethodInventoryV1)> = None;
        for (statement_index, statement) in statements.iter().enumerate() {
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
            let statement_index = u32::try_from(statement_index)
                .map_err(|_| MainExpansionErrorV1::StatementIndexOverflow)?;
            if main_box
                .replace((statement_index, name.as_str(), methods))
                .is_some()
            {
                return Err(MainExpansionErrorV1::DuplicateMainBox);
            }
        }
        let Some((main_statement, box_name, methods)) = main_box else {
            return Err(MainExpansionErrorV1::MainBoxMissing);
        };

        let Some(main_source) = methods.get_declaration("main") else {
            return Err(MainExpansionErrorV1::MainMethodMissing);
        };
        let ASTNode::FunctionDeclaration {
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
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

        let main_method_ordinal = methods
            .get("main")
            .ok_or(MainExpansionErrorV1::MainMethodMissing)?
            .site();
        let mut static_children = Vec::new();
        for entry in methods.iter_compat_name_order() {
            let method_name = entry.name();
            let child_source = entry.declaration();
            if method_name == "main" {
                continue;
            }
            let ASTNode::FunctionDeclaration {
                params,
                param_decls,
                return_type_name,
                body,
                uses,
                attrs,
                is_static,
                ..
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
                statement: main_statement,
                method: entry.site(),
                parts: VerifiedMainStaticChildPartsV1 {
                    params,
                    param_decls,
                    return_type_name: return_type_name.as_deref(),
                    body,
                    uses,
                    attrs,
                },
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
            statement: main_statement,
            method: main_method_ordinal,
            parts: VerifiedMainStaticChildPartsV1 {
                params,
                param_decls,
                return_type_name: return_type_name.as_deref(),
                body,
                uses,
                attrs,
            },
            symbol: crate::mir::naming::encode_static_method(box_name, "main", params.len())
                .into_boxed_str(),
            arity: params.len(),
            _seal: MainStaticChildSealV1,
        });

        Ok(Self {
            root: VerifiedMainRootBodyV1 {
                box_name,
                source: main_source,
                parts: VerifiedMainRootPartsV1 {
                    params,
                    param_decls,
                    return_type_name: return_type_name.as_deref(),
                    body,
                    uses,
                    attrs,
                },
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

    pub(in crate::mir::builder) fn to_owned_root_lowering(
        &self,
    ) -> OwnedVerifiedMainRootLoweringV1 {
        OwnedVerifiedMainRootLoweringV1 {
            box_name: self.root.box_name.to_owned(),
            callable_main_symbol: self
                .callable_main_compat
                .as_ref()
                .map(|child| child.symbol.to_string()),
            params: self.root.parts.params.to_vec(),
            param_decls: self.root.parts.param_decls.to_vec(),
            return_type_name: self.root.parts.return_type_name.map(str::to_owned),
            body: self.root.parts.body.to_vec(),
            uses: self.root.parts.uses.to_vec(),
            attrs: self.root.parts.attrs.clone(),
        }
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
                methods: crate::ast::BoxMethodInventoryV1::from_legacy_ast_map(methods),
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
        let zeta = &expansion.static_children()[1];
        let (symbol, params, param_decls, result, body, uses, attrs) =
            zeta.to_owned_lowering().into_parts();
        assert_eq!(symbol, "Main.zeta/2");
        assert_eq!(params, vec!["p0".to_owned(), "p1".to_owned()]);
        assert!(param_decls.is_empty());
        assert!(result.is_none());
        assert!(body.is_empty());
        assert!(uses.is_empty());
        assert_eq!(attrs, DeclarationAttrs::default());
        let (box_name, callable_symbol, params, param_decls, result, body, uses, attrs) =
            expansion.to_owned_root_lowering().into_parts();
        assert_eq!(box_name, "Main");
        assert_eq!(callable_symbol.as_deref(), Some("Main.main/1"));
        assert_eq!(params, vec!["p0".to_owned()]);
        assert!(param_decls.is_empty());
        assert!(result.is_none());
        assert!(body.is_empty());
        assert!(uses.is_empty());
        assert_eq!(attrs, DeclarationAttrs::default());
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

    #[test]
    fn app_shape_ignores_non_main_top_level_statements() {
        let mut source = program({
            let mut methods = HashMap::new();
            methods.insert("main".to_owned(), function("main", true, 0));
            methods.insert("helper".to_owned(), function("helper", true, 1));
            methods
        });
        let ASTNode::Program { statements, .. } = &mut source else {
            unreachable!("program helper creates a Program");
        };
        statements.insert(
            0,
            ASTNode::Literal {
                value: crate::ast::LiteralValue::Integer(7),
                span: Span::unknown(),
            },
        );

        let expansion = VerifiedMainExpansionV1::from_program(&source).unwrap();
        assert_eq!(expansion.root().box_name(), "Main");
        assert_eq!(expansion.static_children().len(), 1);
        assert_eq!(expansion.static_children()[0].symbol(), "Main.helper/1");
        assert_eq!(
            expansion.callable_main_compat().unwrap().symbol(),
            "Main.main/0"
        );
    }

    #[test]
    fn script_shape_without_static_main_stays_out_of_this_product() {
        let source = ASTNode::Program {
            statements: vec![ASTNode::Literal {
                value: crate::ast::LiteralValue::Integer(1),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        };

        assert_eq!(
            VerifiedMainExpansionV1::from_program(&source).unwrap_err(),
            MainExpansionErrorV1::MainBoxMissing
        );
    }

    #[test]
    fn child_and_root_static_contracts_are_checked_before_builder_effects() {
        let mut methods = HashMap::new();
        methods.insert("main".to_owned(), function("main", true, 0));
        methods.insert("instance".to_owned(), function("instance", false, 0));
        assert_eq!(
            VerifiedMainExpansionV1::from_program(&program(methods)).unwrap_err(),
            MainExpansionErrorV1::StaticChildMustBeStatic {
                method: "instance".to_owned(),
            }
        );

        let mut methods = HashMap::new();
        methods.insert("main".to_owned(), function("main", false, 0));
        assert_eq!(
            VerifiedMainExpansionV1::from_program(&program(methods)).unwrap_err(),
            MainExpansionErrorV1::StaticChildMustBeStatic {
                method: "main".to_owned(),
            }
        );
    }

    #[test]
    fn duplicate_main_boxes_are_rejected_without_order_dependence() {
        let mut first_methods = HashMap::new();
        first_methods.insert("main".to_owned(), function("main", true, 0));
        let mut second_methods = HashMap::new();
        second_methods.insert("main".to_owned(), function("main", true, 0));

        let mut source = program(first_methods);
        let ASTNode::Program { statements, .. } = &mut source else {
            unreachable!("program helper creates a Program");
        };
        let ASTNode::BoxDeclaration {
            name,
            methods,
            is_static,
            fields,
            field_decls,
            public_fields,
            private_fields,
            constructors,
            init_fields,
            weak_fields,
            delegates,
            invariants,
            transitions,
            is_interface,
            is_sync,
            is_record,
            type_parameters,
            extends,
            implements,
            static_init,
            attrs,
            span,
        } = &statements[0]
        else {
            unreachable!("program helper creates a Main box");
        };
        statements.push(ASTNode::BoxDeclaration {
            name: name.clone(),
            methods: crate::ast::BoxMethodInventoryV1::from_legacy_ast_map(second_methods),
            is_static: *is_static,
            fields: fields.clone(),
            field_decls: field_decls.clone(),
            public_fields: public_fields.clone(),
            private_fields: private_fields.clone(),
            constructors: constructors.clone(),
            init_fields: init_fields.clone(),
            weak_fields: weak_fields.clone(),
            delegates: delegates.clone(),
            invariants: invariants.clone(),
            transitions: transitions.clone(),
            is_interface: *is_interface,
            is_sync: *is_sync,
            is_record: *is_record,
            type_parameters: type_parameters.clone(),
            extends: extends.clone(),
            implements: implements.clone(),
            static_init: static_init.clone(),
            attrs: attrs.clone(),
            span: *span,
        });

        assert_eq!(
            VerifiedMainExpansionV1::from_program(&source).unwrap_err(),
            MainExpansionErrorV1::DuplicateMainBox
        );
    }

    #[test]
    fn raw_root_selector_accepts_script_without_main_box() {
        let source = ASTNode::Program {
            statements: vec![ASTNode::Literal {
                value: crate::ast::LiteralValue::Integer(1),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        };
        assert!(matches!(
            VerifiedRawRootExpansionV1::from_program(&source).unwrap(),
            VerifiedRawRootExpansionV1::Script
        ));
    }

    #[test]
    fn raw_root_selector_rejects_duplicate_main_before_app_expansion() {
        let mut methods = HashMap::new();
        methods.insert("main".to_owned(), function("main", true, 0));
        let mut source = program(methods);
        let ASTNode::Program { statements, .. } = &mut source else {
            unreachable!("program helper creates a Program");
        };
        let duplicate = statements[0].clone();
        statements.push(duplicate);
        assert_eq!(
            VerifiedRawRootExpansionV1::from_program(&source).unwrap_err(),
            MainExpansionErrorV1::DuplicateMainBox
        );
    }
}
