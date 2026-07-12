use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PathSegmentV0 {
    Field(PathFieldV0),
    Index(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PathFieldV0 {
    Body,
    Type,
    Expr,
    Cond,
    Then,
    Else,
    Start,
    End,
    Lhs,
    Rhs,
    Recv,
    Args,
    Name,
    Method,
    Field,
    VarName,
    Op,
    Value,
}

impl PathFieldV0 {
    pub const ALL: [Self; 18] = [
        Self::Body,
        Self::Type,
        Self::Expr,
        Self::Cond,
        Self::Then,
        Self::Else,
        Self::Start,
        Self::End,
        Self::Lhs,
        Self::Rhs,
        Self::Recv,
        Self::Args,
        Self::Name,
        Self::Method,
        Self::Field,
        Self::VarName,
        Self::Op,
        Self::Value,
    ];

    pub const fn wire_text(self) -> &'static str {
        match self {
            Self::Body => "body",
            Self::Type => "type",
            Self::Expr => "expr",
            Self::Cond => "cond",
            Self::Then => "then",
            Self::Else => "else",
            Self::Start => "start",
            Self::End => "end",
            Self::Lhs => "lhs",
            Self::Rhs => "rhs",
            Self::Recv => "recv",
            Self::Args => "args",
            Self::Name => "name",
            Self::Method => "method",
            Self::Field => "field",
            Self::VarName => "var_name",
            Self::Op => "op",
            Self::Value => "value",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PathV0 {
    segments: Vec<PathSegmentV0>,
}

impl PathV0 {
    pub fn root_body() -> Self {
        Self {
            segments: vec![PathSegmentV0::Field(PathFieldV0::Body)],
        }
    }

    pub fn field(&self, field: PathFieldV0) -> Self {
        let mut next = self.clone();
        next.segments.push(PathSegmentV0::Field(field));
        next
    }

    pub fn index(&self, index: usize) -> Self {
        let mut next = self.clone();
        next.segments.push(PathSegmentV0::Index(index));
        next
    }

    pub fn segments(&self) -> &[PathSegmentV0] {
        &self.segments
    }
}

impl fmt::Display for PathV0 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("$")?;
        for segment in &self.segments {
            match segment {
                PathSegmentV0::Field(field) => write!(formatter, ".{}", field.wire_text())?,
                PathSegmentV0::Index(index) => write!(formatter, "[{index}]")?,
            }
        }
        Ok(())
    }
}
