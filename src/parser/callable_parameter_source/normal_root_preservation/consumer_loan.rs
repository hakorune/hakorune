//! Scoped parser-owned view for the already-preserved normal root.
//!
//! The issuer reuses the complete Program authority internally, but never
//! exposes its raw Program getter, source rows, positions, or parser identity.
//! App replaces the admitted root declaration with an opaque `RootMain` item;
//! Script exposes one paired statement cursor.  The HRTB callback prevents all
//! borrowed syntax from escaping this boundary.

use crate::ast::{ASTNode, DeclarationAttrs};

use super::{
    ParserNormalAppRootRelationV1, ParserNormalRootPreservationV1, ParserNormalRootRelationV1,
};
use crate::parser::callable_parameter_source::main_app_entry::ParserMainAppEntryOutsideReasonV1;
use crate::parser::callable_parameter_source::normal_root_source::{
    ParserNormalRootScriptTerminalV1, ParserNormalRootSourceDispositionV1,
    ParserNormalRootSourceIncompleteV1, ParserNormalRootSourceIntegrityIssueV1,
    ParserNormalRootSourceUnavailableV1,
};
use crate::parser::callable_parameter_source::script_source_authority::{
    with_parser_normal_program_source_loan, ParserNormalProgramBodySyntaxKindV1,
    ParserNormalProgramSourceAuthorityDispositionV1,
    ParserNormalProgramSourceAuthorityIncompleteV1,
    ParserNormalProgramSourceAuthorityIntegrityIssueV1,
    ParserNormalProgramSourceAuthorityUnavailableV1, ParserNormalProgramSourceLoanRejectV1,
    ParserNormalProgramSourceLoanV1, ParserNormalProgramStatementCursorV1,
};
use crate::parser::initial_callable_program_source::InitialCallableFinalSlotV1;

#[derive(Debug)]
pub(crate) enum ParserNormalRootConsumerLoanV1<'source> {
    App(ParserNormalAppRootLoanV1<'source>),
    Script(ParserNormalScriptRootLoanV1<'source>),
}

#[derive(Debug)]
pub(crate) struct ParserNormalAppRootLoanV1<'source> {
    root: ParserNormalAppRootBodyLoanV1<'source>,
    program: ParserNormalAppProgramCursorV1<'source>,
    _callable_relation: ParserNormalAppRootCallableRelationRefV1<'source>,
}

#[derive(Debug)]
pub(crate) struct ParserNormalAppRootBodyLoanV1<'source> {
    result_syntax: ParserNormalAppResultSyntaxV1<'source>,
    body: &'source [ASTNode],
    uses: &'source [String],
    attrs: &'source DeclarationAttrs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalAppResultSyntaxV1<'source> {
    Implicit,
    Explicit(&'source str),
}

#[derive(Debug)]
pub(crate) struct ParserNormalAppProgramCursorV1<'source> {
    statements: ParserNormalProgramStatementCursorV1<'source>,
    remaining_before_root: usize,
    root_emitted: bool,
}

#[derive(Debug)]
pub(crate) enum ParserNormalAppProgramItemLoanV1<'source> {
    RootMain,
    Sibling {
        kind: ParserNormalProgramBodySyntaxKindV1,
        statement: &'source ASTNode,
    },
}

#[derive(Debug)]
struct ParserNormalAppRootCallableRelationRefV1<'source> {
    _relation: &'source ParserNormalAppRootRelationV1,
}

#[derive(Debug)]
pub(crate) struct ParserNormalScriptRootLoanV1<'source> {
    statements: ParserNormalScriptStatementCursorV1<'source>,
}

#[derive(Debug)]
pub(crate) struct ParserNormalScriptStatementCursorV1<'source> {
    statements: ParserNormalProgramStatementCursorV1<'source>,
}

