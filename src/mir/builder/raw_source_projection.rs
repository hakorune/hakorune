//! RAW-SOURCE0-PLAN0: owned source projection for the disconnected Raw owner.
//!
//! `VerifiedRawRootExpansionV1` borrows the input AST.  This module converts
//! that source-only proof into owned locators before any Builder, token,
//! collector, ledger, or publication effect.  It deliberately has no
//! production consumer yet; BIND0 will attach the compiler-owned token.

use crate::ast::ASTNode;

use super::main_expansion::{VerifiedMainExpansionV1, VerifiedRawRootExpansionV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawSourceOriginV1 {
    BareAst,
    ReplCompatibility,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct RawSourceLocatorV1 {
    top_level_statement: usize,
    box_name: Box<str>,
    method_name: Box<str>,
    symbol: Box<str>,
    arity: usize,
}

impl RawSourceLocatorV1 {
    pub(in crate::mir) fn top_level_statement(&self) -> usize {
        self.top_level_statement
    }

    pub(in crate::mir) fn box_name(&self) -> &str {
        &self.box_name
    }

    pub(in crate::mir) fn method_name(&self) -> &str {
        &self.method_name
    }

    pub(in crate::mir) fn symbol(&self) -> &str {
        &self.symbol
    }

    pub(in crate::mir) const fn arity(&self) -> usize {
        self.arity
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum OwnedRawRootProjectionV1 {
    Script { statement_count: usize },
    App {
        main: RawSourceLocatorV1,
        static_children: Box<[RawSourceLocatorV1]>,
        callable_main: RawSourceLocatorV1,
    },
}

impl OwnedRawRootProjectionV1 {
    pub(in crate::mir) const fn is_script(&self) -> bool {
        matches!(self, Self::Script { .. })
    }

    pub(in crate::mir::builder) fn first_static_child(&self) -> Option<&RawSourceLocatorV1> {
        match self {
            Self::Script { .. } => None,
            Self::App { static_children, .. } => static_children.first(),
        }
    }

    pub(in crate::mir::builder) fn from_verified(
        source: &ASTNode,
        expansion: &VerifiedRawRootExpansionV1<'_>,
    ) -> Result<Self, RawSourceProjectionErrorV1> {
        let ASTNode::Program { statements, .. } = source else {
            return Err(RawSourceProjectionErrorV1::RootMustBeProgram);
        };
        match expansion {
            VerifiedRawRootExpansionV1::Script => Ok(Self::Script {
                statement_count: statements.len(),
            }),
            VerifiedRawRootExpansionV1::App(main) => {
                Self::from_main_expansion(statements, main)
            }
        }
    }

    fn from_main_expansion(
        statements: &[ASTNode],
        expansion: &VerifiedMainExpansionV1<'_>,
    ) -> Result<Self, RawSourceProjectionErrorV1> {
        let statement_index = statements
            .iter()
            .position(|statement| {
                matches!(
                    statement,
                    ASTNode::BoxDeclaration {
                        name,
                        is_static: true,
                        ..
                    } if name == expansion.root().box_name()
                )
            })
            .ok_or(RawSourceProjectionErrorV1::MainBoxLocatorMissing)?;
        let box_name = expansion.root().box_name();
        let main = locator(
            statement_index,
            box_name,
            "main",
            expansion.root().source(),
            crate::mir::naming::encode_static_method(
                box_name,
                "main",
                function_arity(expansion.root().source())?,
            ),
        )?;
        let static_children = expansion
            .static_children()
            .iter()
            .map(|child| {
                locator(
                    statement_index,
                    box_name,
                    child.method_name(),
                    child.source(),
                    child.symbol().to_owned(),
                )
            })
            .collect::<Result<Box<[_]>, _>>()?;
        let callable = expansion
            .callable_main_compat()
            .ok_or(RawSourceProjectionErrorV1::CallableMainProjectionMissing)?;
        let callable_main = locator(
            statement_index,
            box_name,
            callable.method_name(),
            callable.source(),
            callable.symbol().to_owned(),
        )?;
        Ok(Self::App {
            main,
            static_children,
            callable_main,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawSourceProjectionErrorV1 {
    RootMustBeProgram,
    MainBoxLocatorMissing,
    MainSourceMustBeFunction,
    CallableMainProjectionMissing,
}

fn locator(
    top_level_statement: usize,
    box_name: &str,
    method_name: &str,
    source: &ASTNode,
    symbol: String,
) -> Result<RawSourceLocatorV1, RawSourceProjectionErrorV1> {
    Ok(RawSourceLocatorV1 {
        top_level_statement,
        box_name: box_name.into(),
        method_name: method_name.into(),
        symbol: symbol.into_boxed_str(),
        arity: function_arity(source)?,
    })
}

fn function_arity(source: &ASTNode) -> Result<usize, RawSourceProjectionErrorV1> {
    let ASTNode::FunctionDeclaration { params, .. } = source else {
        return Err(RawSourceProjectionErrorV1::MainSourceMustBeFunction);
    };
    Ok(params.len())
}

#[derive(Debug)]
pub(in crate::mir) struct OwnedRawSourceV1 {
    ast: ASTNode,
    origin: RawSourceOriginV1,
    projection: OwnedRawRootProjectionV1,
}

impl OwnedRawSourceV1 {
    pub(in crate::mir) fn bind(
        ast: ASTNode,
        origin: RawSourceOriginV1,
    ) -> Result<Self, RawSourceProjectionErrorV1> {
        Self::bind_with_owner(ast, origin).map_err(|(_, error)| error)
    }

    pub(in crate::mir) fn bind_with_owner(
        ast: ASTNode,
        origin: RawSourceOriginV1,
    ) -> Result<Self, (ASTNode, RawSourceProjectionErrorV1)> {
        let expansion = match VerifiedRawRootExpansionV1::from_program(&ast) {
            Ok(expansion) => expansion,
            Err(_) => return Err((ast, RawSourceProjectionErrorV1::RootMustBeProgram)),
        };
        let projection = match OwnedRawRootProjectionV1::from_verified(&ast, &expansion) {
            Ok(projection) => projection,
            Err(error) => return Err((ast, error)),
        };
        Ok(Self {
            ast,
            origin,
            projection,
        })
    }

    pub(in crate::mir) fn origin(&self) -> RawSourceOriginV1 {
        self.origin
    }

    pub(in crate::mir) fn projection(&self) -> &OwnedRawRootProjectionV1 {
        &self.projection
    }

    pub(in crate::mir) fn ast(&self) -> &ASTNode {
        &self.ast
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, DeclarationAttrs, Span};
    use std::collections::HashMap;

    fn function(name: &str, arity: usize) -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: name.into(),
            params: (0..arity).map(|index| format!("p{index}")).collect(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: Vec::new(),
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    fn program() -> ASTNode {
        let mut methods = HashMap::new();
        methods.insert("main".into(), function("main", 1));
        methods.insert("helper".into(), function("helper", 2));
        ASTNode::Program {
            statements: vec![ASTNode::BoxDeclaration {
                name: "Main".into(),
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
    fn projection_owns_locators_without_borrowing_the_ast() {
        let source = program();
        let owned = OwnedRawSourceV1::bind(source, RawSourceOriginV1::BareAst).unwrap();
        assert_eq!(owned.origin(), RawSourceOriginV1::BareAst);
        let OwnedRawRootProjectionV1::App {
            main,
            static_children,
            callable_main,
        } = owned.projection()
        else {
            panic!("expected app projection");
        };
        assert_eq!(main.symbol(), "Main.main/1");
        assert_eq!(static_children[0].symbol(), "Main.helper/2");
        assert_eq!(callable_main.method_name(), "main");
        assert_eq!(owned.ast().to_string().is_empty(), false);
    }

    #[test]
    fn script_projection_has_no_synthetic_root_locator() {
        let source = ASTNode::Program {
            statements: vec![ASTNode::Literal {
                value: crate::ast::LiteralValue::Integer(1),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        };
        let owned = OwnedRawSourceV1::bind(source, RawSourceOriginV1::BareAst).unwrap();
        assert!(matches!(
            owned.projection(),
            OwnedRawRootProjectionV1::Script { statement_count: 1 }
        ));
    }
}
