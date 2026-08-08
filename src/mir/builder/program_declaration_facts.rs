//! Source-only declaration facts for the selected normal Program root.
//!
//! The product preserves source-order declaration updates while keeping AST
//! observation separate from Builder mutation. Static-table facts, instance
//! lifecycle, and callable lowering remain outside this module.

use std::collections::{BTreeSet, HashMap};

use super::compilation_context::CompilationContext;
use super::declaration_order::sorted_method_entries;
use super::static_scalar_facts::{infer_static_scalar_method_fact, StaticScalarMethodFact};
use crate::ast::{ASTNode, BoxMethodInventoryV1, EnumVariantDecl, FieldDecl};
use crate::mir::resolved_semantics::{
    admit_direct_enum_match_v1, EnumMatchAdmissionV1, EnumMatchDemandV1, EnumVariantAdmissionV1,
    EnumVariantDemandV1, FullyExplicitRecordLiteralAdmissionV1, RecordSchemaDemandV1,
};

#[derive(Debug)]
pub(super) struct PreparedNormalProgramDeclarationFactsV1 {
    operations: Box<[NormalProgramDeclarationFactOperationV1]>,
    effective_enum_operations: HashMap<Box<str>, usize>,
    effective_record_operations: HashMap<Box<str>, usize>,
    _seal: PreparedNormalProgramDeclarationFactsSealV1,
}

#[derive(Debug)]
struct PreparedNormalProgramDeclarationFactsSealV1;

#[derive(Debug)]
enum NormalProgramDeclarationFactOperationV1 {
    Brand {
        name: String,
        underlying_type_name: String,
    },
    Enum {
        name: String,
        type_parameters: Vec<String>,
        variants: Vec<EnumVariantDecl>,
    },
    Record {
        name: String,
        type_parameters: Vec<String>,
        field_decls: Vec<FieldDecl>,
    },
    InstanceBox {
        name: String,
        fields: Vec<String>,
        field_decls: Vec<FieldDecl>,
        init_fields: Vec<String>,
        weak_fields: Vec<String>,
    },
    StaticBox {
        name: String,
        scalar_updates: Box<[StaticScalarFactUpdateV1]>,
    },
}

#[derive(Debug)]
struct StaticScalarFactUpdateV1 {
    method_symbol: String,
    fact: Option<StaticScalarMethodFact>,
}

impl PreparedNormalProgramDeclarationFactsV1 {
    pub(super) fn collect(root: &ASTNode) -> Self {
        let mut operations = Vec::new();
        let mut effective_enum_operations = HashMap::new();
        let mut effective_record_operations = HashMap::new();
        collect_operations(
            root,
            &mut operations,
            &mut effective_enum_operations,
            &mut effective_record_operations,
        );
        Self {
            operations: operations.into_boxed_slice(),
            effective_enum_operations,
            effective_record_operations,
            _seal: PreparedNormalProgramDeclarationFactsSealV1,
        }
    }

    pub(super) fn with_record_schema_demand_view<R>(
        &self,
        use_view: impl FnOnce(&dyn RecordSchemaDemandV1) -> R,
    ) -> R {
        use_view(&RecordSchemaDemandViewV1 { facts: self })
    }

    pub(super) fn with_enum_variant_demand_view<R>(
        &self,
        use_view: impl FnOnce(&dyn EnumVariantDemandV1) -> R,
    ) -> R {
        use_view(&EnumVariantDemandViewV1 { facts: self })
    }

    pub(super) fn with_enum_match_demand_view<R>(
        &self,
        use_view: impl FnOnce(&dyn EnumMatchDemandV1) -> R,
    ) -> R {
        use_view(&EnumMatchDemandViewV1 { facts: self })
    }

    pub(super) fn install_into(self, context: &mut CompilationContext) {
        for operation in self.operations.into_vec() {
            match operation {
                NormalProgramDeclarationFactOperationV1::Brand {
                    name,
                    underlying_type_name,
                } => context.register_brand_decl(name, underlying_type_name),
                NormalProgramDeclarationFactOperationV1::Enum {
                    name,
                    type_parameters,
                    variants,
                } => context.register_enum_decl(name, type_parameters, variants),
                NormalProgramDeclarationFactOperationV1::Record {
                    name,
                    type_parameters,
                    field_decls,
                } => context.register_record_decl(name, type_parameters, &field_decls),
                NormalProgramDeclarationFactOperationV1::InstanceBox {
                    name,
                    fields,
                    field_decls,
                    init_fields,
                    weak_fields,
                } => context.register_user_box_declared_fields(
                    name,
                    &fields,
                    &field_decls,
                    &init_fields,
                    &weak_fields,
                ),
                NormalProgramDeclarationFactOperationV1::StaticBox {
                    name,
                    scalar_updates,
                } => {
                    context.register_user_box(name);
                    for update in scalar_updates.into_vec() {
                        context.install_static_scalar_method_fact_update(
                            update.method_symbol,
                            update.fact,
                        );
                    }
                }
            }
        }
    }
}

