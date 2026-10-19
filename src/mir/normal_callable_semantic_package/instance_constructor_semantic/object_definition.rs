//! Declaration projection inside the existing exact ordinary-Box syntax loan.
//! No runtime layout, constructor-body analysis or name-based source lookup.

use super::{
    ASTNode, CanonicalObjectDefinitionV1, CanonicalObjectIdV1,
    InstanceConstructorBirthLookupErrorV1, InstanceConstructorSemanticBatchIssueV1,
    ParserOrdinaryBoxSourceRowV1, VerifiedInstanceConstructorSemanticBatchV1,
};
use crate::mir::function::{
    ObjectDestructionDispositionV1 as Destruction,
    ObjectDestructionUnavailableV1 as DestructionUnavailable,
};
use crate::mir::function::{ObjectLayoutUnavailableV1 as Unavailable, UserBoxFieldDecl};

#[derive(Debug)]
pub(super) struct ObjectDeclarationSourceV1 {
    name: Box<str>,
    fields: Box<[UserBoxFieldDecl]>,
    declaration_shape: Result<(), Unavailable>,
    ordinary_members: bool,
}

pub(super) fn capture_declaration(
    declaration: &ASTNode,
) -> Result<ObjectDeclarationSourceV1, InstanceConstructorSemanticBatchIssueV1> {
    let ASTNode::BoxDeclaration {
        name,
        fields,
        field_decls,
        is_static,
        is_interface,
        is_record,
        extends,
        delegates,
        is_sync,
        type_parameters,
        implements,
        attrs,
        invariants,
        transitions,
        static_init,
        public_fields: _,
        private_fields: _,
        methods,
        constructors,
        init_fields,
        weak_fields,
        span: _,
    } = declaration
    else {
        return Err(InstanceConstructorSemanticBatchIssueV1::SourceCoverage);
    };
    if *is_static
        || *is_interface
        || *is_record
        || fields.len() != field_decls.len()
        || fields
            .iter()
            .zip(field_decls)
            .any(|(name, field)| name != &field.name)
        || u32::try_from(field_decls.len()).is_err()
    {
        return Err(InstanceConstructorSemanticBatchIssueV1::SourceCoverage);
    }
    // Keep the source declaration as the only issuer. `CompilationContext`
    // applies the same order/dedup rule when it normalizes `fields` plus the
    // legacy `init_fields` list; mirror that projection here before the
    // definition crosses the package boundary.
    let mut projected_fields: Vec<UserBoxFieldDecl> = field_decls
        .iter()
        .map(|field| UserBoxFieldDecl {
            name: field.name.clone(),
            declared_type_name: field.declared_type_name.clone(),
            is_weak: field.is_weak,
        })
        .collect();
    for field_name in init_fields {
        if projected_fields
            .iter()
            .any(|field| field.name == *field_name)
        {
            continue;
        }
        projected_fields.push(UserBoxFieldDecl {
            name: field_name.clone(),
            declared_type_name: None,
            is_weak: weak_fields.iter().any(|weak| weak == field_name),
        });
    }
    for field in &mut projected_fields {
        if weak_fields.iter().any(|weak| weak == &field.name) {
            field.is_weak = true;
        }
    }
    // Unsupported declarations retain their identity and source fields. Never
    // interpret a zero-local-field inherited object as a plain empty object.
    let declaration_shape = if !extends.is_empty() {
        Err(Unavailable::Inheritance)
    } else if !delegates.is_empty() {
        Err(Unavailable::Delegation)
    } else if *is_sync {
        Err(Unavailable::Synchronization)
    } else if !type_parameters.is_empty() {
        Err(Unavailable::Generic)
    } else if !implements.is_empty() {
        Err(Unavailable::InterfaceImplementation)
    } else if !attrs.is_empty() {
        Err(Unavailable::Attributes)
    } else if !invariants.is_empty() || !transitions.is_empty() {
        Err(Unavailable::Contracts)
    } else if static_init.is_some() {
        Err(Unavailable::StaticInitialization)
    } else {
        Ok(())
    };
    Ok(ObjectDeclarationSourceV1 {
        name: name.as_str().into(),
        fields: projected_fields.into_boxed_slice(),
        declaration_shape,
        ordinary_members: !(methods.iter_selected_declaration_order().any(|entry| {
            use crate::ast::{
                BoxMethodGeneratedProvenanceV1 as Generated, BoxMethodProvenanceV1 as Provenance,
            };
            !matches!(
                entry.provenance(),
                Provenance::ExplicitSource { .. }
                    | Provenance::Generated(Generated::MacroOrImport { .. })
            ) || !ordinary_member(entry.declaration(), Some(entry.name()))
        }) || constructors
            .values()
            .any(|node| !ordinary_member(node, None))),
    })
}

