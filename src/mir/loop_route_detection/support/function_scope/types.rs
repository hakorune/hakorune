//! Type definitions for function scope capture

use crate::mir::ValueId;

/// Classification of captured variable (Phase 100 expansion)
///
/// - `Explicit`: Captured for condition/carrier usage (traditional, Phase 200-A/B)
/// - `Pinned`: Read-only local from loop-outer scope, used as receiver in loop body (Phase 100)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CapturedKind {
    /// Traditional capture (condition variables, carriers)
    Explicit,
    /// Phase 100: read-only local (dynamic construction allowed, immutable in loop)
    Pinned,
}

/// A variable captured from function scope for use in loop conditions/body.
///
/// Example: `local digits = "0123456789"` in JsonParser._atoi()
///
/// # Invariants
///
/// - `name`: Variable name as it appears in the source code
/// - `host_id`: MIR ValueId of the original definition in the host function
/// - `is_immutable`: True if the variable is never reassigned in the function
/// - `kind`: Classification as Explicit (traditional) or Pinned (Phase 100 read-only)
#[derive(Debug, Clone)]
pub struct CapturedVar {
    /// Variable name (e.g., "digits", "table", "s")
    pub name: String,

    /// MIR ValueId of the original definition in the host function
    pub host_id: ValueId,

    /// Whether this variable is never reassigned in the function
    ///
    /// Phase 200-B will implement assignment analysis to determine this.
    /// For now, this is always set to true as a conservative default.
    pub is_immutable: bool,

    /// Phase 100: Classification of captured variable
    pub kind: CapturedKind,
}

/// Environment containing function-scoped captured variables.
///
/// Phase 200-A: Type definition only, not yet integrated with ConditionEnv.
/// Phase 200-B: Will be populated by FunctionScopeCaptureAnalyzer and
///               integrated into ConditionEnv via ConditionEnvBuilder v2.
#[derive(Debug, Clone, Default)]
pub struct CapturedEnv {
    /// List of captured variables
    pub vars: Vec<CapturedVar>,
}

impl CapturedEnv {
    /// Create a new empty environment
    pub fn new() -> Self {
        Self { vars: Vec::new() }
    }

    /// Check if the environment is empty
    pub fn is_empty(&self) -> bool {
        self.vars.is_empty()
    }

    /// Add a captured variable to the environment
    pub fn add_var(&mut self, var: CapturedVar) {
        self.vars.push(var);
    }

    /// Add an explicit captured variable (traditional capture)
    pub fn insert(&mut self, name: String, host_id: ValueId) {
        self.add_var(CapturedVar {
            name,
            host_id,
            is_immutable: true,
            kind: CapturedKind::Explicit,
        });
    }

    /// Add a pinned captured variable (Phase 100: read-only local)
    pub fn insert_pinned(&mut self, name: String, host_id: ValueId) {
        self.add_var(CapturedVar {
            name,
            host_id,
            is_immutable: true,
            kind: CapturedKind::Pinned,
        });
    }

    /// Look up a captured variable by name
    ///
    /// Returns `Some(&CapturedVar)` if found, `None` otherwise.
    pub fn get(&self, name: &str) -> Option<&CapturedVar> {
        self.vars.iter().find(|v| v.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_captured_env_empty() {
        let env = CapturedEnv::new();
        assert!(env.is_empty());
        assert!(env.get("digits").is_none());
    }

    #[test]
    fn test_captured_env_add_and_get() {
        let mut env = CapturedEnv::new();
        env.add_var(CapturedVar {
            name: "digits".to_string(),
            host_id: ValueId(42),
            is_immutable: true,
            kind: CapturedKind::Explicit,
        });

        assert!(!env.is_empty());
        let var = env.get("digits").unwrap();
        assert_eq!(var.name, "digits");
        assert_eq!(var.host_id, ValueId(42));
        assert!(var.is_immutable);
        assert_eq!(var.kind, CapturedKind::Explicit);
    }

    #[test]
    fn test_captured_env_multiple_vars() {
        let mut env = CapturedEnv::new();
        env.add_var(CapturedVar {
            name: "digits".to_string(),
            host_id: ValueId(42),
            is_immutable: true,
            kind: CapturedKind::Explicit,
        });
        env.add_var(CapturedVar {
            name: "table".to_string(),
            host_id: ValueId(100),
            is_immutable: true,
            kind: CapturedKind::Pinned,
        });

        assert_eq!(env.vars.len(), 2);
        assert!(env.get("digits").is_some());
        assert!(env.get("table").is_some());
        assert!(env.get("nonexistent").is_none());
    }

    #[test]
    fn test_captured_env_insert_explicit() {
        let mut env = CapturedEnv::new();
        env.insert("x".to_string(), ValueId(10));

        let var = env.get("x").unwrap();
        assert_eq!(var.kind, CapturedKind::Explicit);
    }

    #[test]
    fn test_captured_env_insert_pinned() {
        let mut env = CapturedEnv::new();
        env.insert_pinned("s".to_string(), ValueId(20));

        let var = env.get("s").unwrap();
        assert_eq!(var.kind, CapturedKind::Pinned);
    }
}
