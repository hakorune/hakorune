use crate::ast::ASTNode;
use crate::parser::cursor::TokenCursor;
use crate::parser::ParseError;

mod precedence;
mod primary;
mod record;

/// TokenCursorを使用した式パーサー（実験的実装）
pub struct ExprParserWithCursor;

impl ExprParserWithCursor {
    /// 式をパース（TokenCursor版）
    pub fn parse_expression(cursor: &mut TokenCursor) -> Result<ASTNode, ParseError> {
        // 式モードで実行（改行を自動的にスキップ）
        cursor.with_expr_mode(|c| {
            c.skip_newlines();
            Self::parse_or_expr(c)
        })
    }
}
