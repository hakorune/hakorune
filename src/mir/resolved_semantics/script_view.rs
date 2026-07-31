//! Script-only syntax view.
//!
//! FunctionSyntaxViewV1 deliberately remains Function/Lambda-only. This
//! sibling carries the Program root contract without manufacturing a
//! FunctionDeclaration or depending on builder work-plan types.

use crate::ast::{ASTNode, LiteralValue, Span};

use super::owner_root_profile::SemanticOwnerRootProfileV1;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ScriptSyntaxViewV1<'a> {
    program: &'a ASTNode,
    body: &'a [ASTNode],
}

impl<'a> ScriptSyntaxViewV1<'a> {
    pub(crate) fn from_program(program: &'a ASTNode) -> Option<Self> {
        let ASTNode::Program { statements, .. } = program else {
            return None;
        };
        Some(Self {
            program,
            body: statements,
        })
    }

    pub(crate) const fn program(self) -> &'a ASTNode {
        self.program
    }

    pub(crate) const fn body(self) -> &'a [ASTNode] {
        self.body
    }

    pub(crate) const fn root_profile(self) -> SemanticOwnerRootProfileV1 {
        SemanticOwnerRootProfileV1::Script
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn script_view_accepts_only_program_root() {
        let program = ASTNode::Program {
            statements: Vec::new(),
            span: Span::unknown(),
        };
        let view = ScriptSyntaxViewV1::from_program(&program).unwrap();
        assert!(view.body().is_empty());
        assert_eq!(view.root_profile(), SemanticOwnerRootProfileV1::Script);
        assert!(ScriptSyntaxViewV1::from_program(&ASTNode::Literal {
            value: LiteralValue::Integer(0),
            span: Span::unknown(),
        })
        .is_none());
    }
}
