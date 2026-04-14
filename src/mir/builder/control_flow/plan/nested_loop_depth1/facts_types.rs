use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::facts::nested_loop_profile::NestedLoopBodyProfile;
use crate::mir::builder::control_flow::plan::facts::stmt_view::StmtOnlyBlockRecipe;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum NestedLoopDepth1Kind {
    BreakContinuePure,
    NoBreakOrContinuePure,
    MethodCall,
    NoBreakOrContinue,
}

impl NestedLoopDepth1Kind {
    pub fn profile(self) -> NestedLoopBodyProfile {
        match self {
            NestedLoopDepth1Kind::BreakContinuePure => NestedLoopBodyProfile {
                allow_calls: false,
                require_call: false,
                allow_break_in_if: true,
                allow_continue_in_if: true,
                allow_trailing_continue: true,
            },
            NestedLoopDepth1Kind::NoBreakOrContinuePure => NestedLoopBodyProfile {
                allow_calls: false,
                require_call: false,
                allow_break_in_if: false,
                allow_continue_in_if: false,
                allow_trailing_continue: false,
            },
            NestedLoopDepth1Kind::MethodCall => NestedLoopBodyProfile {
                allow_calls: true,
                require_call: true,
                allow_break_in_if: true,
                allow_continue_in_if: true,
                allow_trailing_continue: false,
            },
            NestedLoopDepth1Kind::NoBreakOrContinue => NestedLoopBodyProfile {
                allow_calls: true,
                require_call: true,
                allow_break_in_if: false,
                allow_continue_in_if: false,
                allow_trailing_continue: false,
            },
        }
    }

    pub fn context_name(self) -> &'static str {
        match self {
            NestedLoopDepth1Kind::BreakContinuePure => "<nested-break-continue-pure>",
            NestedLoopDepth1Kind::NoBreakOrContinuePure => "<nested-no-bc-pure>",
            NestedLoopDepth1Kind::MethodCall => "<nested-methodcall>",
            NestedLoopDepth1Kind::NoBreakOrContinue => "<nested>",
        }
    }
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct NestedLoopDepth1Facts {
    pub kind: NestedLoopDepth1Kind,
    pub condition: ASTNode,
    pub body: RecipeBody,
    pub body_stmt_only: Option<StmtOnlyBlockRecipe>,
}