fn collect_operations(
    node: &ASTNode,
    operations: &mut Vec<NormalProgramDeclarationFactOperationV1>,
    effective_enum_operations: &mut HashMap<Box<str>, usize>,
    effective_record_operations: &mut HashMap<Box<str>, usize>,
) {
    match node {
        ASTNode::Program { statements, .. } => {
            for statement in statements {
                collect_operations(
                    statement,
                    operations,
                    effective_enum_operations,
                    effective_record_operations,
                );
            }
        }
        ASTNode::BrandDeclaration {
            name,
            underlying_type_name,
            ..
        } => operations.push(NormalProgramDeclarationFactOperationV1::Brand {
            name: name.clone(),
            underlying_type_name: underlying_type_name.clone(),
        }),
        ASTNode::EnumDeclaration {
            name,
            variants,
            type_parameters,
            ..
        } => {
            let operation_ordinal = operations.len();
            operations.push(NormalProgramDeclarationFactOperationV1::Enum {
                name: name.clone(),
                type_parameters: type_parameters.clone(),
                variants: variants.clone(),
            });
            effective_enum_operations.insert(name.clone().into_boxed_str(), operation_ordinal);
        }
        ASTNode::BoxDeclaration {
            name,
            fields,
            field_decls,
            methods,
            is_static,
            is_record,
            is_sync,
            init_fields,
            weak_fields,
            type_parameters,
            ..
        } => {
            if *is_sync {
                return;
            }
            if *is_record {
                let operation_ordinal = operations.len();
                operations.push(NormalProgramDeclarationFactOperationV1::Record {
                    name: name.clone(),
                    type_parameters: type_parameters.clone(),
                    field_decls: field_decls.clone(),
                });
                effective_record_operations
                    .insert(name.clone().into_boxed_str(), operation_ordinal);
            } else if *is_static {
                operations.push(NormalProgramDeclarationFactOperationV1::StaticBox {
                    name: name.clone(),
                    scalar_updates: collect_static_scalar_updates(name, methods),
                });
            } else {
                operations.push(NormalProgramDeclarationFactOperationV1::InstanceBox {
                    name: name.clone(),
                    fields: fields.clone(),
                    field_decls: field_decls.clone(),
                    init_fields: init_fields.clone(),
                    weak_fields: weak_fields.clone(),
                });
            }
        }
        _ => {}
    }
}

struct EnumVariantDemandViewV1<'facts> {
    facts: &'facts PreparedNormalProgramDeclarationFactsV1,
}

struct EnumMatchDemandViewV1<'facts> {
    facts: &'facts PreparedNormalProgramDeclarationFactsV1,
}

impl EnumMatchDemandV1 for EnumMatchDemandViewV1<'_> {
    fn admit_direct_enum_match(
        &self,
        enum_name: &str,
        arms: &[crate::ast::EnumMatchArm],
        else_expr: Option<&ASTNode>,
    ) -> Option<EnumMatchAdmissionV1> {
        let operation_ordinal = self.facts.effective_enum_operations.get(enum_name)?;
        let NormalProgramDeclarationFactOperationV1::Enum {
            type_parameters,
            variants,
            ..
        } = self.facts.operations.get(*operation_ordinal)?
        else {
            return None;
        };
        admit_direct_enum_match_v1(type_parameters, variants, arms, else_expr)
    }
}

impl EnumVariantDemandV1 for EnumVariantDemandViewV1<'_> {
    fn admit_direct_variant(
        &self,
        enum_name: &str,
        variant_name: &str,
        arguments: &[ASTNode],
    ) -> Option<EnumVariantAdmissionV1> {
        let operation_ordinal = self.facts.effective_enum_operations.get(enum_name)?;
        let NormalProgramDeclarationFactOperationV1::Enum {
            type_parameters,
            variants,
            ..
        } = self.facts.operations.get(*operation_ordinal)?
        else {
            return None;
        };
        if !type_parameters.is_empty() {
            return None;
        }
        let (tag, variant) = variants
            .iter()
            .enumerate()
            .find(|(_, variant)| variant.name == variant_name)?;
        if variant.requires_compat_payload_box()
            || arguments.len() != variant.payload_arity()
            || arguments.len() > u32::MAX as usize
            || (crate::semantics::option_contract::requires_non_nullish_payload(
                enum_name,
                variant_name,
            ) && crate::mir::builder::exprs_enum_match::enum_variant_arguments_are_statically_nullish_v1(arguments))
        {
            return None;
        }
        Some(EnumVariantAdmissionV1::new(
            u32::try_from(tag).ok()?,
            arguments.len() as u32,
            variant.payload_type_name.clone().map(Into::into),
        ))
    }
}

