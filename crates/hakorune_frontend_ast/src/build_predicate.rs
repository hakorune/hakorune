/// Build-time conditional predicate carried by `when`.
///
/// This is intentionally separate from ordinary expression AST. `when` is
/// evaluated from build configuration before resolution/MIR, not at runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildPredicate {
    BuildFlag(String),
    Feature(String),
    TargetEq { key: String, value: String },
    BackendEq { key: String, value: String },
    Not(Box<BuildPredicate>),
    All(Vec<BuildPredicate>),
    Any(Vec<BuildPredicate>),
}
