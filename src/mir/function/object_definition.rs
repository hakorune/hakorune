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

/// Issued from the whole source declaration, independently of construction/layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectDestructionUnavailableV1 {
    Declaration(ObjectLayoutUnavailableV1),
    WeakField,
    FieldType,
    MemberRole,
}

/// Source declaration requirements, independently of concrete field-End proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FieldTeardownRequirementsV1 {
    array: bool,
    object: bool,
    dynamic: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectDestructionDispositionV1 {
    NoHook(FieldTeardownRequirementsV1),
    Unavailable(ObjectDestructionUnavailableV1),
}

impl ObjectDestructionDispositionV1 {
    // Exact static profiles remain bounded projections of the source descriptor.
    pub(crate) const NUMERIC_NO_HOOK: Self = Self::no_hook_fields(false, false, false);
    pub(crate) const ARRAY_FIELDS_NO_HOOK: Self = Self::no_hook_fields(true, false, false);
    pub(crate) const OBJECT_FIELDS_NO_HOOK: Self = Self::no_hook_fields(false, true, false);

    pub(in crate::mir) const fn no_hook_fields(array: bool, object: bool, dynamic: bool) -> Self {
        Self::NoHook(FieldTeardownRequirementsV1 { array, object, dynamic })
    }
    /// Declaration eligibility only; does not grant field initialization or End.
    pub(crate) fn has_no_hook(self) -> bool { matches!(self, Self::NoHook(_)) }

    pub(crate) fn supports_static_no_hook(self) -> bool {
        matches!(self, Self::NoHook(fields) if !fields.dynamic)
    }
    pub(crate) fn supports_static_owned_fields(self) -> bool {
        matches!(self, Self::NoHook(fields) if !fields.dynamic && (fields.array || fields.object))
    }
    pub(crate) fn supports_static_object_fields(self) -> bool {
        matches!(self, Self::NoHook(fields) if !fields.dynamic && fields.object)
    }
    /// `Some` is a separately sealed field inventory, never inferred from NoHook.
    pub(crate) fn supports_static_execution(self, has_children: bool) -> bool {
        self == Self::NUMERIC_NO_HOOK || (self.supports_static_owned_fields() && has_children)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CanonicalObjectDefinitionV1 {
    diagnostic_name: Box<str>,
    fields: Box<[UserBoxFieldDecl]>,
    fixed_array_fields: Box<[CanonicalFieldRefV1]>,
    local_layout: Result<(), ObjectLayoutUnavailableV1>,
    destruction: ObjectDestructionDispositionV1,
    runtime_layout: Option<CanonicalObjectLayoutV1>,
}

impl CanonicalObjectDefinitionV1 {
    pub(in crate::mir) fn from_source_declaration(
        diagnostic_name: Box<str>,
        fields: Box<[UserBoxFieldDecl]>,
        local_layout: Result<(), ObjectLayoutUnavailableV1>,
        destruction: ObjectDestructionDispositionV1,
        fixed_array_fields: Box<[CanonicalFieldRefV1]>,
    ) -> Self {
        Self {
            diagnostic_name,
            fields,
            fixed_array_fields,
            local_layout,
            destruction,
            runtime_layout: None,
        }
    }

    pub(crate) fn diagnostic_name(&self) -> &str {
        &self.diagnostic_name
    }
    pub(crate) fn fields(&self) -> &[UserBoxFieldDecl] {
        &self.fields
    }

    /// Source-qualified Array storage; original declaration spelling is unchanged.
    pub(crate) fn is_array_field(&self, ordinal: usize) -> bool {
        self.fields.get(ordinal).is_some_and(|field| !field.is_weak
            && (field.declared_type_name.as_deref() == Some("ArrayBox")
                || self.fixed_array_fields.iter().any(|field| field.declaration_ordinal() as usize == ordinal)))
    }

    pub(in crate::mir) fn validate_fixed_array_fields(&self, object: CanonicalObjectIdV1) -> Result<(), String> {
        let mut prior = None;
        for field in &self.fixed_array_fields {
            let ordinal = field.declaration_ordinal();
            if field.object() != object || prior.is_some_and(|old| old >= ordinal)
                || self.fields.get(ordinal as usize).is_none_or(|decl| decl.is_weak || decl.declared_type_name.is_some()) {
                return Err("[freeze:contract][mir/object-definition/fixed-array-source-drift]".into());
            }
            prior = Some(ordinal);
        }
        Ok(())
    }

    pub(crate) fn destruction_disposition(&self) -> ObjectDestructionDispositionV1 {
        self.destruction
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

    pub(crate) fn canonical_array_field(&self, field: CanonicalFieldRefV1) -> bool {
        self.canonical_object_definition(field.object()).is_some_and(|definition|
            definition.is_array_field(field.declaration_ordinal() as usize))
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