impl ObjectDeclarationSourceV1 {
    pub(super) fn fields(&self) -> &[UserBoxFieldDecl] { &self.fields }

    pub(super) fn seal(self, fixed_array_fields: Box<[hakorune_mir_defs::CanonicalFieldRefV1]>) -> CanonicalObjectDefinitionV1 {
        let Self {
            name,
            fields: projected_fields,
            declaration_shape,
            ordinary_members,
        } = self;
        let destruction = if let Err(reason) = declaration_shape {
            Destruction::Unavailable(DestructionUnavailable::Declaration(reason))
        } else if projected_fields.iter().any(|field| field.is_weak) {
            Destruction::Unavailable(DestructionUnavailable::WeakField)
        } else if projected_fields.iter().enumerate().any(|(ordinal, field)| {
            !fixed_array_fields.iter().any(|field| field.declaration_ordinal() as usize == ordinal)
                && field.declared_type_name.as_deref() != Some("ArrayBox")
                && field
                    .declared_type_name
                    .as_deref()
                    .and_then(
                        crate::mir::declared_type_storage::exact_numeric_storage_for_declared_type,
                    )
                    .is_none()
                && !is_user_object_field(field)
                && field.declared_type_name.is_some()
        }) {
            Destruction::Unavailable(DestructionUnavailable::FieldType)
        } else if !ordinary_members {
            Destruction::Unavailable(DestructionUnavailable::MemberRole)
        } else {
            let array = !fixed_array_fields.is_empty() || projected_fields.iter()
                .any(|field| field.declared_type_name.as_deref() == Some("ArrayBox"));
            let dynamic = projected_fields.iter().enumerate().any(|(ordinal, field)|
                field.declared_type_name.is_none() && !fixed_array_fields.iter()
                    .any(|field| field.declaration_ordinal() as usize == ordinal));
            Destruction::no_hook_fields(array, projected_fields.iter().any(is_user_object_field), dynamic)
        };
        CanonicalObjectDefinitionV1::from_source_declaration(
            name,
            projected_fields,
            declaration_shape,
            destruction,
            fixed_array_fields,
        )
    }
}

/// A non-builtin, non-scalar declared field type is a user-object class.
/// Builtin boxes (`MapBox` etc.) and untyped fields stay unsupported here.
fn is_user_object_field(field: &UserBoxFieldDecl) -> bool {
    field.declared_type_name.as_deref().is_some_and(|name| {
        name != "ArrayBox"
            && crate::mir::declared_type_storage::exact_numeric_storage_for_declared_type(name)
                .is_none()
            && !crate::box_trait::is_builtin_box(name)
    })
}

