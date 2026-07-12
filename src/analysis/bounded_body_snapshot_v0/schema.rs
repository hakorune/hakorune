use super::path::PathFieldV0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SnapshotLimitsV0 {
    pub max_depth: usize,
    pub max_node_count: usize,
    pub max_children_per_body: usize,
    pub max_arguments: usize,
    pub max_literal_bytes: usize,
    pub max_atom_bytes: usize,
    pub max_total_text_bytes: usize,
}

impl SnapshotLimitsV0 {
    pub const SCHEMA: Self = Self {
        max_depth: 64,
        max_node_count: 32_768,
        max_children_per_body: 2_048,
        max_arguments: 128,
        max_literal_bytes: 65_536,
        max_atom_bytes: 1_024,
        max_total_text_bytes: 4_194_304,
    };
}

pub struct DepthConventionV0;

impl DepthConventionV0 {
    pub const ROOT_BODY_CONTAINER: usize = 0;
    pub const TOP_LEVEL_NODE: usize = 1;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireClassificationV0 {
    Accepted,
    KnownUnsupported,
    SchemaMismatchStop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WireStmtKindV0 {
    Local,
    Expr,
    If,
    Loop,
    LoopRange,
    Return,
    Break,
    Continue,
}

impl WireStmtKindV0 {
    pub const ALL: [Self; 8] = [
        Self::Local,
        Self::Expr,
        Self::If,
        Self::Loop,
        Self::LoopRange,
        Self::Return,
        Self::Break,
        Self::Continue,
    ];

    pub fn from_wire_text(value: &str) -> Option<Self> {
        Some(match value {
            "Local" => Self::Local,
            "Expr" => Self::Expr,
            "If" => Self::If,
            "Loop" => Self::Loop,
            "LoopRange" => Self::LoopRange,
            "Return" => Self::Return,
            "Break" => Self::Break,
            "Continue" => Self::Continue,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WireExprKindV0 {
    Int,
    Str,
    Bool,
    Null,
    Var,
    Binary,
    Compare,
    Logical,
    Call,
    Method,
    Field,
}

impl WireExprKindV0 {
    pub const ALL: [Self; 11] = [
        Self::Int,
        Self::Str,
        Self::Bool,
        Self::Null,
        Self::Var,
        Self::Binary,
        Self::Compare,
        Self::Logical,
        Self::Call,
        Self::Method,
        Self::Field,
    ];

    pub fn from_wire_text(value: &str) -> Option<Self> {
        Some(match value {
            "Int" => Self::Int,
            "Str" => Self::Str,
            "Bool" => Self::Bool,
            "Null" => Self::Null,
            "Var" => Self::Var,
            "Binary" => Self::Binary,
            "Compare" => Self::Compare,
            "Logical" => Self::Logical,
            "Call" => Self::Call,
            "Method" => Self::Method,
            "Field" => Self::Field,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WireNodeKindV0 {
    Stmt(WireStmtKindV0),
    Expr(WireExprKindV0),
}

impl WireNodeKindV0 {
    pub fn atom_schema(self) -> &'static [AtomSpecV0] {
        match self {
            Self::Stmt(WireStmtKindV0::Local) => &ATOM_NAME_TEXT,
            Self::Stmt(WireStmtKindV0::LoopRange) => &ATOM_VAR_NAME_TEXT,
            Self::Stmt(_) => &NO_ATOMS,
            Self::Expr(WireExprKindV0::Int) => &ATOM_VALUE_I64,
            Self::Expr(WireExprKindV0::Str) => &ATOM_VALUE_LITERAL_TEXT,
            Self::Expr(WireExprKindV0::Bool) => &ATOM_VALUE_BOOL,
            Self::Expr(WireExprKindV0::Null) => &ATOM_VALUE_NULL,
            Self::Expr(WireExprKindV0::Var | WireExprKindV0::Call) => &ATOM_NAME_TEXT,
            Self::Expr(WireExprKindV0::Method) => &ATOM_METHOD_TEXT,
            Self::Expr(WireExprKindV0::Field) => &ATOM_FIELD_TEXT,
            Self::Expr(
                WireExprKindV0::Binary | WireExprKindV0::Compare | WireExprKindV0::Logical,
            ) => &ATOM_OP_TEXT,
        }
    }

    pub fn child_schema(self) -> &'static [ChildSpecV0] {
        match self {
            Self::Stmt(WireStmtKindV0::Local | WireStmtKindV0::Expr | WireStmtKindV0::Return) => {
                &CHILD_EXPR
            }
            Self::Stmt(WireStmtKindV0::If) => &CHILD_IF,
            Self::Stmt(WireStmtKindV0::Loop) => &CHILD_LOOP,
            Self::Stmt(WireStmtKindV0::LoopRange) => &CHILD_LOOP_RANGE,
            Self::Stmt(WireStmtKindV0::Break | WireStmtKindV0::Continue) => &NO_CHILDREN,
            Self::Expr(
                WireExprKindV0::Int
                | WireExprKindV0::Str
                | WireExprKindV0::Bool
                | WireExprKindV0::Null
                | WireExprKindV0::Var,
            ) => &NO_CHILDREN,
            Self::Expr(
                WireExprKindV0::Binary | WireExprKindV0::Compare | WireExprKindV0::Logical,
            ) => &CHILD_BINARY_LIKE,
            Self::Expr(WireExprKindV0::Call) => &CHILD_ARGS,
            Self::Expr(WireExprKindV0::Method) => &CHILD_METHOD,
            Self::Expr(WireExprKindV0::Field) => &CHILD_RECV,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AtomKeyV0 {
    Name,
    VarName,
    Value,
    Op,
    Method,
    Field,
}

impl AtomKeyV0 {
    pub const fn wire_text(self) -> &'static str {
        match self {
            Self::Name => "name",
            Self::VarName => "var_name",
            Self::Value => "value",
            Self::Op => "op",
            Self::Method => "method",
            Self::Field => "field",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtomValueKindV0 {
    I64,
    Bool,
    Text,
    Null,
}

impl AtomValueKindV0 {
    pub const fn wire_text(self) -> &'static str {
        match self {
            Self::I64 => "I64",
            Self::Bool => "Bool",
            Self::Text => "Text",
            Self::Null => "Null",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextClassV0 {
    Atom,
    Literal,
}

impl TextClassV0 {
    pub const fn wire_text(self) -> &'static str {
        match self {
            Self::Atom => "Atom",
            Self::Literal => "Literal",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtomSpecV0 {
    pub key: AtomKeyV0,
    pub value_kind: AtomValueKindV0,
    pub text_class: Option<TextClassV0>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildCardinalityV0 {
    One,
    List,
    OptionalList,
}

impl ChildCardinalityV0 {
    pub const fn wire_text(self) -> &'static str {
        match self {
            Self::One => "One",
            Self::List => "List",
            Self::OptionalList => "OptionalList",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChildSpecV0 {
    pub role: ChildRoleV0,
    pub cardinality: ChildCardinalityV0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChildRoleV0 {
    Expr,
    Cond,
    Then,
    Else,
    Body,
    Start,
    End,
    Lhs,
    Rhs,
    Recv,
    Args,
}

impl ChildRoleV0 {
    pub const fn wire_text(self) -> &'static str {
        match self {
            Self::Expr => "expr",
            Self::Cond => "cond",
            Self::Then => "then",
            Self::Else => "else",
            Self::Body => "body",
            Self::Start => "start",
            Self::End => "end",
            Self::Lhs => "lhs",
            Self::Rhs => "rhs",
            Self::Recv => "recv",
            Self::Args => "args",
        }
    }

    pub const fn path_field(self) -> PathFieldV0 {
        match self {
            Self::Expr => PathFieldV0::Expr,
            Self::Cond => PathFieldV0::Cond,
            Self::Then => PathFieldV0::Then,
            Self::Else => PathFieldV0::Else,
            Self::Body => PathFieldV0::Body,
            Self::Start => PathFieldV0::Start,
            Self::End => PathFieldV0::End,
            Self::Lhs => PathFieldV0::Lhs,
            Self::Rhs => PathFieldV0::Rhs,
            Self::Recv => PathFieldV0::Recv,
            Self::Args => PathFieldV0::Args,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperatorV0 {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    BitAnd,
    BitOr,
    BitXor,
    ShiftLeft,
    ShiftRight,
}

impl BinaryOperatorV0 {
    pub const ALL: [Self; 10] = [
        Self::Add,
        Self::Subtract,
        Self::Multiply,
        Self::Divide,
        Self::Modulo,
        Self::BitAnd,
        Self::BitOr,
        Self::BitXor,
        Self::ShiftLeft,
        Self::ShiftRight,
    ];
    pub const fn wire_text(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::Modulo => "%",
            Self::BitAnd => "&",
            Self::BitOr => "|",
            Self::BitXor => "^",
            Self::ShiftLeft => "<<",
            Self::ShiftRight => ">>",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOperatorV0 {
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}

impl CompareOperatorV0 {
    pub const ALL: [Self; 6] = [
        Self::Equal,
        Self::NotEqual,
        Self::Less,
        Self::Greater,
        Self::LessEqual,
        Self::GreaterEqual,
    ];
    pub const fn wire_text(self) -> &'static str {
        match self {
            Self::Equal => "==",
            Self::NotEqual => "!=",
            Self::Less => "<",
            Self::Greater => ">",
            Self::LessEqual => "<=",
            Self::GreaterEqual => ">=",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOperatorV0 {
    And,
    Or,
}

impl LogicalOperatorV0 {
    pub const ALL: [Self; 2] = [Self::And, Self::Or];
    pub const fn wire_text(self) -> &'static str {
        match self {
            Self::And => "&&",
            Self::Or => "||",
        }
    }
}

const NO_ATOMS: [AtomSpecV0; 0] = [];
const ATOM_NAME_TEXT: [AtomSpecV0; 1] = [AtomSpecV0 {
    key: AtomKeyV0::Name,
    value_kind: AtomValueKindV0::Text,
    text_class: Some(TextClassV0::Atom),
}];
const ATOM_VAR_NAME_TEXT: [AtomSpecV0; 1] = [AtomSpecV0 {
    key: AtomKeyV0::VarName,
    value_kind: AtomValueKindV0::Text,
    text_class: Some(TextClassV0::Atom),
}];
const ATOM_VALUE_I64: [AtomSpecV0; 1] = [AtomSpecV0 {
    key: AtomKeyV0::Value,
    value_kind: AtomValueKindV0::I64,
    text_class: None,
}];
const ATOM_VALUE_LITERAL_TEXT: [AtomSpecV0; 1] = [AtomSpecV0 {
    key: AtomKeyV0::Value,
    value_kind: AtomValueKindV0::Text,
    text_class: Some(TextClassV0::Literal),
}];
const ATOM_VALUE_BOOL: [AtomSpecV0; 1] = [AtomSpecV0 {
    key: AtomKeyV0::Value,
    value_kind: AtomValueKindV0::Bool,
    text_class: None,
}];
const ATOM_VALUE_NULL: [AtomSpecV0; 1] = [AtomSpecV0 {
    key: AtomKeyV0::Value,
    value_kind: AtomValueKindV0::Null,
    text_class: None,
}];
const ATOM_OP_TEXT: [AtomSpecV0; 1] = [AtomSpecV0 {
    key: AtomKeyV0::Op,
    value_kind: AtomValueKindV0::Text,
    text_class: Some(TextClassV0::Atom),
}];
const ATOM_METHOD_TEXT: [AtomSpecV0; 1] = [AtomSpecV0 {
    key: AtomKeyV0::Method,
    value_kind: AtomValueKindV0::Text,
    text_class: Some(TextClassV0::Atom),
}];
const ATOM_FIELD_TEXT: [AtomSpecV0; 1] = [AtomSpecV0 {
    key: AtomKeyV0::Field,
    value_kind: AtomValueKindV0::Text,
    text_class: Some(TextClassV0::Atom),
}];

const NO_CHILDREN: [ChildSpecV0; 0] = [];
const CHILD_EXPR: [ChildSpecV0; 1] = [ChildSpecV0 {
    role: ChildRoleV0::Expr,
    cardinality: ChildCardinalityV0::One,
}];
const CHILD_IF: [ChildSpecV0; 3] = [
    ChildSpecV0 {
        role: ChildRoleV0::Cond,
        cardinality: ChildCardinalityV0::One,
    },
    ChildSpecV0 {
        role: ChildRoleV0::Then,
        cardinality: ChildCardinalityV0::List,
    },
    ChildSpecV0 {
        role: ChildRoleV0::Else,
        cardinality: ChildCardinalityV0::OptionalList,
    },
];
const CHILD_LOOP: [ChildSpecV0; 2] = [
    ChildSpecV0 {
        role: ChildRoleV0::Cond,
        cardinality: ChildCardinalityV0::One,
    },
    ChildSpecV0 {
        role: ChildRoleV0::Body,
        cardinality: ChildCardinalityV0::List,
    },
];
const CHILD_LOOP_RANGE: [ChildSpecV0; 3] = [
    ChildSpecV0 {
        role: ChildRoleV0::Start,
        cardinality: ChildCardinalityV0::One,
    },
    ChildSpecV0 {
        role: ChildRoleV0::End,
        cardinality: ChildCardinalityV0::One,
    },
    ChildSpecV0 {
        role: ChildRoleV0::Body,
        cardinality: ChildCardinalityV0::List,
    },
];
const CHILD_BINARY_LIKE: [ChildSpecV0; 2] = [
    ChildSpecV0 {
        role: ChildRoleV0::Lhs,
        cardinality: ChildCardinalityV0::One,
    },
    ChildSpecV0 {
        role: ChildRoleV0::Rhs,
        cardinality: ChildCardinalityV0::One,
    },
];
const CHILD_ARGS: [ChildSpecV0; 1] = [ChildSpecV0 {
    role: ChildRoleV0::Args,
    cardinality: ChildCardinalityV0::List,
}];
const CHILD_METHOD: [ChildSpecV0; 2] = [
    ChildSpecV0 {
        role: ChildRoleV0::Recv,
        cardinality: ChildCardinalityV0::One,
    },
    ChildSpecV0 {
        role: ChildRoleV0::Args,
        cardinality: ChildCardinalityV0::List,
    },
];
const CHILD_RECV: [ChildSpecV0; 1] = [ChildSpecV0 {
    role: ChildRoleV0::Recv,
    cardinality: ChildCardinalityV0::One,
}];
