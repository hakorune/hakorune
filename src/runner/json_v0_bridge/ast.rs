use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(super) struct ProgramV0 {
    pub(super) version: i32,
    pub(super) kind: String,
    pub(super) body: Vec<StmtV0>,
    #[serde(default)]
    pub(super) attrs: ProgramAttrsV0,
    #[serde(default)]
    pub(super) defs: Vec<FuncDefV0>,
    /// Phase 29bq: using alias mappings (alias -> module path)
    /// e.g., {"FuncScannerBox": "lang.compiler.entry.func_scanner.FuncScannerBox"}
    #[serde(default)]
    pub(super) imports: std::collections::BTreeMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub(super) struct ProgramAttrsV0 {
    #[serde(default)]
    pub(super) runes: Vec<RuneAttrV0>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(super) struct RuneAttrV0 {
    pub(super) name: String,
    #[serde(default)]
    pub(super) args: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(super) struct FuncDefV0 {
    pub(super) name: String,
    pub(super) params: Vec<String>,
    pub(super) body: ProgramV0,
    #[serde(default)]
    pub(super) attrs: FuncAttrsV0,
    #[serde(rename = "box")]
    pub(super) box_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub(super) struct FuncAttrsV0 {
    #[serde(default)]
    pub(super) runes: Vec<RuneAttrV0>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub(super) enum StmtV0 {
    Return {
        expr: ExprV0,
    },
    Extern {
        iface: String,
        method: String,
        args: Vec<ExprV0>,
    },
    Expr {
        expr: ExprV0,
    },
    Local {
        name: String,
        expr: ExprV0,
    },
    If {
        cond: ExprV0,
        then: Vec<StmtV0>,
        #[serde(rename = "else", default)]
        r#else: Option<Vec<StmtV0>>,
    },
    Loop {
        cond: ExprV0,
        body: Vec<StmtV0>,
    },
    Throw {
        expr: ExprV0,
    },
    Break,
    Continue,
    Try {
        #[serde(rename = "try")]
        try_body: Vec<StmtV0>,
        #[serde(default)]
        catches: Vec<CatchV0>,
        #[serde(default)]
        finally: Vec<StmtV0>,
    },
    // Internal marker emitted by selfhost parser for DropScope registration.
    // The lowerer normalizes this into nested Try(finally) wrappers before MIR lowering.
    FiniReg {
        #[serde(default)]
        prelude: Vec<StmtV0>,
        #[serde(default)]
        fini: Vec<StmtV0>,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub(super) struct CatchV0 {
    #[serde(rename = "param", default)]
    pub(super) param: Option<String>,
    #[serde(rename = "typeHint", default)]
    pub(super) type_hint: Option<String>,
    #[serde(default)]
    pub(super) body: Vec<StmtV0>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(super) struct MatchArmV0 {
    pub(super) label: String,
    pub(super) expr: ExprV0,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub(super) enum ExprV0 {
    Int {
        value: serde_json::Value,
    },
    Str {
        value: String,
    },
    Bool {
        value: bool,
    },
    Null,
    Binary {
        op: String,
        lhs: Box<ExprV0>,
        rhs: Box<ExprV0>,
    },
    Extern {
        iface: String,
        method: String,
        args: Vec<ExprV0>,
    },
    Compare {
        op: String,
        lhs: Box<ExprV0>,
        rhs: Box<ExprV0>,
    },
    Logical {
        op: String,
        lhs: Box<ExprV0>,
        rhs: Box<ExprV0>,
    },
    Call {
        name: String,
        args: Vec<ExprV0>,
    },
    Method {
        recv: Box<ExprV0>,
        method: String,
        args: Vec<ExprV0>,
    },
    New {
        class: String,
        args: Vec<ExprV0>,
    },
    Var {
        name: String,
    },
    Throw {
        expr: Box<ExprV0>,
    },
    BlockExpr {
        prelude: Vec<StmtV0>,
        /// Stage-B currently emits this as a statement wrapper (`{"type":"Expr","expr":...}`).
        /// Keep it flexible at the schema edge and validate in the lowerer.
        tail: serde_json::Value,
    },
    Ternary {
        cond: Box<ExprV0>,
        then: Box<ExprV0>,
        #[serde(rename = "else")]
        r#else: Box<ExprV0>,
    },
    Match {
        scrutinee: Box<ExprV0>,
        arms: Vec<MatchArmV0>,
        #[serde(rename = "else")]
        r#else: Box<ExprV0>,
    },
}