#[derive(Debug)]
pub(crate) struct ParserNormalScriptStatementLoanV1<'source> {
    kind: ParserNormalProgramBodySyntaxKindV1,
    statement: &'source ASTNode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalRootConsumerLoanRejectV1 {
    Outside(ParserMainAppEntryOutsideReasonV1),
    ScriptTerminal(ParserNormalRootScriptTerminalV1),
    SourceAuthorityUnavailable(ParserNormalRootConsumerSourceUnavailableV1),
    Incomplete(ParserNormalRootConsumerIncompleteV1),
    IntegrityInvalid(ParserNormalRootConsumerIntegrityIssueV1),
    DiscardedBeforeA,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalRootConsumerSourceUnavailableV1 {
    Root(ParserNormalRootSourceUnavailableV1),
    Program(ParserNormalProgramSourceAuthorityUnavailableV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalRootConsumerIncompleteV1 {
    Root(ParserNormalRootSourceIncompleteV1),
    Program(ParserNormalProgramSourceAuthorityIncompleteV1),
    AppRootStatementMissing,
    AppRootMethodMissing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalRootConsumerIntegrityIssueV1 {
    Root(ParserNormalRootSourceIntegrityIssueV1),
    Program(ParserNormalProgramSourceAuthorityIntegrityIssueV1),
    ParserWitnessMismatch,
    ReadyStoredAsTerminal,
    AppRootSlotMismatch,
    AppRootStatementKindMismatch,
    AppRootDeclarationMismatch,
    AppRootMethodMustBeFunction,
    AppRootMethodContractMismatch,
}

pub(crate) fn with_parser_normal_root_consumer_loan<R>(
    root: &ParserNormalRootPreservationV1,
    source_authority: &ParserNormalProgramSourceAuthorityDispositionV1,
    ast: &ASTNode,
    callback: impl for<'source> FnOnce(ParserNormalRootConsumerLoanV1<'source>) -> R,
) -> Result<R, ParserNormalRootConsumerLoanRejectV1> {
    let ParserNormalRootPreservationV1::Ready(preserved) = root else {
        return Err(map_terminal_root(root));
    };

    with_parser_normal_program_source_loan(source_authority, ast, |program| {
        if !preserved._invocation.same_as(program.invocation_witness()) {
            return Err(ParserNormalRootConsumerLoanRejectV1::IntegrityInvalid(
                ParserNormalRootConsumerIntegrityIssueV1::ParserWitnessMismatch,
            ));
        }

        match &preserved.relation {
            ParserNormalRootRelationV1::App(relation) => {
                let loan = build_app_root_loan(program, relation)?;
                Ok(callback(ParserNormalRootConsumerLoanV1::App(loan)))
            }
            ParserNormalRootRelationV1::Script => {
                let loan = ParserNormalScriptRootLoanV1 {
                    statements: ParserNormalScriptStatementCursorV1 {
                        statements: program.statements(),
                    },
                };
                Ok(callback(ParserNormalRootConsumerLoanV1::Script(loan)))
            }
        }
    })
    .map_err(map_program_loan_reject)?
}

fn build_app_root_loan<'source>(
    program: ParserNormalProgramSourceLoanV1<'source>,
    relation: &'source ParserNormalAppRootRelationV1,
) -> Result<ParserNormalAppRootLoanV1<'source>, ParserNormalRootConsumerLoanRejectV1> {
    let InitialCallableFinalSlotV1::BoxMethod { statement, method } = relation._final_slot else {
        return Err(ParserNormalRootConsumerLoanRejectV1::IntegrityInvalid(
            ParserNormalRootConsumerIntegrityIssueV1::AppRootSlotMismatch,
        ));
    };
    let root_position = statement as usize;
    let mut validation_cursor = program.statements();
    let Some(root_statement) = validation_cursor.nth(root_position) else {
        return Err(ParserNormalRootConsumerLoanRejectV1::Incomplete(
            ParserNormalRootConsumerIncompleteV1::AppRootStatementMissing,
        ));
    };
    if root_statement.source_row().kind() != ParserNormalProgramBodySyntaxKindV1::BoxDeclaration {
        return Err(ParserNormalRootConsumerLoanRejectV1::IntegrityInvalid(
            ParserNormalRootConsumerIntegrityIssueV1::AppRootStatementKindMismatch,
        ));
    }
    let ASTNode::BoxDeclaration {
        methods, is_static, ..
    } = root_statement.statement()
    else {
        return Err(ParserNormalRootConsumerLoanRejectV1::IntegrityInvalid(
            ParserNormalRootConsumerIntegrityIssueV1::AppRootDeclarationMismatch,
        ));
    };
    if !is_static {
        return Err(ParserNormalRootConsumerLoanRejectV1::IntegrityInvalid(
            ParserNormalRootConsumerIntegrityIssueV1::AppRootDeclarationMismatch,
        ));
    }
    let Some(method) = methods
        .iter_selected_declaration_order()
        .nth(method.inventory_ordinal() as usize)
    else {
        return Err(ParserNormalRootConsumerLoanRejectV1::Incomplete(
            ParserNormalRootConsumerIncompleteV1::AppRootMethodMissing,
        ));
    };
    let ASTNode::FunctionDeclaration {
        params,
        param_decls,
        return_type_name,
        body,
        uses,
        is_static,
        attrs,
        ..
    } = method.declaration()
    else {
        return Err(ParserNormalRootConsumerLoanRejectV1::IntegrityInvalid(
            ParserNormalRootConsumerIntegrityIssueV1::AppRootMethodMustBeFunction,
        ));
    };
    if !is_static || !params.is_empty() || !param_decls.is_empty() {
        return Err(ParserNormalRootConsumerLoanRejectV1::IntegrityInvalid(
            ParserNormalRootConsumerIntegrityIssueV1::AppRootMethodContractMismatch,
        ));
    }

    Ok(ParserNormalAppRootLoanV1 {
        root: ParserNormalAppRootBodyLoanV1 {
            result_syntax: match return_type_name.as_deref() {
                Some(name) => ParserNormalAppResultSyntaxV1::Explicit(name),
                None => ParserNormalAppResultSyntaxV1::Implicit,
            },
            body,
            uses,
            attrs,
        },
        program: ParserNormalAppProgramCursorV1 {
            statements: program.statements(),
            remaining_before_root: root_position,
            root_emitted: false,
        },
        _callable_relation: ParserNormalAppRootCallableRelationRefV1 {
            _relation: relation,
        },
    })
}

