//! Source-issued object definitions inside the unpublished module candidate.
//! Runtime layout allocation remains owned by the later typed-object planner.

use super::{MirModule, TypedObjectPlan, UserBoxFieldDecl};
use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};

/// Source declaration restrictions, not construction-body or runtime capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectLayoutUnavailableV1 {
    Inheritance,
    Delegation,
    Synchronization,
    Generic,
    InterfaceImplementation,
    Attributes,
    Contracts,
    StaticInitialization,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CanonicalObjectLayoutUnavailableV1 {
    Declaration(ObjectLayoutUnavailableV1),
    WeakField(u32),
    FieldType(u32),
}

pub(crate) type CanonicalObjectLayoutV1 =
    Result<TypedObjectPlan, CanonicalObjectLayoutUnavailableV1>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CanonicalObjectDefinitionV1 {
    diagnostic_name: Box<str>,
    fields: Box<[UserBoxFieldDecl]>,
    local_layout: Result<(), ObjectLayoutUnavailableV1>,
    runtime_layout: Option<CanonicalObjectLayoutV1>,
}

impl CanonicalObjectDefinitionV1 {
    pub(in crate::mir) fn from_source_declaration(
        diagnostic_name: Box<str>,
        fields: Box<[UserBoxFieldDecl]>,
        local_layout: Result<(), ObjectLayoutUnavailableV1>,
    ) -> Self {
        Self {
            diagnostic_name,
            fields,
            local_layout,
            runtime_layout: None,
        }
    }

    pub(crate) fn diagnostic_name(&self) -> &str {
        &self.diagnostic_name
    }
    pub(crate) fn fields(&self) -> &[UserBoxFieldDecl] {
        &self.fields
    }

    pub(crate) fn runtime_layout(&self) -> Option<&CanonicalObjectLayoutV1> {
        self.runtime_layout.as_ref()
    }

    pub(crate) fn local_fields_for_layout(
        &self,
    ) -> Result<&[UserBoxFieldDecl], ObjectLayoutUnavailableV1> {
        self.local_layout?;
        Ok(&self.fields)
    }
}

impl MirModule {
    /// Called after the planner has prepared and checked the entire allocation.
    pub(in crate::mir) fn install_object_layouts_preflighted(
        &mut self,
        layouts: Vec<CanonicalObjectLayoutV1>,
    ) {
        let Some(definitions) = self.canonical_object_definitions.as_mut() else {
            assert!(layouts.is_empty());
            return;
        };
        assert_eq!(definitions.len(), layouts.len());
        for (definition, layout) in definitions.iter_mut().zip(layouts) {
            match &definition.runtime_layout {
                Some(existing) => assert_eq!(existing, &layout),
                None => definition.runtime_layout = Some(layout),
            }
        }
    }

    pub(crate) fn canonical_object_definitions(&self) -> Option<&[CanonicalObjectDefinitionV1]> {
        self.canonical_object_definitions.as_deref()
    }

    pub(crate) fn canonical_object_definition(
        &self,
        id: CanonicalObjectIdV1,
    ) -> Option<&CanonicalObjectDefinitionV1> {
        self.canonical_object_definitions
            .as_deref()?
            .get(id.declaration_index() as usize)
    }

    pub(crate) fn canonical_field_definition(
        &self,
        field: CanonicalFieldRefV1,
    ) -> Option<&UserBoxFieldDecl> {
        self.canonical_object_definition(field.object())?
            .fields()
            .get(field.declaration_ordinal() as usize)
    }

    pub(in crate::mir) fn preflight_object_definition_install(&self) -> Result<(), String> {
        if self.canonical_object_definitions.is_some() {
            return Err("[freeze:contract][mir/object-definitions/already-installed]".into());
        }
        Ok(())
    }

    pub(in crate::mir) fn install_object_definitions_preflighted(
        &mut self,
        definitions: Box<[CanonicalObjectDefinitionV1]>,
    ) {
        debug_assert!(self.canonical_object_definitions.is_none());
        self.canonical_object_definitions = Some(definitions);
    }
}
