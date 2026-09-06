//! Declaration projection inside the existing exact ordinary-Box syntax loan.
//! No runtime layout, constructor-body analysis or name-based source lookup.

use super::{ASTNode, CanonicalObjectDefinitionV1, InstanceConstructorSemanticBatchIssueV1};
use crate::mir::function::{
    ObjectDestructionDispositionV1 as Destruction,
    ObjectDestructionUnavailableV1 as DestructionUnavailable,
};
use crate::mir::function::{ObjectLayoutUnavailableV1 as Unavailable, UserBoxFieldDecl};

pub(super) fn issue(
    declaration: &ASTNode,
) -> Result<CanonicalObjectDefinitionV1, InstanceConstructorSemanticBatchIssueV1> {
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
        init_fields: _,
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
    // This is an explicit closed source profile, not a runtime-layout result.
    // The exhaustive Box pattern above forces review when a hook/storage role
    // is added to the AST. Method bodies do not issue destruction obligations.
    let destruction = if let Err(reason) = declaration_shape {
        Destruction::Unavailable(DestructionUnavailable::Declaration(reason))
    } else if !weak_fields.is_empty() || field_decls.iter().any(|field| field.is_weak) {
        Destruction::Unavailable(DestructionUnavailable::WeakField)
    } else if field_decls
        .iter()
        .any(|field| field.declared_type_name.as_deref() != Some("i64"))
    {
        Destruction::Unavailable(DestructionUnavailable::FieldType)
    } else if methods.iter_selected_declaration_order().any(|entry| {
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
        .any(|node| !ordinary_member(node, None))
    {
        Destruction::Unavailable(DestructionUnavailable::MemberRole)
    } else {
        Destruction::PlainI64NoHook
    };
    Ok(CanonicalObjectDefinitionV1::from_source_declaration(
        name.as_str().into(),
        field_decls
            .iter()
            .map(|field| UserBoxFieldDecl {
                name: field.name.clone(),
                declared_type_name: field.declared_type_name.clone(),
                is_weak: field.is_weak,
            })
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        declaration_shape,
        destruction,
    ))
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
        ] {
            assert_eq!(
                issue(&declaration(text)).unwrap().destruction_disposition(),
                Destruction::PlainI64NoHook
            );
        }
        for text in ["box Plain { value }", "box Plain { value: StringBox }"] {
            let definition = issue(&declaration(text)).unwrap();
            assert!(definition.local_fields_for_layout().is_ok());
            assert_eq!(
                definition.destruction_disposition(),
                Destruction::Unavailable(DestructionUnavailable::FieldType)
            );
        }
        let mut node = declaration("box Plain { value: i64 }");
        let ASTNode::BoxDeclaration { field_decls, .. } = &mut node else {
            unreachable!()
        };
        field_decls[0].is_weak = true;
        assert_eq!(
            issue(&node).unwrap().destruction_disposition(),
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
            let definition = issue(&declaration(text)).unwrap();
            assert_eq!(definition.diagnostic_name(), "Plain");
            assert_eq!(
                definition.local_fields_for_layout().unwrap(),
                definition.fields()
            );
        }
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
            let definition = issue(&node).unwrap();
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
                issue(&node),
                Err(InstanceConstructorSemanticBatchIssueV1::SourceCoverage)
            ));
        }
    }
}