fn ordinary_member(node: &ASTNode, expected_name: Option<&str>) -> bool {
    matches!(node, ASTNode::FunctionDeclaration { name, attrs, .. }
        if name != "fini" && attrs.is_empty()
            && expected_name.is_none_or(|expected| expected == name))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn declaration(text: &str) -> ASTNode {
        let ASTNode::Program { mut statements, .. } =
            crate::parser::NyashParser::parse_from_string(text).unwrap()
        else {
            panic!("program")
        };
        statements.remove(0)
    }

    #[test]
    fn destruction_is_explicit_source_profile_not_layout_or_birth_success() {
        for text in [
            "box Plain {}",
            "box Plain { value: i64 }",
            "box Plain { value: i64\nbirth(x) { me.value = x } }",
            "box Plain { value: i64\nhelper() { return 1 } }",
            "box Plain { value: usize }",
            "box Plain { value: u32\ncount: i8\nsize: isize }",
        ] {
            assert_eq!(
                capture_declaration(&declaration(text))
                    .unwrap()
                    .seal(Box::new([]))
                    .destruction_disposition(),
                Destruction::NUMERIC_NO_HOOK
            );
        }
        for text in ["box Plain { value }", "box Plain { value: StringBox }"] {
            let definition = capture_declaration(&declaration(text)).unwrap().seal(Box::new([]));
            assert!(definition.local_fields_for_layout().is_ok());
            assert_eq!(
                definition.destruction_disposition(),
                if text == "box Plain { value }" {
                    Destruction::no_hook_fields(false, false, true)
                } else { Destruction::Unavailable(DestructionUnavailable::FieldType) }
            );
            // A source-declared empty slot is not an executable teardown profile,
            // even if a caller presents an empty child inventory.
            assert!(!definition.destruction_disposition().supports_static_execution(true));
        }
        let mut node = declaration("box Plain { value: i64 }");
        let ASTNode::BoxDeclaration { field_decls, .. } = &mut node else {
            unreachable!()
        };
        field_decls[0].is_weak = true;
        assert_eq!(
            capture_declaration(&node)
                .unwrap()
                .seal(Box::new([]))
                .destruction_disposition(),
            Destruction::Unavailable(DestructionUnavailable::WeakField)
        );
    }

    #[test]
    fn local_layout_does_not_depend_on_birth_or_initializer_execution() {
        for text in [
            "box Plain {}",
            "box Plain { value: i64 }",
            "box Plain { value: i64 = 1 }",
            "box Plain { value: i64\nbirth(x) { me.value = x } }",
        ] {
            let definition = capture_declaration(&declaration(text)).unwrap().seal(Box::new([]));
            assert_eq!(definition.diagnostic_name(), "Plain");
            assert_eq!(
                definition.local_fields_for_layout().unwrap(),
                definition.fields()
            );
        }
    }

    #[test]
    fn legacy_init_fields_project_after_explicit_fields_once_in_source_order() {
        let definition = capture_declaration(&declaration(
            "box Holder { explicit: i64\ninit { count, items, explicit, count } }",
        ))
        .unwrap()
        .seal(Box::new([]));
        let fields = definition.fields();
        assert_eq!(
            fields
                .iter()
                .map(|field| field.name.as_str())
                .collect::<Vec<_>>(),
            ["explicit", "count", "items"]
        );
        assert_eq!(fields[0].declared_type_name.as_deref(), Some("i64"));
        assert!(fields[1].declared_type_name.is_none());
        assert!(fields[2].declared_type_name.is_none());
        assert_eq!(definition.local_fields_for_layout(), Ok(fields));
        assert_eq!(definition.destruction_disposition(), Destruction::no_hook_fields(false, false, true));
        assert!(!definition.destruction_disposition().supports_static_execution(false));
    }

    #[test]
    fn legacy_init_weak_field_is_projected_as_weak_before_destruction_classification() {
        let mut node = declaration("box Holder { init { handle } }");
        let ASTNode::BoxDeclaration { weak_fields, .. } = &mut node else {
            unreachable!()
        };
        weak_fields.push("handle".into());
        let definition = capture_declaration(&node).unwrap().seal(Box::new([]));
        assert_eq!(definition.fields().len(), 1);
        assert!(definition.fields()[0].is_weak);
        assert_eq!(
            definition.destruction_disposition(),
            Destruction::Unavailable(DestructionUnavailable::WeakField)
        );
    }

    #[test]
    fn unsupported_structure_preserves_definition_without_plain_layout() {
        for reason in [
            Unavailable::Inheritance,
            Unavailable::Delegation,
            Unavailable::Synchronization,
            Unavailable::Generic,
            Unavailable::InterfaceImplementation,
            Unavailable::Attributes,
            Unavailable::Contracts,
            Unavailable::StaticInitialization,
        ] {
            // Projection-unit counterexamples; this is not source issuance proof.
            let mut node = declaration("box Plain {}");
            let ASTNode::BoxDeclaration {
                extends,
                delegates,
                is_sync,
                type_parameters,
                implements,
                attrs,
                invariants,
                static_init,
                ..
            } = &mut node
            else {
                unreachable!()
            };
            match reason {
                Unavailable::Inheritance => extends.push("Parent".into()),
                Unavailable::Delegation => delegates.push(
                    crate::ast::DelegateDecl::explicit_source("delegate".into(), Vec::new(), 0),
                ),
                Unavailable::Synchronization => *is_sync = true,
                Unavailable::Generic => type_parameters.push("T".into()),
                Unavailable::InterfaceImplementation => implements.push("Interface".into()),
                Unavailable::Attributes => attrs.runes.push(crate::ast::RuneAttr {
                    name: "Internal".into(),
                    args: Vec::new(),
                }),
                Unavailable::Contracts => invariants.push(declaration("box Contract {}")),
                Unavailable::StaticInitialization => *static_init = Some(Vec::new()),
            }
            let definition = capture_declaration(&node).unwrap().seal(Box::new([]));
            assert_eq!(definition.local_fields_for_layout(), Err(reason));
            assert_eq!(
                definition.destruction_disposition(),
                Destruction::Unavailable(DestructionUnavailable::Declaration(reason))
            );
            assert_eq!(definition.diagnostic_name(), "Plain");
            assert!(definition.fields().is_empty());
        }
    }

    #[test]
    fn foreign_declaration_kind_and_field_inventory_drift_reject() {
        for kind in 0..4 {
            let mut node = declaration("box Plain { value: i64 }");
            let ASTNode::BoxDeclaration {
                is_static,
                is_interface,
                is_record,
                fields,
                ..
            } = &mut node
            else {
                unreachable!()
            };
            match kind {
                0 => *is_static = true,
                1 => *is_interface = true,
                2 => *is_record = true,
                _ => fields.clear(),
            }
            assert!(matches!(
                capture_declaration(&node),
                Err(InstanceConstructorSemanticBatchIssueV1::SourceCoverage)
            ));
        }
    }
}

