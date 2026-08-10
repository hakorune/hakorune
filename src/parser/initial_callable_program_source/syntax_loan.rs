use crate::ast::ASTNode;

use super::model::InitialCallableFinalSlotV1;
use crate::parser::callable_source_anchor::{
    CallableDeclarationAnchorV1, PreparedCallableSourceV1,
};

#[derive(Debug, Clone, Copy)]
pub(in crate::parser) struct InitialCallableProgramSyntaxRowRefV1<'syntax> {
    anchor: &'syntax CallableDeclarationAnchorV1,
    source: &'syntax PreparedCallableSourceV1,
    declaration: &'syntax ASTNode,
}

impl<'syntax> InitialCallableProgramSyntaxRowRefV1<'syntax> {
    pub(super) fn from_verified(
        ast: &'syntax ASTNode,
        source: &'syntax PreparedCallableSourceV1,
        slot: InitialCallableFinalSlotV1,
    ) -> Self {
        let declaration = declaration_at(ast, slot);
        Self {
            anchor: source.anchor(),
            source,
            declaration,
        }
    }

    pub(in crate::parser) fn anchor(&self) -> &'syntax CallableDeclarationAnchorV1 {
        self.anchor
    }

    pub(in crate::parser) fn source(&self) -> &'syntax PreparedCallableSourceV1 {
        self.source
    }

    pub(in crate::parser) fn declaration(&self) -> &'syntax ASTNode {
        self.declaration
    }
}

#[derive(Debug)]
pub(in crate::parser) struct InitialCallableProgramSyntaxLoanV1<'syntax> {
    rows: Box<[InitialCallableProgramSyntaxRowRefV1<'syntax>]>,
}

impl<'syntax> InitialCallableProgramSyntaxLoanV1<'syntax> {
    pub(super) fn new(rows: Box<[InitialCallableProgramSyntaxRowRefV1<'syntax>]>) -> Self {
        Self { rows }
    }

    pub(in crate::parser) fn rows(&self) -> &[InitialCallableProgramSyntaxRowRefV1<'syntax>] {
        &self.rows
    }
}

pub(in crate::parser) fn declaration_at(
    ast: &ASTNode,
    slot: InitialCallableFinalSlotV1,
) -> &ASTNode {
    let ASTNode::Program { statements, .. } = ast else {
        unreachable!("verified initial callable source retains a Program")
    };
    match slot {
        InitialCallableFinalSlotV1::TopLevel { statement } => &statements[statement as usize],
        InitialCallableFinalSlotV1::BoxMethod { statement, method } => {
            let ASTNode::BoxDeclaration { methods, .. } = &statements[statement as usize] else {
                unreachable!("verified Box method slot retains a Box declaration")
            };
            methods
                .iter_selected_declaration_order()
                .nth(method.inventory_ordinal() as usize)
                .expect("verified Box method slot remains in range")
                .declaration()
        }
    }
}
