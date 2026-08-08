use crate::{BoxMemberGateSiteV1, BoxMethodCompatibilityOriginV1, BoxMethodSourceSelectionV1};

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
    provenance: DelegateDeclarationProvenanceV1,
    source_member_ordinal: Option<u32>,
}

/// Describes where a delegate declaration came from without granting resolver
/// or generated-method authority.
///
/// Legacy JSON cannot be upgraded to a selected source declaration. Only the
/// Rust parser constructs `ExplicitSource`; compatibility decoders must retain
/// their explicit compatibility origin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DelegateDeclarationProvenanceV1 {
    ExplicitSource {
        selection: BoxMethodSourceSelectionV1,
    },
    CompatibilityOnly {
        origin: BoxMethodCompatibilityOriginV1,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DelegateSelectionErrorV1 {
    CompatibilityCannotBeSelected,
    MissingSourceMemberOrdinal,
}

impl DelegateDecl {
    pub fn explicit_source(
        field_name: String,
        exposes: Vec<DelegateExposeDecl>,
        source_member_ordinal: u32,
    ) -> Self {
        Self {
            field_name,
            exposes,
            provenance: DelegateDeclarationProvenanceV1::ExplicitSource {
                selection: BoxMethodSourceSelectionV1::Direct,
            },
            source_member_ordinal: Some(source_member_ordinal),
        }
    }

    pub fn compatibility_only(
        field_name: String,
        exposes: Vec<DelegateExposeDecl>,
        origin: BoxMethodCompatibilityOriginV1,
    ) -> Self {
        Self {
            field_name,
            exposes,
            provenance: DelegateDeclarationProvenanceV1::CompatibilityOnly { origin },
            source_member_ordinal: None,
        }
    }

    pub const fn provenance(&self) -> &DelegateDeclarationProvenanceV1 {
        &self.provenance
    }

    pub fn explicit_source_selection(&self) -> Option<&BoxMethodSourceSelectionV1> {
        match &self.provenance {
            DelegateDeclarationProvenanceV1::ExplicitSource { selection } => Some(selection),
            DelegateDeclarationProvenanceV1::CompatibilityOnly { .. } => None,
        }
    }

    /// Returns the declaration's exact syntactic Box-member ordinal.
    ///
    /// Compatibility declarations deliberately have no source authority and
    /// therefore return `None`.
    pub const fn source_member_ordinal(&self) -> Option<u32> {
        self.source_member_ordinal
    }

    /// Adds one selected member-gate segment while the branch carrier remains
    /// unpublished. Compatibility declarations fail instead of being promoted.
    pub fn prepend_selected_gate(
        &mut self,
        gate_site: BoxMemberGateSiteV1,
    ) -> Result<(), DelegateSelectionErrorV1> {
        match &mut self.provenance {
            DelegateDeclarationProvenanceV1::ExplicitSource { selection } => {
                let branch_member_ordinal = self
                    .source_member_ordinal
                    .ok_or(DelegateSelectionErrorV1::MissingSourceMemberOrdinal)?;
                selection.prepend_gate(gate_site, branch_member_ordinal);
                self.source_member_ordinal = Some(gate_site.box_member_ordinal());
                Ok(())
            }
            DelegateDeclarationProvenanceV1::CompatibilityOnly { .. } => {
                Err(DelegateSelectionErrorV1::CompatibilityCannotBeSelected)
            }
        }
    }
}

#[cfg(test)]
mod delegate_tests {
    use super::*;

    #[test]
    fn selected_gate_path_uses_exact_member_ordinal_and_rebases_for_parent() {
        let mut declaration = DelegateDecl::explicit_source("inner".into(), Vec::new(), 4);

        declaration
            .prepend_selected_gate(BoxMemberGateSiteV1::from_box_member_ordinal(7))
            .expect("fresh source delegate may enter selected gate");
        declaration
            .prepend_selected_gate(BoxMemberGateSiteV1::from_box_member_ordinal(3))
            .expect("nested selected delegate may enter its parent gate");

        assert_eq!(declaration.source_member_ordinal(), Some(3));
        let Some(BoxMethodSourceSelectionV1::SelectedBuildGate { path }) =
            declaration.explicit_source_selection()
        else {
            panic!("selected delegate must retain its exact gate path")
        };
        assert_eq!(path.len(), 2);
        assert_eq!(path[0].gate_site().box_member_ordinal(), 3);
        assert_eq!(path[0].branch_member_ordinal(), 7);
        assert_eq!(path[1].gate_site().box_member_ordinal(), 7);
        assert_eq!(path[1].branch_member_ordinal(), 4);
    }

    #[test]
    fn compatibility_delegate_cannot_acquire_source_selection() {
        let mut declaration = DelegateDecl::compatibility_only(
            "legacy".into(),
            Vec::new(),
            BoxMethodCompatibilityOriginV1::LegacyJsonV1,
        );

        assert_eq!(
            declaration.prepend_selected_gate(BoxMemberGateSiteV1::from_box_member_ordinal(1)),
            Err(DelegateSelectionErrorV1::CompatibilityCannotBeSelected)
        );
        assert_eq!(declaration.source_member_ordinal(), None);
    }
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
