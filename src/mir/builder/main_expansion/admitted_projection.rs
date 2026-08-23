//! Owned, pre-effect normal-root projection for the selected source-backed lane.
//!
//! The parser-preserved source relation is projected exactly once before any
//! Builder effect. Later lowering only borrows this owned syntax package; it
//! never reopens the final parser source or reclassifies App versus Script.

use crate::ast::{ASTNode, BoxMethodInventoryOrdinalV1};
use crate::parser::{
    CallableDeclarationIdentityV1, FinalCallableDeclarationModeV1,
    FinalCallableSemanticSyntaxRowRefV1, InitialCallableFinalSlotV1,
    ParserNormalRootExecutionRoleV1, VerifiedFinalCallableProgramSourceV1,
};

use super::{
    MainExpansionErrorV1, MainExpansionSealV1, MainRootBodySealV1, MainStaticChildSealV1,
    MainStaticChildSourceIdentityV1, OwnedVerifiedMainRootLoweringV1,
    OwnedVerifiedMainStaticChildLoweringV1, VerifiedMainExpansionV1, VerifiedMainRootBodyV1,
    VerifiedMainRootPartsV1, VerifiedMainStaticChildPartsV1, VerifiedMainStaticChildV1,
    VerifiedRawRootExpansionV1,
};
use crate::mir::builder::normal_root_execution::NormalRootExecutionProjectionPermitV1;

#[derive(Debug)]
pub(in crate::mir) enum PreparedAdmittedNormalRootExpansionV1 {
    ProgramRuntime,
    App(PreparedAdmittedMainExpansionV1),
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedAdmittedMainExpansionV1 {
    root: PreparedAdmittedMainRootV1,
    static_children: Box<[PreparedAdmittedMainStaticChildV1]>,
    callable_main: PreparedAdmittedMainStaticChildV1,
}

#[derive(Debug)]
struct PreparedAdmittedMainRootV1 {
    source: ASTNode,
    lowering: OwnedVerifiedMainRootLoweringV1,
}

#[derive(Debug)]
struct PreparedAdmittedMainStaticChildV1 {
    method_name: String,
    source: ASTNode,
    statement: u32,
    method: BoxMethodInventoryOrdinalV1,
    identity: CallableDeclarationIdentityV1,
    lowering: OwnedVerifiedMainStaticChildLoweringV1,
}

impl PreparedAdmittedNormalRootExpansionV1 {
    pub(in crate::mir::builder) fn issue(
        source: &VerifiedFinalCallableProgramSourceV1,
        _permit: NormalRootExecutionProjectionPermitV1,
    ) -> Result<Self, MainExpansionErrorV1> {
        with_admitted_normal_root_expansion_v1(source, |expansion| match expansion {
            VerifiedRawRootExpansionV1::Script => Ok(Self::ProgramRuntime),
            VerifiedRawRootExpansionV1::App(main) => {
                PreparedAdmittedMainExpansionV1::from_verified(&main).map(Self::App)
            }
        })?
    }

    pub(in crate::mir) const fn is_app_mode(&self) -> bool {
        matches!(self, Self::App(_))
    }

    /// Consume the already-owned exact projection into the existing lowerer.
    ///
    /// This operation is infallible and does not touch the parser source. The
    /// callback-scoped view cannot outlive this affine owner, and no caller
    /// can repeat the lowering loan.
    pub(in crate::mir::builder) fn consume_lowering_view_once<R>(
        self,
        callback: impl for<'source> FnOnce(VerifiedRawRootExpansionV1<'source>) -> R,
    ) -> R {
        match self {
            Self::ProgramRuntime => callback(VerifiedRawRootExpansionV1::Script),
            Self::App(main) => main.consume_lowering_view_once(callback),
        }
    }

    pub(in crate::mir) fn discard_unconnected(self) {
        match self {
            Self::ProgramRuntime => {}
            Self::App(main) => drop(main),
        }
    }
}

