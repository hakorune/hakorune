//! Declaration projection inside the existing exact ordinary-Box syntax loan.
//! No runtime layout, constructor-body analysis or name-based source lookup.

use super::{ASTNode, CanonicalObjectDefinitionV1, InstanceConstructorSemanticBatchIssueV1};
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
        ..
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
    let local_layout = if !extends.is_empty() {
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
        local_layout,
    ))
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