// Same-source definition loans remain in the declaration owner after transfer.
impl VerifiedInstanceConstructorSemanticBatchV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn field_declared_type_name(
        &self, field: hakorune_mir_defs::CanonicalFieldRefV1,
    ) -> Result<Option<Box<str>>, InstanceConstructorBirthLookupErrorV1> {
        let definitions = self.object_definitions.borrow();
        let definition = definitions.as_ref().ok_or(InstanceConstructorBirthLookupErrorV1::ObjectDefinitionsTransferred)?
            .get(field.object().declaration_index() as usize)
            .ok_or(InstanceConstructorBirthLookupErrorV1::ObjectDefinitionMissing)?;
        Ok(definition.fields().get(field.declaration_ordinal() as usize)
            .ok_or(InstanceConstructorBirthLookupErrorV1::ObjectDefinitionMissing)?
            .declared_type_name.clone().map(String::into_boxed_str))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn is_array_field(
        &self, field: hakorune_mir_defs::CanonicalFieldRefV1,
    ) -> Result<bool, InstanceConstructorBirthLookupErrorV1> {
        let definitions = self.object_definitions.borrow();
        let definition = definitions.as_ref().ok_or(InstanceConstructorBirthLookupErrorV1::ObjectDefinitionsTransferred)?
            .get(field.object().declaration_index() as usize)
            .ok_or(InstanceConstructorBirthLookupErrorV1::ObjectDefinitionMissing)?;
        Ok(definition.is_array_field(field.declaration_ordinal() as usize))
    }

    /// One-way projection while co-sealing New claims, before collector transfer.
    /// `object_for` deliberately remains usable after the payload has moved.
    pub(in crate::mir::normal_callable_semantic_package) fn destruction_for(
        &self,
        source: &ParserOrdinaryBoxSourceRowV1,
    ) -> Result<
        (
            CanonicalObjectIdV1,
            crate::mir::function::ObjectDestructionDispositionV1,
        ),
        InstanceConstructorBirthLookupErrorV1,
    > {
        self.with_source_object_definition(source, |object, definition| {
            (object, definition.destruction_disposition())
        })
    }

    pub(in crate::mir::normal_callable_semantic_package) fn with_source_object_definition<R>(
        &self,
        source: &ParserOrdinaryBoxSourceRowV1,
        consume: impl FnOnce(CanonicalObjectIdV1, &CanonicalObjectDefinitionV1) -> R,
    ) -> Result<R, InstanceConstructorBirthLookupErrorV1> {
        let object = self.object_for(source)?;
        let definitions = self.object_definitions.borrow();
        let definitions = definitions
            .as_ref()
            .ok_or(InstanceConstructorBirthLookupErrorV1::ObjectDefinitionsTransferred)?;
        let definition = definitions
            .get(object.declaration_index() as usize)
            .ok_or(InstanceConstructorBirthLookupErrorV1::ObjectDefinitionMissing)?;
        Ok(consume(object, definition))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn has_pending_object_definitions(
        &self,
    ) -> bool {
        self.object_definitions.borrow().is_some()
    }

    pub(in crate::mir::normal_callable_semantic_package) fn take_object_definitions(
        &self,
    ) -> Option<Box<[CanonicalObjectDefinitionV1]>> {
        self.object_definitions.borrow_mut().take()
    }

    pub(crate) fn object_for(
        &self,
        source: &ParserOrdinaryBoxSourceRowV1,
    ) -> Result<CanonicalObjectIdV1, InstanceConstructorBirthLookupErrorV1> {
        self.object_sources
            .iter()
            .find(|(own, _)| own.same_source_as(source))
            .map(|(_, id)| *id)
            .ok_or(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch)
    }

    /// Source-level field lookup on the owning Box's canonical definition.
    /// Declaration order carries the ordinal; the caller must not resolve a
    /// field by spelling alone without this canonical reference.
    pub(crate) fn field_declaration(
        &self,
        source: &ParserOrdinaryBoxSourceRowV1,
        field: &str,
    ) -> Result<
        Option<(
            hakorune_mir_defs::CanonicalFieldRefV1,
            Option<Box<str>>,
            bool,
        )>,
        InstanceConstructorBirthLookupErrorV1,
    > {
        self.with_source_object_definition(source, |object, definition| {
            definition
                .fields()
                .iter()
                .enumerate()
                .find(|(_, declaration)| declaration.name == field)
                .map(|(ordinal, declaration)| {
                    (
                        hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(
                            object, ordinal,
                        )
                        .expect("field ordinal must fit u32"),
                        declaration
                            .declared_type_name
                            .as_deref()
                            .map(Box::<str>::from),
                        declaration.is_weak,
                    )
                })
        })
    }
}