struct RecordSchemaDemandViewV1<'facts> {
    facts: &'facts PreparedNormalProgramDeclarationFactsV1,
}

impl RecordSchemaDemandV1 for RecordSchemaDemandViewV1<'_> {
    fn admit_fully_explicit_literal(
        &self,
        record_type_name: &str,
        fields: &[(String, ASTNode)],
    ) -> Option<FullyExplicitRecordLiteralAdmissionV1> {
        let operation_ordinal = self
            .facts
            .effective_record_operations
            .get(record_type_name)?;
        let NormalProgramDeclarationFactOperationV1::Record {
            type_parameters,
            field_decls,
            ..
        } = self.facts.operations.get(*operation_ordinal)?
        else {
            return None;
        };
        if !type_parameters.is_empty()
            || field_decls.len() != fields.len()
            || fields.len() > u32::MAX as usize
        {
            return None;
        }
        let declared = field_decls
            .iter()
            .map(|field| field.name.as_str())
            .collect::<BTreeSet<_>>();
        let explicit = fields
            .iter()
            .map(|(name, _)| name.as_str())
            .collect::<BTreeSet<_>>();
        (declared.len() == field_decls.len()
            && explicit.len() == fields.len()
            && declared == explicit)
            .then_some(FullyExplicitRecordLiteralAdmissionV1::new(
                fields.len() as u32
            ))
    }
}

