use crate::ast::ASTNode;
use crate::mir::builder::control_flow::cleanup::policies::cond_prelude_vocab::{
    classify_cond_prelude_stmt, CondPreludeStmtKind,
};

pub(super) enum StmtOnlyPreludeView<'a> {
    Assignment {
        target: &'a ASTNode,
        value: &'a ASTNode,
    },
    If {
        condition: &'a ASTNode,
        then_body: &'a [ASTNode],
        else_body: Option<&'a [ASTNode]>,
    },
    Loop,
    Local {
        variables: &'a [String],
        initial_values: &'a [Option<Box<ASTNode>>],
    },
    MethodCall(&'a ASTNode),
    FunctionCall(&'a ASTNode),
    Print {
        expression: &'a ASTNode,
    },
}

pub(super) fn stmt_only_prelude_view(stmt: &ASTNode) -> Option<StmtOnlyPreludeView<'_>> {
    match classify_cond_prelude_stmt(stmt)? {
        CondPreludeStmtKind::Assignment => {
            let ASTNode::Assignment { target, value, .. } = stmt else {
                return None;
            };
            Some(StmtOnlyPreludeView::Assignment {
                target: target.as_ref(),
                value: value.as_ref(),
            })
        }
        CondPreludeStmtKind::If => {
            let ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } = stmt
            else {
                return None;
            };
            Some(StmtOnlyPreludeView::If {
                condition: condition.as_ref(),
                then_body,
                else_body: else_body.as_deref(),
            })
        }
        CondPreludeStmtKind::Loop => Some(StmtOnlyPreludeView::Loop),
        CondPreludeStmtKind::Local => {
            let ASTNode::Local {
                variables,
                initial_values,
                ..
            } = stmt
            else {
                return None;
            };
            Some(StmtOnlyPreludeView::Local {
                variables,
                initial_values,
            })
        }
        CondPreludeStmtKind::MethodCall => Some(StmtOnlyPreludeView::MethodCall(stmt)),
        CondPreludeStmtKind::FunctionCall => Some(StmtOnlyPreludeView::FunctionCall(stmt)),
        CondPreludeStmtKind::Print => {
            let ASTNode::Print { expression, .. } = stmt else {
                return None;
            };
            Some(StmtOnlyPreludeView::Print {
                expression: expression.as_ref(),
            })
        }
    }
}
