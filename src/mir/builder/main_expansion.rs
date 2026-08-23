//! HEADERPORT0-I0-MAINROLE0-S0: source-only Main expansion.
//!
//! `Main` is a source declaration which expands into a root entry, ordinary
//! static children, and an optional callable compatibility child.  This
//! product owns only source references and deterministic symbols.  It has no
//! Builder, collector, ValueId, metadata, header cache, or publication route.

mod admitted_projection;

pub(in crate::mir) use admitted_projection::PreparedAdmittedNormalRootExpansionV1;

use crate::ast::{ASTNode, BoxMethodInventoryOrdinalV1, DeclarationAttrs, ParamDecl};
use crate::parser::{CallableDeclarationIdentityV1, FinalCallableSemanticSyntaxLoanErrorV1};

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
    PreservedRootUnavailable,
    RootRelationMismatch,
    CallableIdentityMissing,
    CallableIdentityDuplicate,
    ParserSyntax(FinalCallableSemanticSyntaxLoanErrorV1),
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
    identity: MainStaticChildSourceIdentityV1,
    _seal: MainStaticChildSealV1,
}

#[derive(Debug)]
enum MainStaticChildSourceIdentityV1 {
    Parser(CallableDeclarationIdentityV1),
    Compatibility,
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

    pub(in crate::mir) fn parser_identity(&self) -> Option<&CallableDeclarationIdentityV1> {
        match &self.identity {
            MainStaticChildSourceIdentityV1::Parser(identity) => Some(identity),
            MainStaticChildSourceIdentityV1::Compatibility => None,
        }
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

/// Borrowed lowering projection for either an admitted source-backed root or
/// the isolated compatibility classifier.
///
/// This type is not the App/ProgramRuntime authority. Production source-backed
/// callers project it from the parser-preserved relation; only compatibility
/// callers may construct it by scanning a raw Program.
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

    /// Derive the execution-mode bit from an already selected projection.
    /// Source-backed callers have already consumed the parser authority;
    /// compatibility callers are explicitly isolated at their front door.
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
                identity: MainStaticChildSourceIdentityV1::Compatibility,
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
            identity: MainStaticChildSourceIdentityV1::Compatibility,
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
pub(in crate::mir) fn with_test_main_static_children<R>(
    root: PreparedAdmittedNormalRootExpansionV1,
    callback: impl for<'src> FnOnce(&[VerifiedMainStaticChildV1<'src>]) -> R,
) -> Result<R, MainExpansionErrorV1> {
    root.consume_lowering_view_once(|expansion| {
        let VerifiedRawRootExpansionV1::App(expansion) = expansion else {
            return Err(MainExpansionErrorV1::MainBoxMissing);
        };
        Ok(callback(expansion.static_children()))
    })
}

#[cfg(test)]
#[path = "main_expansion_tests.rs"]
mod tests;