/// Project lowering syntax from the already-admitted parser relation.
///
/// This projector is private to the sole normal-root consumer module. Callable
/// identities select the exact rows; names and ordinals are syntax/placement
/// data only and never decide App versus ProgramRuntime.
fn with_admitted_normal_root_expansion_v1<R>(
    source: &VerifiedFinalCallableProgramSourceV1,
    callback: impl for<'src> FnOnce(VerifiedRawRootExpansionV1<'src>) -> R,
) -> Result<R, MainExpansionErrorV1> {
    let root = source
        .normal_root_execution()
        .ready_source()
        .ok_or(MainExpansionErrorV1::PreservedRootUnavailable)?;
    match root.role() {
        ParserNormalRootExecutionRoleV1::ProgramRuntime => {
            if root.app_relation().is_some() {
                return Err(MainExpansionErrorV1::RootRelationMismatch);
            }
            Ok(callback(VerifiedRawRootExpansionV1::Script))
        }
        ParserNormalRootExecutionRoleV1::App => {
            let relation = root
                .app_relation()
                .ok_or(MainExpansionErrorV1::RootRelationMismatch)?;
            source
                .with_callable_semantic_syntax(|loan| {
                    let main = exact_callable_identity_row(loan.rows(), relation.main_callable())?;
                    let (main_statement, _) = exact_main_box_slot(main)?;
                    if main_statement != relation.main_statement() {
                        return Err(MainExpansionErrorV1::RootRelationMismatch);
                    }
                    let ASTNode::FunctionDeclaration {
                        params,
                        param_decls,
                        return_type_name,
                        body,
                        uses,
                        attrs,
                        ..
                    } = main.declaration()
                    else {
                        return Err(MainExpansionErrorV1::MainMethodMustBeFunction);
                    };
                    let box_name = main
                        .owner_name()
                        .ok_or(MainExpansionErrorV1::RootRelationMismatch)?;
                    let mut static_children = Vec::with_capacity(relation.static_children().len());
                    for identity in relation.static_children() {
                        let row = exact_callable_identity_row(loan.rows(), identity)?;
                        static_children
                            .push(admitted_main_static_child(row, relation.main_statement())?);
                    }
                    let callable_main_compat =
                        Some(admitted_main_static_child(main, relation.main_statement())?);
                    Ok(callback(VerifiedRawRootExpansionV1::App(
                        VerifiedMainExpansionV1 {
                            root: VerifiedMainRootBodyV1 {
                                box_name,
                                source: main.declaration(),
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
                        },
                    )))
                })
                .map_err(MainExpansionErrorV1::ParserSyntax)?
        }
    }
}

fn exact_callable_identity_row<'row, 'source>(
    rows: &'row [FinalCallableSemanticSyntaxRowRefV1<'source>],
    identity: &CallableDeclarationIdentityV1,
) -> Result<&'row FinalCallableSemanticSyntaxRowRefV1<'source>, MainExpansionErrorV1> {
    let mut matches = rows.iter().filter(|row| row.identity().same_as(identity));
    let row = matches
        .next()
        .ok_or(MainExpansionErrorV1::CallableIdentityMissing)?;
    if matches.next().is_some() {
        return Err(MainExpansionErrorV1::CallableIdentityDuplicate);
    }
    Ok(row)
}

fn exact_main_box_slot(
    row: &FinalCallableSemanticSyntaxRowRefV1<'_>,
) -> Result<(u32, BoxMethodInventoryOrdinalV1), MainExpansionErrorV1> {
    if row.mode() != FinalCallableDeclarationModeV1::StaticBoxMethod {
        return Err(MainExpansionErrorV1::StaticChildMustBeStatic {
            method: "<parser-identity>".to_owned(),
        });
    }
    let InitialCallableFinalSlotV1::BoxMethod { statement, method } = row.final_slot() else {
        return Err(MainExpansionErrorV1::RootRelationMismatch);
    };
    Ok((statement, method))
}

