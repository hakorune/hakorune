/// Explicit method exposure carried by `delegate <field> exposes { ... }`.
///
/// Stage0 owns only parser/transport. Collision checks and forwarding method
/// generation are Stage1 responsibilities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegateExposeDecl {
    pub source_name: String,
    pub exposed_name: String,
}

/// Box-level delegation metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegateDecl {
    pub field_name: String,
    pub exposes: Vec<DelegateExposeDecl>,
}

/// Box-level lifecycle transition metadata.
///
/// Stage0 owns only parser/transport. Transition legality, enum validation,
/// and lifecycle verifier facts are Stage1 responsibilities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionDecl {
    pub from_state: String,
    pub to_state: String,
    pub method_name: String,
}

/// Function or constructor parameter declaration metadata.
///
/// `params: Vec<String>` remains the canonical names-only surface for existing
/// AST v0 consumers. This richer shape preserves source type annotations for
/// later exact numeric and verifier rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParamDecl {
    pub name: String,
    pub declared_type_name: Option<String>,
}

impl ParamDecl {
    pub fn names(param_decls: &[ParamDecl]) -> Vec<String> {
        param_decls.iter().map(|decl| decl.name.clone()).collect()
    }

    /// Return the richer parameter declarations when present, or synthesize a
    /// names-only declaration view for older AST v0 inputs that only populated
    /// `params`.
    ///
    /// This keeps the compatibility boundary local to AST data shaping. Callers
    /// should consume the returned `ParamDecl` view instead of reimplementing
    /// their own `param_decls`/`params` selection policy.
    pub fn with_name_fallback<'a>(
        param_decls: &'a [ParamDecl],
        params: &'a [String],
    ) -> std::borrow::Cow<'a, [ParamDecl]> {
        if param_decls.is_empty() && !params.is_empty() {
            std::borrow::Cow::Owned(Self::from_names(params))
        } else {
            std::borrow::Cow::Borrowed(param_decls)
        }
    }

    pub fn from_names(params: &[String]) -> Vec<ParamDecl> {
        params
            .iter()
            .map(|name| ParamDecl {
                name: name.clone(),
                declared_type_name: None,
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractKind {
    Requires,
    Ensures,
}
