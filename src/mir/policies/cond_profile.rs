//! CondProfile (analysis-only, structure-first).
//!
//! C19-A: type-only introduction. No routing, no lowering.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CondSkeleton {
    LoopCond,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CondParam {
    LoopVar(String),
    Bound(BoundExpr),
    Step(StepExpr),
    Cmp(CmpOp),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp {
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundExpr {
    LiteralI64(i64),
    Var(String),
    /// length(var)
    LengthOfVar(String),
    /// length(haystack) - length(needle)
    LengthMinusVar {
        haystack: String,
        needle: String,
    },
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepExpr {
    Delta(i64),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CondProfile {
    pub skeleton: CondSkeleton,
    pub params: Vec<CondParam>,
}

impl CondProfile {
    pub fn new(skeleton: CondSkeleton, params: Vec<CondParam>) -> Self {
        Self { skeleton, params }
    }

    /// Extract loop variable name from CondProfile (SSOT for idx_var).
    /// Returns None if LoopVar parameter is not present.
    pub fn loop_var_name(&self) -> Option<&str> {
        self.params.iter().find_map(|param| {
            if let CondParam::LoopVar(name) = param {
                Some(name.as_str())
            } else {
                None
            }
        })
    }
}
