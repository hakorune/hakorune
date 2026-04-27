use crate::ast::ASTNode;
use crate::mir::builder::control_flow::recipes::{
    refs::{StmtRef, StmtSpan},
    RecipeBody,
};

pub(in crate::mir::builder) enum BodyView<'a> {
    Recipe(&'a RecipeBody),
    Slice(&'a [ASTNode]),
}

impl<'a> BodyView<'a> {
    pub fn len(&self) -> usize {
        match self {
            BodyView::Recipe(body) => body.len(),
            BodyView::Slice(body) => body.len(),
        }
    }

    pub fn get_stmt(&self, stmt_ref: StmtRef) -> Option<&'a ASTNode> {
        match self {
            BodyView::Recipe(body) => body.get_ref(stmt_ref),
            BodyView::Slice(body) => body.get(stmt_ref.index()),
        }
    }

    pub fn get_span(&self, span: StmtSpan) -> Option<&'a [ASTNode]> {
        let (start, end) = span.indices();
        if start > end || end > self.len() {
            return None;
        }
        match self {
            BodyView::Recipe(body) => body.body.get(start..end),
            BodyView::Slice(body) => body.get(start..end),
        }
    }
}
