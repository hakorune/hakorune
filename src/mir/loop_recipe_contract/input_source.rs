//! Exact source relations for initialized Loop input values.
//!
//! This contract is deliberately neutral: it carries resolver-issued source
//! evidence only and never allocates Recipe, CFG, or physical identities. A
//! producer may assemble private observation DTOs, but downstream consumers
//! receive one complete, move-only set rather than a singular input or an
//! iterator that could silently select a partial view.

use std::collections::BTreeSet;

use crate::mir::resolved_semantics::{
    BindingOriginV1, BindingRefV1, FunctionOwnerIdV1, SourceBindingSiteV1, SourceExprSiteV1,
};

use super::{LoopValueClassV1, LoopValueKeyV1, VerifiedLoopCoreProductV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopInitializedLocalInputSourceRelationV1 {
    declaration: SourceBindingSiteV1,
    initializer: SourceExprSiteV1,
    source_binding: BindingRefV1,
    recipe_value: LoopValueKeyV1,
    class: LoopValueClassV1,
}

impl LoopInitializedLocalInputSourceRelationV1 {
    pub(crate) fn new(
        declaration: SourceBindingSiteV1,
        initializer: SourceExprSiteV1,
        source_binding: BindingRefV1,
        recipe_value: LoopValueKeyV1,
        class: LoopValueClassV1,
    ) -> Self {
        Self {
            declaration,
            initializer,
            source_binding,
            recipe_value,
            class,
        }
    }

    pub(crate) fn declaration(&self) -> &SourceBindingSiteV1 {
        &self.declaration
    }

    pub(crate) fn initializer(&self) -> &SourceExprSiteV1 {
        &self.initializer
    }

    pub(crate) const fn source_binding(&self) -> BindingRefV1 {
        self.source_binding
    }

    pub(crate) const fn recipe_value(&self) -> LoopValueKeyV1 {
        self.recipe_value
    }

    pub(crate) const fn class(&self) -> LoopValueClassV1 {
        self.class
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopInitializedLocalInputSourceSetRejectV1 {
    InputCountMismatch { expected: usize, actual: usize },
    ForeignOwner { binding: BindingRefV1 },
    DuplicateRecipeInput { value: LoopValueKeyV1 },
    ForeignRecipeInput { value: LoopValueKeyV1 },
    MissingRecipeInput { value: LoopValueKeyV1 },
    DuplicateSourceBinding { binding: BindingRefV1 },
    MissingRecipeValue { value: LoopValueKeyV1 },
    ClassMismatch { value: LoopValueKeyV1 },
    MissingCarrier { value: LoopValueKeyV1 },
    DuplicateCarrier { value: LoopValueKeyV1 },
    MissingBindingRelation { binding: super::LoopBindingKeyV1 },
    BindingMismatch { value: LoopValueKeyV1 },
    DeclarationMismatch { value: LoopValueKeyV1 },
    NonLocalDeclaration { value: LoopValueKeyV1 },
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopInitializedLocalInputSourceSetV1 {
    owner: FunctionOwnerIdV1,
    rows: Box<[LoopInitializedLocalInputSourceRelationV1]>,
}

impl VerifiedLoopInitializedLocalInputSourceSetV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn rows(&self) -> &[LoopInitializedLocalInputSourceRelationV1] {
        &self.rows
    }
}

pub(crate) fn issue_initialized_local_input_source_set_v1(
    core: &VerifiedLoopCoreProductV1,
    mut rows: Vec<LoopInitializedLocalInputSourceRelationV1>,
) -> Result<VerifiedLoopInitializedLocalInputSourceSetV1, LoopInitializedLocalInputSourceSetRejectV1>
{
    let recipe = core.recipe().as_recipe();
    if recipe.inputs.len() != rows.len() {
        return Err(
            LoopInitializedLocalInputSourceSetRejectV1::InputCountMismatch {
                expected: recipe.inputs.len(),
                actual: rows.len(),
            },
        );
    }

    let expected_inputs: BTreeSet<_> = recipe.inputs.iter().copied().collect();
    let mut seen_inputs = BTreeSet::new();
    let mut seen_bindings = BTreeSet::new();
    for row in &rows {
        if row.source_binding.owner() != core.owner() {
            return Err(LoopInitializedLocalInputSourceSetRejectV1::ForeignOwner {
                binding: row.source_binding,
            });
        }
        if !seen_inputs.insert(row.recipe_value) {
            return Err(
                LoopInitializedLocalInputSourceSetRejectV1::DuplicateRecipeInput {
                    value: row.recipe_value,
                },
            );
        }
        if !expected_inputs.contains(&row.recipe_value) {
            return Err(
                LoopInitializedLocalInputSourceSetRejectV1::ForeignRecipeInput {
                    value: row.recipe_value,
                },
            );
        }
        if !seen_bindings.insert(row.source_binding) {
            return Err(
                LoopInitializedLocalInputSourceSetRejectV1::DuplicateSourceBinding {
                    binding: row.source_binding,
                },
            );
        }
        let Some(value) = recipe
            .values
            .iter()
            .find(|value| value.key == row.recipe_value)
        else {
            return Err(
                LoopInitializedLocalInputSourceSetRejectV1::MissingRecipeValue {
                    value: row.recipe_value,
                },
            );
        };
        if value.class != row.class {
            return Err(LoopInitializedLocalInputSourceSetRejectV1::ClassMismatch {
                value: row.recipe_value,
            });
        }
        let carriers: Vec<_> = recipe
            .carriers
            .iter()
            .filter(|carrier| carrier.entry_value == row.recipe_value)
            .collect();
        let Some(carrier) = carriers.first() else {
            return Err(LoopInitializedLocalInputSourceSetRejectV1::MissingCarrier {
                value: row.recipe_value,
            });
        };
        if carriers.len() != 1 {
            return Err(
                LoopInitializedLocalInputSourceSetRejectV1::DuplicateCarrier {
                    value: row.recipe_value,
                },
            );
        }
        let bindings: Vec<_> = core
            .binding_relations()
            .iter()
            .filter(|relation| relation.recipe_binding() == carrier.binding)
            .collect();
        let Some(binding) = bindings.first() else {
            return Err(
                LoopInitializedLocalInputSourceSetRejectV1::MissingBindingRelation {
                    binding: carrier.binding,
                },
            );
        };
        if bindings.len() != 1
            || binding.source_binding() != row.source_binding
            || binding.class() != row.class
        {
            return Err(
                LoopInitializedLocalInputSourceSetRejectV1::BindingMismatch {
                    value: row.recipe_value,
                },
            );
        }
        if !matches!(row.declaration, SourceBindingSiteV1::Local { .. }) {
            return Err(
                LoopInitializedLocalInputSourceSetRejectV1::NonLocalDeclaration {
                    value: row.recipe_value,
                },
            );
        }
        if !matches!(binding.declaration(), BindingOriginV1::Source(site) if site == &row.declaration)
        {
            return Err(
                LoopInitializedLocalInputSourceSetRejectV1::DeclarationMismatch {
                    value: row.recipe_value,
                },
            );
        }
    }
    for value in expected_inputs {
        if !seen_inputs.contains(&value) {
            return Err(LoopInitializedLocalInputSourceSetRejectV1::MissingRecipeInput { value });
        }
    }
    rows.sort_by_key(|row| row.recipe_value);
    Ok(VerifiedLoopInitializedLocalInputSourceSetV1 {
        owner: core.owner(),
        rows: rows.into_boxed_slice(),
    })
}