fn admitted_main_static_child<'row>(
    row: &'row FinalCallableSemanticSyntaxRowRefV1<'_>,
    expected_statement: u32,
) -> Result<VerifiedMainStaticChildV1<'row>, MainExpansionErrorV1> {
    let (statement, method) = exact_main_box_slot(row)?;
    if statement != expected_statement {
        return Err(MainExpansionErrorV1::RootRelationMismatch);
    }
    let ASTNode::FunctionDeclaration {
        name,
        params,
        param_decls,
        return_type_name,
        body,
        uses,
        attrs,
        ..
    } = row.declaration()
    else {
        return Err(MainExpansionErrorV1::MainMethodMustBeFunction);
    };
    let box_name = row
        .owner_name()
        .ok_or(MainExpansionErrorV1::RootRelationMismatch)?;
    Ok(VerifiedMainStaticChildV1 {
        method_name: name,
        source: row.declaration(),
        statement,
        method,
        parts: VerifiedMainStaticChildPartsV1 {
            params,
            param_decls,
            return_type_name: return_type_name.as_deref(),
            body,
            uses,
            attrs,
        },
        symbol: crate::mir::naming::encode_static_method(box_name, name, params.len())
            .into_boxed_str(),
        arity: params.len(),
        identity: MainStaticChildSourceIdentityV1::Parser(row.identity().clone()),
        _seal: MainStaticChildSealV1,
    })
}

impl PreparedAdmittedMainExpansionV1 {
    fn from_verified(main: &VerifiedMainExpansionV1<'_>) -> Result<Self, MainExpansionErrorV1> {
        let static_children = main
            .static_children()
            .iter()
            .map(PreparedAdmittedMainStaticChildV1::from_verified)
            .collect::<Result<Box<[_]>, _>>()?;
        let callable_main = main
            .callable_main_compat()
            .ok_or(MainExpansionErrorV1::RootRelationMismatch)
            .and_then(PreparedAdmittedMainStaticChildV1::from_verified)?;
        Ok(Self {
            root: PreparedAdmittedMainRootV1 {
                source: main.root().source().clone(),
                lowering: main.to_owned_root_lowering(),
            },
            static_children,
            callable_main,
        })
    }

    fn consume_lowering_view_once<R>(
        self,
        callback: impl for<'source> FnOnce(VerifiedRawRootExpansionV1<'source>) -> R,
    ) -> R {
        let static_children = self
            .static_children
            .iter()
            .map(PreparedAdmittedMainStaticChildV1::as_borrowed)
            .collect::<Box<[_]>>();
        let callable_main_compat = Some(self.callable_main.as_borrowed());
        callback(VerifiedRawRootExpansionV1::App(VerifiedMainExpansionV1 {
            root: self.root.as_borrowed(),
            static_children,
            callable_main_compat,
            _seal: MainExpansionSealV1,
        }))
    }
}

impl PreparedAdmittedMainRootV1 {
    fn as_borrowed(&self) -> VerifiedMainRootBodyV1<'_> {
        VerifiedMainRootBodyV1 {
            box_name: &self.lowering.box_name,
            source: &self.source,
            parts: VerifiedMainRootPartsV1 {
                params: &self.lowering.params,
                param_decls: &self.lowering.param_decls,
                return_type_name: self.lowering.return_type_name.as_deref(),
                body: &self.lowering.body,
                uses: &self.lowering.uses,
                attrs: &self.lowering.attrs,
            },
            _seal: MainRootBodySealV1,
        }
    }
}

impl PreparedAdmittedMainStaticChildV1 {
    fn from_verified(child: &VerifiedMainStaticChildV1<'_>) -> Result<Self, MainExpansionErrorV1> {
        Ok(Self {
            method_name: child.method_name().to_owned(),
            source: child.source().clone(),
            statement: child.statement_index(),
            method: child.method_ordinal(),
            identity: child
                .parser_identity()
                .cloned()
                .ok_or(MainExpansionErrorV1::RootRelationMismatch)?,
            lowering: child.to_owned_lowering(),
        })
    }

    fn as_borrowed(&self) -> VerifiedMainStaticChildV1<'_> {
        VerifiedMainStaticChildV1 {
            method_name: &self.method_name,
            source: &self.source,
            statement: self.statement,
            method: self.method,
            parts: VerifiedMainStaticChildPartsV1 {
                params: &self.lowering.params,
                param_decls: &self.lowering.param_decls,
                return_type_name: self.lowering.return_type_name.as_deref(),
                body: &self.lowering.body,
                uses: &self.lowering.uses,
                attrs: &self.lowering.attrs,
            },
            symbol: self.lowering.symbol.clone().into_boxed_str(),
            arity: self.lowering.params.len(),
            identity: MainStaticChildSourceIdentityV1::Parser(self.identity.clone()),
            _seal: MainStaticChildSealV1,
        }
    }
}