fn map_program_loan_reject(
    reject: ParserNormalProgramSourceLoanRejectV1,
) -> ParserNormalRootConsumerLoanRejectV1 {
    match reject {
        ParserNormalProgramSourceLoanRejectV1::SourceAuthorityUnavailable(reason) => {
            ParserNormalRootConsumerLoanRejectV1::SourceAuthorityUnavailable(
                ParserNormalRootConsumerSourceUnavailableV1::Program(reason),
            )
        }
        ParserNormalProgramSourceLoanRejectV1::Incomplete(reason) => {
            ParserNormalRootConsumerLoanRejectV1::Incomplete(
                ParserNormalRootConsumerIncompleteV1::Program(reason),
            )
        }
        ParserNormalProgramSourceLoanRejectV1::IntegrityInvalid(reason) => {
            ParserNormalRootConsumerLoanRejectV1::IntegrityInvalid(
                ParserNormalRootConsumerIntegrityIssueV1::Program(reason),
            )
        }
    }
}

fn map_terminal_root(
    root: &ParserNormalRootPreservationV1,
) -> ParserNormalRootConsumerLoanRejectV1 {
    let ParserNormalRootPreservationV1::Terminal(root) = root else {
        return ParserNormalRootConsumerLoanRejectV1::IntegrityInvalid(
            ParserNormalRootConsumerIntegrityIssueV1::ReadyStoredAsTerminal,
        );
    };
    match root {
        ParserNormalRootSourceDispositionV1::Outside(reason) => {
            ParserNormalRootConsumerLoanRejectV1::Outside(*reason)
        }
        ParserNormalRootSourceDispositionV1::ScriptTerminal(reason) => {
            ParserNormalRootConsumerLoanRejectV1::ScriptTerminal(*reason)
        }
        ParserNormalRootSourceDispositionV1::SourceAuthorityUnavailable(reason) => {
            ParserNormalRootConsumerLoanRejectV1::SourceAuthorityUnavailable(
                ParserNormalRootConsumerSourceUnavailableV1::Root(*reason),
            )
        }
        ParserNormalRootSourceDispositionV1::Incomplete(reason) => {
            ParserNormalRootConsumerLoanRejectV1::Incomplete(
                ParserNormalRootConsumerIncompleteV1::Root(*reason),
            )
        }
        ParserNormalRootSourceDispositionV1::IntegrityInvalid(reason) => {
            ParserNormalRootConsumerLoanRejectV1::IntegrityInvalid(
                ParserNormalRootConsumerIntegrityIssueV1::Root(*reason),
            )
        }
        ParserNormalRootSourceDispositionV1::DiscardedBeforeA => {
            ParserNormalRootConsumerLoanRejectV1::DiscardedBeforeA
        }
        ParserNormalRootSourceDispositionV1::AppReady(_)
        | ParserNormalRootSourceDispositionV1::ScriptReady(_) => {
            ParserNormalRootConsumerLoanRejectV1::IntegrityInvalid(
                ParserNormalRootConsumerIntegrityIssueV1::ReadyStoredAsTerminal,
            )
        }
    }
}

