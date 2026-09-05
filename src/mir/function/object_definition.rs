//! Source-issued object definitions inside the unpublished module candidate.
//! Runtime layout allocation remains owned by the later typed-object planner.

use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};
use super::{MirModule, UserBoxFieldDecl};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CanonicalObjectDefinitionV1 {
    diagnostic_name: Box<str>,
    fields: Box<[UserBoxFieldDecl]>,
}

impl CanonicalObjectDefinitionV1 {
    pub(in crate::mir) fn from_source_declaration(
        diagnostic_name: Box<str>, fields: Box<[UserBoxFieldDecl]>,
    ) -> Self {
        Self { diagnostic_name, fields }
    }

    pub(crate) fn diagnostic_name(&self) -> &str { &self.diagnostic_name }
    pub(crate) fn fields(&self) -> &[UserBoxFieldDecl] { &self.fields }
}

impl MirModule {
    pub(crate) fn canonical_object_definition(
        &self, id: CanonicalObjectIdV1,
    ) -> Option<&CanonicalObjectDefinitionV1> {
        self.canonical_object_definitions.as_deref()?.get(id.declaration_index() as usize)
    }

    pub(crate) fn canonical_field_definition(
        &self, field: CanonicalFieldRefV1,
    ) -> Option<&UserBoxFieldDecl> {
        self.canonical_object_definition(field.object())?.fields()
            .get(field.declaration_ordinal() as usize)
    }

    pub(in crate::mir) fn preflight_object_definition_install(&self) -> Result<(), String> {
        if self.canonical_object_definitions.is_some() {
            return Err("[freeze:contract][mir/object-definitions/already-installed]".into());
        }
        Ok(())
    }

    pub(in crate::mir) fn install_object_definitions_preflighted(
        &mut self, definitions: Box<[CanonicalObjectDefinitionV1]>,
    ) {
        debug_assert!(self.canonical_object_definitions.is_none());
        self.canonical_object_definitions = Some(definitions);
    }
}