fn collect_static_scalar_updates(
    box_name: &str,
    methods: &BoxMethodInventoryV1,
) -> Box<[StaticScalarFactUpdateV1]> {
    if box_name != "HakoAllocObjectLifecycleFacadeReason" {
        return Box::default();
    }

    sorted_method_entries(methods)
        .into_iter()
        .filter_map(|(method_name, declaration)| {
            let ASTNode::FunctionDeclaration { params, body, .. } = declaration else {
                return None;
            };
            let method_symbol = format!("{box_name}.{method_name}/{}", params.len());
            Some(StaticScalarFactUpdateV1 {
                fact: infer_static_scalar_method_fact(&method_symbol, params, body),
                method_symbol,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{DeclarationAttrs, EnumVariantDecl, LiteralValue, Span};

    fn literal(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn static_method(name: &str, body: Vec<ASTNode>) -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: name.to_owned(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body,
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    fn box_declaration(
        name: &str,
        is_static: bool,
        is_record: bool,
        is_sync: bool,
        fields: Vec<String>,
        field_decls: Vec<FieldDecl>,
        init_fields: Vec<String>,
        weak_fields: Vec<String>,
        methods: HashMap<String, ASTNode>,
    ) -> ASTNode {
        ASTNode::BoxDeclaration {
            name: name.to_owned(),
            fields,
            field_decls,
            public_fields: Vec::new(),
            private_fields: Vec::new(),
            methods: BoxMethodInventoryV1::from_legacy_ast_map(methods),
            constructors: HashMap::new(),
            init_fields,
            weak_fields,
            delegates: Vec::new(),
            invariants: Vec::new(),
            transitions: Vec::new(),
            is_interface: false,
            is_record,
            extends: Vec::new(),
            implements: Vec::new(),
            type_parameters: Vec::new(),
            is_sync,
            is_static,
            static_init: None,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    fn program(statements: Vec<ASTNode>) -> ASTNode {
        ASTNode::Program {
            statements,
            span: Span::unknown(),
        }
    }

    #[test]
    fn source_facts_install_all_indexer_lanes_in_source_order() {
        let record_default = Box::new(literal(9));
        let reason_methods = HashMap::from([(
            "answer".to_owned(),
            static_method(
                "answer",
                vec![ASTNode::Return {
                    value: Some(Box::new(literal(7))),
                    span: Span::unknown(),
                }],
            ),
        )]);
        let root = program(vec![
            ASTNode::BrandDeclaration {
                name: "Token".to_owned(),
                underlying_type_name: "i64".to_owned(),
                span: Span::unknown(),
            },
            ASTNode::EnumDeclaration {
                name: "Option".to_owned(),
                variants: vec![EnumVariantDecl {
                    name: "Only".to_owned(),
                    payload_type_name: None,
                    record_field_decls: Vec::new(),
                    tuple_payload_type_names: Vec::new(),
                }],
                type_parameters: vec!["T".to_owned()],
                attrs: DeclarationAttrs::default(),
                span: Span::unknown(),
            },
            box_declaration(
                "Pair",
                false,
                true,
                false,
                Vec::new(),
                vec![FieldDecl {
                    name: "count".to_owned(),
                    declared_type_name: Some("i64".to_owned()),
                    is_weak: false,
                    default_value: Some(record_default),
                }],
                Vec::new(),
                Vec::new(),
                HashMap::new(),
            ),
            box_declaration(
                "Page",
                false,
                false,
                false,
                vec!["title".to_owned()],
                Vec::new(),
                vec!["next".to_owned()],
                vec!["next".to_owned()],
                HashMap::new(),
            ),
            box_declaration(
                "HakoAllocObjectLifecycleFacadeReason",
                true,
                false,
                false,
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                reason_methods,
            ),
            box_declaration(
                "Ignored",
                false,
                false,
                true,
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                HashMap::new(),
            ),
            ASTNode::BrandDeclaration {
                name: "Token".to_owned(),
                underlying_type_name: "u64".to_owned(),
                span: Span::unknown(),
            },
        ]);

        let mut context = CompilationContext::new();
        PreparedNormalProgramDeclarationFactsV1::collect(&root).install_into(&mut context);

        assert_eq!(context.brand_decls["Token"], "u64");
        assert_eq!(context.enum_decls["Option"].variants[0].name, "Only");
        assert_eq!(context.record_decls["Pair"].default_field_names, ["count"]);
        assert!(matches!(
            context.record_field_defaults["Pair"]["count"],
            ASTNode::Literal {
                value: LiteralValue::Integer(9),
                ..
            }
        ));
        assert_eq!(context.user_defined_boxes["Page"], ["title", "next"]);
        assert!(context.user_box_field_decls["Page"][1].is_weak);
        assert!(context
            .static_scalar_method_fact("HakoAllocObjectLifecycleFacadeReason.answer/0")
            .is_some());
        assert!(!context.user_defined_boxes.contains_key("Ignored"));
    }

    #[test]
    fn source_facts_keep_static_scalar_updates_in_source_order() {
        let verified = box_declaration(
            "HakoAllocObjectLifecycleFacadeReason",
            true,
            false,
            false,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            HashMap::from([(
                "answer".to_owned(),
                static_method(
                    "answer",
                    vec![ASTNode::Return {
                        value: Some(Box::new(literal(7))),
                        span: Span::unknown(),
                    }],
                ),
            )]),
        );
        let no_longer_verified = box_declaration(
            "HakoAllocObjectLifecycleFacadeReason",
            true,
            false,
            false,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            HashMap::from([("answer".to_owned(), static_method("answer", Vec::new()))]),
        );

        let mut context = CompilationContext::new();
        PreparedNormalProgramDeclarationFactsV1::collect(&program(vec![
            verified,
            no_longer_verified,
        ]))
        .install_into(&mut context);

        assert!(context
            .static_scalar_method_fact("HakoAllocObjectLifecycleFacadeReason.answer/0")
            .is_none());
    }

    #[test]
    fn record_schema_view_accepts_only_the_effective_fully_explicit_schema() {
        let root = program(vec![
            box_declaration(
                "Pair",
                false,
                true,
                false,
                Vec::new(),
                vec![FieldDecl {
                    name: "first".to_owned(),
                    declared_type_name: Some("i64".to_owned()),
                    is_weak: false,
                    default_value: None,
                }],
                Vec::new(),
                Vec::new(),
                HashMap::new(),
            ),
            box_declaration(
                "Pair",
                false,
                true,
                false,
                Vec::new(),
                vec![FieldDecl {
                    name: "second".to_owned(),
                    declared_type_name: Some("i64".to_owned()),
                    is_weak: false,
                    default_value: Some(Box::new(literal(9))),
                }],
                Vec::new(),
                Vec::new(),
                HashMap::new(),
            ),
        ]);
        let facts = PreparedNormalProgramDeclarationFactsV1::collect(&root);
        facts.with_record_schema_demand_view(|schemas| {
            assert!(schemas
                .admit_fully_explicit_literal("Pair", &[("second".to_owned(), literal(2))],)
                .is_some());
            assert!(schemas
                .admit_fully_explicit_literal("Pair", &[("first".to_owned(), literal(1))])
                .is_none());
            assert!(schemas.admit_fully_explicit_literal("Pair", &[]).is_none());
        });
    }
}