impl<'source> ParserNormalAppRootLoanV1<'source> {
    pub(crate) fn root(&self) -> &ParserNormalAppRootBodyLoanV1<'source> {
        &self.root
    }

    pub(crate) fn program_items(&mut self) -> &mut ParserNormalAppProgramCursorV1<'source> {
        &mut self.program
    }
}

impl<'source> ParserNormalAppRootBodyLoanV1<'source> {
    pub(crate) const fn result_syntax(&self) -> ParserNormalAppResultSyntaxV1<'source> {
        self.result_syntax
    }

    pub(crate) const fn body(&self) -> &'source [ASTNode] {
        self.body
    }

    pub(crate) const fn uses(&self) -> &'source [String] {
        self.uses
    }

    pub(crate) const fn attrs(&self) -> &'source DeclarationAttrs {
        self.attrs
    }
}

impl<'source> Iterator for ParserNormalAppProgramCursorV1<'source> {
    type Item = ParserNormalAppProgramItemLoanV1<'source>;

    fn next(&mut self) -> Option<Self::Item> {
        let statement = self.statements.next()?;
        if !self.root_emitted && self.remaining_before_root == 0 {
            self.root_emitted = true;
            return Some(ParserNormalAppProgramItemLoanV1::RootMain);
        }
        if !self.root_emitted {
            self.remaining_before_root -= 1;
        }
        Some(ParserNormalAppProgramItemLoanV1::Sibling {
            kind: statement.source_row().kind(),
            statement: statement.statement(),
        })
    }
}

impl<'source> ParserNormalScriptRootLoanV1<'source> {
    pub(crate) fn statements(&mut self) -> &mut ParserNormalScriptStatementCursorV1<'source> {
        &mut self.statements
    }
}

impl<'source> Iterator for ParserNormalScriptStatementCursorV1<'source> {
    type Item = ParserNormalScriptStatementLoanV1<'source>;

    fn next(&mut self) -> Option<Self::Item> {
        let statement = self.statements.next()?;
        Some(ParserNormalScriptStatementLoanV1 {
            kind: statement.source_row().kind(),
            statement: statement.statement(),
        })
    }
}

impl<'source> ParserNormalScriptStatementLoanV1<'source> {
    pub(crate) const fn kind(&self) -> ParserNormalProgramBodySyntaxKindV1 {
        self.kind
    }

    pub(crate) const fn statement(&self) -> &'source ASTNode {
        self.statement
    }
}
